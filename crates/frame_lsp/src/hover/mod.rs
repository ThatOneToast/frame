use crate::ide::cursor::{CursorSlot, SemanticCursor};
use docs::{
    INCLUDE_DOC, SURFACE_GLASS_DOC, SURFACE_MAIN_DOC, SURFACE_PANEL_DOC, WIDTH_PERCENT_DOC,
};
use frame_core::{language, symbols::SymbolIndex};
use helpers::line_at;
use values::contextual_value_doc;

mod docs;
mod helpers;
mod values;

pub use helpers::word_at;

#[allow(dead_code)]
pub fn hover_doc_at(source: &str, offset: usize) -> Option<String> {
    hover_doc_at_with_symbols(source, offset, None)
}

pub fn hover_doc_at_with_symbols(
    source: &str,
    offset: usize,
    symbols: Option<&SymbolIndex>,
) -> Option<String> {
    let word = word_at(source, offset)?;
    let line = line_at(source, offset);
    let words = line.split_whitespace().collect::<Vec<_>>();

    // Build semantic cursor for context-aware hover
    let cursor = SemanticCursor::at(source, offset);

    match cursor.slot {
        CursorSlot::DataReference => {
            let name = word.strip_prefix('$').unwrap_or(word);
            if let Some(sym) = cursor
                .scope
                .local_state
                .iter()
                .chain(&cursor.scope.local_props)
                .chain(&cursor.scope.loop_vars)
                .find(|s| s.name == name)
            {
                let kind_label = match sym.kind {
                    super::ide::cursor::SymbolKind::State => "state",
                    super::ide::cursor::SymbolKind::Prop => "prop",
                    super::ide::cursor::SymbolKind::LoopVar => "loop variable",
                    _ => "variable",
                };
                return Some(format!(
                    "## `${}`\n\n{} {}\n\n{}",
                    sym.name,
                    kind_label,
                    sym.detail,
                    if kind_label == "prop" {
                        "Props are passed from the parent component."
                    } else if kind_label == "loop variable" {
                        "Loop variable scoped to this iteration."
                    } else {
                        "Text interpolation escapes by default."
                    }
                ));
            }
            return hover_doc(word);
        }
        CursorSlot::HandlerReference => {
            let name = word.strip_prefix('@').unwrap_or(word);
            if let Some(sym) = cursor.scope.handlers.iter().find(|s| s.name == name) {
                return Some(format!(
                    "## `@{}`\n\nHandler reference `{}`.\n\n{}",
                    sym.name,
                    sym.name,
                    sym.detail
                ));
            }
            return hover_doc(word);
        }
        _ => {}
    }

    if line.starts_with("#include") {
        return Some(INCLUDE_DOC.to_string());
    }

    match words.as_slice() {
        ["surface", "panel"] if word == "panel" || word == "surface" => {
            return Some(
                language::hover_doc_for("surface panel")
                    .unwrap_or_else(|| SURFACE_PANEL_DOC.to_string()),
            );
        }
        ["surface", "main"] if word == "main" || word == "surface" => {
            return Some(
                language::hover_doc_for("surface main")
                    .unwrap_or_else(|| SURFACE_MAIN_DOC.to_string()),
            );
        }
        ["surface", "glass"] if word == "glass" || word == "surface" => {
            return Some(
                language::hover_doc_for("surface glass")
                    .unwrap_or_else(|| SURFACE_GLASS_DOC.to_string()),
            );
        }
        ["width" | "height", value] if value.ends_with('%') => {
            return Some(
                language::hover_doc_for("width 25%")
                    .unwrap_or_else(|| WIDTH_PERCENT_DOC.to_string()),
            );
        }
        _ => {}
    }

    // Prefer caller-supplied symbols, fall back to cursor symbols
    let symbols = symbols.unwrap_or(&cursor.symbols);

    if let Some(color) = symbols.colors.get(word) {
        return Some(format!(
            "## `{}`\n\nCustom color token.\n\nValue:\n\n```css\n{}\n```\n\nUse it anywhere Frame accepts color intent, including `background`, `color`, `border`, `glow`, and `ring`.\n\n### Frame\n\n```frame\ncard BrandCard {{\n  background {}\n  color {}\n}}\n```",
            color.name,
            color.value.as_deref().unwrap_or("custom color"),
            color.name,
            color.name
        ));
    }

    if let Some(gradient) = symbols.gradients.get(word) {
        return Some(format!(
            "## `{}`\n\nCustom gradient token.\n\nGenerated behavior:\n\n```css\n{}\n```\n\nUse it for hero cards, highlighted dashboard cards, panels, and sign-in screens.\n\n### Frame\n\n```frame\ncard HeroCard {{\n  background {}\n  color white\n}}\n```",
            gradient.name,
            gradient.value.as_deref().unwrap_or("linear-gradient(...)"),
            gradient.name
        ));
    }

    if let Some(keyframes) = symbols.keyframes.get(word) {
        return Some(format!(
            "## `{}`\n\nCustom keyframes animation.\n\nGenerated CSS:\n\n```css\n{}\n```\n\nUse it with a structured animation block:\n\n```frame\ncard Panel {{\n  animation {} {{\n    duration 240ms\n    ease smooth\n    fill both\n  }}\n}}\n```",
            keyframes.name,
            keyframes
                .value
                .as_deref()
                .unwrap_or("@keyframes frame-Name"),
            keyframes.name
        ));
    }

    if let Some(declaration) = symbols.declarations.get(word) {
        // Check if this is used as a base style by another declaration
        let inherited_by: Vec<&str> = symbols
            .declarations
            .values()
            .filter(|d| d.base.as_deref() == Some(word))
            .map(|d| d.name.as_str())
            .collect();
        let mut result = format!(
            "## `{}`\n\nFrame style declaration.\n\nKind: `{}`\n\nUse it as a style binding or automatic style lookup target in UI nodes.",
            declaration.name,
            declaration_kind_label(&declaration.kind)
        );
        if !inherited_by.is_empty() {
            result.push_str(&format!(
                "\n\nInherited by: {}.",
                inherited_by.join(", ")
            ));
        }
        return Some(result);
    }

    if let Some(component) = symbols.components.get(word) {
        return Some(format!(
            "## `{}`\n\nFrame component.\n\nInvoke it in view with `{}(...)`. Components encapsulate props, state, and a semantic view tree.",
            component.name, component.name
        ));
    }

    if let Some(doc) = contextual_value_doc(word, &words, Some(symbols)) {
        return Some(doc);
    }

    hover_doc(word)
}

pub fn hover_doc(word: &str) -> Option<String> {
    if let Some(doc) = language::hover_doc_for(word) {
        return Some(doc);
    }

    Some(match word {
        _ if word.starts_with('$') => {
            "$value reads typed component state, props, or loop variables."
        }
        _ if word.starts_with('@') => {
            "@handler references an external event handler."
        }
        _ => return None,
    }.to_string())
}

fn declaration_kind_label(kind: &frame_core::symbols::SymbolKind) -> &'static str {
    use frame_core::symbols::SymbolKind;
    match kind {
        SymbolKind::Declaration(frame_core::DeclarationKind::Grid) => "grid",
        SymbolKind::Declaration(frame_core::DeclarationKind::Area) => "area",
        SymbolKind::Declaration(frame_core::DeclarationKind::Card) => "card",
        SymbolKind::Declaration(frame_core::DeclarationKind::Stack) => "stack",
        SymbolKind::Declaration(frame_core::DeclarationKind::Row) => "row",
        SymbolKind::Declaration(frame_core::DeclarationKind::Button) => "button",
        SymbolKind::Declaration(frame_core::DeclarationKind::Text) => "text",
        SymbolKind::Declaration(frame_core::DeclarationKind::Tokens) => "tokens",
        SymbolKind::Declaration(frame_core::DeclarationKind::Center) => "center",
        SymbolKind::Declaration(frame_core::DeclarationKind::Split) => "split",
        SymbolKind::Declaration(frame_core::DeclarationKind::Overlay) => "overlay",
        SymbolKind::Declaration(frame_core::DeclarationKind::Dock) => "dock",
        SymbolKind::Declaration(frame_core::DeclarationKind::Keyframes) => "keyframes",
        SymbolKind::Declaration(frame_core::DeclarationKind::Supports) => "supports",
        SymbolKind::Declaration(frame_core::DeclarationKind::StyleGroup) => "style-group",
        SymbolKind::Declaration(frame_core::DeclarationKind::StyleOrder) => "style-order",
        SymbolKind::Declaration(frame_core::DeclarationKind::Html) => "html",
        SymbolKind::Declaration(frame_core::DeclarationKind::Body) => "body",
        SymbolKind::Declaration(frame_core::DeclarationKind::Unknown(_)) => "declaration",
        SymbolKind::Color => "color token",
        SymbolKind::Gradient => "gradient token",
        SymbolKind::Keyframes => "keyframes",
        SymbolKind::GridSection { .. } => "grid section",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hover_docs_for_concepts() {
        let doc = hover_doc("grid").expect("grid should have docs");

        assert!(doc.contains("Defines a layout container"));
        assert!(doc.contains("```frame"));

        assert!(hover_doc("component")
            .expect("component docs")
            .contains("Frame UI component"));
        assert!(hover_doc("field")
            .expect("field docs")
            .contains("Groups a label"));
        assert!(hover_doc("action")
            .expect("action docs")
            .contains("on press @handler"));
        assert!(hover_doc("$draft")
            .expect("data ref docs")
            .contains("typed component state"));
        assert!(hover_doc("@sendMessage")
            .expect("handler docs")
            .contains("external event handler"));
    }

    #[test]
    fn finds_word_at_offset() {
        let source = "card ProjectCard {\n  surface panel\n}\n";
        let offset = source.find("surface").unwrap() + 2;

        assert_eq!(word_at(source, offset), Some("surface"));
    }

    #[test]
    fn returns_surface_value_hover_docs() {
        let source = "area Sidebar {\n  surface panel\n}\n";
        let offset = source.find("panel").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("panel should have docs");

        assert!(doc.contains("surface"));
        assert!(doc.contains("panel"));
    }

    #[test]
    fn returns_columns_and_alignment_hover_docs() {
        assert!(hover_doc("columns").unwrap().contains("25% 50% 25%"));
        assert!(hover_doc("align").unwrap().contains("cross-axis"));
        assert!(hover_doc("justify").unwrap().contains("main-axis"));
        assert!(hover_doc("display").unwrap().contains("display: ..."));
        assert!(hover_doc("flex").unwrap().contains("flex direction"));
        assert!(hover_doc("inline-size").unwrap().contains("logical inline"));
        assert!(hover_doc("decoration")
            .unwrap()
            .contains("text-decoration-line"));
        assert!(hover_doc("whitespace").unwrap().contains("white-space"));
        assert!(hover_doc("word-break").unwrap().contains("word-break"));
        assert!(hover_doc("hyphenate").unwrap().contains("hyphens"));
        assert!(hover_doc("focus-visible")
            .unwrap()
            .contains(":focus-visible"));
        assert!(hover_doc("focus-within").unwrap().contains(":focus-within"));
        assert!(hover_doc("checked").unwrap().contains(":checked"));
        assert!(hover_doc("invalid").unwrap().contains(":invalid"));
        assert!(hover_doc("required").unwrap().contains(":required"));
        assert!(hover_doc("target").unwrap().contains(":target"));
        assert!(hover_doc("lift").unwrap().contains("small%44"));
        assert!(hover_doc("tilt").unwrap().contains("subtle%23"));
        assert!(hover_doc("supports").unwrap().contains("@supports"));
        assert!(hover_doc("style-group").unwrap().contains("cascade layers"));
        assert!(hover_doc("style-order")
            .unwrap()
            .contains("style group order"));
    }

    #[test]
    fn returns_percentage_hover_docs() {
        let source = "card A {\n  width 25%\n}\n";
        let offset = source.find("25%").unwrap() + 1;

        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("available width"));
    }

    #[test]
    fn returns_contextual_value_hover_docs() {
        let source = "grid Dashboard {\n  columns sidebar content\n\n  below tablet {\n    columns content\n  }\n}\n";

        let offset = source.find("sidebar").unwrap() + 1;
        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("Grid column value"));

        let offset = source.find("tablet").unwrap() + 1;
        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("Responsive breakpoint"));
    }

    #[test]
    fn returns_project_keyframes_hover_docs() {
        let source = "keyframes FloatIn {\n  from {\n    opacity 0\n  }\n}\ncard Panel {\n  animation FloatIn\n}\n";
        let document = frame_parser::parse(source).expect("parse");
        let symbols = frame_core::symbols::index_document(source, &document);
        let offset = source.rfind("FloatIn").unwrap() + 1;
        let doc = hover_doc_at_with_symbols(source, offset, Some(&symbols)).expect("hover doc");

        assert!(doc.contains("Custom keyframes animation"));
        assert!(doc.contains("@keyframes frame-FloatIn"));
    }

    #[test]
    fn returns_state_hover_with_local_context() {
        let source = "component ChatApp {\n  state {\n    draft text = \"\"\n  }\n  view {\n    text $draft\n  }\n}\n";
        let offset = source.find("$draft").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("state hover should exist");

        assert!(doc.contains("$draft"));
        assert!(doc.contains("state text"));
        assert!(doc.contains("Text interpolation escapes by default"));
    }

    #[test]
    fn returns_prop_hover_with_type() {
        let source = "component ChatApp {\n  props {\n    channel text\n  }\n  view {\n    text $channel\n  }\n}\n";
        let offset = source.find("$channel").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("prop hover should exist");

        assert!(doc.contains("$channel"));
        assert!(doc.contains("prop text"));
    }

    #[test]
    fn returns_loop_var_hover() {
        let source = "component ChatApp {\n  view {\n    list Messages {\n      for msg in $messages {\n        text $msg\n      }\n    }\n  }\n}\n";
        let offset = source.rfind("$msg").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("loop var hover should exist");

        assert!(doc.contains("$msg"));
        assert!(doc.contains("for msg in ..."));
    }

    #[test]
    fn returns_handler_hover_with_references() {
        let source = "component ChatApp {\n  view {\n    action Send {\n      on press @sendMessage\n    }\n    action Cancel {\n      on press @sendMessage\n    }\n  }\n}\n";
        let offset = source.find("@sendMessage").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("handler hover should exist");

        assert!(doc.contains("@sendMessage"));
        assert!(doc.contains("Handler reference"));
    }

    #[test]
    fn returns_class_hover_docs() {
        let doc = hover_doc("class").expect("class should have docs");
        assert!(doc.contains("CSS class"));
    }

    #[test]
    fn returns_rel_hover_docs() {
        let doc = hover_doc("rel").expect("rel should have docs");
        assert!(doc.contains("noopener"));
    }

    #[test]
    fn returns_in_hover_with_loop_context() {
        let doc = hover_doc("in").expect("in should have docs");
        assert!(doc.contains("for"));
        assert!(doc.contains("loop"));
    }

    #[test]
    fn returns_inherited_base_style_hover() {
        let source = "card MetricCardBase {\n  padding medium\n}\ncard MetricCardModel extends MetricCardBase {\n}\n";
        let document = frame_parser::parse(source).expect("parse");
        let symbols = frame_core::symbols::index_document(source, &document);
        let offset = source.find("MetricCardBase").unwrap() + 1;
        let doc = hover_doc_at_with_symbols(source, offset, Some(&symbols)).expect("hover doc");

        assert!(doc.contains("MetricCardBase"));
        assert!(doc.contains("Inherited by"));
        assert!(doc.contains("MetricCardModel"));
    }

    #[test]
    fn returns_named_grid_track_hover() {
        let source = "grid Dashboard {\n  columns sidebar content\n}\n";
        let document = frame_parser::parse(source).expect("parse");
        let symbols = frame_core::symbols::index_document(source, &document);
        let offset = source.find("sidebar").unwrap() + 1;
        let doc = hover_doc_at_with_symbols(source, offset, Some(&symbols)).expect("hover doc");

        assert!(doc.contains("sidebar"));
        assert!(doc.contains("Grid column value"));
    }
}
