use std::path::Path;

use tower_lsp::lsp_types::{DocumentLink, Range, Url};

use crate::diagnostics::position_for_offset;

pub fn document_links(source: &str, uri: Option<&Url>) -> Vec<DocumentLink> {
    let docs = [
        ("grid", "grid", "docs/grid.md"),
        ("surface", "surfaces", "docs/surfaces.md"),
        ("svelte", "svelte", "docs/svelte.md"),
        ("hover", "effects", "docs/effects.md"),
        ("diagnostics", "diagnostics", "docs/diagnostics.md"),
        ("code-actions", "code-actions", "docs/code-actions.md"),
    ];

    let mut links = Vec::new();
    for (word, slug, path) in docs {
        let mut search_start = 0usize;
        while let Some(relative) = source[search_start..].find(word) {
            let start = search_start + relative;
            search_start = start + word.len();
            links.push(DocumentLink {
                range: Range {
                    start: position_for_offset(source, start),
                    end: position_for_offset(source, start + word.len()),
                },
                target: Url::parse(&format!("docs://frame/{slug}")).ok(),
                tooltip: Some(format!("Read more in `{path}`")),
                data: None,
            });
        }
    }

    links.extend(include_links(source, uri));
    links
}

fn include_links(source: &str, uri: Option<&Url>) -> Vec<DocumentLink> {
    let Some(uri) = uri else {
        return Vec::new();
    };
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
            let path = include_candidate(parent, target);
            let target_url = Url::from_file_path(path).ok()?;
            Some(DocumentLink {
                range: Range {
                    start: position_for_offset(source, target_start),
                    end: position_for_offset(source, target_start + target.len()),
                },
                target: Some(target_url),
                tooltip: Some(format!("Open included Frame file `{target}`")),
                data: None,
            })
        })
        .collect()
}

fn include_candidate(parent: &Path, target: &str) -> std::path::PathBuf {
    let candidate = parent.join(target);
    if candidate.extension().is_some() {
        candidate
    } else {
        candidate.with_extension("frame")
    }
}
