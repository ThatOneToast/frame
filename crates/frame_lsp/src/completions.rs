use frame_core::{knowledge, tokens};

use crate::context::{completion_context, CompletionScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionSuggestion {
    pub label: String,
    pub detail: &'static str,
    pub documentation: String,
    pub insert_text: Option<String>,
    pub is_snippet: bool,
}

const DECLARATIONS: &[&str] = &[
    "tokens", "grid", "area", "card", "stack", "row", "button", "text", "center", "split",
    "overlay", "dock",
];

struct FrameSnippet {
    label: &'static str,
    body: &'static str,
    documentation: &'static str,
}

const SNIPPETS: &[FrameSnippet] = &[
    FrameSnippet {
        label: "dashboard",
        body: "grid Dashboard {\n  columns sidebar content inspector\n  gap medium\n  height screen\n}\n\narea Sidebar {\n  in Dashboard\n  place sidebar\n  surface panel\n  padding medium\n}\n\narea Content {\n  in Dashboard\n  place content\n  surface main\n  padding large\n}\n\narea Inspector {\n  in Dashboard\n  place inspector\n  surface panel\n  padding medium\n}",
        documentation: "Creates a named dashboard grid with sidebar, content, and inspector areas.\n\nSvelte:\n\n```svelte\n<div class=\"fr-Dashboard\">\n  <aside class=\"fr-Sidebar\">Channels</aside>\n  <main class=\"fr-Content\">Messages</main>\n  <section class=\"fr-Inspector\">Details</section>\n</div>\n```",
    },
    FrameSnippet {
        label: "dashboard-percent",
        body: "grid Dashboard {\n  columns 25% 50% 25%\n  gap medium\n  height screen\n}\n\narea Sidebar {\n  in Dashboard\n  col 1\n  surface panel\n  padding medium\n}\n\narea Content {\n  in Dashboard\n  col 2\n  surface main\n  padding large\n}\n\narea Inspector {\n  in Dashboard\n  col 3\n  surface panel\n  padding medium\n}",
        documentation: "Creates a dashboard grid with explicit percentage columns.\n\nSvelte:\n\n```svelte\n<div class=\"fr-Dashboard\">\n  <aside class=\"fr-Sidebar\">Channels</aside>\n  <main class=\"fr-Content\">Messages</main>\n  <section class=\"fr-Inspector\">Details</section>\n</div>\n```",
    },
    FrameSnippet {
        label: "hover-card",
        body: "card HoverCard {\n  surface gradient dusk\n  padding large\n  radius large\n  shadow medium\n  text bright\n\n  hover {\n    lift small\n    glow accent\n    brighten subtle\n  }\n}",
        documentation: "Creates an interactive card with a gradient surface and hover effects.\n\nSvelte:\n\n```svelte\n<a class=\"fr-HoverCard\">Docs</a>\n```",
    },
    FrameSnippet {
        label: "toolbar",
        body: "row Toolbar {\n  align center\n  justify between\n  gap small\n  padding medium\n  surface panel\n}",
        documentation: "Creates a horizontal toolbar layout.\n\nSvelte:\n\n```svelte\n<div class=\"fr-Toolbar\">\n  <button>Back</button>\n  <button>Save</button>\n</div>\n```",
    },
    FrameSnippet {
        label: "empty-state",
        body: "center EmptyState {\n  height screen\n  surface main\n  text muted\n}",
        documentation: "Creates a centered empty state.\n\nSvelte:\n\n```svelte\n<section class=\"fr-EmptyState\">\n  <h2>No messages yet</h2>\n  <p>Select a channel to begin.</p>\n</section>\n```",
    },
];

const GRID_PROPERTIES: &[&str] = &[
    "columns", "rows", "gap", "height", "width", "padding", "surface", "align", "justify",
];

const AREA_PROPERTIES: &[&str] = &[
    "in", "place", "col", "row", "span", "surface", "padding", "margin", "width", "height",
    "align", "justify", "border", "shadow",
];

const CARD_PROPERTIES: &[&str] = &[
    "surface", "padding", "margin", "radius", "border", "shadow", "text", "color", "width",
    "height", "align", "justify", "hover", "focus", "active", "disabled",
];

const COMMON_PROPERTIES: &[&str] = &[
    "surface",
    "padding",
    "margin",
    "gap",
    "width",
    "height",
    "align",
    "justify",
    "text",
    "color",
    "background",
    "border",
    "shadow",
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
const GRADIENT_VALUES: &[&str] = &["dusk", "midnight", "aurora"];
const BORDER_WIDTH_VALUES: &[&str] = &["small", "medium", "large"];

const SURFACE_VALUES: &[&str] = &[
    "panel",
    "main",
    "glass",
    "raised",
    "flat",
    "gradient dusk",
    "gradient midnight",
    "gradient aurora",
];

const PERCENT_SIZE_VALUES: &[&str] = &[
    "fill", "content", "screen", "auto", "25%", "33%", "50%", "66%", "75%", "100%", "sidebar",
    "narrow", "wide",
];

const TYPOGRAPHY: &[&str] = &[
    "heading", "body", "caption", "mono", "bold", "semibold", "normal", "thin",
];

pub fn completions_at(source: &str, offset: usize) -> Vec<CompletionSuggestion> {
    let context = completion_context(source, offset);
    let line_words = line_words_at(source, offset);

    match context.scope {
        CompletionScope::Root => {
            let mut items = suggestions(DECLARATIONS, "declaration", "Starts a Frame declaration.");
            items.extend(snippet_suggestions());
            items
        }
        CompletionScope::State { property } => property
            .as_deref()
            .map(|property| value_completions(property, &line_words))
            .filter(|items| !items.is_empty())
            .unwrap_or_else(|| {
                suggestions(
                    tokens::EFFECTS,
                    "effect",
                    "Effect used inside an interaction state.",
                )
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
                        )
                    }
                    "place" => {
                        if let Some(grid) = area_grid {
                            if let Some(columns) = context.symbols.grids.get(&grid) {
                                return dynamic_suggestions(
                                    columns.clone(),
                                    "grid area",
                                    "Named column or area from the referenced grid.",
                                );
                            }
                        }
                    }
                    _ => {}
                }

                return value_completions(&property, &line_words);
            }

            match kind.as_str() {
                "grid" => suggestions(
                    GRID_PROPERTIES,
                    "grid property",
                    "Property for grid layout and child placement.",
                ),
                "area" => suggestions(
                    AREA_PROPERTIES,
                    "area property",
                    "Property for a child region inside a grid.",
                ),
                "card" | "button" => suggestions(
                    CARD_PROPERTIES,
                    "component property",
                    "Property for a reusable UI surface.",
                ),
                _ => suggestions(
                    COMMON_PROPERTIES,
                    "property",
                    "Adds design intent to this declaration.",
                ),
            }
        }
    }
}

fn value_completions(keyword: &str, line_words: &[String]) -> Vec<CompletionSuggestion> {
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
            suggestions(GRADIENT_VALUES, "gradient", "Named Frame gradient.")
        }
        "surface" => suggestions(
            SURFACE_VALUES,
            "surface value",
            "Named visual surface or gradient.",
        ),
        "background" => suggestions(
            &[
                "main", "panel", "accent", "danger", "success", "warning", "info",
            ],
            "background value",
            "Surface or semantic color background.",
        ),
        "padding" | "margin" | "gap" | "radius" => {
            suggestions(tokens::SPACING, "space value", "Named spacing token.")
        }
        "shadow" => suggestions(tokens::SHADOWS, "shadow value", "Named shadow depth."),
        "width" | "height" | "min-width" | "max-width" | "min-height" | "max-height" => {
            suggestions(
                PERCENT_SIZE_VALUES,
                "size value",
                "Named or percentage sizing intent.",
            )
        }
        "theme" | "text" | "color" | "glow" | "ring" => {
            suggestions(tokens::COLORS, "color value", "Named color intent.")
        }
        "align" => suggestions(tokens::ALIGN, "alignment value", "Cross-axis alignment."),
        "justify" => suggestions(
            tokens::JUSTIFY,
            "justification value",
            "Main-axis distribution.",
        ),
        "border" if line_words.get(1).map(String::as_str) == Some("width") => {
            suggestions(BORDER_WIDTH_VALUES, "border width", "Named border width.")
        }
        "border" => suggestions(
            &["none", "soft", "accent", "danger", "success", "width"],
            "border value",
            "Named border style.",
        ),
        "font" | "size" | "weight" | "line" | "letter" => {
            suggestions(TYPOGRAPHY, "type value", "Typography token.")
        }
        "lift" | "brighten" | "dim" | "blur" | "press" | "scale" | "fade" | "slide" => {
            suggestions(tokens::SPACING, "effect value", "Effect strength token.")
        }
        _ => Vec::new(),
    }
}

fn suggestions(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
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
        })
        .collect()
}

fn snippet_suggestions() -> Vec<CompletionSuggestion> {
    SNIPPETS
        .iter()
        .map(|snippet| CompletionSuggestion {
            label: snippet.label.to_string(),
            detail: "Frame snippet",
            documentation: snippet.documentation.to_string(),
            insert_text: Some(snippet.body.to_string()),
            is_snippet: true,
        })
        .collect()
}

fn dynamic_suggestions(
    mut labels: Vec<String>,
    detail: &'static str,
    documentation: &'static str,
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
        })
        .collect()
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
        "columns" => "Defines grid columns. Examples: `columns sidebar content`, `columns 25% 50% 25%`, `columns responsive cards`.",
        "rows" => "Defines grid rows. Examples: `rows header main footer` or `rows auto fill auto`.",
        "gap" => "Sets spacing between children using `none`, `small`, `medium`, `large`, or `xlarge`.",
        "height" => "Sets height intent. Use `screen`, `fill`, `content`, `auto`, or percentages like `50%`.",
        "width" => "Sets width intent. Use `fill`, `content`, `sidebar`, `narrow`, `wide`, or percentages like `25%`.",
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
        "shadow" => "Sets depth using shadow tokens like `soft`, `medium`, or `deep`.",
        "color" => "Sets text color using semantic color tokens.",
        "background" => "Sets background with surface or semantic color tokens.",
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

    #[test]
    fn root_scope_only_suggests_declarations() {
        let labels = labels_for("");

        assert!(labels.contains(&"grid".to_string()));
        assert!(labels.contains(&"card".to_string()));
        assert!(!labels.contains(&"panel".to_string()));
        assert!(!labels.contains(&"medium".to_string()));
    }

    #[test]
    fn grid_block_suggests_grid_properties() {
        let labels = labels_for("grid Dashboard {\n  ");

        assert!(labels.contains(&"columns".to_string()));
        assert!(labels.contains(&"surface".to_string()));
        assert!(!labels.contains(&"card".to_string()));
    }

    #[test]
    fn grid_columns_suggests_column_values() {
        let labels = labels_for("grid Dashboard {\n  columns ");

        assert!(labels.contains(&"responsive".to_string()));
        assert!(labels.contains(&"sidebar".to_string()));
        assert!(labels.contains(&"inspector".to_string()));
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
    fn card_block_suggests_card_properties() {
        let labels = labels_for("card ProjectCard {\n  ");

        assert!(labels.contains(&"surface".to_string()));
        assert!(labels.contains(&"hover".to_string()));
        assert!(!labels.contains(&"columns".to_string()));
    }

    #[test]
    fn state_blocks_suggest_effects() {
        let labels = labels_for("card ProjectCard {\n  hover {\n    ");

        assert!(labels.contains(&"lift".to_string()));
        assert!(labels.contains(&"glow".to_string()));
        assert!(!labels.contains(&"grid".to_string()));
    }

    #[test]
    fn property_values_are_contextual() {
        assert!(labels_for("card A {\n  surface ").contains(&"gradient dusk".to_string()));
        assert!(labels_for("card A {\n  width ").contains(&"50%".to_string()));
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
                "aurora".to_string()
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
}
