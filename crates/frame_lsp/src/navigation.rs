use frame_core::{Node, Span};
use frame_parser::parse;

use crate::document_symbols::collect_document_symbols;
use crate::embedded::{frame_block_at, frame_blocks};
use crate::hover::word_at;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTarget {
    pub span: Span,
}

pub fn definition_at(source: &str, offset: usize) -> Option<NavigationTarget> {
    let (frame_source, frame_offset, base) = frame_source_at(source, offset)?;
    let word = word_at(frame_source, frame_offset)?;
    let line = line_at(frame_source, frame_offset);
    let words = line.split_whitespace().collect::<Vec<_>>();
    let symbols = collect_document_symbols(frame_source);

    if words.first() == Some(&"in") && words.get(1) == Some(&word) {
        return symbols.declarations.get(word).map(|span| NavigationTarget {
            span: add_base(*span, base),
        });
    }

    if words.first() == Some(&"place") && words.get(1) == Some(&word) {
        let grid = area_grid_before(frame_source, frame_offset)?;
        let span = symbols.grid_columns.get(&grid)?.get(word)?;
        return Some(NavigationTarget {
            span: add_base(*span, base),
        });
    }

    if let Some(class_name) = word.strip_prefix("fr-") {
        return symbols
            .declarations
            .get(class_name)
            .map(|span| NavigationTarget {
                span: add_base(*span, base),
            });
    }

    None
}

pub fn references_at(source: &str, offset: usize) -> Vec<NavigationTarget> {
    let Some((frame_source, frame_offset, base)) = frame_source_at(source, offset) else {
        return Vec::new();
    };
    let Some(word) = word_at(frame_source, frame_offset) else {
        return Vec::new();
    };
    let Ok(document) = parse(frame_source) else {
        return Vec::new();
    };

    let mut targets = Vec::new();
    for declaration in &document.declarations {
        if declaration.name.text == word {
            targets.push(NavigationTarget {
                span: add_base(declaration.name.span, base),
            });
        }

        for node in &declaration.body {
            let Node::Statement(statement) = node else {
                continue;
            };
            if statement.words.get(1).map(String::as_str) == Some(word)
                && statement
                    .words
                    .first()
                    .is_some_and(|keyword| matches!(keyword.as_str(), "in" | "place"))
            {
                if let Some(span) = word_span_in_statement(frame_source, statement.span, word) {
                    targets.push(NavigationTarget {
                        span: add_base(span, base),
                    });
                }
            }
        }
    }

    if targets.is_empty() {
        collect_grid_section_references(frame_source, base, word)
    } else {
        targets
    }
}

fn collect_grid_section_references(source: &str, base: usize, word: &str) -> Vec<NavigationTarget> {
    let symbols = collect_document_symbols(source);
    let mut targets = Vec::new();

    for columns in symbols.grid_columns.values() {
        if let Some(span) = columns.get(word) {
            targets.push(NavigationTarget {
                span: add_base(*span, base),
            });
        }
    }

    let Ok(document) = parse(source) else {
        return targets;
    };

    for declaration in document.declarations {
        for node in declaration.body {
            let Node::Statement(statement) = node else {
                continue;
            };
            if statement.words.first().map(String::as_str) == Some("place")
                && statement.words.get(1).map(String::as_str) == Some(word)
            {
                if let Some(span) = word_span_in_statement(source, statement.span, word) {
                    targets.push(NavigationTarget {
                        span: add_base(span, base),
                    });
                }
            }
        }
    }

    targets
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

fn word_span_in_statement(source: &str, span: Span, word: &str) -> Option<Span> {
    let relative = source[span.start..span.end].find(word)?;
    Some(Span {
        start: span.start + relative,
        end: span.start + relative + word.len(),
    })
}

fn frame_source_at(source: &str, offset: usize) -> Option<(&str, usize, usize)> {
    let blocks = frame_blocks(source);
    if blocks.is_empty() {
        return Some((source, offset, 0));
    }

    frame_block_at(source, offset).map(|block| {
        (
            block.content,
            offset.saturating_sub(block.content_start),
            block.content_start,
        )
    })
}

fn add_base(span: Span, base: usize) -> Span {
    Span {
        start: span.start + base,
        end: span.end + base,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_grid_definition_from_area_in_reference() {
        let source =
            "grid Dashboard {\n  columns sidebar content\n}\narea Sidebar {\n  in Dashboard\n}\n";
        let offset = source.rfind("Dashboard").unwrap() + 1;
        let target = definition_at(source, offset).expect("definition should exist");

        assert_eq!(&source[target.span.start..target.span.end], "Dashboard");
    }

    #[test]
    fn finds_grid_section_definition_from_place_reference() {
        let source = "grid Dashboard {\n  columns sidebar content\n}\narea Sidebar {\n  in Dashboard\n  place sidebar\n}\n";
        let offset = source.rfind("sidebar").unwrap() + 1;
        let target = definition_at(source, offset).expect("definition should exist");

        assert_eq!(&source[target.span.start..target.span.end], "sidebar");
    }

    #[test]
    fn finds_declaration_references() {
        let source = "grid Dashboard {\n}\narea Sidebar {\n  in Dashboard\n}\n";
        let offset = source.find("Dashboard").unwrap() + 1;
        let references = references_at(source, offset);

        assert_eq!(references.len(), 2);
    }
}
