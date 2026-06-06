use frame_core::{knowledge, symbols::index_document, tokens};
use frame_parser::parse;
use std::path::PathBuf;

use crate::context::{completion_context, CompletionScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionSuggestion {
    pub label: String,
    pub detail: &'static str,
    pub documentation: String,
    pub insert_text: Option<String>,
    pub is_snippet: bool,
    pub category: CompletionCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionCategory {
    Snippet,
    Declaration,
    Include,
    LayoutProperty,
    VisualProperty,
    MotionProperty,
    TypographyProperty,
    TokenProperty,
    AdvancedProperty,
    StateBlock,
    Value,
    ProjectSymbol,
    GridReference,
    GridSection,
    KeyframeSelector,
    AnimationOption,
}

impl CompletionCategory {
    pub fn label(self) -> &'static str {
        match self {
            CompletionCategory::Snippet => "Snippet",
            CompletionCategory::Declaration => "Declaration",
            CompletionCategory::Include => "Include",
            CompletionCategory::LayoutProperty => "Layout",
            CompletionCategory::VisualProperty => "Visual",
            CompletionCategory::MotionProperty => "Motion",
            CompletionCategory::TypographyProperty => "Typography",
            CompletionCategory::TokenProperty => "Token",
            CompletionCategory::AdvancedProperty => "Advanced",
            CompletionCategory::StateBlock => "State",
            CompletionCategory::Value => "Value",
            CompletionCategory::ProjectSymbol => "Project Symbol",
            CompletionCategory::GridReference => "Grid Reference",
            CompletionCategory::GridSection => "Grid Section",
            CompletionCategory::KeyframeSelector => "Keyframe Selector",
            CompletionCategory::AnimationOption => "Animation Option",
        }
    }

    pub fn sort_prefix(self) -> &'static str {
        match self {
            CompletionCategory::Snippet => "00",
            CompletionCategory::GridReference
            | CompletionCategory::GridSection
            | CompletionCategory::ProjectSymbol => "01",
            CompletionCategory::KeyframeSelector => "02",
            CompletionCategory::Declaration => "03",
            CompletionCategory::LayoutProperty => "04",
            CompletionCategory::VisualProperty => "05",
            CompletionCategory::MotionProperty | CompletionCategory::AnimationOption => "06",
            CompletionCategory::TypographyProperty => "07",
            CompletionCategory::StateBlock => "08",
            CompletionCategory::TokenProperty => "09",
            CompletionCategory::Value => "10",
            CompletionCategory::Include => "11",
            CompletionCategory::AdvancedProperty => "12",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnippetScope {
    Root,
    Grid,
    Component,
    State,
    Keyframes,
    Animation,
}

const DECLARATIONS: &[&str] = &[
    "tokens",
    "grid",
    "area",
    "card",
    "stack",
    "row",
    "button",
    "text",
    "center",
    "split",
    "overlay",
    "dock",
    "keyframes",
];

struct FrameSnippet {
    label: &'static str,
    body: &'static str,
    documentation: &'static str,
    scopes: &'static [SnippetScope],
}

const SNIPPETS: &[FrameSnippet] = &[
    FrameSnippet {
        label: "dashboard",
        body: "grid Dashboard {\n  columns sidebar content inspector\n  gap medium\n  height screen\n}\n\narea Sidebar {\n  in Dashboard\n  place sidebar\n  surface panel\n  padding medium\n}\n\narea Content {\n  in Dashboard\n  place content\n  surface main\n  padding large\n}\n\narea Inspector {\n  in Dashboard\n  place inspector\n  surface panel\n  padding medium\n}",
        documentation: "Creates a named dashboard grid with sidebar, content, and inspector areas.\n\nSvelte:\n\n```svelte\n<div class=\"fr-Dashboard\">\n  <aside class=\"fr-Sidebar\">Channels</aside>\n  <main class=\"fr-Content\">Messages</main>\n  <section class=\"fr-Inspector\">Details</section>\n</div>\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "dashboard-percent",
        body: "grid Dashboard {\n  columns 25% 50% 25%\n  gap medium\n  height screen\n}\n\narea Sidebar {\n  in Dashboard\n  col 1\n  surface panel\n  padding medium\n}\n\narea Content {\n  in Dashboard\n  col 2\n  surface main\n  padding large\n}\n\narea Inspector {\n  in Dashboard\n  col 3\n  surface panel\n  padding medium\n}",
        documentation: "Creates a dashboard grid with explicit percentage columns.\n\nSvelte:\n\n```svelte\n<div class=\"fr-Dashboard\">\n  <aside class=\"fr-Sidebar\">Channels</aside>\n  <main class=\"fr-Content\">Messages</main>\n  <section class=\"fr-Inspector\">Details</section>\n</div>\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "hover-card",
        body: "card HoverCard {\n  surface gradient dusk\n  padding large\n  radius large\n  shadow medium\n  text bright\n\n  hover {\n    lift small\n    glow accent\n    brighten subtle\n  }\n}",
        documentation: "Creates an interactive card with a gradient surface and hover effects.\n\nSvelte:\n\n```svelte\n<a class=\"fr-HoverCard\">Docs</a>\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "toolbar",
        body: "row Toolbar {\n  align center\n  justify between\n  gap small\n  padding medium\n  surface panel\n}",
        documentation: "Creates a horizontal toolbar layout.\n\nSvelte:\n\n```svelte\n<div class=\"fr-Toolbar\">\n  <button>Back</button>\n  <button>Save</button>\n</div>\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "empty-state",
        body: "center EmptyState {\n  height screen\n  surface main\n  text muted\n}",
        documentation: "Creates a centered empty state.\n\nSvelte:\n\n```svelte\n<section class=\"fr-EmptyState\">\n  <h2>No messages yet</h2>\n  <p>Select a channel to begin.</p>\n</section>\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "keyframe-animation",
        body: "keyframes FloatIn {\n  from {\n    opacity 0\n    transform translateY(12px) scale(0.98)\n  }\n\n  to {\n    opacity 1\n    transform translateY(0) scale(1)\n  }\n}\n\ncard Panel {\n  animation FloatIn {\n    duration 240ms\n    ease smooth\n    fill both\n  }\n}",
        documentation: "Creates custom keyframes and applies them to a component with structured animation controls.\n\nCSS output includes `@keyframes frame-FloatIn` and an `animation` declaration on `.fr-Panel`.",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "responsive-breakpoint",
        body: "grid AppShell {\n  columns sidebar content inspector\n\n  below tablet {\n    columns content\n    rows sidebar content inspector\n  }\n}",
        documentation: "Creates a responsive grid override. `below tablet` emits a media query for tablet-and-smaller viewports.",
        scopes: &[SnippetScope::Root, SnippetScope::Grid],
    },
    FrameSnippet {
        label: "container-query",
        body: "grid Cards {\n  columns responsive cards\n\n  container narrow {\n    columns content\n  }\n}",
        documentation: "Creates a container-query override for a grid when its container becomes narrow.",
        scopes: &[SnippetScope::Root, SnippetScope::Grid],
    },
    FrameSnippet {
        label: "below tablet block",
        body: "below tablet {\n  columns content\n}",
        documentation: "Adds a viewport-responsive override for tablet-and-smaller layouts.\n\nCSS output: `@media (max-width: 1023px)`.",
        scopes: &[SnippetScope::Grid],
    },
    FrameSnippet {
        label: "container narrow block",
        body: "container narrow {\n  columns content\n}",
        documentation: "Adds a container query override for a narrow component container.\n\nCSS output: `@container (max-width: 42rem)`.",
        scopes: &[SnippetScope::Grid],
    },
    FrameSnippet {
        label: "hover state",
        body: "hover {\n  lift small\n  glow accent\n  brighten subtle\n}",
        documentation: "Adds common hover feedback for an interactive component.",
        scopes: &[SnippetScope::Component],
    },
    FrameSnippet {
        label: "focus state",
        body: "focus {\n  ring accent\n}",
        documentation: "Adds an accessible focus-visible ring for keyboard navigation.",
        scopes: &[SnippetScope::Component],
    },
    FrameSnippet {
        label: "animation block",
        body: "animation FloatIn {\n  duration 240ms\n  ease smooth\n  fill both\n}",
        documentation: "Applies custom keyframes with explicit timing and fill behavior.",
        scopes: &[SnippetScope::Component],
    },
    FrameSnippet {
        label: "hover effects",
        body: "lift small\nglow accent\nbrighten subtle",
        documentation: "Adds common state effects inside `hover`, `focus`, `active`, or `disabled` blocks.",
        scopes: &[SnippetScope::State],
    },
    FrameSnippet {
        label: "from/to keyframes",
        body: "from {\n  opacity 0\n  transform translateY(12px) scale(0.98)\n}\n\nto {\n  opacity 1\n  transform translateY(0) scale(1)\n}",
        documentation: "Adds starting and ending animation states inside a `keyframes` declaration.",
        scopes: &[SnippetScope::Keyframes],
    },
    FrameSnippet {
        label: "50% keyframe",
        body: "50% {\n  opacity 0.72\n}",
        documentation: "Adds an intermediate keyframe selector.",
        scopes: &[SnippetScope::Keyframes],
    },
    FrameSnippet {
        label: "full animation controls",
        body: "duration 240ms\ndelay 0ms\nease smooth\niteration 1\ndirection normal\nfill both\nplay-state running",
        documentation: "Adds every supported structured animation control.",
        scopes: &[SnippetScope::Animation],
    },
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
    "active",
    "disabled",
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

const COLUMN_VALUES: &[&str] = &[
    "responsive",
    "cards",
    "auto",
    "fill",
    "sidebar",
    "content",
    "main",
    "inspector",
    "header",
    "footer",
];

const ROW_VALUES: &[&str] = &["auto", "fill", "header", "main", "content", "footer"];
const TRACK_VALUES: &[&str] = &[
    "rail", "panel", "side", "header", "composer", "fill", "auto", "content",
];
const GRADIENT_VALUES: &[&str] = &["dusk", "midnight", "aurora", "ember", "ocean", "forest"];
const BORDER_WIDTH_VALUES: &[&str] = &["small", "medium", "large"];

const SURFACE_VALUES: &[&str] = &[
    "panel",
    "main",
    "glass",
    "raised",
    "flat",
    "overlay",
    "inset",
    "sunken",
    "gradient dusk",
    "gradient midnight",
    "gradient aurora",
    "gradient ember",
    "gradient ocean",
    "gradient forest",
];

const PERCENT_SIZE_VALUES: &[&str] = &[
    "fill", "content", "screen", "auto", "25%", "33%", "50%", "66%", "75%", "100%", "sidebar",
    "narrow", "wide", "zero", "modal", "icon",
];

const TYPOGRAPHY: &[&str] = &[
    "heading", "body", "caption", "mono", "bold", "semibold", "normal", "thin",
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
    let symbols = parse(source)
        .ok()
        .map(|document| index_document(source, &document))
        .unwrap_or_default();

    if line.trim_start().starts_with("#include") {
        return include_suggestions(include_files);
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

fn value_completions(
    keyword: &str,
    line_words: &[String],
    symbols: &frame_core::symbols::SymbolIndex,
) -> Vec<CompletionSuggestion> {
    match keyword {
        "columns" if line_words.get(1).map(String::as_str) == Some("responsive") => suggestions(
            &["cards"],
            "grid value",
            "Completes `columns responsive cards`, an auto-fitting card grid.",
        ),
        "columns" => suggestions(
            COLUMN_VALUES,
            "grid value",
            "Grid column value. Custom area names are also valid.",
        ),
        "rows" => suggestions(
            ROW_VALUES,
            "row value",
            "Named or automatic grid row value.",
        ),
        "tracks" if line_words.get(1).map(String::as_str) == Some("columns") => suggestions(
            TRACK_VALUES,
            "column track",
            "App layout track such as rail, panel, fill, or side.",
        ),
        "tracks" if line_words.get(1).map(String::as_str) == Some("rows") => suggestions(
            TRACK_VALUES,
            "row track",
            "App layout track such as header, fill, composer, or auto.",
        ),
        "tracks" => suggestions(
            &["columns", "rows"],
            "track axis",
            "Choose whether these tracks describe columns or rows.",
        ),
        "areas" => suggestions(
            COLUMN_VALUES,
            "grid area row",
            "Named area in a grid template row.",
        ),
        "layout" => suggestions(
            tokens::LAYOUTS,
            "layout preset",
            "Dense component layout preset.",
        ),
        "display" => suggestions(
            tokens::DISPLAY,
            "display value",
            "CSS display mode exposed through structured Frame syntax.",
        ),
        "visibility" => suggestions(
            tokens::VISIBILITY,
            "visibility value",
            "Visibility behavior without changing the element's declaration.",
        ),
        "flex" if line_words.get(1).map(String::as_str) == Some("direction") => suggestions(
            tokens::FLEX_DIRECTIONS,
            "flex direction",
            "Main-axis direction for a flex container.",
        ),
        "flex" if line_words.get(1).map(String::as_str) == Some("wrap") => suggestions(
            tokens::FLEX_WRAPS,
            "flex wrap",
            "Whether flex items stay on one line or wrap.",
        ),
        "flex" if line_words.get(1).map(String::as_str) == Some("basis") => suggestions(
            PERCENT_SIZE_VALUES,
            "flex basis",
            "Named or percentage flex-basis value.",
        ),
        "flex"
            if matches!(
                line_words.get(1).map(String::as_str),
                Some("grow" | "shrink")
            ) =>
        {
            suggestions(
                &["0", "1", "2"],
                "flex factor",
                "Non-negative flex grow or shrink factor.",
            )
        }
        "flex" => suggestions(
            &["direction", "wrap", "grow", "shrink", "basis"],
            "flex option",
            "Structured flexbox option.",
        ),
        "flow" => suggestions(
            tokens::GRID_FLOWS,
            "grid flow",
            "Use `vertical` to stack named columns as rows.",
        ),
        "place" => suggestions(
            COLUMN_VALUES,
            "grid area",
            "Named grid slot from `columns`.",
        ),
        "col" => suggestions(
            &["1", "2", "3", "4"],
            "column index",
            "Numeric grid column.",
        ),
        "row" => suggestions(&["1", "2", "3", "4"], "row index", "Numeric grid row."),
        "span" => suggestions(
            &["2", "3", "4"],
            "span count",
            "Number of grid tracks to span.",
        ),
        "surface" if line_words.get(1).map(String::as_str) == Some("gradient") => {
            let mut items = suggestions(GRADIENT_VALUES, "gradient", "Named Frame gradient.");
            items.extend(dynamic_suggestions(
                symbols.gradient_names(),
                "custom gradient",
                "Custom gradient token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items
        }
        "surface" => {
            let mut items = suggestions(
                SURFACE_VALUES,
                "surface value",
                "Named visual surface or gradient.",
            );
            items.extend(dynamic_suggestions(
                symbols.color_names(),
                "custom color",
                "Custom color token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items.extend(dynamic_suggestions(
                symbols.gradient_names(),
                "custom gradient",
                "Custom gradient token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items
        }
        "background" => {
            let mut items = suggestions(
                &[
                    "main", "panel", "accent", "danger", "success", "warning", "info",
                ],
                "background value",
                "Surface, color, or gradient background.",
            );
            items.extend(dynamic_suggestions(
                symbols.color_names(),
                "custom color",
                "Custom color token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items.extend(dynamic_suggestions(
                symbols.gradient_names(),
                "custom gradient",
                "Custom gradient token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items
        }
        "padding" | "margin" | "gap" | "radius" => {
            let mut items = suggestions(tokens::SPACING, "space value", "Named spacing token.");
            if matches!(keyword, "padding" | "margin") {
                items.extend(suggestions(
                    tokens::EDGES,
                    "box edge",
                    "Target one side or axis, such as `padding top medium`.",
                ));
            }
            items
        }
        "shadow" => suggestions(tokens::SHADOWS, "shadow value", "Named shadow depth."),
        "overflow" => suggestions(tokens::OVERFLOWS, "overflow value", "Overflow intent."),
        "scroll" => suggestions(tokens::SCROLL_AXES, "scroll axis", "Scrollable axis."),
        "scrollbar" => suggestions(
            tokens::SCROLLBARS,
            "scrollbar density",
            "Scrollbar density for dense app panels.",
        ),
        "box" => suggestions(tokens::BOX_SIZING, "box sizing", "Sizing model intent."),
        "square" => suggestions(
            tokens::SQUARES,
            "square size",
            "Named square size for icons, avatars, and status dots.",
        ),
        "self" => suggestions(
            tokens::SELF_ALIGN,
            "self alignment",
            "Align this item in both axes.",
        ),
        "nudge" => suggestions(
            tokens::NUDGES,
            "nudge",
            "Small positional nudge for badges.",
        ),
        "wrap" => suggestions(tokens::TEXT_WRAPS, "text wrap", "Text wrapping behavior."),
        "case" => suggestions(tokens::TEXT_CASES, "text case", "Text casing behavior."),
        "align-text" => suggestions(
            tokens::TEXT_ALIGN,
            "text alignment",
            "Text alignment intent.",
        ),
        "control" => suggestions(
            tokens::CONTROLS,
            "control affordance",
            "Control reset behavior.",
        ),
        "width" | "height" | "min-width" | "max-width" | "min-height" | "max-height"
        | "inline-size" | "block-size" | "min-inline-size" | "max-inline-size"
        | "min-block-size" | "max-block-size" => suggestions(
            PERCENT_SIZE_VALUES,
            "size value",
            "Named or percentage sizing intent.",
        ),
        "theme" | "text" | "color" | "glow" | "ring" => {
            let mut items = suggestions(tokens::COLORS, "color value", "Named color intent.");
            items.extend(dynamic_suggestions(
                symbols.color_names(),
                "custom color",
                "Custom color token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items
        }
        "align" => suggestions(tokens::ALIGN, "alignment value", "Cross-axis alignment."),
        "anchor" => suggestions(
            tokens::ANCHORS,
            "anchor value",
            "Anchor this declaration to an edge using sticky positioning.",
        ),
        "justify" => suggestions(
            tokens::JUSTIFY,
            "justification value",
            "Main-axis distribution.",
        ),
        "border" if line_words.get(1).map(String::as_str) == Some("width") => {
            suggestions(BORDER_WIDTH_VALUES, "border width", "Named border width.")
        }
        "border" => {
            let mut items = suggestions(
                &[
                    "none", "soft", "strong", "accent", "muted", "danger", "success", "warning",
                    "width", "radius",
                ],
                "border value",
                "Named border style.",
            );
            items.extend(dynamic_suggestions(
                symbols.color_names(),
                "custom color",
                "Custom color token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items
        }
        "transition" => suggestions(
            tokens::TRANSITIONS,
            "transition value",
            "Named transition intent.",
        ),
        "duration" => suggestions(
            tokens::DURATIONS,
            "duration value",
            "Named duration intent.",
        ),
        "ease" => suggestions(tokens::EASES, "ease value", "Named easing intent."),
        "animation" | "animate" => suggestions(&[], "animation value", "Named animation intent.")
            .into_iter()
            .chain(suggestions(
                tokens::ANIMATIONS,
                "animation preset",
                "Named animation preset.",
            ))
            .chain(dynamic_suggestions(
                symbols.keyframe_names(),
                "custom keyframes",
                "Custom keyframes declaration from the project graph.",
                CompletionCategory::ProjectSymbol,
            ))
            .collect(),
        "delay" => suggestions(
            &["0ms", "80ms", "120ms", "240ms", "1s"],
            "animation delay",
            "Time before an animation starts.",
        ),
        "iteration" => suggestions(
            &["1", "2", "3", "infinite"],
            "animation iteration",
            "Number of animation repeats.",
        ),
        "direction" => suggestions(
            tokens::ANIMATION_DIRECTIONS,
            "animation direction",
            "Direction used when an animation runs or repeats.",
        ),
        "fill" => suggestions(
            tokens::ANIMATION_FILLS,
            "animation fill mode",
            "How animation styles apply before and after playback.",
        ),
        "play-state" => suggestions(
            tokens::ANIMATION_PLAY_STATES,
            "animation play state",
            "Whether an animation is running or paused.",
        ),
        "type" => suggestions(
            tokens::GRADIENT_TYPES,
            "gradient type",
            "Gradient type. `layered` can combine corner gradients with linear stops.",
        ),
        "angle" => suggestions(
            &[
                "0deg", "45deg", "90deg", "135deg", "180deg", "225deg", "270deg",
            ],
            "gradient angle",
            "Linear gradient angle.",
        ),
        "stop" => {
            let mut items = suggestions(tokens::COLORS, "gradient color", "Gradient stop color.");
            items.extend(dynamic_suggestions(
                symbols.color_names(),
                "custom color",
                "Custom color token from the project graph.",
                CompletionCategory::ProjectSymbol,
            ));
            items
        }
        "corner" => suggestions(
            tokens::GRADIENT_CORNERS,
            "gradient corner",
            "Adds a radial gradient layer from a corner.",
        ),
        "gradient" => dynamic_suggestions(
            symbols.gradient_names(),
            "custom gradient",
            "Custom gradient token from the project graph.",
            CompletionCategory::ProjectSymbol,
        ),
        "font" | "size" | "weight" => suggestions(TYPOGRAPHY, "type value", "Typography token."),
        "line" => suggestions(tokens::LINES, "line height", "Line height intent."),
        "letter" => suggestions(tokens::LETTERS, "letter spacing", "Letter spacing intent."),
        "lift" | "brighten" | "dim" | "blur" | "press" | "scale" | "fade" | "slide" => {
            suggestions(tokens::SPACING, "effect value", "Effect strength token.")
        }
        "opacity" => suggestions(
            &["0", "0.25", "0.5", "0.75", "1"],
            "opacity value",
            "Opacity value used in keyframes.",
        ),
        "transform" => suggestions(
            &[
                "translateY(12px)",
                "translateY(0)",
                "scale(0.98)",
                "scale(1)",
            ],
            "transform value",
            "Transform function used in keyframes.",
        ),
        _ => Vec::new(),
    }
}

fn is_inside_block(source: &str, offset: usize, block: &str) -> bool {
    let safe_offset = offset.min(source.len());
    let mut stack = Vec::new();
    let mut line_start = 0usize;

    for (index, character) in source[..safe_offset].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }

    stack
        .last()
        .is_some_and(|header| header == block || header.starts_with(&format!("{block} ")))
}

fn is_inside_keyframe_selector(source: &str, offset: usize) -> bool {
    let safe_offset = offset.min(source.len());
    let mut stack = Vec::new();
    let mut line_start = 0usize;

    for (index, character) in source[..safe_offset].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }

    stack
        .last()
        .is_some_and(|header| matches!(header.as_str(), "from" | "to") || header.ends_with('%'))
}

fn include_suggestions(mut include_files: Vec<PathBuf>) -> Vec<CompletionSuggestion> {
    include_files.sort();
    include_files.dedup();
    include_files
        .into_iter()
        .filter_map(|path| {
            let label = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(ToOwned::to_owned)?;
            Some(CompletionSuggestion {
                label,
                detail: "Frame include",
                documentation: include_documentation(),
                insert_text: path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(ToOwned::to_owned),
                is_snippet: false,
                category: CompletionCategory::Include,
            })
        })
        .collect()
}

fn include_documentation() -> String {
    "Includes another Frame file before the current declarations.\n\nWhere: root of a `.frame` file.\n\nFrame:\n\n```frame\n#include tokens\n#include ./cards.frame\n\ncard LocalCard {\n  surface panel\n  padding medium\n}\n```\n\nSvelte:\n\n```svelte\n<script lang=\"ts\">\n  import { ui } from '$lib/frame/generated';\n  import '$lib/frame/generated.css';\n</script>\n```\n\nRelated: `tokens`, `grid`, `area`\n\nDocs: `docs/imports.md`".to_string()
}

fn suggestions(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
) -> Vec<CompletionSuggestion> {
    suggestions_with_category(labels, detail, documentation, category_for_detail(detail))
}

fn suggestions_with_category(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
    category: CompletionCategory,
) -> Vec<CompletionSuggestion> {
    labels
        .iter()
        .map(|label| CompletionSuggestion {
            label: (*label).to_string(),
            detail,
            documentation: completion_documentation(label)
                .or_else(|| knowledge::completion_doc(label))
                .unwrap_or_else(|| documentation.to_string()),
            insert_text: None,
            is_snippet: false,
            category,
        })
        .collect()
}

fn property_suggestions(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
) -> Vec<CompletionSuggestion> {
    labels
        .iter()
        .map(|label| {
            let category = property_category(label);
            CompletionSuggestion {
                label: (*label).to_string(),
                detail,
                documentation: completion_documentation(label)
                    .or_else(|| knowledge::completion_doc(label))
                    .unwrap_or_else(|| documentation.to_string()),
                insert_text: None,
                is_snippet: false,
                category,
            }
        })
        .collect()
}

fn snippet_suggestions(scope: SnippetScope) -> Vec<CompletionSuggestion> {
    SNIPPETS
        .iter()
        .filter(|snippet| snippet.scopes.contains(&scope))
        .map(|snippet| CompletionSuggestion {
            label: snippet.label.to_string(),
            detail: "Frame snippet",
            documentation: snippet.documentation.to_string(),
            insert_text: Some(snippet.body.to_string()),
            is_snippet: true,
            category: CompletionCategory::Snippet,
        })
        .collect()
}

fn dynamic_suggestions(
    mut labels: Vec<String>,
    detail: &'static str,
    documentation: &'static str,
    category: CompletionCategory,
) -> Vec<CompletionSuggestion> {
    labels.sort();
    labels
        .into_iter()
        .map(|label| CompletionSuggestion {
            label,
            detail,
            documentation: documentation.to_string(),
            insert_text: None,
            is_snippet: false,
            category,
        })
        .collect()
}

fn category_for_detail(detail: &str) -> CompletionCategory {
    match detail {
        "declaration" => CompletionCategory::Declaration,
        "include" => CompletionCategory::Include,
        "token property" | "gradient property" => CompletionCategory::TokenProperty,
        "advanced css" => CompletionCategory::AdvancedProperty,
        "animation option" => CompletionCategory::AnimationOption,
        "keyframe selector" => CompletionCategory::KeyframeSelector,
        detail
            if detail.contains("grid") || detail.contains("layout") || detail.contains("area") =>
        {
            CompletionCategory::LayoutProperty
        }
        detail
            if detail.contains("effect")
                || detail.contains("transition")
                || detail.contains("animation") =>
        {
            CompletionCategory::MotionProperty
        }
        _ => CompletionCategory::Value,
    }
}

fn property_category(label: &str) -> CompletionCategory {
    match label {
        "columns" | "rows" | "tracks" | "areas" | "flow" | "section" | "layout" | "display"
        | "gap" | "height" | "width" | "min-height" | "max-height" | "min-width" | "max-width"
        | "inline-size" | "block-size" | "min-inline-size" | "max-inline-size"
        | "min-block-size" | "max-block-size" | "place" | "in" | "col" | "row" | "span"
        | "position" | "offset" | "z" | "align" | "justify" | "self" | "anchor" | "padding"
        | "margin" | "overflow" | "scroll" | "scrollbar" | "box" | "flex" | "square" | "nudge" => {
            CompletionCategory::LayoutProperty
        }
        "surface" | "background" | "theme" | "text" | "color" | "palette" | "tone" | "opacity"
        | "radius" | "border" | "shadow" | "outline" | "visibility" | "control" | "interactive" => {
            CompletionCategory::VisualProperty
        }
        "font" | "size" | "weight" | "line" | "letter" | "truncate" | "wrap" | "case"
        | "align-text" => CompletionCategory::TypographyProperty,
        "transition" | "duration" | "ease" | "animation" | "animate" | "delay" | "iteration"
        | "direction" | "fill" | "play-state" | "transform" | "filter" => {
            CompletionCategory::MotionProperty
        }
        "hover" | "focus" | "active" | "disabled" => CompletionCategory::StateBlock,
        "advanced" | "css" => CompletionCategory::AdvancedProperty,
        "type" | "angle" | "stop" | "corner" | "at" | "shape" | "gradient" => {
            CompletionCategory::TokenProperty
        }
        _ => CompletionCategory::Value,
    }
}

fn line_words_at(source: &str, offset: usize) -> Vec<String> {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);

    source[start..safe_offset]
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect()
}

fn line_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    &source[start..safe_offset]
}

fn completion_documentation(label: &str) -> Option<String> {
    Some(match label {
        "tokens" => "Declares reusable design tokens for a Frame file.",
        "grid" => "Defines a layout container. Use `columns`, `rows`, `gap`, and child `area` declarations.",
        "area" => "Defines a child region inside a grid. Usually includes `in GridName` and `place name` or `col 1`.",
        "card" => "Defines a reusable content surface. Good for panels, links, tiles, and settings sections.",
        "stack" => "Defines a vertical flex layout. Use `gap` and `align` for spacing and cross-axis placement.",
        "row" => "As a declaration, creates a horizontal layout for NavBars and toolbars. As a property, places an area in a grid row.",
        "button" => "Defines an interactive control surface with focus, active, and disabled states.",
        "text" => "Defines reusable typography or text color intent.",
        "center" => "Centers content. Good for empty states and loading states.",
        "split" => "Defines a two-region layout. For exact horizontal ratios, use `grid` with percentage `columns`.",
        "overlay" => "Defines a fixed layer above the page. Use for modals, command palettes, and blocking dialogs.",
        "dock" => "Defines a docked command region. Current output docks to the bottom; use `row NavBar` for top navigation.",
        "keyframes" => "Defines reusable animation keyframes.\n\nExample:\n\nkeyframes FloatIn {\n  from {\n    opacity 0\n  }\n\n  to {\n    opacity 1\n  }\n}",
        "columns" => "Defines grid columns. Examples: `columns sidebar content`, `columns 25% 50% 25%`, `columns responsive cards`.",
        "rows" => "Defines grid rows. Examples: `rows header main footer` or `rows auto fill auto`.",
        "flow" => "Controls grid section direction. Use `flow vertical` to stack named `columns` as rows.",
        "section" => "Starts spacing and alignment controls for a named grid section.\n\nExample:\n\nsection title {\n  padding bottom small\n}",
        "gap" => "Sets spacing between children using `none`, `small`, `medium`, `large`, or `xlarge`.",
        "display" => "Sets display behavior with structured values like `block`, `flex`, `grid`, `contents`, or `none`.",
        "height" => "Sets height intent. Use `screen`, `fill`, `content`, `auto`, or percentages like `50%`.",
        "width" => "Sets width intent. Use `fill`, `content`, `sidebar`, `narrow`, `wide`, or percentages like `25%`.",
        "inline-size" => "Sets logical inline size. In horizontal writing modes this usually maps to width.",
        "block-size" => "Sets logical block size. In horizontal writing modes this usually maps to height.",
        "min-inline-size" => "Sets minimum logical inline size.",
        "max-inline-size" => "Sets maximum logical inline size.",
        "min-block-size" => "Sets minimum logical block size.",
        "max-block-size" => "Sets maximum logical block size.",
        "padding" => "Sets inner spacing using Frame spacing tokens.",
        "margin" => "Sets outer spacing using Frame spacing tokens.",
        "surface" => "Sets background surface intent: `panel`, `main`, `glass`, `raised`, `flat`, or `gradient ...`.",
        "align" => "Controls vertical or cross-axis placement: `start`, `center`, `end`, or `stretch`.",
        "justify" => "Controls horizontal or main-axis distribution: `start`, `center`, `end`, `between`, `around`, or `evenly`.",
        "in" => "References the parent grid for an `area`.",
        "place" => "Claims a named grid slot from the referenced grid.",
        "col" => "Places an area in a numeric grid column. Use with percentage grids.",
        "span" => "Makes an area span multiple grid tracks.",
        "radius" => "Sets corner shape using radius tokens like `large`, `pill`, or `none`.",
        "border" => "Sets border intent such as `soft`, `accent`, `danger`, `success`, or `none`.",
        "visibility" => "Sets visibility as `visible`, `hidden`, or `collapse`.",
        "flex" => "Sets flexbox controls. Use `flex direction row`, `flex wrap wrap`, `flex grow 1`, `flex shrink 0`, or `flex basis fill`.",
        "shadow" => "Sets depth using shadow tokens like `soft`, `medium`, or `deep`.",
        "color" => "Sets text color using semantic color tokens.",
        "background" => "Sets background with surface or semantic color tokens.",
        "advanced" => "Starts an explicit escape hatch block for scoped raw CSS declarations.\n\nExample:\n\nadvanced {\n  css \"backdrop-filter\" blur(12px)\n}",
        "gradient" => "Defines or selects a structured gradient token.\n\nToken example:\n\ngradient hero-gradient {\n  type linear\n  angle 135deg\n  stop brand-purple 0%\n  stop brand-panel 100%\n}",
        "below" => "Starts a responsive override for viewports below a breakpoint.\n\nExample:\n\nbelow tablet {\n  columns content\n}",
        "above" => "Starts a responsive override for viewports at or above a breakpoint.",
        "between" => "Starts a responsive override between two breakpoints.\n\nExample:\n\nbetween tablet desktop {\n  columns sidebar content\n}",
        "container" => "Starts a container query override.\n\nExample:\n\ncontainer narrow {\n  columns content\n}",
        "theme" => "Applies semantic color intent to text and border.",
        "hover" => "Starts hover effects. Only effect keywords are valid inside.",
        "focus" => "Starts keyboard focus effects, usually `ring accent`.",
        "active" => "Starts pressed-state effects, usually `press`.",
        "disabled" => "Starts unavailable-state effects, usually `dim medium`.",
        "responsive" => "Use with `columns responsive cards` for an auto-fitting card grid.",
        "cards" => "Completes `columns responsive cards`.",
        "auto" => "Automatic sizing.",
        "fill" => "Fill available space.",
        "sidebar" => "Common named grid slot or sidebar width token.",
        "content" => "Common content slot or max-content sizing token.",
        "main" => "Primary page/content surface or grid slot.",
        "inspector" => "Common right-side panel grid slot.",
        "header" => "Common top grid row or slot.",
        "footer" => "Common bottom grid row or slot.",
        "panel" => "Secondary surface for sidebars, inspectors, cards, menus, and tool panels.",
        "glass" => "Translucent elevated surface for overlays and floating panels.",
        "raised" => "Elevated solid surface for cards and controls.",
        "flat" => "Transparent or visually flat surface.",
        "gradient dusk" => "Gradient surface for highlighted cards.",
        "gradient midnight" => "Dark gradient surface for hero or feature cards.",
        "gradient aurora" => "Colorful gradient surface for high-emphasis cards.",
        "dusk" => "Gradient name used after `surface gradient`.",
        "midnight" => "Gradient name used after `surface gradient`.",
        "aurora" => "Gradient name used after `surface gradient`.",
        "small" => "Small spacing, radius, shadow, or effect strength.",
        "medium" => "Default spacing, radius, shadow, or effect strength.",
        "large" => "Large spacing, radius, shadow, or effect strength.",
        "xlarge" => "Extra large spacing or radius.",
        "none" => "Removes spacing, radius, border, or shadow depending on property.",
        "pill" => "Fully rounded pill radius.",
        "full" => "Fully rounded radius.",
        "bright" => "High-emphasis text color.",
        "muted" => "Secondary text color for captions, metadata, and helper text.",
        "accent" => "Interactive emphasis color for primary actions, links, and focus rings.",
        "primary" => "Primary semantic color.",
        "secondary" => "Secondary semantic color.",
        "danger" => "Destructive or error semantic color.",
        "success" => "Positive or completed semantic color.",
        "warning" => "Caution semantic color.",
        "info" => "Informational semantic color.",
        "lift" => "Moves a component upward for hover elevation.",
        "glow" => "Adds semantic glow, commonly `glow accent`.",
        "brighten" => "Increases brightness for interactive feedback.",
        "dim" => "Reduces emphasis, commonly for disabled state.",
        "blur" => "Applies blur effect.",
        "press" => "Adds a pressed movement for active controls.",
        "ring" => "Adds an accessible focus ring.",
        "scale" => "Slightly scales an element.",
        "fade" => "Reduces opacity.",
        "slide" => "Expresses slide movement intent.",
        "from" => "Keyframe selector for the starting state. Generates `from { ... }` inside `@keyframes`.",
        "to" => "Keyframe selector for the ending state. Generates `to { ... }` inside `@keyframes`.",
        "0%" | "25%" | "50%" | "75%" | "100%" => "Percentage keyframe selector for intermediate animation states.",
        "delay" => "Sets how long an animation waits before starting.",
        "iteration" => "Sets how many times an animation repeats. Use a number or `infinite`.",
        "direction" => "Sets animation playback direction.",
        "play-state" => "Controls whether an animation is running or paused.",
        "opacity" => "Animates opacity in keyframes. Generates `opacity: ...`.",
        "transform" => "Animates transform functions in keyframes. Generates `transform: ...`.",
        _ => return None,
    }.to_string())
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
        assert!(labels.contains(&"keyframes".to_string()));
        assert!(!labels.contains(&"panel".to_string()));
        assert!(!labels.contains(&"medium".to_string()));
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
        assert!(labels.contains(&"hover state".to_string()));
        assert!(labels.contains(&"animation block".to_string()));
        assert!(!labels.contains(&"columns".to_string()));
    }

    #[test]
    fn state_blocks_suggest_effects() {
        let labels = labels_for("card ProjectCard {\n  hover {\n    ");

        assert!(labels.contains(&"lift".to_string()));
        assert!(labels.contains(&"glow".to_string()));
        assert!(labels.contains(&"hover effects".to_string()));
        assert!(!labels.contains(&"grid".to_string()));
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
