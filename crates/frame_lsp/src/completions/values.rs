use crate::completions::suggestions::{
    dynamic_suggestions, motion_amount_suggestions, suggestions,
};
use crate::completions::types::{CompletionCategory, CompletionSuggestion};
use frame_core::tokens;

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
const BORDER_STYLE_VALUES: &[&str] = &[
    "none", "solid", "dashed", "dotted", "double", "groove", "ridge", "inset", "outset",
];
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

pub(crate) fn value_completions(
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
        "decoration" => suggestions(
            tokens::TEXT_DECORATIONS,
            "text decoration",
            "Text decoration line intent.",
        ),
        "whitespace" => suggestions(
            tokens::WHITE_SPACE,
            "white-space value",
            "White-space preservation and wrapping behavior.",
        ),
        "word-break" => suggestions(
            tokens::WORD_BREAKS,
            "word-break value",
            "Word breaking behavior for narrow text.",
        ),
        "hyphenate" => suggestions(
            tokens::HYPHENS,
            "hyphenation value",
            "Hyphenation behavior.",
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
        "border" if line_words.get(1).map(String::as_str) == Some("style") => suggestions(
            BORDER_STYLE_VALUES,
            "border style",
            "CSS border line style through structured Frame syntax.",
        ),
        "border" => {
            let mut items = suggestions(
                &[
                    "none", "soft", "strong", "accent", "muted", "danger", "success", "warning",
                    "width", "radius", "style",
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
        "outline" if line_words.get(1).map(String::as_str) == Some("offset") => suggestions(
            tokens::SPACING,
            "outline offset",
            "Spacing token used for outline offset.",
        ),
        "outline" => {
            let mut items = suggestions(
                &["none", "offset", "accent", "danger", "success", "warning"],
                "outline value",
                "Outline color or offset control.",
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
        "lift" | "sink" => motion_amount_suggestions(tokens::MOVEMENT_AMOUNTS, "movement amount"),
        "shift" if line_words.len() <= 1 => suggestions(
            &["left", "right", "up", "down"],
            "shift direction",
            "Direction for movement intent.",
        ),
        "shift" => motion_amount_suggestions(tokens::MOVEMENT_AMOUNTS, "movement amount"),
        "grow" | "shrink" => motion_amount_suggestions(tokens::VISUAL_AMOUNTS, "visual amount"),
        "tilt" if line_words.len() <= 1 => suggestions(
            &["left", "right"],
            "tilt direction",
            "Direction for rotation intent.",
        ),
        "tilt" => motion_amount_suggestions(tokens::VISUAL_AMOUNTS, "visual amount"),
        "brighten" | "dim" | "blur" | "press" | "pop" | "scale" | "fade" | "slide" => {
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
