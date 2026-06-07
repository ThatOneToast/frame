use std::collections::HashMap;

use tokio::sync::Mutex;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CodeActionOptions, CodeActionParams, CodeActionProviderCapability, CompletionItem,
        CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentFormattingParams,
        DocumentLinkOptions, DocumentLinkParams, DocumentSymbolParams, DocumentSymbolResponse,
        FoldingRangeParams, FoldingRangeProviderCapability, GotoDefinitionParams,
        GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
        InitializeParams, InitializeResult, InitializedParams, InsertTextFormat, Location,
        MarkupContent, MarkupKind, MessageType, OneOf, Position, Range, ReferenceParams,
        SemanticTokenModifier, SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult, ServerCapabilities,
        TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Url, WorkDoneProgressOptions,
    },
    Client, LanguageServer,
};

use crate::{
    code_actions, completions, diagnostics, document_links, document_symbols, embedded, folding,
    formatting, hover, navigation, semantic_tokens, support,
};

pub struct Backend {
    pub client: Client,
    pub documents: Mutex<HashMap<Url, String>>,
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
                document_symbol_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_link_provider: Some(DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: None,
                        resolve_provider: Some(false),
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                    },
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensOptions {
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::KEYWORD,
                                SemanticTokenType::CLASS,
                                SemanticTokenType::PROPERTY,
                                SemanticTokenType::ENUM_MEMBER,
                                SemanticTokenType::VARIABLE,
                                SemanticTokenType::NUMBER,
                                SemanticTokenType::COMMENT,
                            ],
                            token_modifiers: vec![SemanticTokenModifier::DECLARATION],
                        },
                        range: None,
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                    }
                    .into(),
                ),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
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
        let Some((completion_source, completion_offset)) = frame_source_at(source, offset) else {
            return Ok(Some(CompletionResponse::Array(Vec::new())));
        };
        let include_files = support::include_files_for_uri(&uri);
        let include_prefix = support::included_source_prefix(completion_source, &uri);
        let merged_source;
        let (completion_source, completion_offset) = if include_prefix.is_empty() {
            (completion_source, completion_offset)
        } else {
            merged_source = format!("{include_prefix}\n{completion_source}");
            (
                merged_source.as_str(),
                completion_offset + include_prefix.len() + 1,
            )
        };
        let items = completions::completions_at_with_includes(
            completion_source,
            completion_offset,
            include_files,
            Some(&uri),
        )
        .into_iter()
        .map(|suggestion| CompletionItem {
            sort_text: Some(format!(
                "{}_{}",
                suggestion.category.sort_prefix(),
                suggestion.label
            )),
            kind: Some(completion_kind(suggestion.category)),
            detail: Some(format!(
                "Frame {} • {}",
                suggestion.category.label(),
                suggestion.detail
            )),
            label: suggestion.label,
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: suggestion.documentation,
                },
            )),
            insert_text: suggestion.insert_text,
            insert_text_format: suggestion.is_snippet.then_some(InsertTextFormat::SNIPPET),
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
        let Some((hover_source, hover_offset)) = frame_source_at(source, offset) else {
            return Ok(None);
        };
        let merged_source = support::merged_frame_source(hover_source, &uri);
        let symbols = frame_parser::parse(&merged_source)
            .ok()
            .map(|document| frame_core::symbols::index_document(&merged_source, &document));
        let Some(contents) =
            hover::hover_doc_at_with_symbols(hover_source, hover_offset, symbols.as_ref())
        else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: contents,
            }),
            range: None,
        }))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&uri) else {
            return Ok(None);
        };
        if !embedded::frame_blocks(source).is_empty() {
            return Ok(Some(format_embedded_document(source)));
        }

        Ok(Some(vec![TextEdit {
            range: full_document_range(source),
            new_text: formatting::format_document(source),
        }]))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&params.text_document.uri) else {
            return Ok(None);
        };

        Ok(document_symbols::lsp_document_symbols(source))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&uri) else {
            return Ok(None);
        };
        let offset = diagnostics::offset_for_position(source, position);
        if let Some(location) = support::include_definition_location(source, offset, &uri) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }
        if let Some(location) = support::imported_symbol_definition_location(source, offset, &uri) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }
        let Some(target) = navigation::definition_at(source, offset) else {
            return Ok(None);
        };

        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri,
            range: diagnostics::range_for_span(source, target.span),
        })))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&uri) else {
            return Ok(None);
        };
        let offset = diagnostics::offset_for_position(source, position);
        let mut locations = navigation::references_at(source, offset)
            .into_iter()
            .map(|target| Location {
                uri: uri.clone(),
                range: diagnostics::range_for_span(source, target.span),
            })
            .collect::<Vec<_>>();
        locations.extend(support::imported_symbol_reference_locations(
            source, offset, &uri,
        ));

        Ok(Some(locations))
    }

    async fn document_link(
        &self,
        params: DocumentLinkParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::DocumentLink>>> {
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&params.text_document.uri) else {
            return Ok(None);
        };

        Ok(Some(document_links::document_links(
            source,
            Some(&params.text_document.uri),
        )))
    }

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<tower_lsp::lsp_types::CodeActionResponse>> {
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&params.text_document.uri) else {
            return Ok(None);
        };

        Ok(Some(code_actions::code_actions_for_source(
            source,
            &params.text_document.uri,
        )))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&params.text_document.uri) else {
            return Ok(None);
        };

        Ok(Some(SemanticTokensResult::Tokens(
            semantic_tokens::semantic_tokens(source),
        )))
    }

    async fn folding_range(
        &self,
        params: FoldingRangeParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::FoldingRange>>> {
        let documents = self.documents.lock().await;
        let Some(source) = documents.get(&params.text_document.uri) else {
            return Ok(None);
        };

        Ok(Some(folding::folding_ranges(source)))
    }
}

impl Backend {
    async fn publish_diagnostics(&self, uri: Url, source: &str) {
        let diagnostics = diagnostics::diagnostics_for_uri(source, &uri)
            .into_iter()
            .map(|diagnostic| diagnostics::to_lsp_diagnostic(source, diagnostic))
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

fn completion_kind(category: completions::CompletionCategory) -> CompletionItemKind {
    match category {
        completions::CompletionCategory::Snippet => CompletionItemKind::SNIPPET,
        completions::CompletionCategory::Declaration
        | completions::CompletionCategory::Include
        | completions::CompletionCategory::KeyframeSelector
        | completions::CompletionCategory::StateBlock => CompletionItemKind::KEYWORD,
        completions::CompletionCategory::LayoutProperty
        | completions::CompletionCategory::VisualProperty
        | completions::CompletionCategory::MotionProperty
        | completions::CompletionCategory::TypographyProperty
        | completions::CompletionCategory::TokenProperty
        | completions::CompletionCategory::AdvancedProperty
        | completions::CompletionCategory::AnimationOption => CompletionItemKind::PROPERTY,
        completions::CompletionCategory::ProjectSymbol
        | completions::CompletionCategory::GridReference
        | completions::CompletionCategory::GridSection => CompletionItemKind::CONSTANT,
        completions::CompletionCategory::Value => CompletionItemKind::ENUM_MEMBER,
    }
}

pub fn full_document_range(source: &str) -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: diagnostics::position_for_offset(source, source.len()),
    }
}

pub fn format_embedded_document(source: &str) -> Vec<TextEdit> {
    embedded::frame_blocks(source)
        .into_iter()
        .map(|block| TextEdit {
            range: Range {
                start: diagnostics::position_for_offset(source, block.content_start),
                end: diagnostics::position_for_offset(source, block.content_end),
            },
            new_text: formatting::format_document(block.content),
        })
        .collect()
}

pub fn frame_source_at(source: &str, offset: usize) -> Option<(&str, usize)> {
    let blocks = embedded::frame_blocks(source);
    if blocks.is_empty() {
        return Some((source, offset));
    }

    embedded::frame_block_at(source, offset)
        .map(|block| (block.content, offset.saturating_sub(block.content_start)))
}
