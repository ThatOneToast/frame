use crate::document_symbols::{collect_document_symbols, DocumentSymbols};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionScope {
    Root,
    Declaration {
        kind: String,
        property: Option<String>,
        area_grid: Option<String>,
    },
    State {
        property: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionContext {
    pub scope: CompletionScope,
    pub symbols: DocumentSymbols,
}

pub fn completion_context(source: &str, offset: usize) -> CompletionContext {
    let safe_offset = offset.min(source.len());
    let line_prefix = line_prefix_at(source, safe_offset);
    let property = property_at_line(line_prefix);
    let mut stack = block_stack_before(source, safe_offset);
    let symbols = collect_document_symbols(source);

    if stack.last().is_some_and(|block| is_state(block)) {
        return CompletionContext {
            scope: CompletionScope::State { property },
            symbols,
        };
    }

    let declaration = stack
        .drain(..)
        .find(|block| declaration_kind(block).is_some());

    let Some(declaration) = declaration else {
        return CompletionContext {
            scope: CompletionScope::Root,
            symbols,
        };
    };

    let kind = declaration_kind(&declaration)
        .unwrap_or_default()
        .to_string();
    let area_grid = if kind == "area" {
        area_grid_before(source, safe_offset)
    } else {
        None
    };

    CompletionContext {
        scope: CompletionScope::Declaration {
            kind,
            property,
            area_grid,
        },
        symbols,
    }
}

fn property_at_line(line_prefix: &str) -> Option<String> {
    let trimmed = line_prefix.trim_start();
    if trimmed.is_empty() {
        return None;
    }

    let words = trimmed.split_whitespace().collect::<Vec<_>>();
    if words.len() >= 2 || line_prefix.ends_with(' ') || line_prefix.ends_with('\t') {
        words.first().map(|word| (*word).to_string())
    } else {
        None
    }
}

fn block_stack_before(source: &str, offset: usize) -> Vec<String> {
    let mut stack = Vec::new();
    let mut line_start = 0usize;

    for (index, character) in source[..offset].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }

    stack
}

fn declaration_kind(header: &str) -> Option<&str> {
    let kind = header.split_whitespace().next()?;
    matches!(
        kind,
        "tokens"
            | "grid"
            | "area"
            | "card"
            | "stack"
            | "row"
            | "button"
            | "text"
            | "center"
            | "split"
            | "overlay"
            | "dock"
    )
    .then_some(kind)
}

fn is_state(header: &str) -> bool {
    matches!(header, "hover" | "focus" | "active" | "disabled")
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

fn line_prefix_at(source: &str, offset: usize) -> &str {
    let start = source[..offset].rfind('\n').map_or(0, |index| index + 1);
    &source[start..offset]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_root_scope() {
        assert_eq!(completion_context("", 0).scope, CompletionScope::Root);
    }

    #[test]
    fn detects_grid_property_scope() {
        let source = "grid Dashboard {\n  columns ";

        assert_eq!(
            completion_context(source, source.len()).scope,
            CompletionScope::Declaration {
                kind: "grid".to_string(),
                property: Some("columns".to_string()),
                area_grid: None
            }
        );
    }

    #[test]
    fn detects_area_grid_reference() {
        let source = "grid Dashboard {\n  columns sidebar content\n}\narea Sidebar {\n  in Dashboard\n  place ";

        assert_eq!(
            completion_context(source, source.len()).scope,
            CompletionScope::Declaration {
                kind: "area".to_string(),
                property: Some("place".to_string()),
                area_grid: Some("Dashboard".to_string())
            }
        );
    }

    #[test]
    fn detects_state_scope() {
        let source = "card ProjectCard {\n  hover {\n    ";

        assert_eq!(
            completion_context(source, source.len()).scope,
            CompletionScope::State { property: None }
        );
    }
}
