use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

mod code_actions;
mod completions;
mod context;
mod diagnostics;
mod document_links;
mod document_symbols;
mod embedded;
mod folding;
mod formatting;
mod hover;
mod navigation;
mod semantic_tokens;

use tokio::sync::Mutex;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CodeActionOptions, CodeActionParams, CodeActionProviderCapability, CompletionItem,
        CompletionOptions, CompletionParams, CompletionResponse, DidChangeTextDocumentParams,
        DidOpenTextDocumentParams, DocumentFormattingParams, DocumentLinkOptions,
        DocumentLinkParams, DocumentSymbolParams, DocumentSymbolResponse, FoldingRangeParams,
        FoldingRangeProviderCapability, GotoDefinitionParams, GotoDefinitionResponse, Hover,
        HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
        InitializedParams, InsertTextFormat, Location, MarkupContent, MarkupKind, MessageType,
        OneOf, Position, Range, ReferenceParams, SemanticTokenModifier, SemanticTokenType,
        SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
        SemanticTokensParams, SemanticTokensResult, ServerCapabilities, TextDocumentSyncCapability,
        TextDocumentSyncKind, TextEdit, Url, WorkDoneProgressOptions,
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
        let include_files = include_files_for_uri(&uri);
        let include_prefix = included_source_prefix(completion_source, &uri);
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
        )
        .into_iter()
        .map(|suggestion| CompletionItem {
            label: suggestion.label,
            detail: Some(suggestion.detail.to_string()),
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
        let merged_source = merged_frame_source(hover_source, &uri);
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
        if let Some(location) = include_definition_location(source, offset, &uri) {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }
        if let Some(location) = imported_symbol_definition_location(source, offset, &uri) {
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
        locations.extend(imported_symbol_reference_locations(source, offset, &uri));

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

fn include_files_for_uri(uri: &Url) -> Vec<PathBuf> {
    let Ok(path) = uri.to_file_path() else {
        return Vec::new();
    };
    let Some(parent) = path.parent() else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(parent) else {
        return Vec::new();
    };

    entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "frame")
        })
        .collect()
}

fn merged_frame_source(source: &str, uri: &Url) -> String {
    let prefix = included_source_prefix(source, uri);
    if prefix.is_empty() {
        source.to_string()
    } else {
        format!("{prefix}\n{source}")
    }
}

fn included_source_prefix(source: &str, uri: &Url) -> String {
    let Ok(path) = uri.to_file_path() else {
        return String::new();
    };
    let Some(parent) = path.parent() else {
        return String::new();
    };

    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("#include") {
                return None;
            }
            let target = trimmed.split_whitespace().nth(1)?;
            let mut include_path = parent.join(target);
            if include_path.extension().is_none() {
                include_path = include_path.with_extension("frame");
            }
            fs::read_to_string(include_path).ok()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn imported_symbol_definition_location(source: &str, offset: usize, uri: &Url) -> Option<Location> {
    let (frame_source, frame_offset) = frame_source_at(source, offset)?;
    let word = hover::word_at(frame_source, frame_offset)?;
    let line = line_at(frame_source, frame_offset);
    let words = line.split_whitespace().collect::<Vec<_>>();
    let current_path = uri.to_file_path().ok()?;
    let mut seen = HashSet::new();

    for (path, include_source) in included_sources_for_path(&current_path, frame_source, &mut seen)
    {
        let document = frame_parser::parse(&include_source).ok()?;
        let symbols = frame_core::symbols::index_document(&include_source, &document);
        let symbol = if words.first() == Some(&"in") {
            symbols.grids.get(word)
        } else if words.first() == Some(&"place") {
            let grid = area_grid_before(frame_source, frame_offset)?;
            symbols
                .grid_sections
                .get(&grid)
                .and_then(|sections| sections.get(word))
        } else {
            symbols
                .colors
                .get(word)
                .or_else(|| symbols.gradients.get(word))
                .or_else(|| symbols.grids.get(word))
        };
        if let Some(symbol) = symbol {
            let target_uri = Url::from_file_path(path).ok()?;
            return Some(Location {
                uri: target_uri,
                range: diagnostics::range_for_span(&include_source, symbol.span),
            });
        }
    }

    None
}

fn imported_symbol_reference_locations(source: &str, offset: usize, uri: &Url) -> Vec<Location> {
    let Some((frame_source, frame_offset)) = frame_source_at(source, offset) else {
        return Vec::new();
    };
    let Some(word) = hover::word_at(frame_source, frame_offset) else {
        return Vec::new();
    };
    let Ok(current_path) = uri.to_file_path() else {
        return Vec::new();
    };
    let mut seen = HashSet::new();
    let mut locations = Vec::new();

    for (path, include_source) in included_sources_for_path(&current_path, frame_source, &mut seen)
    {
        let Ok(target_uri) = Url::from_file_path(path) else {
            continue;
        };
        locations.extend(
            word_occurrences(&include_source, word)
                .into_iter()
                .map(|span| Location {
                    uri: target_uri.clone(),
                    range: diagnostics::range_for_span(&include_source, span),
                }),
        );
    }

    locations
}

fn word_occurrences(source: &str, word: &str) -> Vec<frame_core::Span> {
    let mut spans = Vec::new();
    let mut search_start = 0usize;
    while let Some(relative) = source[search_start..].find(word) {
        let start = search_start + relative;
        let end = start + word.len();
        if is_word_boundary(source, start, end) {
            spans.push(frame_core::Span { start, end });
        }
        search_start = end;
    }
    spans
}

fn is_word_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start].chars().next_back();
    let after = source[end..].chars().next();
    !before.is_some_and(is_identifier_character) && !after.is_some_and(is_identifier_character)
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '-' || character == '_'
}

fn included_sources_for_path(
    current_path: &Path,
    source: &str,
    seen: &mut HashSet<PathBuf>,
) -> Vec<(PathBuf, String)> {
    let Some(parent) = current_path.parent() else {
        return Vec::new();
    };
    let mut results = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("#include") {
            continue;
        }
        let Some(target) = trimmed.split_whitespace().nth(1) else {
            continue;
        };
        let mut path = parent.join(target);
        if path.extension().is_none() {
            path = path.with_extension("frame");
        }
        let canonical = fs::canonicalize(&path).unwrap_or(path);
        if !seen.insert(canonical.clone()) {
            continue;
        }
        let Ok(include_source) = fs::read_to_string(&canonical) else {
            continue;
        };
        results.extend(included_sources_for_path(&canonical, &include_source, seen));
        results.push((canonical, include_source));
    }
    results
}

fn area_grid_before(source: &str, offset: usize) -> Option<String> {
    let declaration_start = source[..offset].rfind('{')?;
    source[declaration_start + 1..offset]
        .lines()
        .filter_map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            (words.first() == Some(&"in"))
                .then(|| words.get(1).copied())
                .flatten()
        })
        .next_back()
        .map(ToOwned::to_owned)
}

fn line_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let end = source[safe_offset..]
        .find('\n')
        .map_or(source.len(), |index| safe_offset + index);
    source[start..end].trim()
}

fn include_definition_location(source: &str, offset: usize, uri: &Url) -> Option<Location> {
    let line_start = source[..offset].rfind('\n').map_or(0, |index| index + 1);
    let line_end = source[offset..]
        .find('\n')
        .map_or(source.len(), |index| offset + index);
    let line = &source[line_start..line_end];
    let trimmed = line.trim_start();
    if !trimmed.starts_with("#include") {
        return None;
    }
    let target = trimmed.split_whitespace().nth(1)?;
    let target_start = line_start + line.find(target)?;
    if offset < target_start || offset > target_start + target.len() {
        return None;
    }

    let current_path = uri.to_file_path().ok()?;
    let parent = current_path.parent()?;
    let mut path = parent.join(target);
    if path.extension().is_none() {
        path = path.with_extension("frame");
    }
    let target_uri = Url::from_file_path(path).ok()?;
    Some(Location {
        uri: target_uri,
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
    })
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

fn full_document_range(source: &str) -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: diagnostics::position_for_offset(source, source.len()),
    }
}

fn format_embedded_document(source: &str) -> Vec<TextEdit> {
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

fn frame_source_at(source: &str, offset: usize) -> Option<(&str, usize)> {
    let blocks = embedded::frame_blocks(source);
    if blocks.is_empty() {
        return Some((source, offset));
    }

    embedded::frame_block_at(source, offset)
        .map(|block| (block.content, offset.saturating_sub(block.content_start)))
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

    #[test]
    fn maps_svelte_frame_style_blocks_to_frame_source() {
        let source = "<style lang=\"frame\">\ncard Demo {\n  surface \n}\n</style>";
        let offset = source.find("surface ").unwrap() + "surface ".len();
        let (frame_source, frame_offset) =
            frame_source_at(source, offset).expect("offset should be inside Frame block");

        assert!(frame_source.contains("card Demo"));
        assert_eq!(
            &frame_source[frame_offset - "surface ".len()..frame_offset],
            "surface "
        );
    }

    #[test]
    fn returns_none_outside_svelte_frame_style_blocks() {
        let source =
            "<script>let a = 1;</script>\n<style lang=\"frame\">\ncard Demo {\n}\n</style>";
        let offset = source.find("let").unwrap();

        assert!(frame_source_at(source, offset).is_none());
    }

    #[test]
    fn finds_imported_token_definition_location() {
        let root = std::env::temp_dir().join(format!("frame-lsp-import-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let theme = root.join("theme.frame");
        std::fs::write(&theme, "tokens Brand {\n  color brand-panel #181820\n}\n")
            .expect("theme should be writable");
        let source = "#include theme\n\ncard Hero {\n  background brand-panel\n}\n";
        std::fs::write(&app, source).expect("app should be writable");
        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.rfind("brand-panel").unwrap() + 1;

        let location = imported_symbol_definition_location(source, offset, &uri)
            .expect("definition should resolve");

        let actual = location.uri.to_file_path().unwrap();
        assert_eq!(actual, std::fs::canonicalize(theme).unwrap());
    }

    #[test]
    fn finds_imported_token_reference_locations() {
        let root =
            std::env::temp_dir().join(format!("frame-lsp-references-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let theme = root.join("theme.frame");
        std::fs::write(
            &theme,
            "tokens Brand {\n  color brand-panel #181820\n}\ncard Imported {\n  background brand-panel\n}\n",
        )
        .expect("theme should be writable");
        let source = "#include theme\n\ncard Hero {\n  background brand-panel\n}\n";
        std::fs::write(&app, source).expect("app should be writable");
        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.rfind("brand-panel").unwrap() + 1;

        let locations = imported_symbol_reference_locations(source, offset, &uri);

        assert_eq!(locations.len(), 2);
        assert!(locations.iter().all(|location| location
            .uri
            .to_file_path()
            .unwrap()
            .ends_with("theme.frame")));
    }
}
