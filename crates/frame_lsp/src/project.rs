use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use frame_core::{symbols::index_document, Document, Span};
use frame_parser::parse;
use tower_lsp::lsp_types::{Location, Url};

use crate::diagnostics;

/// Resolve all included Frame files for a given source document.
/// Returns a vec of (canonical path, source text, parsed document, symbol index).
/// The order is depth-first: deepest includes first, then the direct includes.
pub fn resolve_includes(
    current_path: &Path,
    source: &str,
) -> Vec<(PathBuf, String, Document, frame_core::symbols::SymbolIndex)> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();
    collect_includes(current_path, source, &mut seen, &mut results);
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
