use tower_lsp::lsp_types::{DocumentLink, Range, Url};

use crate::diagnostics::position_for_offset;

pub fn document_links(source: &str) -> Vec<DocumentLink> {
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

    links
}
