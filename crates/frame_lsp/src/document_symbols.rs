use std::collections::HashMap;

use frame_core::{DeclarationKind, Node, Span};
use frame_parser::parse;
use tower_lsp::lsp_types::{DocumentSymbol, DocumentSymbolResponse, SymbolKind};

use crate::diagnostics::range_for_span;
use crate::embedded::frame_blocks;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocumentSymbols {
    pub grids: HashMap<String, Vec<String>>,
    pub declarations: HashMap<String, Span>,
    pub grid_columns: HashMap<String, HashMap<String, Span>>,
}

pub fn collect_document_symbols(source: &str) -> DocumentSymbols {
    let mut symbols = DocumentSymbols::default();
    let mut current_grid: Option<String> = None;
    let mut depth = 0usize;

    for raw_line in source.lines() {
        let line = raw_line
            .split_once("//")
            .map_or(raw_line, |(before, _)| before);
        let trimmed = line.trim();

        if trimmed.ends_with('{') {
            depth += 1;
            let header = trimmed.trim_end_matches('{').trim();
            let mut words = header.split_whitespace();
            if words.next() == Some("grid") {
                current_grid = words.next().map(ToOwned::to_owned);
                if let Some(name) = &current_grid {
                    symbols.grids.entry(name.clone()).or_default();
                    symbols.declarations.entry(name.clone()).or_default();
                }
            }
            continue;
        }

        if trimmed == "}" {
            if depth == 1 {
                current_grid = None;
            }
            depth = depth.saturating_sub(1);
            continue;
        }

        if depth == 1 && trimmed.starts_with("columns ") {
            if let Some(grid) = &current_grid {
                let values = trimmed
                    .split_whitespace()
                    .skip(1)
                    .filter(|value| {
                        !matches!(value, &"responsive" | &"cards" | &"subgrid")
                            && value
                                .chars()
                                .next()
                                .is_some_and(|first| first.is_ascii_alphabetic())
                    })
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>();
                symbols.grids.insert(grid.clone(), values);
            }
        }
    }

    if let Ok(document) = parse(source) {
        for declaration in document.declarations {
            symbols
                .declarations
                .insert(declaration.name.text.clone(), declaration.name.span);

            if declaration.kind == DeclarationKind::Grid {
                for node in declaration.body {
                    let Node::Statement(statement) = node else {
                        continue;
                    };
                    if statement.words.first().map(String::as_str) != Some("columns") {
                        continue;
                    }
                    let mut search_start = statement.span.start;
                    for word in statement.words.into_iter().skip(1) {
                        if matches!(word.as_str(), "responsive" | "cards" | "subgrid") {
                            continue;
                        }
                        if let Some(relative) = source[search_start..statement.span.end].find(&word)
                        {
                            let start = search_start + relative;
                            symbols
                                .grid_columns
                                .entry(declaration.name.text.clone())
                                .or_default()
                                .insert(
                                    word.clone(),
                                    Span {
                                        start,
                                        end: start + word.len(),
                                    },
                                );
                            search_start = start + word.len();
                        }
                    }
                }
            }
        }
    }

    symbols
}

pub fn lsp_document_symbols(source: &str) -> Option<DocumentSymbolResponse> {
    let blocks = frame_blocks(source);
    if !blocks.is_empty() {
        let mut symbols = Vec::new();
        for block in blocks {
            symbols.extend(symbols_for_frame_source(
                source,
                block.content,
                block.content_start,
            ));
        }
        return Some(DocumentSymbolResponse::Nested(symbols));
    }

    Some(DocumentSymbolResponse::Nested(symbols_for_frame_source(
        source, source, 0,
    )))
}

#[allow(deprecated)]
fn symbols_for_frame_source(
    full_source: &str,
    frame_source: &str,
    base: usize,
) -> Vec<DocumentSymbol> {
    let Ok(document) = parse(frame_source) else {
        return Vec::new();
    };

    document
        .declarations
        .into_iter()
        .map(|declaration| {
            let children = declaration
                .body
                .iter()
                .filter_map(|node| {
                    let Node::Block(block) = node else {
                        return None;
                    };
                    Some(DocumentSymbol {
                        name: block.name.clone(),
                        detail: Some("state".to_string()),
                        kind: SymbolKind::METHOD,
                        tags: None,
                        deprecated: None,
                        range: range_for_span(
                            full_source,
                            Span {
                                start: block.span.start + base,
                                end: block.span.end + base,
                            },
                        ),
                        selection_range: range_for_span(
                            full_source,
                            Span {
                                start: block.span.start + base,
                                end: block.span.start + block.name.len() + base,
                            },
                        ),
                        children: None,
                    })
                })
                .collect::<Vec<_>>();

            DocumentSymbol {
                name: declaration.name.text,
                detail: Some(declaration_kind_label(&declaration.kind).to_string()),
                kind: symbol_kind(&declaration.kind),
                tags: None,
                deprecated: None,
                range: range_for_span(
                    full_source,
                    Span {
                        start: declaration.span.start + base,
                        end: declaration.span.end + base,
                    },
                ),
                selection_range: range_for_span(
                    full_source,
                    Span {
                        start: declaration.name.span.start + base,
                        end: declaration.name.span.end + base,
                    },
                ),
                children: (!children.is_empty()).then_some(children),
            }
        })
        .collect()
}

fn declaration_kind_label(kind: &DeclarationKind) -> &str {
    match kind {
        DeclarationKind::Grid => "grid",
        DeclarationKind::Area => "area",
        DeclarationKind::Card => "card",
        DeclarationKind::Stack => "stack",
        DeclarationKind::Row => "row",
        DeclarationKind::Button => "button",
        DeclarationKind::Text => "text",
        DeclarationKind::Tokens => "tokens",
        DeclarationKind::Theme => "theme",
        DeclarationKind::Layout => "layout",
        DeclarationKind::Motion => "motion",
        DeclarationKind::Recipe => "recipe",
        DeclarationKind::Center => "center",
        DeclarationKind::Split => "split",
        DeclarationKind::Overlay => "overlay",
        DeclarationKind::Dock => "dock",
        DeclarationKind::Keyframes => "keyframes",
        DeclarationKind::Supports => "supports",
        DeclarationKind::StyleGroup => "style-group",
        DeclarationKind::StyleOrder => "style-order",
        DeclarationKind::Html => "html",
        DeclarationKind::Body => "body",
        DeclarationKind::Unknown(value) => value.as_str(),
    }
}

fn symbol_kind(kind: &DeclarationKind) -> SymbolKind {
    match kind {
        DeclarationKind::Grid => SymbolKind::NAMESPACE,
        DeclarationKind::Supports => SymbolKind::NAMESPACE,
        DeclarationKind::StyleGroup => SymbolKind::NAMESPACE,
        DeclarationKind::StyleOrder => SymbolKind::NAMESPACE,
        DeclarationKind::Area => SymbolKind::FIELD,
        DeclarationKind::Text => SymbolKind::STRING,
        DeclarationKind::Tokens => SymbolKind::CONSTANT,
        DeclarationKind::Theme => SymbolKind::CONSTANT,
        _ => SymbolKind::CLASS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_grid_names_and_columns() {
        let symbols =
            collect_document_symbols("grid Dashboard {\n  columns sidebar content inspector\n}\n");

        assert_eq!(
            symbols.grids.get("Dashboard"),
            Some(&vec![
                "sidebar".to_string(),
                "content".to_string(),
                "inspector".to_string()
            ])
        );
    }
}
