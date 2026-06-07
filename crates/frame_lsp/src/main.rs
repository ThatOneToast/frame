use tokio::sync::Mutex;
use tower_lsp::{LspService, Server};

use std::collections::HashMap;

mod backend;
mod code_actions;
mod completions;
mod context;
mod diagnostics;
mod document_links;
mod document_symbols;
mod embedded;
mod folding;
mod formatting;
mod hover;
mod navigation;
mod semantic_tokens;
mod support;

use backend::Backend;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::Url;

    #[test]
    fn creates_full_document_range() {
        let range = backend::full_document_range("card A {\n}\n");

        assert_eq!(range.start.line, 0);
        assert_eq!(range.end.line, 2);
    }

    #[test]
    fn maps_svelte_frame_style_blocks_to_frame_source() {
        let source = "<style lang=\"frame\">\ncard Demo {\n  surface \n}\n</style>";
        let offset = source.find("surface ").unwrap() + "surface ".len();
        let (frame_source, frame_offset) =
            backend::frame_source_at(source, offset).expect("offset should be inside Frame block");

        assert!(frame_source.contains("card Demo"));
        assert_eq!(
            &frame_source[frame_offset - "surface ".len()..frame_offset],
            "surface "
        );
    }

    #[test]
    fn returns_none_outside_svelte_frame_style_blocks() {
        let source =
            "<script>let a = 1;</script>\n<style lang=\"frame\">\ncard Demo {\n}\n</style>";
        let offset = source.find("let").unwrap();

        assert!(backend::frame_source_at(source, offset).is_none());
    }

    #[test]
    fn finds_imported_token_definition_location() {
        let root = std::env::temp_dir().join(format!("frame-lsp-import-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let theme = root.join("theme.frame");
        std::fs::write(&theme, "tokens Brand {\n  color brand-panel #181820\n}\n")
            .expect("theme should be writable");
        let source = "#include theme\n\ncard Hero {\n  background brand-panel\n}\n";
        std::fs::write(&app, source).expect("app should be writable");
        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.rfind("brand-panel").unwrap() + 1;

        let location = support::imported_symbol_definition_location(source, offset, &uri)
            .expect("definition should resolve");

        let actual = location.uri.to_file_path().unwrap();
        assert_eq!(actual, std::fs::canonicalize(theme).unwrap());
    }

    #[test]
    fn finds_imported_token_reference_locations() {
        let root =
            std::env::temp_dir().join(format!("frame-lsp-references-{}", std::process::id()));
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
        let offset = source.rfind("brand-panel").unwrap() + 1;

        let locations = support::imported_symbol_reference_locations(source, offset, &uri);

        assert_eq!(locations.len(), 2);
        assert!(locations.iter().all(|location| location
            .uri
            .to_file_path()
            .unwrap()
            .ends_with("theme.frame")));
    }
}
