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
    // Also merge the implicit theme file if it exists and isn't already included.
    let theme_prefix = theme_source_prefix(source, uri);
    let full_prefix = if prefix.is_empty() {
        theme_prefix
    } else if theme_prefix.is_empty() {
        prefix
    } else {
        format!("{prefix}\n{theme_prefix}")
    };
    if full_prefix.is_empty() {
        return diagnostics_for_source(source);
    }

    let prefix_len = full_prefix.len() + 1;
    let merged = format!("{full_prefix}\n{source}");
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
        CursorSlot::ViewPrimitive => {
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
        CursorSlot::ViewNodeName => {
            // Node names are user-chosen identifiers, not primitives.
            // Do not flag them as unknown primitives.
        }
        CursorSlot::ViewBody => {
            // Inside a view body, check if the word is a known primitive or declaration.
            // If it's not, it might be a component invocation or an unknown element.
            if let Some(word) = &cursor.word {
                // Skip loop variable names (for item in $items ...)
                if let Some(ref block) = cursor.innermost_block {
                    if block.starts_with("for ") {
                        return diagnostics;
                    }
                }
                // Also skip if the current line is starting a for loop
                if cursor.line_prefix.trim().starts_with("for ") {
                    return diagnostics;
                }
                if !word.starts_with('$')
                    && !word.starts_with('@')
                    && !word.starts_with('"')
                    && !word.starts_with('(')
                    && language::item(word).is_none()
                    && !cursor
                        .scope
                        .local_declarations
                        .iter()
                        .any(|d| d.name == *word)
                {
                    // Only flag if it looks like it could be a primitive (lowercase, no parens)
                    // Component names start with uppercase and have ( for invocations
                    if !word.starts_with(char::is_uppercase)
                        && cursor.enclosing_declaration.is_none()
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
        }
        CursorSlot::StylePropertyName => {
            if let Some(word) = &cursor.word {
                // Skip nested block keywords inside declarations
                if is_declaration_nested_block_keyword(word) {
                    return diagnostics;
                }
                // Skip validation inside nested blocks (gradient, animation, etc.)
                // where the property keywords are different
                if let Some(ref block) = cursor.innermost_block {
                    let block_first = block.split_whitespace().next().unwrap_or("");
                    if matches!(
                        block_first,
                        "gradient"
                            | "animation"
                            | "section"
                            | "from"
                            | "to"
                            | "below"
                            | "above"
                            | "between"
                            | "container"
                    ) || language::state_keywords().contains(&block_first)
                        || is_percentage_selector(block_first)
                    {
                        return diagnostics;
                    }
                }
                // Also skip if the current line is starting a nested block
                if is_line_starting_nested_block(&cursor.line_prefix) {
                    return diagnostics;
                }
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
            // Skip validation inside nested blocks (gradient, animation, etc.)
            // where the property keywords are different
            if let Some(ref block) = cursor.innermost_block {
                let block_first = block.split_whitespace().next().unwrap_or("");
                if matches!(
                    block_first,
                    "gradient"
                        | "animation"
                        | "section"
                        | "from"
                        | "to"
                        | "below"
                        | "above"
                        | "between"
                        | "container"
                ) || language::state_keywords().contains(&block_first)
                    || is_percentage_selector(block_first)
                {
                    return diagnostics;
                }
            }
            // Also skip if the current line is starting a nested block
            if is_line_starting_nested_block(&cursor.line_prefix) {
                return diagnostics;
            }
            // Skip validation for declaration keywords (e.g., "keyframes FloatIn")
            // These are declaration headers, not property-value pairs
            if language::declaration_keywords().contains(&property.as_str()) {
                return diagnostics;
            }
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
                            && !is_dynamic_grid_value(word, property)
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

fn is_declaration_nested_block_keyword(word: &str) -> bool {
    // State keywords that appear as nested blocks in declarations
    if language::state_keywords().contains(&word) {
        return true;
    }
    // Nested block first-word keywords
    let first = word.split_whitespace().next().unwrap_or(word);
    matches!(
        first,
        "gradient"
            | "section"
            | "animation"
            | "below"
            | "above"
            | "between"
            | "container"
            | "selected"
            | "advanced"
            | "from"
            | "to"
    ) || is_percentage_selector(word)
}

fn is_line_starting_nested_block(line_prefix: &str) -> bool {
    // Check if the line prefix looks like it's starting a nested block
    // (e.g., "hover {" or "gradient hero-gradient {")
    let trimmed = line_prefix.trim();
    let first = trimmed.split_whitespace().next().unwrap_or("");
    is_declaration_nested_block_keyword(first)
}

fn quote_aware_tokens(line: &str, line_offset: usize) -> Vec<(&str, usize)> {
    let mut tokens = Vec::new();
    let mut in_quote = false;
    let mut token_start = None;
    for (i, c) in line.char_indices() {
        if c == '"' {
            if in_quote {
                // Closing quote — end of quoted segment, skip it entirely
                in_quote = false;
                token_start = None;
            } else {
                // Opening quote — start skipping; also capture any preceding unquoted token
                if let Some(start) = token_start.take() {
                    let tok = &line[start..i];
                    if !tok.is_empty() {
                        tokens.push((tok, line_offset + start));
                    }
                }
                in_quote = true;
            }
        } else if !in_quote {
            if c.is_whitespace() {
                if let Some(start) = token_start.take() {
                    let tok = &line[start..i];
                    if !tok.is_empty() {
                        tokens.push((tok, line_offset + start));
                    }
                }
            } else if token_start.is_none() {
                token_start = Some(i);
            }
        }
    }
    // Flush trailing unquoted token
    if !in_quote {
        if let Some(start) = token_start {
            let tok = &line[start..];
            if !tok.is_empty() {
                tokens.push((tok, line_offset + start));
            }
        }
    }
    tokens
}

fn is_percentage_selector(name: &str) -> bool {
    name.strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn is_dynamic_grid_value(word: &str, property: &str) -> bool {
    // Grid track properties (columns, rows, tracks) accept dynamic values:
    // percentages (60%), fr units (2fr), minmax(), fit-content(), named tracks, etc.
    let is_grid_property = matches!(
        property,
        "columns" | "rows" | "tracks" | "grid-columns" | "grid-rows"
    );
    if !is_grid_property {
        return false;
    }
    // Allow bare numbers (cursor may strip %), fr units, and grid functions
    if word.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    if let Some(rest) = word.strip_prefix("responsive") {
        return rest.is_empty() || rest.starts_with(' ');
    }
    // Allow fr units (1fr, 2fr, etc.)
    if word.ends_with("fr") {
        return true;
    }
    // Allow percentages
    if word.ends_with('%') {
        return true;
    }
    // Grid functions and keywords
    if matches!(
        word,
        "subgrid" | "minmax" | "fit-content" | "auto" | "fill" | "min" | "max"
    ) {
        return true;
    }
    // Named grid track identifiers are valid (e.g. sidebar, content, header, body)
    // They must be valid identifiers: start with a letter or underscore, contain alphanumeric/dash/underscore
    if (word.starts_with(char::is_alphabetic) || word.starts_with('_'))
        && word.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return true;
    }
    false
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
                // Use a quote-aware token splitter to avoid checking words
                // inside quoted strings (e.g. `text "4 models online"`).
                let line_offset = offset + (line.len() - trimmed.len());
                for (token, token_start) in quote_aware_tokens(trimmed, line_offset) {
                    let check_offset = token_start + token.len() / 2;
                    for d in cursor_diagnostics(source, check_offset) {
                        // Only skip if there's an overlapping diagnostic of same or higher severity
                        let overlaps = diagnostics.iter().any(|existing| {
                            spans_overlap(existing.span, d.span)
                                && existing.severity as u8 <= d.severity as u8
                        });
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

fn theme_source_prefix(source: &str, uri: &Url) -> String {
    let Ok(path) = uri.to_file_path() else {
        return String::new();
    };
    let Some(parent) = path.parent() else {
        return String::new();
    };
    // Don't merge theme if this file IS the theme file
    if path.file_name().is_some_and(|n| n == "app-theme.frame") {
        return String::new();
    }
    let theme = parent.join("app-theme.frame");
    if !theme.exists() {
        return String::new();
    }
    // Don't merge if the source already includes this file
    let canonical_theme = fs::canonicalize(&theme).unwrap_or(theme.clone());
    let already_included = source.lines().any(|line| {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("#include") {
            return false;
        }
        let target = trimmed.split_whitespace().nth(1).unwrap_or("");
        let candidate = include_candidate(parent, target);
        fs::canonicalize(&candidate).unwrap_or(candidate) == canonical_theme
    });
    if already_included {
        return String::new();
    }
    fs::read_to_string(&theme).unwrap_or_default()
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

    #[test]
    fn valid_component_with_state_and_props_has_no_diagnostics() {
        let source = "\
component ChatApp {
  props {
    messages list
    title text
  }
  state {
    draft text = \"\"
    sending bool = false
  }
  view {
    text $title
    action Send {
      text \"Send\"
      on press @sendMessage
      disabled when $sending
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid component, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_keyed_loop_has_no_diagnostics() {
        let source = "\
component MessageList {
  props {
    messages list
  }
  state {
    selected text = \"\"
  }
  view {
    for message in $messages key $selected {
      text $message
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid keyed loop, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_handler_reference_has_no_diagnostics() {
        let source = "\
component ChatApp {
  state {
    sending bool = false
  }
  view {
    action Send {
      text \"Send\"
      on press @sendMessage
      disabled when $sending
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid handler reference, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_style_definitions_have_no_diagnostics() {
        let source = "\
card PrimaryButton {
  surface panel
  padding medium
  gap small
}

grid AppShell {
  columns sidebar content
  overflow hidden
}

text MessageText {
  truncate
  wrap anywhere
}

tokens Brand {
  color brand #7c3aed
  color panel-bg #181820
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid style definitions, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_nested_view_body_has_no_diagnostics() {
        let source = "\
component ChatApp {
  state {
    showPanel bool = false
  }
  view {
    panel Main {
      show when $showPanel
      stack Content {
        text \"Hello\"
      }
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid nested view body, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_conditional_style_binding_has_no_diagnostics() {
        let source = "\
component ChatApp {
  state {
    sending bool = false
  }
  view {
    action Send {
      text \"Send\"
      style when $sending = LoadingButton
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid conditional style binding, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_event_modifiers_have_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    input MessageInput {
      on keydown.enter @submitMessage
      on keydown.escape @cancelMessage
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid event modifiers, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_gradient_blocks_have_no_diagnostics() {
        let source = "\
tokens Brand {
  color brand-purple #7c3aed
  gradient hero-gradient {
    type linear
    angle 135deg
    stop brand-purple 0%
    stop #181820 100%
  }
}

card Hero {
  background hero-gradient
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid gradient blocks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_animation_blocks_have_no_diagnostics() {
        let source = "\
keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px)
  }
  to {
    opacity 1
    transform translateY(0)
  }
}

card Panel {
  animation FloatIn {
    duration 240ms
    delay 0ms
    ease smooth
    iteration 1
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid animation blocks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_responsive_blocks_have_no_diagnostics() {
        let source = "\
grid AppShell {
  columns sidebar content
  below tablet {
    columns content
  }
  container narrow {
    columns content
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid responsive blocks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_slots_have_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    card Dialog {
      text \"Content\"
    }
  }
  slot Header {
    text \"Title\"
  }
  slot Footer {
    text \"Actions\"
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid slots, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_motion_helpers_have_no_diagnostics() {
        let source = "\
card FloatingCard {
  lift small
  surface panel
  hover {
    lift medium
    grow slight
  }
  active {
    press
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid motion helpers, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_dotted_data_ref_has_no_diagnostics() {
        let source = "\
component ChatApp {
  props {
    user text
  }
  view {
    text $user
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid dotted data ref, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_bind_forms_have_no_diagnostics() {
        let source = "\
component ChatApp {
  state {
    email text = \"\"
    active bool = false
    choice text = \"\"
  }
  view {
    field EmailField {
      input EmailInput {
        value bind $email
      }
    }
    toggle ActiveToggle {
      checked bind $active
    }
    select ChoiceSelect {
      selected bind $choice
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid bind forms, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_style_group_and_supports_have_no_diagnostics() {
        let source = "\
style-group components {
  button PrimaryButton {
    surface panel
  }
}

supports display grid {
  grid AppShell {
    columns sidebar content
  }
}

style-order reset, base, components, utilities
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid style groups and supports, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_text_interpolation_has_no_diagnostics() {
        let source = "\
component ChatApp {
  props {
    username text
    count number
  }
  view {
    text $username
    text $count
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid text interpolation, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_component_invocation_with_args_has_no_diagnostics() {
        let source = "\
component Greeting {
  props {
    name text
  }
  view {
    text $name
  }
}

component ChatApp {
  view {
    Greeting(name: \"World\")
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid component invocation, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_image_with_alt_has_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    image Avatar {
      source \"avatar.png\"
      alt \"User avatar\"
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid image with alt, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn cursor_diagnoses_non_first_word_tokens() {
        // The semantic validator catches $missing, and cursor diagnostics should also
        // detect it (even though it is deduplicated in the final list).
        let source = "component ChatApp {\n  view {\n    input MessageInput {\n      value bind $missing\n    }\n  }\n}\n";
        let offset = source.find("$missing").unwrap() + 4;
        let cursor_d = cursor_diagnostics(source, offset);
        assert!(
            cursor_d
                .iter()
                .any(|d| d.message.contains("Unknown state/prop")),
            "cursor_diagnostics at $missing should report unknown state: got {:?}",
            cursor_d
        );

        // The semantic validator already produces a diagnostic for $missing,
        // so the combined list should contain at least one unknown-reference error.
        let diagnostics = diagnostics_for_source(source);
        let unknown_state_errors = diagnostics
            .iter()
            .filter(|d| d.message.contains("Unknown") && d.message.contains("$missing"))
            .count();
        assert_eq!(
            unknown_state_errors, 1,
            "expected one unknown state diagnostic for $missing"
        );
    }

    #[test]
    fn invalid_handler_reports_unknown_handler() {
        let source = "\
component ChatApp {
  state {
    sending bool = false
  }
  view {
    action Send {
      text \"Send\"
      on press @sendMessage
    }
    composer MessageComposer {
      draft bind $sending
      send @deleteMesssage
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown handler")),
            "Expected unknown handler diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn duplicate_property_reports_only_duplicate_key() {
        let source = "\
component ChatApp {
  view {
    input MessageInput {
      id \"name\"
      id \"email\"
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Duplicate property")),
            "Expected duplicate property diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn unknown_primitive_reports_unknown_primitive() {
        let source = "\
component ChatApp {
  view {
    unknownPrimitive Broken {
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown UI primitive")),
            "Expected unknown primitive diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn invalid_surface_value_reports_unknown_surface() {
        let source = "\
card Demo {
  surface nonexistent
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown surface")),
            "Expected unknown surface diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn invalid_event_name_reports_unknown_event() {
        let source = "\
component ChatApp {
  view {
    action Send {
      on invalidEvent @send
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown event")),
            "Expected unknown event diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn invalid_event_modifier_reports_unknown_modifier() {
        let source = "\
component ChatApp {
  view {
    action Send {
      on click.invalidMod @send
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown event modifier")),
            "Expected unknown event modifier diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn duplicate_state_reports_duplicate_state() {
        let source = "\
component ChatApp {
  state {
    draft text = \"\"
    draft text = \"\"
  }
  view {
    text $draft
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Duplicate state")),
            "Expected duplicate state diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn unknown_component_reports_unknown_component() {
        let source = "\
component ChatApp {
  view {
    UnknownComponent()
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown component")),
            "Expected unknown component diagnostic, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn empty_component_reports_empty_hint() {
        let source = "\
component ChatApp {
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("Empty component")),
            "Expected empty component hint, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn missing_required_body_reports_parser_error() {
        let source = "card Broken {\n  magic {\n  }\n}\n";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            diagnostics
                .iter()
                .any(|d| d.message.contains("unknown nested block")),
            "Expected parser error, got: {:?}",
            diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_style_bindings_have_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    action Send:PrimaryButton {
      text \"Send\"
    }
    panel Content:GlassPanel {
      text \"Hello\"
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid style bindings, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_show_when_inside_element_has_no_diagnostics() {
        let source = "\
component ChatApp {
  state {
    showPanel bool = false
  }
  view {
    panel Main {
      show when $showPanel
      text \"Content\"
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid show when, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_component_invocation_with_bind_arg_has_no_diagnostics() {
        let source = "\
component MessageComposer {
  props {
    draft text
  }
  view {
    text $draft
  }
}

component ChatApp {
  state {
    draft text = \"\"
  }
  view {
    MessageComposer(draft bind $draft)
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid component invocation with bind, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_nested_components_in_view_have_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    panel Main {
      stack Content {
        text \"Hello\"
        action Send {
          text \"Send\"
          on press @sendMessage
        }
      }
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for nested components, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_for_loop_with_key_has_no_diagnostics() {
        let source = "\
component ChatApp {
  state {
    items list = []
    selected text = \"\"
  }
  view {
    list ItemList {
      for item in $items key $selected {
        item Entry {
          text $item
        }
      }
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid for loop with key, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_multiple_event_modifiers_have_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    action Send {
      text \"Send\"
      on keydown.ctrl.enter @sendMessage
      on keydown.escape @cancelMessage
      on click.once @handleClick
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid multiple event modifiers, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_image_with_source_and_alt_has_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    image Avatar {
      source \"avatar.png\"
      alt \"User avatar\"
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid image with source and alt, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_component_with_all_block_types_has_no_diagnostics() {
        let source = "\
component ChatApp {
  props {
    title text
    count number
  }
  state {
    draft text = \"\"
    sending bool = false
  }
  view {
    text $title
    action Send {
      text \"Send\"
      on press @sendMessage
      disabled when $sending
      style when $sending = LoadingButton
    }
  }
  slot Default {
    text \"Fallback\"
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for complete component, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_style_group_with_nested_declarations_has_no_diagnostics() {
        let source = "\
style-group components {
  button Primary {
    surface panel
  }
  card Elevated {
    surface raised
  }
}

style-order reset, base, components

supports display grid {
  grid AppShell {
    columns sidebar content
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid style groups and supports, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_grid_with_areas_and_tracks_has_no_diagnostics() {
        let source = "\
grid Dashboard {
  columns sidebar content
  areas sidebar content
}

area Sidebar {
  in Dashboard
  place sidebar
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid grid with areas, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn grid_columns_tracks_conflict_reports_error() {
        let source = "\
grid BadGrid {
  columns sidebar content
  tracks columns rail panel fill
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            !errors.is_empty(),
            "Expected error for columns+tracks conflict"
        );
        assert!(errors
            .iter()
            .any(|d| d.message.contains("columns") && d.message.contains("tracks")));
    }

    #[test]
    fn duplicate_grid_column_name_reports_error() {
        let source = "\
grid BadGrid {
  columns content content
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            !errors.is_empty(),
            "Expected error for duplicate column name"
        );
        assert!(errors.iter().any(|d| d.message.contains("duplicate")));
    }

    #[test]
    fn unknown_explicit_style_binding_reports_actionable_warning() {
        let source = "\
component Test {
  view {
    card MetricCard:ActiveModel {
      surface raised
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let warnings: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .collect();
        assert!(
            !warnings.is_empty(),
            "Expected warning for unknown explicit style binding"
        );
        assert!(warnings.iter().any(|d| d.message.contains("ActiveModel")));
    }

    #[test]
    fn valid_tokens_with_gradients_and_colors_has_no_diagnostics() {
        let source = "\
tokens BrandColors {
  color primary #3B82F6
  color danger #EF4444
  gradient hero-gradient {
    type linear
    angle 135deg
    stop blue 0%
    stop purple 100%
  }
}

card Hero {
  background hero-gradient
  color primary
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid tokens with gradients, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_keyframes_with_named_animation_has_no_diagnostics() {
        let source = "\
keyframes FadeIn {
  from {
    opacity 0
  }
  to {
    opacity 1
  }
}

card Panel {
  animation FadeIn {
    duration 300ms
    ease smooth
    fill forwards
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid keyframes with animation, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_responsive_and_container_blocks_have_no_diagnostics() {
        let source = "\
grid AppShell {
  columns sidebar content
  below mobile {
    columns content
  }
  above desktop {
    columns sidebar content main
  }
  between mobile and tablet {
    columns content
  }
  container narrow {
    columns content
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid responsive blocks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_component_with_slot_has_no_diagnostics() {
        let source = "\
component ChatApp {
  view {
    card Dialog {
      text \"Content\"
    }
  }
  slot Header {
    text \"Title\"
  }
  slot Footer {
    text \"Actions\"
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for component with slots, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_text_interpolation_with_dotted_refs_has_no_diagnostics() {
        let source = "\
component ChatApp {
  props {
    user text
    count number
  }
  view {
    text $user
    text $count
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid text interpolation, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── Real dashboard file diagnostics ──────────────────────────────────

    fn dashboard_path(file: &str) -> std::path::PathBuf {
        let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = std::path::PathBuf::from(manifest)
            .join("../../implementations/llm-dashboard/src")
            .join(file);
        std::fs::canonicalize(&path).unwrap_or(path)
    }

    fn read_dashboard(file: &str) -> String {
        let path = dashboard_path(file);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e))
    }

    #[test]
    fn llm_dashboard_app_has_no_lsp_errors() {
        let source = read_dashboard("app.frame");
        let uri = Url::from_file_path(dashboard_path("app.frame")).expect("valid uri");
        let diagnostics = diagnostics_for_uri(&source, &uri);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "LLM dashboard app.frame LSP errors: {:?}",
            errors
                .iter()
                .map(|d| format!("[{:?}] {}", d.severity, d.message))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn llm_dashboard_theme_has_no_lsp_errors() {
        let source = read_dashboard("app-theme.frame");
        let uri = Url::from_file_path(dashboard_path("app-theme.frame")).expect("valid uri");
        let diagnostics = diagnostics_for_uri(&source, &uri);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "LLM dashboard app-theme.frame LSP errors: {:?}",
            errors
                .iter()
                .map(|d| format!("[{:?}] {}", d.severity, d.message))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn llm_dashboard_app_has_no_lsp_warnings() {
        let source = read_dashboard("app.frame");
        let uri = Url::from_file_path(dashboard_path("app.frame")).expect("valid uri");
        let diagnostics = diagnostics_for_uri(&source, &uri);
        let warnings: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .collect();
        assert!(
            warnings.is_empty(),
            "LLM dashboard app.frame LSP warnings: {:?}",
            warnings
                .iter()
                .map(|d| format!("[{:?}] {}", d.severity, d.message))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn llm_dashboard_theme_has_no_lsp_warnings() {
        let source = read_dashboard("app-theme.frame");
        let uri = Url::from_file_path(dashboard_path("app-theme.frame")).expect("valid uri");
        let diagnostics = diagnostics_for_uri(&source, &uri);
        let warnings: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .collect();
        assert!(
            warnings.is_empty(),
            "LLM dashboard app-theme.frame LSP warnings: {:?}",
            warnings
                .iter()
                .map(|d| format!("[{:?}] {}", d.severity, d.message))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn llm_dashboard_lsp_matches_cli_diagnostics() {
        let app_source = read_dashboard("app.frame");
        let app_uri = Url::from_file_path(dashboard_path("app.frame")).expect("valid uri");
        let app_diags = diagnostics_for_uri(&app_source, &app_uri);
        let app_errors: Vec<_> = app_diags
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            app_errors.is_empty(),
            "CLI reports 0 warnings for app.frame but LSP reports {} errors: {:?}",
            app_errors.len(),
            app_errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );

        let theme_source = read_dashboard("app-theme.frame");
        let theme_uri = Url::from_file_path(dashboard_path("app-theme.frame")).expect("valid uri");
        let theme_diags = diagnostics_for_uri(&theme_source, &theme_uri);
        let theme_errors: Vec<_> = theme_diags
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            theme_errors.is_empty(),
            "CLI reports 0 warnings for app-theme.frame but LSP reports {} errors: {:?}",
            theme_errors.len(),
            theme_errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── Positive regression tests for dashboard-safe syntax ──────────────

    #[test]
    fn lsp_accepts_html_root_declaration() {
        let source = "html {\n  background #0A0A0F\n  color #F8FAFC\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for html root, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_page_body_declaration() {
        let source = "page-body {\n  margin none\n  background #0A0A0F\n  color #F8FAFC\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for page-body, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_fr_grid_tracks() {
        let source = "grid AppShell {\n  tracks columns panel fill\n  tracks rows header fill\n  gap none\n  height screen\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for fr grid tracks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_area_declarations() {
        let source = "grid AppShell {\n  tracks columns panel fill\n  tracks rows header fill\n  gap none\n  height screen\n}\narea Header {\n  in AppShell\n  row 1\n  span 2\n  padding x small\n  align center\n  justify between\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for area declarations, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_abstract_opacity_values() {
        let source = "card Test {\n  opacity none\n  opacity slight\n  opacity subtle\n  opacity half\n  opacity strong\n  opacity full\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for abstract opacity, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_shadow_radius_surface_values() {
        let source = "card Test {\n  shadow small\n  shadow medium\n  shadow large\n  radius small\n  radius medium\n  radius large\n  surface raised\n  surface overlay\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for shadow/radius/surface, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_page_root_declarations() {
        let source = "html {\n  background #0A0A0F\n}\npage-body {\n  margin none\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for page root declarations, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_dashboard_area_placement() {
        let source = "\
grid AppShell {
  tracks columns panel fill
  tracks rows header fill
  gap none
  height screen
}
area Header {
  in AppShell
  row 1
  span 2
}
area Sidebar {
  in AppShell
  row 2
  col 1
}
area Main {
  in AppShell
  row 2
  col 2
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for dashboard area placement, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_dashboard_style_declarations() {
        let source = "\
row NavBar {
  gap large
  align center
  justify between
  color accent
}
stack Logo {
  padding x small
  gap none
  color accent
  weight bold
  size heading
}
card StatusBadge {
  padding x small
  padding y small
  radius small
  color success
  border soft
  size caption
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for dashboard style declarations, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_hover_focus_active_blocks() {
        let source = "\
card Button {
  padding medium
  radius medium
  hover {
    lift small
  }
  focus {
    glow
  }
  active {
    press
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for hover/focus/active blocks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_hex_color_values() {
        let source = "html {\n  background #0A0A0F\n  color #F8FAFC\n}\npage-body {\n  background #0A0A0F\n  color #F8FAFC\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for hex colors in root, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_dashboard_grid_columns() {
        let source = "\
grid PerformanceGrid {
  columns 60% 40%
  gap medium
}
grid DashboardGrid {
  columns 2fr 1fr
  gap medium
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for dashboard grid columns, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── Invalid tests ────────────────────────────────────────────────────

    #[test]
    fn lsp_accepts_named_grid_tracks() {
        // Named grid track identifiers are valid in columns/rows
        let source = "grid Layout {\n  columns sidebar content\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for named grid tracks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_reports_unknown_surface_value() {
        let source = "card Test {\n  surface nonexistent\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let has_error = diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error && d.message.contains("surface"));
        assert!(has_error, "Expected error for unknown surface value");
    }

    #[test]
    fn lsp_reports_invalid_page_body_property() {
        let source = "page-body {\n  nonexistent-prop value\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let has_error = diagnostics.iter().any(|d| d.severity == Severity::Error);
        assert!(has_error, "Expected error for invalid page-body property");
    }

    #[test]
    fn lsp_reports_unknown_style_binding() {
        let source =
            "component Test {\n  view {\n    card MyCard:NonexistentStyle {\n    }\n  }\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let has_warning = diagnostics.iter().any(|d| d.severity == Severity::Warning);
        assert!(has_warning, "Expected warning for unknown style binding");
    }

    // ── Style inheritance tests ──────────────────────────────────────────

    #[test]
    fn lsp_accepts_valid_extends() {
        let source = "card Base {\n  padding medium\n  radius medium\n}\ncard Child extends Base {\n  border soft\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid extends, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_reports_unknown_base_style() {
        let source = "card Child extends MissingBase {\n  padding medium\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let has_error = diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error && d.message.contains("Unknown base style"));
        assert!(has_error, "Expected error for unknown base style");
    }

    #[test]
    fn lsp_reports_kind_mismatch_on_extends() {
        let source =
            "grid Base {\n  gap medium\n}\ncard Child extends Base {\n  padding medium\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let has_error = diagnostics.iter().any(|d| {
            d.severity == Severity::Error && d.message.contains("matching declaration kinds")
        });
        assert!(has_error, "Expected error for kind mismatch on extends");
    }

    #[test]
    fn lsp_reports_inheritance_cycle() {
        let source =
            "card A extends B {\n  padding medium\n}\ncard B extends A {\n  radius medium\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let has_error = diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error && d.message.contains("cycle"));
        assert!(has_error, "Expected error for inheritance cycle");
    }

    #[test]
    fn lsp_accepts_multi_level_inheritance() {
        let source = "card GrandBase {\n  padding medium\n}\ncard Base extends GrandBase {\n  radius medium\n}\ncard Child extends Base {\n  border soft\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for multi-level inheritance, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_extends_with_hover_blocks() {
        let source = "card Base {\n  padding medium\n  hover {\n    lift small\n  }\n}\ncard Child extends Base {\n  radius medium\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for extends with hover blocks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lsp_accepts_extends_grid_with_tracks() {
        let source = "grid Base {\n  tracks columns panel fill\n  gap medium\n}\ngrid Child extends Base {\n  tracks rows header fill\n}\n";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for extends grid with tracks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_show_when_produces_no_diagnostic() {
        let source = "\
component ChatApp {
  state {
    isOpen bool = false
  }
  view {
    panel Main {
      show when $isOpen
      text \"Content\"
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for valid show when, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_prop_reference_produces_no_narration_diagnostic() {
        let source = "\
component ChatApp {
  props {
    title text
  }
  view {
    text $title
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            !diagnostics.iter().any(|d| d.message.contains("references a prop")),
            "Should not produce prop narration diagnostic"
        );
    }

    #[test]
    fn valid_handler_produces_no_narration_diagnostic() {
        let source = "\
component ChatApp {
  view {
    action Send {
      text \"Send\"
      on press @sendMessage
    }
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        assert!(
            !diagnostics.iter().any(|d| d.message.contains("external handler")),
            "Should not produce handler narration diagnostic"
        );
    }

    #[test]
    fn named_grid_tracks_are_valid() {
        let source = "\
grid PerformanceGrid {
  columns chart prompt
  rows header body footer
  gap medium
}
";
        let diagnostics = diagnostics_for_source(source);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Expected no errors for named grid tracks, got: {:?}",
            errors.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn unused_state_warning_points_at_identifier() {
        let source = "\
component ChatApp {
  state {
    unusedState text = \"\"
    activeTab text = \"runs\"
  }
  view {
    text $activeTab
  }
}
";
        let diagnostics = diagnostics_for_source(source);
        let unused_diag = diagnostics
            .iter()
            .find(|d| d.message.contains("unusedState") && d.message.contains("never referenced"));
        assert!(
            unused_diag.is_some(),
            "Expected unused state diagnostic for unusedState"
        );
        let diag = unused_diag.unwrap();
        // The span should be small (just the identifier), not the whole component
        assert!(
            diag.span.end - diag.span.start < 50,
            "Unused state span should be short (just the identifier), got span {}..{}",
            diag.span.start,
            diag.span.end
        );
    }
}
