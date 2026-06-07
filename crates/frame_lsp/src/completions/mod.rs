mod helpers;
mod suggestions;
mod types;
mod values;

use crate::context::{completion_context, CompletionScope};
use frame_core::{symbols::index_document, tokens};
use frame_parser::parse;
use helpers::{
    is_inside_ancestor_block, is_inside_block, is_inside_keyframe_selector, line_at, line_words_at,
};
use std::path::PathBuf;
use suggestions::{
    dynamic_suggestions, include_documentation, include_suggestions, property_suggestions,
    snippet_suggestions, suggestions_with_category, supports_predicate_completions,
};
use types::SnippetScope;
pub use types::{CompletionCategory, CompletionSuggestion};
use values::value_completions;

const DECLARATIONS: &[&str] = &[
    "tokens",
    "grid",
    "area",
    "card",
    "stack",
    "row",
    "text",
    "center",
    "split",
    "overlay",
    "dock",
    "keyframes",
    "supports",
    "style-group",
    "style-order",
    "component",
];

const UI_ELEMENT_KINDS: &[&str] = &[
    "screen", "panel", "section", "stack", "row", "grid", "split", "dock", "overlay", "scroll",
    "action", "link", "menu", "toolbar", "tabs", "field", "input", "editor", "toggle", "choice",
    "select", "composer", "title", "text", "label", "badge", "avatar", "icon", "image", "media",
    "list", "feed", "data", "item", "empty", "card", "dialog", "popover",
];

const UI_KEYWORDS: &[&str] = &[
    "props",
    "state",
    "view",
    "slot",
    "for",
    "in",
    "key",
    "on",
    "bind",
    "when",
    "style",
    "disabled",
    "label",
    "hint",
    "description",
    "placeholder",
    "checked",
    "selected",
    "kind",
    "value",
    "source",
    "goto",
    "send",
    "draft",
    "options",
    "required",
    "decorative",
    "show",
    "hidden",
    "alt",
    "id",
    "class",
    "title",
    "data-test-id",
    "poster",
    "download",
    "new-window",
];

const UI_EVENTS: &[&str] = &[
    "press", "send", "open", "close", "select", "keydown", "keyup", "input", "change", "focus",
    "blur",
];

const UI_MODIFIERS: &[&str] = &[
    "enter", "escape", "ctrl", "shift", "alt", "meta", "prevent", "stop", "once", "capture",
    "passive",
];

const ACTION_BODY_COMPLETIONS: &[&str] = &[
    "on press @handler",
    "label \"Label\"",
    "text \"Label\"",
    "disabled when $state",
    "style StyleName when $state",
];

const FIELD_BODY_COMPLETIONS: &[&str] = &[
    "label \"Label\"",
    "description \"Help text\"",
    "hint \"Helper text\"",
    "input ValueInput",
    "editor TextEditor",
    "toggle Enabled",
    "value bind $state",
];

const INPUT_BODY_COMPLETIONS: &[&str] = &[
    "value bind $state",
    "placeholder \"Text\"",
    "label \"Label\"",
    "on input @handler",
    "disabled when $state",
    "style StyleName when $state",
];

const LIST_BODY_COMPLETIONS: &[&str] = &[
    "source $items",
    "for item in $items {",
    "for item in $items key $item.id {",
    "item Item",
    "empty Empty",
];

const GRID_PROPERTIES: &[&str] = &[
    "columns",
    "rows",
    "tracks",
    "areas",
    "flow",
    "section",
    "gap",
    "display",
    "height",
    "width",
    "inline-size",
    "block-size",
    "min-inline-size",
    "max-inline-size",
    "min-block-size",
    "max-block-size",
    "padding",
    "surface",
    "align",
    "justify",
    "anchor",
    "overflow",
    "scroll",
    "scrollbar",
    "box",
    "visibility",
    "flex",
];

const AREA_PROPERTIES: &[&str] = &[
    "in",
    "place",
    "col",
    "row",
    "span",
    "surface",
    "padding",
    "margin",
    "display",
    "width",
    "height",
    "inline-size",
    "block-size",
    "min-inline-size",
    "max-inline-size",
    "min-block-size",
    "max-block-size",
    "align",
    "justify",
    "border",
    "shadow",
    "anchor",
    "overflow",
    "scroll",
    "scrollbar",
    "box",
    "visibility",
    "flex",
];

const CARD_PROPERTIES: &[&str] = &[
    "surface",
    "background",
    "padding",
    "margin",
    "radius",
    "border",
    "shadow",
    "text",
    "color",
    "width",
    "height",
    "min-width",
    "max-width",
    "min-height",
    "max-height",
    "inline-size",
    "block-size",
    "min-inline-size",
    "max-inline-size",
    "min-block-size",
    "max-block-size",
    "align",
    "justify",
    "layout",
    "anchor",
    "overflow",
    "scroll",
    "scrollbar",
    "box",
    "display",
    "visibility",
    "flex",
    "square",
    "self",
    "nudge",
    "truncate",
    "wrap",
    "case",
    "align-text",
    "decoration",
    "whitespace",
    "word-break",
    "hyphenate",
    "line",
    "letter",
    "control",
    "interactive",
    "transition",
    "duration",
    "ease",
    "animation",
    "advanced",
    "hover",
    "focus",
    "focus-visible",
    "focus-within",
    "active",
    "disabled",
    "checked",
    "invalid",
    "required",
    "target",
];

const COMMON_PROPERTIES: &[&str] = &[
    "surface",
    "padding",
    "margin",
    "gap",
    "display",
    "width",
    "height",
    "min-width",
    "max-width",
    "min-height",
    "max-height",
    "inline-size",
    "block-size",
    "min-inline-size",
    "max-inline-size",
    "min-block-size",
    "max-block-size",
    "align",
    "justify",
    "layout",
    "text",
    "color",
    "background",
    "border",
    "overflow",
    "scroll",
    "scrollbar",
    "box",
    "visibility",
    "flex",
    "square",
    "truncate",
    "wrap",
    "case",
    "align-text",
    "decoration",
    "whitespace",
    "word-break",
    "hyphenate",
    "line",
    "letter",
    "control",
    "interactive",
    "shadow",
    "transition",
    "animation",
    "advanced",
];

const TOKEN_PROPERTIES: &[&str] = &["color", "gradient"];
const GRADIENT_PROPERTIES: &[&str] = &["type", "angle", "stop", "corner"];
const ANIMATION_PROPERTIES: &[&str] = &[
    "duration",
    "delay",
    "iteration",
    "direction",
    "fill",
    "play-state",
    "ease",
];
const KEYFRAME_SELECTORS: &[&str] = &["from", "to", "0%", "25%", "50%", "75%", "100%"];
const ADVANCED_PROPERTIES: &[&str] = &["css"];
const SECTION_PROPERTIES: &[&str] = &[
    "padding",
    "margin",
    "gap",
    "align",
    "justify",
    "width",
    "height",
    "min-width",
    "max-width",
    "min-height",
    "max-height",
    "inline-size",
    "block-size",
    "min-inline-size",
    "max-inline-size",
    "min-block-size",
    "max-block-size",
];

#[cfg(test)]
pub fn completions_at(source: &str, offset: usize) -> Vec<CompletionSuggestion> {
    completions_at_with_includes(source, offset, Vec::new())
}

pub fn completions_at_with_includes(
    source: &str,
    offset: usize,
    include_files: Vec<PathBuf>,
) -> Vec<CompletionSuggestion> {
    let context = completion_context(source, offset);
    let line_words = line_words_at(source, offset);
    let line = line_at(source, offset);
    let parsed_document = parse(source).ok();
    let component_names = parsed_document
        .as_ref()
        .map(|document| {
            document
                .components
                .iter()
                .map(|component| component.name.text.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let symbols = parsed_document
        .as_ref()
        .map(|document| index_document(source, document))
        .unwrap_or_default();

    if line.trim_start().starts_with("#include") {
        return include_suggestions(include_files);
    }

    if line_words.first().map(String::as_str) == Some("supports") {
        return supports_predicate_completions(&line_words);
    }

    if is_inside_ancestor_block(source, offset, "view") {
        if line_words.first().map(String::as_str) == Some("on") {
            let mut items = suggestions_with_category(
                UI_EVENTS,
                "event",
                "Event name for an external handler binding.",
                CompletionCategory::Value,
            );
            items.extend(suggestions_with_category(
                UI_MODIFIERS,
                "event modifier",
                "Keyboard or platform modifier used after an event name.",
                CompletionCategory::Value,
            ));
            return items;
        }
        if line_words.is_empty() || line.trim().is_empty() {
            if let Some(kind) = nearest_ui_element_kind(source, offset) {
                let items = match kind.as_str() {
                    "action" => Some(primitive_body_completions(
                        ACTION_BODY_COMPLETIONS,
                        "action body",
                        "Intent-first action syntax. Prefer `on press @handler` over browser event attributes.",
                    )),
                    "field" => Some(primitive_body_completions(
                        FIELD_BODY_COMPLETIONS,
                        "field body",
                        "Field structure and control binding suggestions.",
                    )),
                    "input" | "editor" | "toggle" | "choice" | "select" => Some(
                        primitive_body_completions(
                            INPUT_BODY_COMPLETIONS,
                            "input body",
                            "Input binding and state-driven control suggestions.",
                        ),
                    ),
                    "list" | "feed" | "data" => Some(primitive_body_completions(
                        LIST_BODY_COMPLETIONS,
                        "collection body",
                        "Collection rendering suggestions with keyed identity and empty states.",
                    )),
                    _ => None,
                };
                if let Some(items) = items {
                    return items;
                }
            }
        }
        let mut items = suggestions_with_category(
            UI_ELEMENT_KINDS,
            "ui primitive",
            "Semantic Frame UI primitive. Renderers lower the intent to their target platform.",
            CompletionCategory::Declaration,
        );
        items.extend(suggestions_with_category(
            UI_KEYWORDS,
            "ui keyword",
            "Experimental Frame UI syntax keyword.",
            CompletionCategory::Value,
        ));
        items.extend(dynamic_suggestions(
            component_names,
            "component",
            "Component declared in this Frame file.",
            CompletionCategory::ProjectSymbol,
        ));
        return items;
    }

    if is_inside_block(source, offset, "gradient") {
        if let Some(property) = line_words.first() {
            return value_completions(property, &line_words, &symbols);
        }
        return suggestions_with_category(
            GRADIENT_PROPERTIES,
            "gradient property",
            "Property inside a custom gradient token.",
            CompletionCategory::TokenProperty,
        );
    }

    if is_inside_block(source, offset, "animation") {
        if let Some(property) = line_words.first() {
            return value_completions(property, &line_words, &symbols);
        }
        let mut items = snippet_suggestions(SnippetScope::Animation);
        items.extend(suggestions_with_category(
            ANIMATION_PROPERTIES,
            "animation option",
            "Timing, easing, iteration, and fill options for a custom animation.",
            CompletionCategory::AnimationOption,
        ));
        return items;
    }

    if is_inside_keyframe_selector(source, offset) {
        if let Some(property) = line_words.first() {
            return value_completions(property, &line_words, &symbols);
        }
        return suggestions_with_category(
            tokens::KEYFRAME_PROPERTIES,
            "keyframe property",
            "Animatable property inside a keyframe selector.",
            CompletionCategory::MotionProperty,
        );
    }

    if is_inside_block(source, offset, "advanced") {
        return suggestions_with_category(
            ADVANCED_PROPERTIES,
            "advanced css",
            "Explicit scoped CSS escape hatch.",
            CompletionCategory::AdvancedProperty,
        );
    }

    if is_inside_block(source, offset, "section") {
        if let Some(property) = line_words.first() {
            return value_completions(property, &line_words, &symbols);
        }
        return suggestions_with_category(
            SECTION_PROPERTIES,
            "section property",
            "Spacing, sizing, and alignment for a named grid section.",
            CompletionCategory::LayoutProperty,
        );
    }

    match context.scope {
        CompletionScope::Root => {
            let mut items = snippet_suggestions(SnippetScope::Root);
            items.extend(suggestions_with_category(
                DECLARATIONS,
                "declaration",
                "Starts a Frame declaration.",
                CompletionCategory::Declaration,
            ));
            items.push(CompletionSuggestion {
                label: "#include".to_string(),
                detail: "include",
                documentation: include_documentation(),
                insert_text: Some("#include ".to_string()),
                is_snippet: false,
                category: CompletionCategory::Include,
            });
            items
        }
        CompletionScope::State { property } => property
            .as_deref()
            .map(|property| value_completions(property, &line_words, &symbols))
            .filter(|items| !items.is_empty())
            .unwrap_or_else(|| {
                let mut items = snippet_suggestions(SnippetScope::State);
                items.extend(suggestions_with_category(
                    tokens::EFFECTS,
                    "effect",
                    "Effect used inside an interaction state.",
                    CompletionCategory::MotionProperty,
                ));
                items
            }),
        CompletionScope::Declaration {
            kind,
            property,
            area_grid,
        } => {
            if let Some(property) = property {
                match property.as_str() {
                    "in" => {
                        return dynamic_suggestions(
                            context.symbols.grids.keys().cloned().collect(),
                            "grid",
                            "Grid declaration in the current document.",
                            CompletionCategory::GridReference,
                        )
                    }
                    "place" => {
                        if let Some(grid) = area_grid {
                            if let Some(columns) = context.symbols.grids.get(&grid) {
                                return dynamic_suggestions(
                                    columns.clone(),
                                    "grid area",
                                    "Named column or area from the referenced grid.",
                                    CompletionCategory::GridSection,
                                );
                            }
                        }
                    }
                    _ => {}
                }

                return value_completions(&property, &line_words, &symbols);
            }

            match kind.as_str() {
                "tokens" => suggestions_with_category(
                    TOKEN_PROPERTIES,
                    "token property",
                    "Token definition for reusable colors and gradients.",
                    CompletionCategory::TokenProperty,
                ),
                "grid" => {
                    let mut items = snippet_suggestions(SnippetScope::Grid);
                    items.extend(property_suggestions(
                        GRID_PROPERTIES,
                        "grid property",
                        "Property for grid layout and child placement.",
                    ));
                    items
                }
                "keyframes" => {
                    let mut items = snippet_suggestions(SnippetScope::Keyframes);
                    items.extend(suggestions_with_category(
                        KEYFRAME_SELECTORS,
                        "keyframe selector",
                        "Selector inside a keyframes declaration.",
                        CompletionCategory::KeyframeSelector,
                    ));
                    items
                }
                "area" => property_suggestions(
                    AREA_PROPERTIES,
                    "area property",
                    "Property for a child region inside a grid.",
                ),
                "card" | "button" => {
                    let mut items = snippet_suggestions(SnippetScope::Component);
                    items.extend(property_suggestions(
                        CARD_PROPERTIES,
                        "component property",
                        "Property for a reusable UI surface.",
                    ));
                    items
                }
                _ => property_suggestions(
                    COMMON_PROPERTIES,
                    "property",
                    "Adds design intent to this declaration.",
                ),
            }
        }
    }
}

fn primitive_body_completions(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
) -> Vec<CompletionSuggestion> {
    labels
        .iter()
        .map(|label| CompletionSuggestion {
            label: (*label).to_string(),
            detail,
            documentation: documentation.to_string(),
            insert_text: Some((*label).to_string()),
            is_snippet: false,
            category: CompletionCategory::Value,
        })
        .collect()
}

fn nearest_ui_element_kind(source: &str, offset: usize) -> Option<String> {
    let mut stack = Vec::new();
    let prefix = &source[..offset.min(source.len())];
    for line in prefix.lines() {
        let trimmed = line.trim();
        if trimmed == "}" {
            stack.pop();
            continue;
        }
        if trimmed.ends_with('{') {
            let first = trimmed.split_whitespace().next().unwrap_or_default();
            if UI_ELEMENT_KINDS.contains(&first) {
                stack.push(first.to_string());
            } else if matches!(first, "component" | "props" | "state" | "view" | "for") {
                stack.push(String::new());
            }
        }
    }
    stack.into_iter().rev().find(|kind| !kind.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels_for(source: &str) -> Vec<String> {
        completions_at(source, source.len())
            .into_iter()
            .map(|item| item.label)
            .collect()
    }

    fn labels_at(source: &str, marker: &str) -> Vec<String> {
        let offset = source.find(marker).expect("marker should exist") + marker.len();
        completions_at(source, offset)
            .into_iter()
            .map(|item| item.label)
            .collect()
    }

    fn items_for(source: &str) -> Vec<CompletionSuggestion> {
        completions_at(source, source.len())
    }

    #[test]
    fn root_scope_only_suggests_declarations() {
        let labels = labels_for("");

        assert!(labels.contains(&"grid".to_string()));
        assert!(labels.contains(&"card".to_string()));
        assert!(labels.contains(&"component".to_string()));
        assert!(labels.contains(&"keyframes".to_string()));
        assert!(!labels.contains(&"panel".to_string()));
        assert!(!labels.contains(&"medium".to_string()));
    }

    #[test]
    fn view_scope_suggests_ui_syntax() {
        let labels = labels_for("component ChatInput {\n  view {\n    ");

        assert!(labels.contains(&"action".to_string()));
        assert!(labels.contains(&"field".to_string()));
        assert!(labels.contains(&"composer".to_string()));
        assert!(labels.contains(&"input".to_string()));
        assert!(labels.contains(&"on".to_string()));
        assert!(labels.contains(&"bind".to_string()));
        assert!(labels.contains(&"when".to_string()));
    }

    #[test]
    fn view_scope_suggests_known_components() {
        let source = "component ChatPanel {\n}\n\ncomponent ChatApp {\n  view {\n    \n  }\n}\n";
        let offset = source.find("    \n").expect("cursor line") + 4;
        let labels = completions_at(source, offset)
            .into_iter()
            .map(|item| item.label)
            .collect::<Vec<_>>();

        assert!(labels.contains(&"ChatPanel".to_string()));
    }

    #[test]
    fn event_lines_suggest_events_and_modifiers() {
        let labels = labels_for("component ChatInput {\n  view {\n    action Send {\n      on ");

        assert!(labels.contains(&"press".to_string()));
        assert!(labels.contains(&"keydown".to_string()));
        assert!(labels.contains(&"enter".to_string()));
        assert!(labels.contains(&"ctrl".to_string()));
    }

    #[test]
    fn primitive_bodies_prioritize_contextual_completions() {
        let labels = labels_for("component Demo {\n  view {\n    action Send {\n      ");
        assert!(labels.contains(&"on press @handler".to_string()));
        assert!(labels.contains(&"disabled when $state".to_string()));
        assert!(!labels.contains(&"screen".to_string()));

        let labels = labels_for("component Demo {\n  view {\n    field EmailField {\n      ");
        assert!(labels.contains(&"input ValueInput".to_string()));
        assert!(labels.contains(&"value bind $state".to_string()));

        let labels = labels_for("component Demo {\n  view {\n    list Messages {\n      ");
        assert!(labels.contains(&"for item in $items key $item.id {".to_string()));
        assert!(labels.contains(&"empty Empty".to_string()));
    }

    #[test]
    fn grid_block_suggests_grid_properties() {
        let items = items_for("grid Dashboard {\n  ");
        let labels = items
            .iter()
            .map(|item| item.label.clone())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"columns".to_string()));
        assert!(labels.contains(&"surface".to_string()));
        assert!(labels.contains(&"below tablet block".to_string()));
        assert!(!labels.contains(&"card".to_string()));
        assert_eq!(
            items
                .iter()
                .find(|item| item.label == "columns")
                .map(|item| item.category),
            Some(CompletionCategory::LayoutProperty)
        );
        assert_eq!(
            items
                .iter()
                .find(|item| item.label == "surface")
                .map(|item| item.category),
            Some(CompletionCategory::VisualProperty)
        );
    }

    #[test]
    fn grid_columns_suggests_column_values() {
        let labels = labels_for("grid Dashboard {\n  columns ");

        assert!(labels.contains(&"responsive".to_string()));
        assert!(labels.contains(&"sidebar".to_string()));
        assert!(labels.contains(&"inspector".to_string()));
    }

    #[test]
    fn grid_flow_and_section_blocks_have_contextual_completions() {
        let labels = labels_for("grid HoverCardInfo {\n  flow ");

        assert_eq!(
            labels,
            vec!["horizontal".to_string(), "vertical".to_string()]
        );

        let labels = labels_for("grid HoverCardInfo {\n  section title {\n    ");

        assert!(labels.contains(&"padding".to_string()));
        assert!(labels.contains(&"margin".to_string()));
        assert!(labels.contains(&"align".to_string()));
        assert!(!labels.contains(&"surface".to_string()));
    }

    #[test]
    fn area_in_suggests_grid_names() {
        let labels =
            labels_for("grid Dashboard {\n  columns sidebar content\n}\narea Sidebar {\n  in ");

        assert_eq!(labels, vec!["Dashboard".to_string()]);
    }

    #[test]
    fn area_place_suggests_referenced_grid_areas() {
        let labels = labels_for(
            "grid Dashboard {\n  columns sidebar content\n}\narea Sidebar {\n  in Dashboard\n  place ",
        );

        assert_eq!(labels, vec!["content".to_string(), "sidebar".to_string()]);
    }

    #[test]
    fn styling_values_suggest_custom_colors_and_gradients() {
        let labels = labels_at(
            "tokens Brand {\n  color brand-panel #181820\n  color brand-text #f8fafc\n  gradient hero-gradient {\n    type linear\n    angle 135deg\n    stop brand-panel 0%\n    stop brand-text 100%\n  }\n}\ncard Hero {\n  background \n}\n",
            "background ",
        );

        assert!(labels.contains(&"brand-panel".to_string()));
        assert!(labels.contains(&"hero-gradient".to_string()));

        let labels = labels_at(
            "tokens Brand {\n  color brand-panel #181820\n}\ncard Hero {\n  border \n}\n",
            "border ",
        );
        assert!(labels.contains(&"brand-panel".to_string()));
    }

    #[test]
    fn gradient_blocks_suggest_gradient_properties_and_values() {
        let labels = labels_for("tokens Brand {\n  gradient hero {\n    ");

        assert!(labels.contains(&"type".to_string()));
        assert!(labels.contains(&"stop".to_string()));
        assert!(labels.contains(&"corner".to_string()));

        let labels = labels_for("tokens Brand {\n  gradient hero {\n    corner ");
        assert!(labels.contains(&"top-left".to_string()));
    }

    #[test]
    fn card_block_suggests_card_properties() {
        let labels = labels_for("card ProjectCard {\n  ");

        assert!(labels.contains(&"surface".to_string()));
        assert!(labels.contains(&"hover".to_string()));
        assert!(labels.contains(&"focus-visible".to_string()));
        assert!(labels.contains(&"focus-within".to_string()));
        assert!(labels.contains(&"checked".to_string()));
        assert!(labels.contains(&"invalid".to_string()));
        assert!(labels.contains(&"required".to_string()));
        assert!(labels.contains(&"target".to_string()));
        assert!(labels.contains(&"hover state".to_string()));
        assert!(labels.contains(&"animation block".to_string()));
        assert!(!labels.contains(&"columns".to_string()));
    }

    #[test]
    fn state_blocks_suggest_effects() {
        let labels = labels_for("card ProjectCard {\n  hover {\n    ");

        assert!(labels.contains(&"lift".to_string()));
        assert!(labels.contains(&"shift".to_string()));
        assert!(labels.contains(&"grow".to_string()));
        assert!(labels.contains(&"glow".to_string()));
        assert!(labels.contains(&"hover effects".to_string()));
        assert!(!labels.contains(&"grid".to_string()));
    }

    #[test]
    fn motion_helpers_suggest_directions_and_tuned_amounts() {
        let labels = labels_for("card ProjectCard {\n  lift ");

        assert!(labels.contains(&"tiny".to_string()));
        assert!(labels.contains(&"small%44".to_string()));
        assert!(labels.contains(&"huge%50".to_string()));

        let labels = labels_for("card ProjectCard {\n  shift ");

        assert_eq!(
            labels,
            vec![
                "left".to_string(),
                "right".to_string(),
                "up".to_string(),
                "down".to_string(),
            ]
        );

        let labels = labels_for("card ProjectCard {\n  tilt right ");

        assert!(labels.contains(&"slight".to_string()));
        assert!(labels.contains(&"subtle%44".to_string()));
        assert!(labels.contains(&"dramatic%50".to_string()));
    }

    #[test]
    fn supports_blocks_suggest_typed_predicates() {
        let labels = labels_for("supports ");

        assert!(labels.contains(&"display".to_string()));
        assert!(labels.contains(&"backdrop".to_string()));
        assert!(labels.contains(&"subgrid".to_string()));

        let labels = labels_for("supports display ");

        assert_eq!(labels, vec!["grid".to_string(), "flex".to_string()]);

        let labels = labels_for("supports selector ");

        assert_eq!(labels, vec!["has".to_string()]);
    }

    #[test]
    fn root_suggests_style_groups() {
        let labels = labels_for("");

        assert!(labels.contains(&"style-group".to_string()));
        assert!(labels.contains(&"style-order".to_string()));
    }

    #[test]
    fn keyframes_and_animation_blocks_have_contextual_completions() {
        let labels = labels_for("keyframes FloatIn {\n  ");

        assert!(labels.contains(&"from".to_string()));
        assert!(labels.contains(&"50%".to_string()));

        let labels = labels_for("keyframes FloatIn {\n  from {\n    ");

        assert!(labels.contains(&"opacity".to_string()));
        assert!(labels.contains(&"transform".to_string()));
        assert!(!labels.contains(&"surface".to_string()));

        let labels = labels_for("card Panel {\n  animation FloatIn {\n    ");

        assert!(labels.contains(&"duration".to_string()));
        assert!(labels.contains(&"fill".to_string()));
        assert!(labels.contains(&"play-state".to_string()));
    }

    #[test]
    fn animation_values_include_custom_keyframes() {
        let items = completions_at(
            "keyframes FloatIn {\n  from {\n    opacity 0\n  }\n}\ncard Panel {\n  animation \n}\n",
            "keyframes FloatIn {\n  from {\n    opacity 0\n  }\n}\ncard Panel {\n  animation "
                .len(),
        );
        let labels = items
            .iter()
            .map(|item| item.label.clone())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"fade-in".to_string()));
        assert!(labels.contains(&"FloatIn".to_string()));
        assert_eq!(
            items
                .iter()
                .find(|item| item.label == "FloatIn")
                .map(|item| item.category),
            Some(CompletionCategory::ProjectSymbol)
        );
    }

    #[test]
    fn property_values_are_contextual() {
        assert!(labels_for("card A {\n  surface ").contains(&"gradient dusk".to_string()));
        assert!(labels_for("card A {\n  width ").contains(&"50%".to_string()));
        assert!(labels_for("card A {\n  inline-size ").contains(&"fill".to_string()));
        assert!(labels_for("card A {\n  display ").contains(&"inline-flex".to_string()));
        assert!(labels_for("card A {\n  visibility ").contains(&"hidden".to_string()));
        assert!(labels_for("card A {\n  flex ").contains(&"direction".to_string()));
        assert!(labels_for("card A {\n  flex direction ").contains(&"column".to_string()));
        assert!(labels_for("card A {\n  flex wrap ").contains(&"wrap".to_string()));
        assert!(labels_for("card A {\n  flex basis ").contains(&"50%".to_string()));
        assert!(labels_for("text A {\n  align-text ").contains(&"justify".to_string()));
        assert!(labels_for("text A {\n  case ").contains(&"capitalize".to_string()));
        assert!(labels_for("text A {\n  decoration ").contains(&"underline".to_string()));
        assert!(labels_for("text A {\n  whitespace ").contains(&"pre-wrap".to_string()));
        assert!(labels_for("text A {\n  word-break ").contains(&"break-word".to_string()));
        assert!(labels_for("text A {\n  hyphenate ").contains(&"auto".to_string()));
        assert!(labels_for("card A {\n  align ").contains(&"stretch".to_string()));
        assert!(labels_for("card A {\n  justify ").contains(&"between".to_string()));
        assert!(labels_for("card A {\n  color ").contains(&"purple".to_string()));
    }

    #[test]
    fn rows_suggests_row_values() {
        let labels = labels_for("grid Dashboard {\n  rows ");

        assert!(labels.contains(&"auto".to_string()));
        assert!(labels.contains(&"header".to_string()));
        assert!(labels.contains(&"footer".to_string()));
        assert!(!labels.contains(&"responsive".to_string()));
    }

    #[test]
    fn chained_values_are_narrowed() {
        assert_eq!(
            labels_for("grid Dashboard {\n  columns responsive "),
            vec!["cards".to_string()]
        );
        assert_eq!(
            labels_for("card A {\n  surface gradient "),
            vec![
                "dusk".to_string(),
                "midnight".to_string(),
                "aurora".to_string(),
                "ember".to_string(),
                "ocean".to_string(),
                "forest".to_string()
            ]
        );
        assert_eq!(
            labels_for("card A {\n  border width "),
            vec![
                "small".to_string(),
                "medium".to_string(),
                "large".to_string()
            ]
        );
        assert!(labels_for("card A {\n  border ").contains(&"style".to_string()));
        assert!(labels_for("card A {\n  border style ").contains(&"dashed".to_string()));
        assert!(labels_for("card A {\n  outline ").contains(&"offset".to_string()));
        assert!(labels_for("card A {\n  outline offset ").contains(&"small".to_string()));
    }

    #[test]
    fn completion_items_include_specific_documentation() {
        let item = completions_at("grid Dashboard {\n  ", "grid Dashboard {\n  ".len())
            .into_iter()
            .find(|item| item.label == "columns")
            .expect("columns completion should exist");

        assert!(item.documentation.contains("columns sidebar content"));
        assert!(item.documentation.contains("columns responsive cards"));
    }

    #[test]
    fn include_line_suggests_frame_files() {
        let labels = completions_at_with_includes(
            "#include ",
            "#include ".len(),
            vec![PathBuf::from("tokens.frame"), PathBuf::from("cards.frame")],
        )
        .into_iter()
        .map(|item| item.label)
        .collect::<Vec<_>>();

        assert_eq!(labels, vec!["cards".to_string(), "tokens".to_string()]);
    }
}
