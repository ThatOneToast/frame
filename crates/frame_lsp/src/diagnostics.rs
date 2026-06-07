use std::{collections::HashSet, fs, path::Path};

use frame_core::{semantic::validate, Diagnostic as FrameDiagnostic, Severity, Span};
use frame_parser::parse;
use tower_lsp::lsp_types::{Diagnostic as LspDiagnostic, DiagnosticSeverity, Position, Range, Url};

use crate::embedded::{frame_blocks, map_diagnostic_from_block};

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

fn diagnostics_for_frame_source(source: &str) -> Vec<FrameDiagnostic> {
    match parse(source) {
        Ok(document) => validate(&document),
        Err(error) => error.diagnostics,
    }
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
}
