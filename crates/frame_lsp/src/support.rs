use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use tower_lsp::lsp_types::{Location, Position, Range, Url};

use crate::{diagnostics, hover};

pub fn include_files_for_uri(uri: &Url) -> Vec<PathBuf> {
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

pub fn merged_frame_source(source: &str, uri: &Url) -> String {
    let prefix = included_source_prefix(source, uri);
    if prefix.is_empty() {
        source.to_string()
    } else {
        format!("{prefix}\n{source}")
    }
}

pub fn included_source_prefix(source: &str, uri: &Url) -> String {
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

pub fn imported_symbol_definition_location(
    source: &str,
    offset: usize,
    uri: &Url,
) -> Option<Location> {
    let (frame_source, frame_offset) = crate::backend::frame_source_at(source, offset)?;
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
                .or_else(|| symbols.keyframes.get(word))
                .or_else(|| symbols.grids.get(word))
                .or_else(|| symbols.declarations.get(word))
                .or_else(|| symbols.components.get(word))
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

pub fn imported_symbol_reference_locations(
    source: &str,
    offset: usize,
    uri: &Url,
) -> Vec<Location> {
    let Some((frame_source, frame_offset)) = crate::backend::frame_source_at(source, offset) else {
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

pub fn word_occurrences(source: &str, word: &str) -> Vec<frame_core::Span> {
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

pub fn is_word_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start].chars().next_back();
    let after = source[end..].chars().next();
    !before.is_some_and(is_identifier_character) && !after.is_some_and(is_identifier_character)
}

pub fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '-' || character == '_'
}

pub fn included_sources_for_path(
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

pub fn area_grid_before(source: &str, offset: usize) -> Option<String> {
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

pub fn line_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let end = source[safe_offset..]
        .find('\n')
        .map_or(source.len(), |index| safe_offset + index);
    source[start..end].trim()
}

pub fn include_definition_location(source: &str, offset: usize, uri: &Url) -> Option<Location> {
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
