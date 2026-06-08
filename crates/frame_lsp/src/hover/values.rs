use super::docs::*;
use frame_core::{language, symbols::SymbolIndex};

pub fn contextual_value_doc(
    word: &str,
    words: &[&str],
    symbols: Option<&SymbolIndex>,
) -> Option<String> {
    let property = words.first().copied()?;
    let values = words.iter().skip(1).copied().collect::<Vec<_>>();
    if !values
        .iter()
        .any(|value| value.trim_end_matches('{') == word)
    {
        return None;
    }

    match property {
        "columns" => Some(format!(
            "## `{word}`\n\nGrid column value.\n\nIn this `columns` statement, `{word}` becomes part of the grid template. Named values can be targeted later with `area ... {{ place {word} }}`.\n\nGenerated CSS contributes to `grid-template-columns` and, for named tracks, `grid-template-areas`."
        )),
        "rows" => Some(format!(
            "## `{word}`\n\nGrid row value.\n\nRows describe vertical layout tracks. Named rows make app shells easier to read, while `auto` and `fill` describe sizing intent."
        )),
        "place" => Some(format!(
            "## `{word}`\n\nGrid placement target.\n\nThis area claims the `{word}` slot from its parent grid.\n\nGenerated CSS:\n\n```css\ngrid-area: {word};\n```"
        )),
        "in" => Some(format!(
            "## `{word}`\n\nParent grid reference.\n\nThis area belongs to `grid {word}`. Go-to-definition can jump to that grid when it is in this file or an included Frame file."
        )),
        "padding" | "margin" | "gap" if language::SPACING.contains(&word) => Some(format!(
            "## `{word}`\n\nSpacing token.\n\nUsed by `{property}` to keep layout spacing consistent.\n\nGenerated CSS uses `var(--frame-space-{word})`."
        )),
        "display" if language::DISPLAY.contains(&word) => Some(format!(
            "## `{word}`\n\nDisplay value.\n\nGenerated CSS writes `display: {word};`."
        )),
        "visibility" if language::VISIBILITY.contains(&word) => Some(format!(
            "## `{word}`\n\nVisibility value.\n\nGenerated CSS writes `visibility: {word};`."
        )),
        "flex" => Some(format!(
            "## `{word}`\n\nFlex option or value.\n\nFrame keeps flex controls structured as `flex direction ...`, `flex wrap ...`, `flex grow ...`, `flex shrink ...`, and `flex basis ...`."
        )),
        "radius" if language::RADII.contains(&word) => Some(format!(
            "## `{word}`\n\nRadius token.\n\nGenerated CSS uses `border-radius: var(--frame-radius-{word})`."
        )),
        "shadow" if language::SHADOWS.contains(&word) => Some(format!(
            "## `{word}`\n\nShadow depth token.\n\nGenerated CSS uses `box-shadow: var(--frame-shadow-{word})`."
        )),
        "surface" if word == "gradient" => Some(SURFACE_GRADIENT_DOC.to_string()),
        "surface" | "background" if is_color_or_surface(word, symbols) => Some(format!(
            "## `{word}`\n\nVisual value for `{property}`.\n\nFrame treats this as design intent and emits the matching surface, color, or custom token variable."
        )),
        "color" | "text" | "theme" | "glow" | "ring" if is_color_or_custom(word, symbols) => {
            Some(format!(
                "## `{word}`\n\nColor intent.\n\nUsed by `{property}` to reference a built-in semantic color or custom color token.\n\nGenerated CSS uses `var(--frame-color-{word})` where applicable."
            ))
        }
        "align" if language::ALIGN.contains(&word) => Some(format!(
            "## `{word}`\n\nAlignment value.\n\nGenerated CSS maps Frame alignment intent to `align-items`."
        )),
        "justify" if language::JUSTIFY.contains(&word) => Some(format!(
            "## `{word}`\n\nJustification value.\n\nGenerated CSS maps Frame justification intent to `justify-content`."
        )),
        "case" if language::TEXT_CASES.contains(&word) => Some(format!(
            "## `{word}`\n\nText transform value.\n\nGenerated CSS maps this to `text-transform`."
        )),
        "align-text" if language::TEXT_ALIGN.contains(&word) => Some(format!(
            "## `{word}`\n\nText alignment value.\n\nGenerated CSS writes `text-align: {word};`."
        )),
        "decoration" if language::TEXT_DECORATIONS.contains(&word) => Some(format!(
            "## `{word}`\n\nText decoration value.\n\nGenerated CSS writes `text-decoration-line: {word};`."
        )),
        "whitespace" if language::WHITE_SPACE.contains(&word) => Some(format!(
            "## `{word}`\n\nWhite-space value.\n\nGenerated CSS writes `white-space: {word};`."
        )),
        "word-break" if language::WORD_BREAKS.contains(&word) => Some(format!(
            "## `{word}`\n\nWord-break value.\n\nGenerated CSS writes `word-break: {word};`."
        )),
        "hyphenate" if language::HYPHENS.contains(&word) => Some(format!(
            "## `{word}`\n\nHyphenation value.\n\nGenerated CSS writes `hyphens: {word};`."
        )),
        "below" | "above" if language::BREAKPOINTS.contains(&word) => Some(format!(
            "## `{word}`\n\nResponsive breakpoint.\n\nUsed by `{property}` to generate a media query for viewport-aware layout changes."
        )),
        "between" if language::BREAKPOINTS.contains(&word) => Some(format!(
            "## `{word}`\n\nResponsive breakpoint boundary.\n\n`between` uses two breakpoint values to generate a bounded media query."
        )),
        "container" if language::CONTAINERS.contains(&word) => Some(format!(
            "## `{word}`\n\nContainer size.\n\nUsed by `container {word}` to generate a container query for component-local responsive behavior."
        )),
        "animation" | "animate" => Some(format!(
            "## `{word}`\n\nAnimation reference.\n\nThis can be a built-in preset or a custom `keyframes` declaration. Generated CSS uses `animation: frame-{word} ...`."
        )),
        "duration" | "delay" => Some(format!(
            "## `{word}`\n\nAnimation timing value.\n\n`{property}` controls when motion happens. Named values and explicit CSS time values such as `240ms` are valid."
        )),
        "ease" if language::EASES.contains(&word) => Some(format!(
            "## `{word}`\n\nEasing value.\n\nControls the feel of transition or animation timing."
        )),
        "iteration" => Some(format!(
            "## `{word}`\n\nAnimation iteration count.\n\nUse a number for finite loops or `infinite` for continuous motion."
        )),
        "direction" if language::ANIMATION_DIRECTIONS.contains(&word) => Some(format!(
            "## `{word}`\n\nAnimation direction.\n\nControls whether keyframes play forward, reverse, or alternate on repeats."
        )),
        "fill" if language::ANIMATION_FILLS.contains(&word) => Some(format!(
            "## `{word}`\n\nAnimation fill mode.\n\nControls whether animation styles apply before or after playback."
        )),
        "play-state" if language::ANIMATION_PLAY_STATES.contains(&word) => Some(format!(
            "## `{word}`\n\nAnimation play state.\n\nControls whether this animation is currently running or paused."
        )),
        "opacity" => Some(format!(
            "## `{word}`\n\nOpacity keyframe value.\n\nGenerated CSS writes `opacity: {word};` inside the current keyframe selector."
        )),
        "transform" => Some(format!(
            "## `{word}`\n\nTransform value.\n\nGenerated CSS writes this transform function as part of `transform: ...` inside the current keyframe selector."
        )),
        _ if word.ends_with('%') => Some(format!(
            "## `{word}`\n\nPercentage value.\n\nIn keyframes this marks a point in the animation timeline. In sizing or grid tracks it represents proportional CSS sizing."
        )),
        _ => None,
    }
}

pub fn is_color_or_surface(word: &str, symbols: Option<&SymbolIndex>) -> bool {
    language::SURFACES.contains(&word)
        || language::COLORS.contains(&word)
        || symbols.is_some_and(|symbols| {
            symbols.colors.contains_key(word) || symbols.gradients.contains_key(word)
        })
}

pub fn is_color_or_custom(word: &str, symbols: Option<&SymbolIndex>) -> bool {
    language::COLORS.contains(&word)
        || symbols.is_some_and(|symbols| symbols.colors.contains_key(word))
}
