use std::collections::HashMap;

mod completions;
mod diagnostics;
mod formatting;
mod hover;

use tokio::sync::Mutex;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionOptions, CompletionParams, CompletionResponse,
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentFormattingParams, Hover,
        HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
        InitializedParams, MarkedString, MessageType, OneOf, Position, Range, ServerCapabilities,
        TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Url,
    },
    Client, LanguageServer, LspService, Server,
};

struct Backend {
    client: Client,
    documents: Mutex<HashMap<Url, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Frame LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let source = params.text_document.text;

        self.documents
            .lock()
            .await
            .insert(uri.clone(), source.clone());
        self.publish_diagnostics(uri, &source).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };

        self.documents
            .lock()
            .await
            .insert(uri.clone(), change.text.clone());
        self.publish_diagnostics(uri, &change.text).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&uri) else {
            return Ok(None);
        };
        let offset = diagnostics::offset_for_position(source, position);
        let items = completions::completions_at(source, offset)
            .into_iter()
            .map(|suggestion| CompletionItem {
                label: suggestion.label.to_string(),
                detail: Some(suggestion.detail.to_string()),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(
                    suggestion.documentation.to_string(),
                )),
                ..CompletionItem::default()
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&uri) else {
            return Ok(None);
        };
        let offset = diagnostics::offset_for_position(source, position);
        let Some(word) = hover::word_at(source, offset) else {
            return Ok(None);
        };
        let Some(contents) = hover::hover_doc(word) else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(contents.to_string())),
            range: None,
        }))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&uri) else {
            return Ok(None);
        };

        Ok(Some(vec![TextEdit {
            range: full_document_range(source),
            new_text: formatting::format_document(source),
        }]))
    }
}

impl Backend {
    async fn publish_diagnostics(&self, uri: Url, source: &str) {
        let diagnostics = diagnostics::diagnostics_for_source(source)
            .into_iter()
            .map(|diagnostic| diagnostics::to_lsp_diagnostic(source, diagnostic))
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

fn full_document_range(source: &str) -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: diagnostics::position_for_offset(source, source.len()),
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_full_document_range() {
        let range = full_document_range("card A {\n}\n");

        assert_eq!(range.start.line, 0);
        assert_eq!(range.end.line, 2);
    }
}
