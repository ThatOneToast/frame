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
mod project;
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

    #[test]
    fn cross_file_completions_include_imported_components() {
        let root =
            std::env::temp_dir().join(format!("frame-lsp-completion-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let components = root.join("components.frame");
        std::fs::write(
            &components,
            "component MessageItem {\n  view {\n    text \"Hello\"\n  }\n}\n",
        )
        .expect("components should be writable");
        let source = "#include components\n\ncomponent ChatApp {\n  view {\n    \n  }\n}\n";
        std::fs::write(&app, source).expect("app should be writable");
        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.find("    \n").unwrap() + 4;

        let items = completions::completions_at_with_includes(
            source,
            offset,
            vec![components.clone()],
            Some(&uri),
        );
        let labels: Vec<String> = items.into_iter().map(|i| i.label).collect();
        assert!(labels.contains(&"MessageItem".to_string()));
    }

    #[test]
    fn cross_file_hover_shows_imported_declaration_docs() {
        let root = std::env::temp_dir().join(format!("frame-lsp-hover-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let styles = root.join("styles.frame");
        std::fs::write(&styles, "card PrimaryAction {\n  surface accent\n}\n")
            .expect("styles should be writable");
        let source = "#include styles\n\ncard Hero {\n  background PrimaryAction\n}\n";
        std::fs::write(&app, source).expect("app should be writable");
        let merged = support::merged_frame_source(source, &Url::from_file_path(&app).unwrap());
        let document = frame_parser::parse(&merged).expect("parse");
        let symbols = frame_core::symbols::index_document(&merged, &document);
        let offset = source.find("PrimaryAction").unwrap() + 2;

        let doc = hover::hover_doc_at_with_symbols(source, offset, Some(&symbols))
            .expect("hover should exist");
        assert!(doc.contains("Frame style declaration"));
    }

    #[test]
    fn cross_file_definition_resolves_imported_grid() {
        let root = std::env::temp_dir().join(format!("frame-lsp-def-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let layout = root.join("layout.frame");
        std::fs::write(&layout, "grid AppShell {\n  columns sidebar content\n}\n")
            .expect("layout should be writable");
        let source = "#include layout\n\narea Sidebar {\n  in AppShell\n  place sidebar\n}\n";
        std::fs::write(&app, source).expect("app should be writable");
        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.rfind("AppShell").unwrap() + 2;

        let location = support::imported_symbol_definition_location(source, offset, &uri)
            .expect("definition should resolve");
        assert!(location
            .uri
            .to_file_path()
            .unwrap()
            .ends_with("layout.frame"));
    }

    #[test]
    fn lsp_cross_file_references_find_token_usages() {
        let root = std::env::temp_dir().join(format!("frame-lsp-ref-token-{}", std::process::id()));
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

        let references = navigation::references_at(source, offset, &uri);

        // 1 local + 2 in theme.frame
        assert_eq!(references.len(), 3);

        let local = references.iter().filter(|r| r.path.is_none()).count();
        let cross = references.iter().filter(|r| r.path.is_some()).count();

        assert_eq!(local, 1);
        assert_eq!(cross, 2);
    }

    #[test]
    fn lsp_cross_file_references_find_component_declarations_and_invocations() {
        let root = std::env::temp_dir().join(format!("frame-lsp-ref-comp-{}", std::process::id()));
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

        let references = navigation::references_at(source, offset, &uri);

        assert_eq!(references.len(), 2);

        let local = references.iter().filter(|r| r.path.is_none()).count();
        let cross = references.iter().filter(|r| r.path.is_some()).count();

        assert_eq!(local, 1);
        assert_eq!(cross, 1);
    }

    #[test]
    fn lsp_cross_file_references_find_handler_refs() {
        let root =
            std::env::temp_dir().join(format!("frame-lsp-ref-handler-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let handlers = root.join("handlers.frame");

        std::fs::write(
            &handlers,
            "component ButtonPanel {\n  view {\n    action Send {\n      on press @sendMessage\n    }\n  }\n}\n",
        )
        .expect("handlers should be writable");

        let source = "#include handlers\n\ncomponent ChatApp {\n  view {\n    button Cancel {\n      on click @sendMessage\n    }\n  }\n}\n";
        std::fs::write(&app, source).expect("app should be writable");

        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.find("@sendMessage").unwrap() + 2;

        let references = navigation::references_at(source, offset, &uri);

        assert_eq!(references.len(), 2);

        let local = references.iter().filter(|r| r.path.is_none()).count();
        let cross = references.iter().filter(|r| r.path.is_some()).count();

        assert_eq!(local, 1);
        assert_eq!(cross, 1);
    }

    #[test]
    fn lsp_cross_file_references_find_state_refs() {
        let root = std::env::temp_dir().join(format!("frame-lsp-ref-state-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temp dir should be writable");
        let app = root.join("app.frame");
        let state_file = root.join("state.frame");

        std::fs::write(
            &state_file,
            "component InputPanel {\n  state {\n    draft text = \"\"\n  }\n  view {\n    input Box {\n      value bind $draft\n    }\n  }\n}\n",
        )
        .expect("state should be writable");

        let source = "#include state\n\ncomponent ChatApp {\n  view {\n    text $draft\n  }\n}\n";
        std::fs::write(&app, source).expect("app should be writable");

        let uri = Url::from_file_path(&app).expect("file uri should build");
        let offset = source.rfind("$draft").unwrap() + 2;

        let references = navigation::references_at(source, offset, &uri);

        // state decl + bind ref in state.frame + text ref in app.frame
        assert_eq!(references.len(), 3);

        let local = references.iter().filter(|r| r.path.is_none()).count();
        let cross = references.iter().filter(|r| r.path.is_some()).count();

        assert_eq!(local, 1);
        assert_eq!(cross, 2);
    }
}
