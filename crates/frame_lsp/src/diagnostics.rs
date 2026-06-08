use std::{collections::HashSet, fs, path::Path};

use frame_core::{language, semantic::validate, Diagnostic as FrameDiagnostic, Severity, Span};
use frame_parser::parse;
use tower_lsp::lsp_types::{Diagnostic as LspDiagnostic, DiagnosticSeverity, Position, Range, Url};

use crate::embedded::{frame_blocks, map_diagnostic_from_block};
use crate::ide::cursor::{CursorSlot, SemanticCursor};

pub fn diagnostics_for_source(source: &str) -> Vec<FrameDiagnostic> {
    let blocks = frame_blocks(source);
    if !blocks.is_empty() {
        return blocks
            .into_iter()
            .flat_map(|block| {
                diagnostics_for_frame_source(block.content)
                    .into_iter()
                    .map(move |diagnostic| map_diagnostic_from_block(diagnostic, &block))
            })
            .collect();
    }

    diagnostics_for_frame_source(source)
}

pub fn diagnostics_for_uri(source: &str, uri: &Url) -> Vec<FrameDiagnostic> {
    let mut diagnostics = diagnostics_for_source_with_imports(source, uri);
    diagnostics.extend(include_diagnostics(source, uri));
    diagnostics.extend(cross_file_diagnostics(source, uri));
    diagnostics
}

fn diagnostics_for_source_with_imports(source: &str, uri: &Url) -> Vec<FrameDiagnostic> {
    let prefix = included_source_prefix(source, uri);
    if prefix.is_empty() {
        return diagnostics_for_source(source);
    }

    let prefix_len = prefix.len() + 1;
    let merged = format!("{prefix}\n{source}");
    diagnostics_for_source(&merged)
        .into_iter()
        .filter_map(|mut diagnostic| {
            if diagnostic.span.start < prefix_len {
                return None;
            }
            diagnostic.span.start -= prefix_len;
            diagnostic.span.end = diagnostic.span.end.saturating_sub(prefix_len);
            Some(diagnostic)
        })
        .collect()
}

pub fn cursor_diagnostics(source: &str, offset: usize) -> Vec<FrameDiagnostic> {
    let cursor = SemanticCursor::at(source, offset);
    let mut diagnostics = Vec::new();
    let span = word_span_at(source, offset);

    match &cursor.slot {
        CursorSlot::ViewPrimitive | CursorSlot::ViewBody | CursorSlot::ViewNodeName => {
            if let Some(word) = &cursor.word {
                if !word.starts_with('$')
                    && !word.starts_with('@')
                    && !word.starts_with('"')
                    && language::item(word).is_none()
                {
                    let message = format!(
                        "Unknown UI primitive `{word}`. Frame prefers UI-native primitives:\n\
                         - `panel` for visible content regions\n\
                         - `stack` for vertical groups\n\
                         - `dock` for app shell layout\n\
                         - `action` for clickable intent\n"
                    );
                    diagnostics.push(FrameDiagnostic::error(message, span));
                }
            }
        }
        CursorSlot::StylePropertyName => {
            if let Some(word) = &cursor.word {
                if !language::property_keywords().contains(&word.as_str()) {
                    let suggestion = closest_match(word, language::property_keywords())
                        .map(|v| format!("\n\nDid you mean `{v}`?"))
                        .unwrap_or_default();
                    diagnostics.push(FrameDiagnostic::error(
                        format!(
                            "Unknown property `{word}`.{suggestion} Use Frame-native layout concepts such as `surface`, `padding`, `gap`, or `align`."
                        ),
                        span,
                    ));
                }
            }
        }
        CursorSlot::StylePropertyValue { property } => {
            if !language::property_keywords().contains(&property.as_str()) {
                let suggestion = closest_match(property, language::property_keywords())
                    .map(|v| format!("\n\nDid you mean `{v}`?"))
                    .unwrap_or_default();
                diagnostics.push(FrameDiagnostic::error(
                    format!(
                        "Unknown property `{property}`.{suggestion} Use Frame-native layout concepts such as `surface`, `padding`, `gap`, or `align`."
                    ),
                    span,
                ));
            } else if let Some(word) = &cursor.word {
                if word != property {
                    if let Some(item) = language::item(property) {
                        if !item.values.is_empty()
                            && !item.values.contains(&word.as_str())
                            && !language::is_known_value(word)
                        {
                            let values = item
                                .values
                                .iter()
                                .map(|v| format!("`{v}`"))
                                .collect::<Vec<_>>()
                                .join(", ");
                            diagnostics.push(FrameDiagnostic::error(
                                format!(
                                    "Invalid value `{word}` for property `{property}`. Valid values include: {values}."
                                ),
                                span,
                            ));
                        }
                    }
                }
            }
        }
        CursorSlot::HandlerReference => {
            if let Some(word) = &cursor.word {
                let handler_name = word.trim_start_matches('@');
                let known = cursor.scope.handlers.iter().any(|h| h.name == handler_name);
                if !known {
                    let suggestion = if cursor.scope.handlers.is_empty() {
                        "\n\nNo handlers are declared in this component.".to_string()
                    } else {
                        let candidates: Vec<&str> = cursor
                            .scope
                            .handlers
                            .iter()
                            .map(|h| h.name.as_str())
                            .collect();
                        closest_match(handler_name, &candidates)
                            .map(|v| format!("\n\nDid you mean `@{v}`?"))
                            .unwrap_or_default()
                    };
                    diagnostics.push(FrameDiagnostic::error(
                        format!(
                            "Unknown handler reference `@{handler_name}`.{suggestion} Define the handler in your external script or add it to the component."
                        ),
                        span,
                    ));
                }
            }
        }
        CursorSlot::DataReference => {
            if let Some(word) = &cursor.word {
                let data_name = word.trim_start_matches('$');
                let known = cursor.scope.local_state.iter().any(|s| s.name == data_name)
                    || cursor.scope.local_props.iter().any(|p| p.name == data_name)
                    || cursor.scope.loop_vars.iter().any(|v| v.name == data_name);
                if !known {
                    let all_names: Vec<&str> = cursor
                        .scope
                        .local_state
                        .iter()
                        .chain(&cursor.scope.local_props)
                        .chain(&cursor.scope.loop_vars)
                        .map(|s| s.name.as_str())
                        .collect();
                    let suggestion = closest_match(data_name, &all_names)
                        .map(|v| format!("\n\nDid you mean `${v}`?"))
                        .unwrap_or_default();
                    diagnostics.push(FrameDiagnostic::error(
                        format!(
                            "Unknown state/prop reference `${data_name}`.{suggestion} Declare it in `state`, `props`, or an enclosing `for` loop."
                        ),
                        span,
                    ));
                }
            }
        }
        _ => {}
    }

    diagnostics
}

fn word_span_at(source: &str, offset: usize) -> Span {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind(|c: char| !is_word_char(c))
        .map_or(0, |i| i + 1);
    let end = source[safe_offset..]
        .find(|c: char| !is_word_char(c))
        .map_or(source.len(), |i| safe_offset + i);
    Span { start, end }
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '$' || c == '@'
}

fn closest_match<'a>(needle: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .copied()
        .map(|candidate| (candidate, edit_distance(needle, candidate)))
        .filter(|(_, distance)| *distance <= 2)
        .min_by_key(|(_, distance)| *distance)
        .map(|(candidate, _)| candidate)
}

fn edit_distance(left: &str, right: &str) -> usize {
    let mut costs = (0..=right.len()).collect::<Vec<_>>();
    for (left_index, left_char) in left.chars().enumerate() {
        let mut previous = left_index;
        costs[0] = left_index + 1;
        for (right_index, right_char) in right.chars().enumerate() {
            let old = costs[right_index + 1];
            costs[right_index + 1] = if left_char == right_char {
                previous
            } else {
                1 + previous.min(costs[right_index]).min(old)
            };
            previous = old;
        }
    }
    *costs.last().unwrap_or(&0)
}

fn spans_overlap(a: Span, b: Span) -> bool {
    a.start < b.end && b.start < a.end
}

fn diagnostics_for_frame_source(source: &str) -> Vec<FrameDiagnostic> {
    let parse_result = parse(source);
    let mut diagnostics = match &parse_result {
        Ok(document) => validate(document),
        Err(error) => error.diagnostics.clone(),
    };

    // Only add cursor diagnostics when the source parses successfully.
    // When the AST is missing the cursor slot heuristics are unreliable.
    if parse_result.is_ok() {
        let mut offset = 0;
        for line in source.lines() {
            let line_end = offset + line.len();
            let trimmed = line.trim_start();
            if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("//") {
                if let Some(first_word) = trimmed.split_whitespace().next() {
                    let word_offset = offset + line.find(first_word).unwrap_or(0);
                    let word_end = word_offset + first_word.len();
                    let check_offset = word_offset + (word_end - word_offset) / 2;
                    for d in cursor_diagnostics(source, check_offset) {
                        let overlaps = diagnostics
                            .iter()
                            .any(|existing| spans_overlap(existing.span, d.span));
                        if !overlaps {
                            diagnostics.push(d);
                        }
                    }
                }
            }
            offset = line_end + 1;
        }
    }

    diagnostics
}

fn include_diagnostics(source: &str, uri: &Url) -> Vec<FrameDiagnostic> {
    let Ok(path) = uri.to_file_path() else {
        return Vec::new();
    };
    let Some(parent) = path.parent() else {
        return Vec::new();
    };

    source
        .lines()
        .scan(0usize, |offset, line| {
            let start = *offset;
            *offset += line.len() + 1;
            Some((start, line))
        })
        .filter_map(|(line_start, line)| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("#include") {
                return None;
            }
            let target = trimmed.split_whitespace().nth(1)?;
            let target_start = line_start + line.find(target).unwrap_or(0);
            let target_path = include_candidate(parent, target);
            if !target_path.exists() {
                return Some(FrameDiagnostic::error(
                    format!(
                        "Could not resolve include `{target}`.\n\nSearched:\n- {}",
                        target_path.display()
                    ),
                    Span {
                        start: target_start,
                        end: target_start + target.len(),
                    },
                ));
            }

            let root = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
            let mut stack = vec![root];
            let mut seen = HashSet::new();
            detect_cycle(&target_path, &mut stack, &mut seen).map(|cycle| {
                FrameDiagnostic::error(
                    format!(
                        "Include cycle detected:\n\n{}\n\nRemove one include to break the cycle.",
                        cycle
                    ),
                    Span {
                        start: target_start,
                        end: target_start + target.len(),
                    },
                )
            })
        })
        .collect()
}

fn cross_file_diagnostics(source: &str, uri: &Url) -> Vec<FrameDiagnostic> {
    let Ok(path) = uri.to_file_path() else {
        return Vec::new();
    };
    let mut diagnostics = Vec::new();

    // Unresolved imported component references
    if let Ok(document) = parse(source) {
        let all_components = crate::project::merged_component_names(&path, source);
        for component in &document.components {
            if let Some(view) = &component.view {
                for node in &view.nodes {
                    check_component_references(node, &all_components, source, &mut diagnostics);
                }
            }
            for slot in &component.slots {
                for node in &slot.nodes {
                    check_component_references(node, &all_components, source, &mut diagnostics);
                }
            }
        }
    }

    // Duplicate symbols across files
    for (name, locations) in crate::project::duplicate_symbols(&path, source) {
        let paths: Vec<String> = locations
            .iter()
            .map(|(p, _)| p.display().to_string())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        for (location_path, span) in locations {
            if location_path == path {
                diagnostics.push(FrameDiagnostic::warning(
                    format!(
                        "Duplicate symbol `{}` defined in multiple files.\n\nFound in:\n{}\n\nRename one declaration or remove the duplicate include.",
                        name,
                        paths.iter().map(|p| format!("- {}", p)).collect::<Vec<_>>().join("\n")
                    ),
                    span,
                ));
            }
        }
    }

    // Shadowed symbols
    for (name, shadow_path, span) in crate::project::shadowed_symbols(&path, source) {
        diagnostics.push(FrameDiagnostic::warning(
            format!(
                "Imported symbol `{}` from `{}` shadows a local symbol.\n\nThis can confuse resolution. Rename the local symbol or the imported one.",
                name,
                shadow_path.display()
            ),
            span,
        ));
    }

    diagnostics
}

#[allow(clippy::only_used_in_recursion)]
fn check_component_references(
    node: &frame_core::UiNode,
    all_components: &std::collections::HashSet<String>,
    source: &str,
    diagnostics: &mut Vec<FrameDiagnostic>,
) {
    match node {
        frame_core::UiNode::Component(invocation) => {
            if !all_components.contains(&invocation.name.text) {
                diagnostics.push(FrameDiagnostic::error(
                    format!(
                        "Unknown component `{}`.\n\nDeclare `component {} {{ ... }}` in this file or import it with `#include path`.",
                        invocation.name.text, invocation.name.text
                    ),
                    invocation.name.span,
                ));
            }
        }
        frame_core::UiNode::Element(element) => {
            for child in &element.children {
                check_component_references(child, all_components, source, diagnostics);
            }
        }
        frame_core::UiNode::Loop(loop_node) => {
            for child in &loop_node.children {
                check_component_references(child, all_components, source, diagnostics);
            }
        }
        frame_core::UiNode::Text(_) => {}
    }
}

fn include_candidate(parent: &Path, target: &str) -> std::path::PathBuf {
    let candidate = parent.join(target);
    if candidate.extension().is_some() {
        candidate
    } else {
        candidate.with_extension("frame")
    }
}

fn included_source_prefix(source: &str, uri: &Url) -> String {
    let Ok(path) = uri.to_file_path() else {
        return String::new();
    };
    let Some(parent) = path.parent() else {
        return String::new();
    };

    let mut seen = HashSet::new();
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("#include") {
                return None;
            }
            let target = trimmed.split_whitespace().nth(1)?;
            let candidate = include_candidate(parent, target);
            read_include_recursive(&candidate, &mut seen)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_include_recursive(path: &Path, seen: &mut HashSet<std::path::PathBuf>) -> Option<String> {
    let canonical = fs::canonicalize(path).ok()?;
    if !seen.insert(canonical.clone()) {
        return None;
    }
    let source = fs::read_to_string(&canonical).ok()?;
    let parent = canonical.parent()?;
    let prefix = source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("#include") {
                return None;
            }
            let target = trimmed.split_whitespace().nth(1)?;
            read_include_recursive(&include_candidate(parent, target), seen)
        })
        .collect::<Vec<_>>()
        .join("\n");
    Some(if prefix.is_empty() {
        source
    } else {
        format!("{prefix}\n{source}")
    })
}

fn detect_cycle(
    path: &Path,
    stack: &mut Vec<std::path::PathBuf>,
    seen: &mut HashSet<std::path::PathBuf>,
) -> Option<String> {
    let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    if let Some(index) = stack.iter().position(|item| item == &canonical) {
        return Some(
            stack[index..]
                .iter()
                .chain(std::iter::once(&canonical))
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(" -> "),
        );
    }
    if !seen.insert(canonical.clone()) {
        return None;
    }

    let source = fs::read_to_string(&canonical).ok()?;
    let parent = canonical.parent()?.to_path_buf();
    stack.push(canonical);
    for line in source.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("#include") {
            continue;
        }
        let Some(target) = trimmed.split_whitespace().nth(1) else {
            continue;
        };
        let candidate = include_candidate(&parent, target);
        if let Some(cycle) = detect_cycle(&candidate, stack, seen) {
            return Some(cycle);
        }
    }
    stack.pop();
    None
}

pub fn to_lsp_diagnostic(source: &str, diagnostic: FrameDiagnostic) -> LspDiagnostic {
    LspDiagnostic {
        range: range_for_span(source, diagnostic.span),
        severity: Some(match diagnostic.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
        }),
        source: Some("frame".to_string()),
        message: diagnostic.message,
        ..LspDiagnostic::default()
    }
}

pub fn range_for_span(source: &str, span: Span) -> Range {
    let start = position_for_offset(source, span.start);
    let mut end = position_for_offset(source, span.end);

    if start == end {
        end.character += 1;
    }

    Range { start, end }
}

pub fn position_for_offset(source: &str, offset: usize) -> Position {
    let mut line = 0;
    let mut character = 0;

    for (index, value) in source.char_indices() {
        if index >= offset {
            break;
        }

        if value == '\n' {
            line += 1;
            character = 0;
        } else {
            character += value.len_utf16() as u32;
        }
    }

    Position { line, character }
}

pub fn offset_for_position(source: &str, position: Position) -> usize {
    let mut line = 0;
    let mut character = 0;

    for (index, value) in source.char_indices() {
        if line == position.line && character == position.character {
            return index;
        }

        if value == '\n' {
            line += 1;
            character = 0;
        } else {
            character += value.len_utf16() as u32;
        }
    }

    source.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_byte_offsets_to_utf16_positions() {
        let source = "grid A {\n  gap médium\n}\n";
        let offset = source.find("médium").expect("sample contains value");

        assert_eq!(
            position_for_offset(source, offset),
            Position {
                line: 1,
                character: 6,
            }
        );
    }

    #[test]
    fn returns_parser_diagnostics() {
        let diagnostics = diagnostics_for_source("card Broken {\n  magic {\n  }\n}\n");

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("unknown nested block"));
    }

    #[test]
    fn returns_semantic_diagnostics() {
        let diagnostics = diagnostics_for_source(
            "grid AppShell {\n  columns sidebar\n}\narea Sidebar {\n  in Missing\n}\n",
        );

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("unknown grid"));
    }

    #[test]
    fn returns_embedded_svelte_frame_diagnostics() {
        let source = "<div />\n<style lang=\"frame\">\nunknown Broken {\n}\n</style>\n";
        let diagnostics = diagnostics_for_source(source);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("unknown declaration"));
        assert!(diagnostics[0].span.start > source.find("<style").unwrap());
    }

    #[test]
    fn cursor_diagnoses_unknown_ui_primitive() {
        let source = "component ChatApp {\n  view {\n    div Broken {\n    }\n  }\n}\n";
        let offset = source.find("div").unwrap() + 1;
        let diagnostics = cursor_diagnostics(source, offset);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("Unknown UI primitive"));
        assert!(diagnostics[0].message.contains("panel"));
    }

    #[test]
    fn cursor_diagnoses_unknown_property() {
        let source = "card Demo {\n  magic\n}\n";
        let offset = source.find("magic").unwrap() + 1;
        let diagnostics = cursor_diagnostics(source, offset);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("Unknown property"));
        assert!(diagnostics[0].message.contains("surface"));
    }

    #[test]
    fn cursor_diagnoses_invalid_value() {
        let source = "card Demo {\n  surface unknown\n}\n";
        let offset = source.find("unknown").unwrap() + 1;
        let diagnostics = cursor_diagnostics(source, offset);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("Invalid value"));
        assert!(diagnostics[0].message.contains("surface"));
    }

    #[test]
    fn cursor_diagnoses_unknown_handler() {
        let source = "component ChatApp {\n  view {\n    composer MessageComposer {\n      send @missing\n    }\n  }\n}\n";
        let offset = source.find("@missing").unwrap() + 1;
        let diagnostics = cursor_diagnostics(source, offset);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("Unknown handler"));
    }

    #[test]
    fn cursor_diagnoses_unknown_state_reference() {
        let source = "component ChatApp {\n  view {\n    text $missing\n  }\n}\n";
        let offset = source.find("$missing").unwrap() + 1;
        let diagnostics = cursor_diagnostics(source, offset);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("Unknown state/prop"));
    }

    #[test]
    fn no_duplicate_cursor_diagnostics() {
        let source = "component ChatApp {\n  view {\n    unknown Broken {\n    }\n  }\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let primitive_errors = diagnostics
            .iter()
            .filter(|d| d.message.contains("Unknown UI primitive"))
            .count();
        assert_eq!(primitive_errors, 1);
    }
}
