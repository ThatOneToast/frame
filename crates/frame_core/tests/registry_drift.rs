//! Registry consistency drift tests.
//! Ensures parser, semantic, LSP, and runtime DOM surfaces stay aligned
//! with the canonical language registry in `crates/frame_core/src/language.rs`.

use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Hardcoded extracts from external sources (these lists act as the
// cross-language contract and must be updated when the sources change).
// ---------------------------------------------------------------------------

/// `ELEMENT_TAGS` keys from `packages/runtime-dom/src/index.ts`.
const RUNTIME_ELEMENT_TAGS: &[&str] = &[
    "a", "action", "area", "article", "audio", "avatar", "badge", "button", "canvas", "card",
    "caption", "choice", "col", "colgroup", "composer", "data", "dd", "details", "dialog", "div",
    "dock", "dl", "dt", "editor", "empty", "feed", "field", "fieldset", "footer", "form", "grid",
    "h1", "h2", "h3", "h4", "h5", "h6", "header", "icon", "image", "img", "input", "item", "label",
    "legend", "link", "li", "list", "main", "media", "menu", "meter", "nav", "ol", "optgroup",
    "option", "output", "overlay", "p", "panel", "path", "picture", "popover", "progress", "row",
    "scroll", "screen", "section", "select", "source", "span", "stack", "summary", "svg", "table",
    "tabs", "tbody", "td", "textarea", "text", "tfoot", "th", "thead", "toggle", "toolbar",
    "track", "tr", "ul", "video",
];

/// `BROWSER_UI_WORDS` from `crates/frame_core/src/semantic/constants.rs`.
const BROWSER_UI_WORDS: &[&str] = &[
    "a", "article", "audio", "button", "canvas", "caption", "col", "colgroup", "dd", "details",
    "div", "dl", "dt", "fieldset", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "header",
    "img", "legend", "li", "main", "meter", "nav", "ol", "optgroup", "option", "output", "p",
    "path", "picture", "progress", "source", "span", "summary", "svg", "table", "tbody", "td",
    "textarea", "tfoot", "th", "thead", "tr", "track", "ul", "video", "area",
];

/// `UI_EVENTS` from `crates/frame_core/src/semantic/constants.rs`.
const SEMANTIC_UI_EVENTS: &[&str] = &[
    "press",
    "send",
    "open",
    "close",
    "select",
    "click",
    "input",
    "change",
    "submit",
    "reset",
    "keydown",
    "keyup",
    "focus",
    "blur",
    "pointerdown",
    "pointerup",
    "pointermove",
    "mouseenter",
    "mouseleave",
];

/// `UI_EVENT_MODIFIERS` from `crates/frame_core/src/semantic/constants.rs`.
const SEMANTIC_UI_EVENT_MODIFIERS: &[&str] = &[
    "enter", "escape", "tab", "space", "ctrl", "shift", "alt", "meta", "left", "right", "up",
    "down", "prevent", "stop", "once", "capture", "passive",
];

/// `SEMANTIC_UI_PRIMITIVES` from `crates/frame_core/src/semantic/constants.rs`.
const SEMANTIC_UI_PRIMITIVES: &[&str] = &[
    "screen", "panel", "section", "stack", "row", "grid", "split", "dock", "overlay", "scroll",
    "action", "link", "menu", "toolbar", "tabs", "field", "input", "editor", "toggle", "choice",
    "select", "composer", "title", "text", "label", "badge", "avatar", "icon", "image", "media",
    "list", "feed", "data", "item", "empty", "card", "dialog", "popover",
];

/// Hardcoded match arms from `frame_parser/src/helpers.rs` `declaration_kind()`.
const PARSER_DECLARATION_KINDS: &[&str] = &[
    "grid",
    "area",
    "card",
    "stack",
    "row",
    "button",
    "text",
    "tokens",
    "theme",
    "motion",
    "recipe",
    "layout",
    "center",
    "split",
    "overlay",
    "dock",
    "keyframes",
    "supports",
    "style-group",
    "style-order",
    "html",
    "page-body",
];

/// Keywords that have dedicated grammar rules in both parser and tree-sitter.
const DEDICATED_RULE_KEYWORDS: &[&str] = &["component", "supports", "style-group", "style-order"];

/// Registry events that are classified as something other than `Event` but are
/// still treated as events by the semantic layer and runtime.
const SEMANTIC_EVENT_EXCEPTIONS: &[&str] = &["press", "blur", "focus", "select", "input"];

/// Registry items that appear in `SEMANTIC_UI_PRIMITIVES` but are classified
/// as `Declaration` in the registry.
const SEMANTIC_PRIMITIVE_DECLARATION_EXCEPTIONS: &[&str] = &[
    "stack", "row", "grid", "split", "dock", "overlay", "card", "text", "button", "center",
];

/// Browser words that are intentionally registered as Frame declarations.
const BROWSER_WORD_REGISTRY_OVERLAPS: &[&str] = &["button", "area"];

/// Registry primitives that are not yet mapped in the runtime DOM.
const RUNTIME_MISSING_PRIMITIVES: &[&str] = &["title"];

// ---------------------------------------------------------------------------
// Forward consistency: registry -> downstream
// ---------------------------------------------------------------------------

#[test]
fn registry_primitives_recognized_by_parser() {
    for name in frame_core::language::ui_primitive_keywords() {
        assert!(
            frame_core::language::is_ui_primitive(name),
            "registry primitive `{name}` not recognized by is_ui_primitive"
        );
    }
}

#[test]
fn parser_declaration_keywords_exist_in_registry() {
    let registry: HashSet<&str> = frame_core::language::declaration_keywords()
        .iter()
        .chain(frame_core::language::ui_primitive_keywords().iter())
        .copied()
        .collect();
    for name in PARSER_DECLARATION_KINDS {
        assert!(
            registry.contains(name),
            "parser declaration_kind keyword `{name}` missing from registry"
        );
    }
}

#[test]
fn registry_declarations_in_parser_or_dedicated() {
    let parser_kinds: HashSet<&str> = PARSER_DECLARATION_KINDS.iter().copied().collect();
    let dedicated: HashSet<&str> = DEDICATED_RULE_KEYWORDS.iter().copied().collect();
    for item in
        frame_core::language::items_by_kind(frame_core::language::LanguageItemKind::Declaration)
    {
        let name = item.name;
        assert!(
            parser_kinds.contains(name) || dedicated.contains(name),
            "registry declaration `{name}` not handled by parser declaration_kind() or dedicated rules"
        );
    }
}

#[test]
fn registry_events_in_semantic_constants() {
    let semantic_events: HashSet<&str> = SEMANTIC_UI_EVENTS.iter().copied().collect();
    for name in frame_core::language::event_keywords() {
        assert!(
            semantic_events.contains(name),
            "registry event `{name}` missing from semantic UI_EVENTS constant"
        );
    }
}

#[test]
fn registry_modifiers_in_semantic_constants() {
    let semantic_modifiers: HashSet<&str> = SEMANTIC_UI_EVENT_MODIFIERS.iter().copied().collect();
    for name in frame_core::language::event_modifiers() {
        assert!(
            semantic_modifiers.contains(name),
            "registry event modifier `{name}` missing from semantic UI_EVENT_MODIFIERS constant"
        );
    }
}

#[test]
fn runtime_element_tags_are_semantic_or_browser() {
    let registry_names: HashSet<&str> = frame_core::language::declaration_keywords()
        .iter()
        .chain(frame_core::language::ui_primitive_keywords().iter())
        .copied()
        .collect();
    let browser_words: HashSet<&str> = BROWSER_UI_WORDS.iter().copied().collect();
    for name in RUNTIME_ELEMENT_TAGS {
        assert!(
            registry_names.contains(name) || browser_words.contains(name),
            "runtime ELEMENT_TAGS key `{name}` is neither a registry declaration/primitive nor a known browser word"
        );
    }
}

#[test]
fn registry_primitives_have_runtime_mapping() {
    let runtime_tags: HashSet<&str> = RUNTIME_ELEMENT_TAGS.iter().copied().collect();
    let exceptions: HashSet<&str> = RUNTIME_MISSING_PRIMITIVES.iter().copied().collect();
    for name in frame_core::language::ui_primitive_keywords() {
        if exceptions.contains(name) {
            continue;
        }
        assert!(
            runtime_tags.contains(name),
            "registry primitive `{name}` missing from runtime ELEMENT_TAGS"
        );
    }
}

// ---------------------------------------------------------------------------
// Reverse consistency: downstream -> registry
// These tests document known classification gaps. If the gap is fixed,
// update the exception list so the test continues to guard against drift.
// ---------------------------------------------------------------------------

#[test]
fn semantic_events_are_registry_events_or_documented() {
    let registry_events: HashSet<&str> = frame_core::language::event_keywords()
        .iter()
        .copied()
        .collect();
    let exceptions: HashSet<&str> = SEMANTIC_EVENT_EXCEPTIONS.iter().copied().collect();
    for name in SEMANTIC_UI_EVENTS {
        assert!(
            registry_events.contains(name) || exceptions.contains(name),
            "semantic event `{name}` missing from registry events (add exception if intentional)"
        );
    }
}

#[test]
fn semantic_primitives_are_registry_primitives_or_documented() {
    let registry_primitives: HashSet<&str> = frame_core::language::ui_primitive_keywords()
        .iter()
        .copied()
        .collect();
    let registry_declarations: HashSet<&str> = frame_core::language::declaration_keywords()
        .iter()
        .copied()
        .collect();
    let exceptions: HashSet<&str> = SEMANTIC_PRIMITIVE_DECLARATION_EXCEPTIONS
        .iter()
        .copied()
        .collect();
    for name in SEMANTIC_UI_PRIMITIVES {
        assert!(
            registry_primitives.contains(name)
                || registry_declarations.contains(name)
                || exceptions.contains(name),
            "semantic primitive `{name}` missing from registry (add exception if intentional)"
        );
    }
}

#[test]
fn runtime_browser_words_are_not_in_registry() {
    // Ensure we don't accidentally register browser words as Frame concepts.
    let registry_names: HashSet<&str> = frame_core::language::declaration_keywords()
        .iter()
        .chain(frame_core::language::ui_primitive_keywords().iter())
        .copied()
        .collect();
    let overlaps: HashSet<&str> = BROWSER_WORD_REGISTRY_OVERLAPS.iter().copied().collect();
    for name in BROWSER_UI_WORDS {
        if overlaps.contains(name) {
            continue;
        }
        assert!(
            !registry_names.contains(name),
            "browser word `{name}` should not be in registry as declaration or primitive (add exception if intentional)"
        );
    }
}

// ---------------------------------------------------------------------------
// Registry internal health
// ---------------------------------------------------------------------------

#[test]
fn registry_has_no_duplicate_names() {
    let mut seen = HashSet::new();
    for item in
        frame_core::language::items_by_kind(frame_core::language::LanguageItemKind::Primitive)
    {
        assert!(
            seen.insert(item.name),
            "duplicate primitive name: {}",
            item.name
        );
    }
    for item in
        frame_core::language::items_by_kind(frame_core::language::LanguageItemKind::Declaration)
    {
        assert!(
            seen.insert(item.name),
            "duplicate declaration name: {}",
            item.name
        );
    }
    for item in frame_core::language::items_by_kind(frame_core::language::LanguageItemKind::Event) {
        assert!(
            seen.insert(item.name),
            "duplicate event name: {}",
            item.name
        );
    }
    for item in
        frame_core::language::items_by_kind(frame_core::language::LanguageItemKind::EventModifier)
    {
        assert!(
            seen.insert(item.name),
            "duplicate modifier name: {}",
            item.name
        );
    }
}
