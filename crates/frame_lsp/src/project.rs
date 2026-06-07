use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use frame_core::{
    symbols::index_document, Document, Node, Span, TextValue, UiComponentArgumentValue, UiNode,
    UiPropertyValue,
};
use frame_parser::parse;
use tower_lsp::lsp_types::{Location, Url};

use crate::diagnostics;

/// Detect the project theme file for a given Frame source path.
/// Looks for `app-theme.frame` in the same directory as the source file.
pub fn resolve_theme_file(current_path: &Path) -> Option<PathBuf> {
    let parent = current_path.parent()?;
    let theme = parent.join("app-theme.frame");
    if theme.exists() {
        Some(fs::canonicalize(&theme).unwrap_or(theme))
    } else {
        None
    }
}

/// Resolve all included Frame files for a given source document, plus the implicit theme file.
/// Returns a vec of (canonical path, source text, parsed document, symbol index).
/// The order is: explicit includes (depth-first), then the implicit theme file.
pub fn resolve_includes(
    current_path: &Path,
    source: &str,
) -> Vec<(PathBuf, String, Document, frame_core::symbols::SymbolIndex)> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();
    collect_includes(current_path, source, &mut seen, &mut results);
    // Append implicit theme file after explicit includes so local symbols take precedence.
    if let Some(theme) = resolve_theme_file(current_path) {
        if !seen.contains(&theme) {
            if let Ok(theme_source) = fs::read_to_string(&theme) {
                if let Ok(document) = parse(&theme_source) {
                    let symbols = index_document(&theme_source, &document);
                    results.push((theme, theme_source, document, symbols));
                }
            }
        }
    }
    results
}

fn collect_includes(
    current_path: &Path,
    source: &str,
    seen: &mut HashSet<PathBuf>,
    results: &mut Vec<(PathBuf, String, Document, frame_core::symbols::SymbolIndex)>,
) {
    let Some(parent) = current_path.parent() else {
        return;
    };
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
        let parent_path = canonical.clone();
        collect_includes(&parent_path, &include_source, seen, results);
        if let Ok(document) = parse(&include_source) {
            let symbols = index_document(&include_source, &document);
            results.push((canonical, include_source, document, symbols));
        }
    }
}

/// Returns files that are plausible targets for a given symbol kind based on include names/paths.
pub fn include_files_for_symbol(
    current_path: &Path,
    source: &str,
    _symbol_name: &str,
    symbol_kind_hint: &str,
) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let Some(parent) = current_path.parent() else {
        return candidates;
    };

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

        let path_str = path.to_string_lossy().to_lowercase();
        let include_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let is_match = match symbol_kind_hint {
            "style" | "theme" => {
                path_str.contains("style")
                    || path_str.contains("theme")
                    || include_name.contains("style")
                    || include_name.contains("theme")
            }
            "component" => path_str.contains("component") || include_name.contains("component"),
            "handler" => path_str.contains("handler") || include_name.contains("handler"),
            _ => false,
        };

        if is_match {
            candidates.push(path);
        }
    }

    // Also include the theme file as a candidate for style/theme symbols.
    if symbol_kind_hint == "style" || symbol_kind_hint == "theme" {
        if let Some(theme) = resolve_theme_file(current_path) {
            candidates.push(theme);
        }
    }

    candidates
}

/// Build a merged symbol index from the current source plus all included files.
pub fn merged_symbols(current_path: &Path, source: &str) -> frame_core::symbols::SymbolIndex {
    let mut symbols = frame_core::symbols::SymbolIndex::default();
    if let Ok(document) = parse(source) {
        let local = index_document(source, &document);
        symbols.merge(local);
    }
    for (_, _, _, included) in resolve_includes(current_path, source) {
        symbols.merge(included);
    }
    symbols
}

/// Build a merged component name set from the current source plus all included files.
pub fn merged_component_names(current_path: &Path, source: &str) -> HashSet<String> {
    let mut names = HashSet::new();
    if let Ok(document) = parse(source) {
        for component in &document.components {
            names.insert(component.name.text.clone());
        }
    }
    for (_, _, document, _) in resolve_includes(current_path, source) {
        for component in &document.components {
            names.insert(component.name.text.clone());
        }
    }
    names
}

#[allow(dead_code)]
/// Find a declaration symbol across included files.
pub fn find_declaration_across_files(
    current_path: &Path,
    source: &str,
    name: &str,
) -> Option<(PathBuf, Span)> {
    // Check local first
    if let Ok(document) = parse(source) {
        for decl in &document.declarations {
            if decl.name.text == name {
                return Some((current_path.to_path_buf(), decl.name.span));
            }
        }
    }
    // Check includes
    for (path, include_source, document, _) in resolve_includes(current_path, source) {
        for decl in &document.declarations {
            if decl.name.text == name {
                return Some((path, decl.name.span));
            }
        }
        // Also check if the symbol exists in the included source via raw text search
        // for cases where parsing might fail partially
        if let Some(span) = find_word_span(&include_source, name) {
            return Some((path, span));
        }
    }
    None
}

#[allow(dead_code)]
/// Find a component symbol across included files.
pub fn find_component_across_files(
    current_path: &Path,
    source: &str,
    name: &str,
) -> Option<(PathBuf, Span)> {
    if let Ok(document) = parse(source) {
        for component in &document.components {
            if component.name.text == name {
                return Some((current_path.to_path_buf(), component.name.span));
            }
        }
    }
    for (path, _, document, _) in resolve_includes(current_path, source) {
        for component in &document.components {
            if component.name.text == name {
                return Some((path, component.name.span));
            }
        }
    }
    None
}

#[allow(dead_code)]
/// Check if a style name exists in any included file.
pub fn style_exists_across_files(current_path: &Path, source: &str, name: &str) -> bool {
    let symbols = merged_symbols(current_path, source);
    symbols.declarations.contains_key(name)
}

#[allow(dead_code)]
/// Check if a component name exists in any included file.
pub fn component_exists_across_files(current_path: &Path, source: &str, name: &str) -> bool {
    merged_component_names(current_path, source).contains(name)
}

#[allow(dead_code)]
/// Produce a Location for a symbol in an included file.
pub fn location_for_symbol(path: &Path, span: Span) -> Option<Location> {
    let target_uri = Url::from_file_path(path).ok()?;
    let source = fs::read_to_string(path).ok()?;
    Some(Location {
        uri: target_uri,
        range: diagnostics::range_for_span(&source, span),
    })
}

#[allow(dead_code)]
fn find_word_span(source: &str, word: &str) -> Option<Span> {
    let mut search_start = 0usize;
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(header) = trimmed.strip_suffix('{') {
            let parts: Vec<_> = header.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Some(kind) = parts.first() {
                    if matches!(
                        *kind,
                        "card"
                            | "stack"
                            | "row"
                            | "grid"
                            | "area"
                            | "text"
                            | "button"
                            | "tokens"
                            | "center"
                            | "split"
                            | "overlay"
                            | "dock"
                            | "keyframes"
                            | "supports"
                    ) && parts[1] == word
                    {
                        if let Some(relative) = line.find(word) {
                            let start = search_start + relative;
                            return Some(Span {
                                start,
                                end: start + word.len(),
                            });
                        }
                    }
                }
            }
        }
        search_start += line.len() + 1;
    }
    None
}

/// Collect duplicate symbol names across the current file and all included files.
pub fn duplicate_symbols(current_path: &Path, source: &str) -> Vec<(String, Vec<(PathBuf, Span)>)> {
    let mut symbol_locations: HashMap<String, Vec<(PathBuf, Span)>> = HashMap::new();

    if let Ok(document) = parse(source) {
        for decl in &document.declarations {
            symbol_locations
                .entry(decl.name.text.clone())
                .or_default()
                .push((current_path.to_path_buf(), decl.name.span));
        }
        for component in &document.components {
            symbol_locations
                .entry(component.name.text.clone())
                .or_default()
                .push((current_path.to_path_buf(), component.name.span));
        }
    }

    for (path, _, document, _) in resolve_includes(current_path, source) {
        for decl in &document.declarations {
            symbol_locations
                .entry(decl.name.text.clone())
                .or_default()
                .push((path.clone(), decl.name.span));
        }
        for component in &document.components {
            symbol_locations
                .entry(component.name.text.clone())
                .or_default()
                .push((path.clone(), component.name.span));
        }
    }

    symbol_locations
        .into_iter()
        .filter(|(_, locations)| locations.len() > 1)
        .collect()
}

/// Check for imported symbols that shadow local symbols.
pub fn shadowed_symbols(current_path: &Path, source: &str) -> Vec<(String, PathBuf, Span)> {
    let mut shadowed = Vec::new();
    let local_names = if let Ok(document) = parse(source) {
        document
            .declarations
            .iter()
            .map(|d| d.name.text.clone())
            .chain(document.components.iter().map(|c| c.name.text.clone()))
            .collect::<HashSet<_>>()
    } else {
        HashSet::new()
    };

    for (path, _, document, _) in resolve_includes(current_path, source) {
        for decl in &document.declarations {
            if local_names.contains(&decl.name.text) {
                shadowed.push((decl.name.text.clone(), path.clone(), decl.name.span));
            }
        }
        for component in &document.components {
            if local_names.contains(&component.name.text) {
                shadowed.push((
                    component.name.text.clone(),
                    path.clone(),
                    component.name.span,
                ));
            }
        }
    }

    shadowed
}

/// Collect all AST-aware reference spans for a given word in a Frame source string.
///
/// This walks the parsed AST and finds declarations, component names, style bindings,
/// handler references, data references (state/prop), component invocations, and
/// statement usages that match the word.
pub fn collect_references_in_source(source: &str, word: &str) -> Vec<Span> {
    let Ok(document) = parse(source) else {
        return Vec::new();
    };
    let mut spans = Vec::new();

    // Declarations and their statement bodies
    for decl in &document.declarations {
        if decl.name.text == word {
            spans.push(decl.name.span);
        }
        for node in &decl.body {
            match node {
                Node::Statement(statement) => {
                    for w in &statement.words {
                        if w == word {
                            if let Some(span) = word_span_in_source(source, statement.span, word) {
                                if !spans.contains(&span) {
                                    spans.push(span);
                                }
                            }
                            break;
                        }
                    }
                }
                Node::Block(block) => {
                    for inner in &block.body {
                        if let Node::Statement(statement) = inner {
                            for w in &statement.words {
                                if w == word {
                                    if let Some(span) =
                                        word_span_in_source(source, statement.span, word)
                                    {
                                        if !spans.contains(&span) {
                                            spans.push(span);
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Components
    for component in &document.components {
        if component.name.text == word {
            spans.push(component.name.span);
        }
        if let Some(props) = &component.props {
            for prop in &props.values {
                if prop.name.text == word {
                    spans.push(prop.name.span);
                }
            }
        }
        if let Some(state) = &component.state {
            for value in &state.values {
                if value.name.text == word {
                    spans.push(value.name.span);
                }
            }
        }
        if let Some(view) = &component.view {
            collect_ui_node_references(&view.nodes, word, &mut spans);
        }
        for slot in &component.slots {
            collect_ui_node_references(&slot.nodes, word, &mut spans);
        }
    }

    spans.sort_by_key(|s| s.start);
    spans.dedup();
    spans
}

fn collect_ui_node_references(nodes: &[UiNode], word: &str, spans: &mut Vec<Span>) {
    for node in nodes {
        match node {
            UiNode::Element(element) => {
                if element.name.text == word {
                    spans.push(element.name.span);
                }
                if let Some(style) = &element.style {
                    if style.name.text == word {
                        spans.push(style.span);
                    }
                }
                for prop in &element.properties {
                    if prop.name.text == word {
                        spans.push(prop.name.span);
                    }
                    collect_property_value_references(&prop.value, word, spans);
                }
                for event in &element.events {
                    if event.handler.name.text == word {
                        spans.push(event.handler.span);
                    }
                    for modifier in &event.modifiers {
                        if modifier.text == word {
                            spans.push(modifier.span);
                        }
                    }
                }
                collect_ui_node_references(&element.children, word, spans);
            }
            UiNode::Text(text) => {
                if let TextValue::Data(data_ref) = &text.value {
                    if data_ref.name.text == word {
                        spans.push(data_ref.span);
                    }
                }
            }
            UiNode::Component(invocation) => {
                if invocation.name.text == word {
                    spans.push(invocation.name.span);
                }
                for arg in &invocation.arguments {
                    if arg.name.text == word {
                        spans.push(arg.name.span);
                    }
                    collect_argument_value_references(&arg.value, word, spans);
                }
            }
            UiNode::Loop(loop_node) => {
                if loop_node.item.text == word {
                    spans.push(loop_node.item.span);
                }
                if loop_node.collection.name.text == word {
                    spans.push(loop_node.collection.span);
                }
                if let Some(key) = &loop_node.key {
                    if key.name.text == word {
                        spans.push(key.span);
                    }
                }
                collect_ui_node_references(&loop_node.children, word, spans);
            }
        }
    }
}

fn collect_property_value_references(value: &UiPropertyValue, word: &str, spans: &mut Vec<Span>) {
    match value {
        UiPropertyValue::Data(data_ref) => {
            if data_ref.name.text == word {
                spans.push(data_ref.span);
            }
        }
        UiPropertyValue::Bind(data_ref) => {
            if data_ref.name.text == word {
                spans.push(data_ref.span);
            }
        }
        UiPropertyValue::Handler(handler_ref) => {
            if handler_ref.name.text == word {
                spans.push(handler_ref.span);
            }
        }
        UiPropertyValue::Conditional(binding) => {
            if binding.condition.name.text == word {
                spans.push(binding.condition.span);
            }
        }
        UiPropertyValue::StyleWhen { condition, style } => {
            if condition.name.text == word {
                spans.push(condition.span);
            }
            if style.name.text == word {
                spans.push(style.span);
            }
        }
        _ => {}
    }
}

fn collect_argument_value_references(
    value: &UiComponentArgumentValue,
    word: &str,
    spans: &mut Vec<Span>,
) {
    match value {
        UiComponentArgumentValue::Data(data_ref) => {
            if data_ref.name.text == word {
                spans.push(data_ref.span);
            }
        }
        UiComponentArgumentValue::Bind(data_ref) => {
            if data_ref.name.text == word {
                spans.push(data_ref.span);
            }
        }
        _ => {}
    }
}

fn word_span_in_source(source: &str, span: Span, word: &str) -> Option<Span> {
    if source.is_empty() || span.end > source.len() || span.start > span.end {
        return None;
    }
    let relative = source[span.start..span.end].find(word)?;
    Some(Span {
        start: span.start + relative,
        end: span.start + relative + word.len(),
    })
}
