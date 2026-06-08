use std::path::PathBuf;

use frame_core::{DeclarationKind, Document, Node, Span};
use frame_parser::parse;
use tower_lsp::lsp_types::Url;

use crate::document_symbols::collect_document_symbols;
use crate::embedded::{frame_block_at, frame_blocks};
use crate::hover::word_at;
use crate::ide::cursor::SemanticCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTarget {
    pub span: Span,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    Declaration,
    ComponentInvocation,
    StyleApplication,
    AutomaticStyleLookup,
    StateRead,
    StateWriteBinding,
    PropRead,
    HandlerReference,
    TokenUsage,
    IncludeUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceTarget {
    pub target: NavigationTarget,
    pub kind: ReferenceKind,
}

pub fn definition_at(source: &str, offset: usize) -> Option<NavigationTarget> {
    let (frame_source, frame_offset, base) = frame_source_at(source, offset)?;
    let word = word_at(frame_source, frame_offset)?;
    let line = line_at(frame_source, frame_offset);
    let words = line.split_whitespace().collect::<Vec<_>>();
    let symbols = collect_document_symbols(frame_source);
    let cursor = SemanticCursor::at(frame_source, frame_offset);

    if words.first() == Some(&"in") && words.get(1) == Some(&word) {
        return symbols.declarations.get(word).map(|span| NavigationTarget {
            span: add_base(*span, base),
            path: None,
        });
    }

    if words.first() == Some(&"place") && words.get(1) == Some(&word) {
        let grid = area_grid_before(frame_source, frame_offset)?;
        let span = symbols.grid_columns.get(&grid)?.get(word)?;
        return Some(NavigationTarget {
            span: add_base(*span, base),
            path: None,
        });
    }

    if let Some(class_name) = word.strip_prefix("fr-") {
        return symbols
            .declarations
            .get(class_name)
            .map(|span| NavigationTarget {
                span: add_base(*span, base),
                path: None,
            });
    }

    // Use SemanticCursor symbols for declarations and components
    if let Some(declaration) = cursor.symbols.declarations.get(word) {
        return Some(NavigationTarget {
            span: add_base(declaration.span, base),
            path: None,
        });
    }

    if let Some(component) = cursor.symbols.components.get(word) {
        return Some(NavigationTarget {
            span: add_base(component.span, base),
            path: None,
        });
    }

    None
}

pub fn references_at_with_context(
    source: &str,
    offset: usize,
    uri: &Url,
    include_declaration: bool,
) -> Vec<ReferenceTarget> {
    let Some((frame_source, frame_offset, base)) = frame_source_at(source, offset) else {
        return Vec::new();
    };
    let Some(word) = word_at(frame_source, frame_offset) else {
        return Vec::new();
    };

    let clean_word = clean_word_for_references(word);
    let cursor = SemanticCursor::at(frame_source, frame_offset);

    let mut targets = Vec::new();

    // Current file references
    let document = parse(frame_source).ok();
    for span in crate::project::collect_references_in_source(frame_source, clean_word) {
        let kind = document
            .as_ref()
            .map(|doc| classify_span(frame_source, span, clean_word, doc, &cursor))
            .unwrap_or(ReferenceKind::TokenUsage);
        if !include_declaration && kind == ReferenceKind::Declaration {
            continue;
        }
        targets.push(ReferenceTarget {
            target: NavigationTarget {
                span: add_base(span, base),
                path: None,
            },
            kind,
        });
    }

    // Fall back to old grid-section logic for the current file when the AST walk
    // found nothing. This preserves behavior for grid columns that might not be
    // captured as explicit statement-word matches in some edge cases.
    if targets.is_empty() {
        for target in collect_grid_section_references(frame_source, base, clean_word) {
            let kind = ReferenceKind::TokenUsage;
            if !include_declaration && kind == ReferenceKind::Declaration {
                continue;
            }
            targets.push(ReferenceTarget {
                target: NavigationTarget {
                    span: target.span,
                    path: target.path,
                },
                kind,
            });
        }
    }

    // Included file references
    if let Ok(current_path) = uri.to_file_path() {
        for (path, include_source, _, _) in
            crate::project::resolve_includes(&current_path, frame_source)
        {
            let include_doc = parse(&include_source).ok();
            for span in crate::project::collect_references_in_source(&include_source, clean_word) {
                let kind = include_doc
                    .as_ref()
                    .map(|doc| classify_span(&include_source, span, clean_word, doc, &cursor))
                    .unwrap_or(ReferenceKind::TokenUsage);
                if !include_declaration && kind == ReferenceKind::Declaration {
                    continue;
                }
                targets.push(ReferenceTarget {
                    target: NavigationTarget {
                        span,
                        path: Some(path.clone()),
                    },
                    kind,
                });
            }
        }
    }

    targets.sort_by_key(|t| (t.target.path.clone(), t.target.span.start));
    targets.dedup_by(|a, b| a.target.span == b.target.span && a.target.path == b.target.path);
    targets
}

#[allow(dead_code)]
pub fn references_at(source: &str, offset: usize, uri: &Url) -> Vec<NavigationTarget> {
    references_at_with_context(source, offset, uri, true)
        .into_iter()
        .map(|r| r.target)
        .collect()
}

fn classify_span(
    source: &str,
    span: Span,
    word: &str,
    document: &Document,
    cursor: &SemanticCursor,
) -> ReferenceKind {
    // Declarations
    for decl in &document.declarations {
        if decl.name.span == span && decl.name.text == word {
            return ReferenceKind::Declaration;
        }
    }

    // Components
    for component in &document.components {
        if component.name.span == span && component.name.text == word {
            return ReferenceKind::Declaration;
        }
    }

    // Component props
    for component in &document.components {
        if let Some(props) = &component.props {
            for prop in &props.values {
                if prop.name.span == span && prop.name.text == word {
                    return ReferenceKind::Declaration;
                }
            }
        }
    }

    // Component state
    for component in &document.components {
        if let Some(state) = &component.state {
            for value in &state.values {
                if value.name.span == span && value.name.text == word {
                    return ReferenceKind::Declaration;
                }
            }
        }
    }

    // Handler reference (@word)
    if source.as_bytes()[span.start] == b'@' {
        return ReferenceKind::HandlerReference;
    }

    // Data reference ($word)
    if source.as_bytes()[span.start] == b'$' {
        let before = &source[..span.start];
        if before.trim_end().ends_with("bind") {
            return ReferenceKind::StateWriteBinding;
        }
        if cursor.scope.local_props.iter().any(|s| s.name == word) {
            return ReferenceKind::PropRead;
        }
        return ReferenceKind::StateRead;
    }

    // Include usage
    let line_start = source[..span.start].rfind('\n').map_or(0, |i| i + 1);
    let line_end = source[span.start..]
        .find('\n')
        .map_or(source.len(), |i| span.start + i);
    let line_text = source[line_start..line_end].trim();
    if line_text.starts_with("#include") {
        return ReferenceKind::IncludeUsage;
    }

    // Component invocation (word followed by '(')
    if span.end < source.len() && source[span.end..].starts_with('(') {
        return ReferenceKind::ComponentInvocation;
    }

    // Style application (word preceded by ':')
    if span.start > 0 && source.as_bytes()[span.start - 1] == b':' {
        return ReferenceKind::StyleApplication;
    }

    // Automatic style lookup (style when ... = word)
    if line_text.contains("when") && line_text.ends_with(word) {
        return ReferenceKind::AutomaticStyleLookup;
    }

    // Token usage inside tokens declarations
    for decl in &document.declarations {
        if matches!(decl.kind, DeclarationKind::Tokens)
            && decl.span.start <= span.start
            && decl.span.end >= span.end
        {
            return ReferenceKind::TokenUsage;
        }
    }

    ReferenceKind::TokenUsage
}

fn clean_word_for_references(word: &str) -> &str {
    let word = word.split('(').next().unwrap_or(word);
    if let Some(pos) = word.find(':') {
        // For "Send:PrimaryButton", return the style name "PrimaryButton"
        &word[pos + 1..]
    } else if let Some(stripped) = word.strip_prefix('$') {
        stripped
    } else if let Some(stripped) = word.strip_prefix('@') {
        stripped
    } else {
        word
    }
}

fn collect_grid_section_references(source: &str, base: usize, word: &str) -> Vec<NavigationTarget> {
    let symbols = collect_document_symbols(source);
    let mut targets = Vec::new();

    for columns in symbols.grid_columns.values() {
        if let Some(span) = columns.get(word) {
            targets.push(NavigationTarget {
                span: add_base(*span, base),
                path: None,
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
                        path: None,
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

    fn dummy_uri() -> Url {
        Url::parse("file:///dummy.frame").unwrap()
    }

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
        let references = references_at(source, offset, &dummy_uri());

        assert_eq!(references.len(), 2);
    }

    #[test]
    fn finds_component_invocation_references() {
        let source = r#"
component ChatPanel {
  view {
    text "Hello"
  }
}
component ChatApp {
  view {
    ChatPanel()
  }
}
"#;
        let offset = source.find("ChatPanel()").unwrap() + 2;
        let references = references_at(source, offset, &dummy_uri());

        let names: Vec<String> = references
            .iter()
            .map(|r| source[r.span.start..r.span.end].to_string())
            .collect();
        assert!(names.contains(&"ChatPanel".to_string()));
        assert_eq!(references.len(), 2);
    }

    #[test]
    fn finds_style_binding_references() {
        let source = r#"
card PrimaryButton {
  surface accent
}
component ChatApp {
  view {
    button Send:PrimaryButton {
      text "Send"
    }
  }
}
"#;
        let offset = source.rfind("PrimaryButton").unwrap() + 2;
        let references = references_at(source, offset, &dummy_uri());

        let names: Vec<String> = references
            .iter()
            .map(|r| source[r.span.start..r.span.end].to_string())
            .collect();
        assert!(names.contains(&"PrimaryButton".to_string()));
        assert_eq!(references.len(), 2);
    }

    #[test]
    fn finds_handler_reference_references() {
        let source = r#"
component ChatApp {
  view {
    button Send {
      on click @sendMessage
    }
    button Cancel {
      on press @sendMessage
    }
  }
}
"#;
        let offset = source.find("@sendMessage").unwrap() + 2;
        let references = references_at(source, offset, &dummy_uri());

        assert_eq!(references.len(), 2);
        for r in &references {
            assert_eq!(&source[r.span.start..r.span.end], "@sendMessage");
        }
    }

    #[test]
    fn finds_state_reference_references() {
        let source = r#"
component ChatApp {
  state {
    draft text = ""
  }
  view {
    input MessageBox {
      value bind $draft
    }
    text $draft
  }
}
"#;
        let offset = source.rfind("$draft").unwrap() + 2;
        let references = references_at(source, offset, &dummy_uri());

        let names: Vec<String> = references
            .iter()
            .map(|r| source[r.span.start..r.span.end].to_string())
            .collect();
        assert!(names.contains(&"$draft".to_string()));
        assert_eq!(references.len(), 3); // state decl + bind + text
    }

    #[test]
    fn finds_cross_file_declaration_references() {
        let root =
            std::env::temp_dir().join(format!("frame-nav-references-{}", std::process::id()));
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
        let offset = source.rfind("brand-panel").unwrap() + 2;

        let references = references_at(source, offset, &uri);

        // Current file: "background brand-panel" in card Hero
        // Included file: "color brand-panel" in tokens Brand + "background brand-panel" in card Imported
        assert_eq!(references.len(), 3);

        let local_count = references.iter().filter(|r| r.path.is_none()).count();
        let cross_count = references.iter().filter(|r| r.path.is_some()).count();

        assert_eq!(local_count, 1);
        assert_eq!(cross_count, 2);
    }

    #[test]
    fn finds_cross_file_component_invocation_references() {
        let root =
            std::env::temp_dir().join(format!("frame-nav-comp-references-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let components = root.join("components.frame");

        std::fs::write(
            &components,
            "component MessageItem {\n  view {\n    text \"Hello\"\n  }\n}\n",
        )
        .expect("components should be writable");

        let source =
            "#include components\n\ncomponent ChatApp {\n  view {\n    MessageItem()\n  }\n}\n";
        std::fs::write(&app, source).expect("app should be writable");

        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.find("MessageItem()").unwrap() + 2;

        let references = references_at(source, offset, &uri);

        // Local: MessageItem() invocation
        // Cross-file: component MessageItem declaration
        assert_eq!(references.len(), 2);

        let local_count = references.iter().filter(|r| r.path.is_none()).count();
        let cross_count = references.iter().filter(|r| r.path.is_some()).count();

        assert_eq!(local_count, 1);
        assert_eq!(cross_count, 1);
    }

    #[test]
    fn include_declaration_true_returns_declaration_and_usages() {
        let source = "grid Dashboard {\n}\narea Sidebar {\n  in Dashboard\n}\n";
        let offset = source.find("Dashboard").unwrap() + 1;
        let references = references_at_with_context(source, offset, &dummy_uri(), true);

        let declarations: Vec<_> = references
            .iter()
            .filter(|r| r.kind == ReferenceKind::Declaration)
            .collect();
        let usages: Vec<_> = references
            .iter()
            .filter(|r| r.kind != ReferenceKind::Declaration)
            .collect();

        assert_eq!(declarations.len(), 1);
        assert_eq!(usages.len(), 1);
        assert_eq!(references.len(), 2);
    }

    #[test]
    fn include_declaration_false_returns_only_usages() {
        let source = "grid Dashboard {\n}\narea Sidebar {\n  in Dashboard\n}\n";
        let offset = source.find("Dashboard").unwrap() + 1;
        let references = references_at_with_context(source, offset, &dummy_uri(), false);

        assert!(references
            .iter()
            .all(|r| r.kind != ReferenceKind::Declaration));
        assert_eq!(references.len(), 1);
        let r = &references[0];
        assert_eq!(&source[r.target.span.start..r.target.span.end], "Dashboard");
    }

    #[test]
    fn state_declaration_and_reads_classified_correctly() {
        let source = r#"
component ChatApp {
  state {
    draft text = ""
  }
  view {
    input MessageBox {
      value bind $draft
    }
    text $draft
  }
}
"#;
        let offset = source.rfind("$draft").unwrap() + 2;
        let references = references_at_with_context(source, offset, &dummy_uri(), true);

        let declarations: Vec<_> = references
            .iter()
            .filter(|r| r.kind == ReferenceKind::Declaration)
            .collect();
        let state_reads: Vec<_> = references
            .iter()
            .filter(|r| r.kind == ReferenceKind::StateRead)
            .collect();
        let write_bindings: Vec<_> = references
            .iter()
            .filter(|r| r.kind == ReferenceKind::StateWriteBinding)
            .collect();

        assert_eq!(declarations.len(), 1);
        assert_eq!(state_reads.len(), 1);
        assert_eq!(write_bindings.len(), 1);
        assert_eq!(references.len(), 3);
    }

    #[test]
    fn handler_references_classified_correctly() {
        let source = r#"
component ChatApp {
  view {
    button Send {
      on click @sendMessage
    }
    button Cancel {
      on press @sendMessage
    }
  }
}
"#;
        let offset = source.find("@sendMessage").unwrap() + 2;
        let references = references_at_with_context(source, offset, &dummy_uri(), true);

        assert!(references
            .iter()
            .all(|r| r.kind == ReferenceKind::HandlerReference));
        assert_eq!(references.len(), 2);
    }

    #[test]
    fn component_declaration_and_invocation_classified_correctly() {
        let source = r#"
component ChatPanel {
  view {
    text "Hello"
  }
}
component ChatApp {
  view {
    ChatPanel()
  }
}
"#;
        let offset = source.find("ChatPanel()").unwrap() + 2;
        let references = references_at_with_context(source, offset, &dummy_uri(), true);

        let declarations: Vec<_> = references
            .iter()
            .filter(|r| r.kind == ReferenceKind::Declaration)
            .collect();
        let invocations: Vec<_> = references
            .iter()
            .filter(|r| r.kind == ReferenceKind::ComponentInvocation)
            .collect();

        assert_eq!(declarations.len(), 1);
        assert_eq!(invocations.len(), 1);
        assert_eq!(references.len(), 2);
    }
}
