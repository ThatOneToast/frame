use crate::style::tokens::{token_reference, TokenContract, TokenKind};
use crate::{
    language, symbols::SymbolIndex, Declaration, DeclarationKind, Diagnostic, Node, Statement,
};

use super::declarations::validate_keyframe_block;
use super::helpers::*;

pub(crate) fn validate_statements(
    declaration: &Declaration,
    symbols: &SymbolIndex,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let token_owner = matches!(
        declaration.kind,
        DeclarationKind::Tokens | DeclarationKind::Theme
    );
    for node in &declaration.body {
        match node {
            Node::Statement(statement) if token_owner => {
                super::declarations::validate_token_statement(statement, symbols, diagnostics);
            }
            Node::Statement(statement) => {
                validate_statement(statement, symbols, contract, diagnostics)
            }
            Node::Block(block) if token_owner => {
                super::declarations::validate_token_block(block, symbols, diagnostics);
            }
            Node::Block(block)
                if declaration.kind == DeclarationKind::Grid
                    && block.name.starts_with("section ") =>
            {
                super::declarations::validate_section_block(block, contract, diagnostics);
            }
            Node::Block(block) if block.name == "advanced" => {
                super::declarations::validate_advanced_block(block, diagnostics);
            }
            Node::Block(block) if block.name.starts_with("animation ") => {
                validate_animation_block(block, symbols, diagnostics);
            }
            Node::Block(block) if is_condition_block(&block.name) => {
                validate_condition_block(declaration, block, symbols, contract, diagnostics);
            }
            Node::Block(block) if declaration.kind == DeclarationKind::Keyframes => {
                validate_keyframe_block(block, diagnostics);
            }
            Node::Block(block) => {
                for node in &block.body {
                    if let Node::Statement(statement) = node {
                        validate_effect_statement(statement, symbols, diagnostics);
                    }
                }
            }
        }
    }
}

pub(crate) fn validate_condition_block(
    declaration: &Declaration,
    block: &crate::Block,
    symbols: &SymbolIndex,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    validate_condition_header(&block.name, contract, diagnostics, block.span);
    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match declaration.kind {
            DeclarationKind::Grid => validate_statement(statement, symbols, contract, diagnostics),
            DeclarationKind::Area => validate_statement(statement, symbols, contract, diagnostics),
            _ => validate_statement(statement, symbols, contract, diagnostics),
        }
    }
}

fn known_breakpoint(name: &str, contract: &TokenContract) -> bool {
    contract.get(TokenKind::Breakpoint, name).is_some() || is_css_length(name)
}

fn breakpoint_error(name: &str, contract: &TokenContract) -> String {
    let suggestion = crate::style::closest_name(name, contract.names(TokenKind::Breakpoint))
        .map(|value| format!(" Did you mean `{value}`?"))
        .unwrap_or_default();
    let known = contract
        .names(TokenKind::Breakpoint)
        .iter()
        .map(|value| format!("`{value}`"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Unknown breakpoint `{name}`.{suggestion}\n\nUse a breakpoint token ({known}), declare `breakpoint {name} <length>` in a `tokens` block, or use a raw length like `48rem`."
    )
}

pub(crate) fn validate_condition_header(
    name: &str,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
    span: crate::Span,
) {
    let words = name.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["below" | "above", breakpoint] => {
            if !known_breakpoint(breakpoint, contract) {
                diagnostics.push(Diagnostic::error(
                    breakpoint_error(breakpoint, contract),
                    span,
                ));
            }
        }
        ["between", start, end] => {
            for breakpoint in [start, end] {
                if !known_breakpoint(breakpoint, contract) {
                    diagnostics.push(Diagnostic::error(
                        breakpoint_error(breakpoint, contract),
                        span,
                    ));
                }
            }
        }
        ["container", container]
            if contract.get(TokenKind::Container, container).is_none()
                && !is_css_length(container) =>
        {
            let suggestion =
                crate::style::closest_name(container, contract.names(TokenKind::Container))
                    .map(|value| format!(" Did you mean `{value}`?"))
                    .unwrap_or_default();
            let known = contract
                .names(TokenKind::Container)
                .iter()
                .map(|value| format!("`{value}`"))
                .collect::<Vec<_>>()
                .join(", ");
            diagnostics.push(Diagnostic::error(
                    format!(
                        "Unknown container size `{container}`.{suggestion}\n\nUse a container token ({known}), declare `container {container} <length>` in a `tokens` block, or use a raw length."
                    ),
                    span,
                ));
        }
        _ => {}
    }
}

/// Accepts `0` or a number with a CSS length unit.
pub(crate) fn is_css_length(value: &str) -> bool {
    if value == "0" {
        return true;
    }
    const UNITS: &[&str] = &[
        "rem", "em", "px", "vh", "vw", "svh", "svw", "dvh", "dvw", "ch", "ex", "%",
    ];
    UNITS.iter().any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number| !number.is_empty() && number.parse::<f64>().is_ok())
    })
}

pub(crate) fn validate_animation_block(
    block: &crate::Block,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(name) = block.name.split_whitespace().nth(1) else {
        diagnostics.push(Diagnostic::error(
            "animation blocks expect an animation name, for example `animation FloatIn { ... }`",
            block.span,
        ));
        return;
    };

    if !language::ANIMATIONS.contains(&name) && !symbols.keyframes.contains_key(name) {
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown animation `{name}`.\n\nUse a preset like `fade-in`, `slide-up`, `pop-in`, or define `keyframes {name} {{ ... }}`."
            ),
            block.span,
        ));
    }

    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match statement.words.first().map(String::as_str) {
            Some("duration" | "delay") => validate_animation_time(statement, diagnostics),
            Some("ease") => validate_value(statement, language::EASES, diagnostics),
            Some("iteration") => validate_animation_iteration(statement, diagnostics),
            Some("direction") => validate_value(statement, language::ANIMATION_DIRECTIONS, diagnostics),
            Some("fill") => validate_value(statement, language::ANIMATION_FILLS, diagnostics),
            Some("play-state") => validate_value(statement, language::ANIMATION_PLAY_STATES, diagnostics),
            Some(other) => diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown animation option `{other}`.\n\nUse `duration`, `delay`, `ease`, `iteration`, `direction`, `fill`, or `play-state`."
                ),
                statement.span,
            )),
            None => {}
        }
    }
}

/// Validate every `token(kind.name)` reference in a statement against the
/// resolved contract. Returns `true` when the statement used token references
/// (which replace keyword-specific value validation).
pub(crate) fn validate_token_references(
    statement: &Statement,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    let mut found = false;
    for word in statement.words.iter().skip(1) {
        if !word.starts_with("token(") {
            continue;
        }
        found = true;
        let Some((kind, name)) = token_reference(word) else {
            let kinds = TokenKind::ALL
                .iter()
                .map(|kind| format!("`{}`", kind.keyword()))
                .collect::<Vec<_>>()
                .join(", ");
            diagnostics.push(Diagnostic::error(
                format!(
                    "Malformed token reference `{word}`.\n\nUse `token(kind.name)` where kind is one of: {kinds}."
                ),
                statement.span,
            ));
            continue;
        };
        if contract.get(kind, name).is_none() {
            let suggestion = crate::style::closest_name(name, contract.names(kind))
                .map(|value| format!(" Did you mean `token({}.{value})`?", kind.keyword()))
                .unwrap_or_default();
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown {} token `{name}`.{suggestion}\n\nDeclare `{} {name} <value>` in a `tokens` block before referencing it.",
                    kind.keyword(),
                    kind.keyword()
                ),
                statement.span,
            ));
        }
    }
    found
}

/// Whether a bare value names a user-declared token of the given kind.
pub(crate) fn contract_value(contract: &TokenContract, kind: TokenKind, value: &str) -> bool {
    contract.get(kind, value).is_some()
}

pub(crate) fn validate_statement(
    statement: &Statement,
    symbols: &SymbolIndex,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(keyword) = first_word(statement) else {
        return;
    };

    if language::property_keywords().contains(&keyword)
        && validate_token_references(statement, contract, diagnostics)
    {
        return;
    }

    if !language::property_keywords().contains(&keyword) {
        if let Some(alias) = css_property_alias(keyword) {
            let example_value = alias_example_value(alias);
            diagnostics.push(Diagnostic::error(
                format!(
                    "`{keyword}` is a raw CSS property name. In Frame, write design intent instead.\n\nUse `{alias} ...` here.\n\nExample:\n\n```frame\n{alias} {example_value}\n```\n\nWhy: Frame keeps common CSS concepts discoverable as guided properties, while raw CSS belongs in an explicit `advanced {{ css \"{keyword}\" value }}` escape hatch."
                ),
                statement.span,
            ));
            return;
        }

        let suggestion = closest(keyword, language::property_keywords())
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown property `{keyword}`.{suggestion}\n\nFrame properties describe design intent, such as `surface panel`, `padding large`, `columns responsive cards`, and `hover` effects."
            ),
            statement.span,
        ));
        return;
    }

    match first_word(statement) {
        Some("padding" | "margin") => validate_box_space(statement, contract, diagnostics),
        Some("display") => validate_value(statement, language::DISPLAY, diagnostics),
        Some("visibility") => validate_value(statement, language::VISIBILITY, diagnostics),
        Some("flex") => validate_flex(statement, diagnostics),
        Some("gap") => validate_value_with_tokens(
            statement,
            language::SPACING,
            TokenKind::Space,
            contract,
            diagnostics,
        ),
        Some("radius") => validate_value_with_tokens(
            statement,
            language::RADII,
            TokenKind::Radius,
            contract,
            diagnostics,
        ),
        Some("surface") => validate_surface(statement, symbols, contract, diagnostics),
        Some("shadow") => validate_value_with_tokens(
            statement,
            language::SHADOWS,
            TokenKind::Shadow,
            contract,
            diagnostics,
        ),
        Some("border") => validate_border(statement, symbols, diagnostics),
        Some("outline") => validate_outline(statement, symbols, diagnostics),
        Some(
            "height" | "width" | "min-height" | "max-height" | "min-width" | "max-width"
            | "inline-size" | "block-size" | "min-inline-size" | "max-inline-size"
            | "min-block-size" | "max-block-size",
        ) => validate_size_value(statement, contract, diagnostics),
        Some("align") => validate_value(statement, language::ALIGN, diagnostics),
        Some("justify") => validate_value(statement, language::JUSTIFY, diagnostics),
        Some("tracks") => validate_tracks(statement, diagnostics),
        Some("areas") => validate_areas(statement, diagnostics),
        Some("layout") => validate_value(statement, language::LAYOUTS, diagnostics),
        Some("overflow") => validate_value(statement, language::OVERFLOWS, diagnostics),
        Some("scroll") => validate_value(statement, language::SCROLL_AXES, diagnostics),
        Some("scrollbar") => validate_value(statement, language::SCROLLBARS, diagnostics),
        Some("box") => validate_value(statement, language::BOX_SIZING, diagnostics),
        Some("square") => validate_value(statement, language::SQUARES, diagnostics),
        Some("self") => validate_value(statement, language::SELF_ALIGN, diagnostics),
        Some("nudge") => validate_value(statement, language::NUDGES, diagnostics),
        Some("wrap") => validate_value(statement, language::TEXT_WRAPS, diagnostics),
        Some("case") => validate_value(statement, language::TEXT_CASES, diagnostics),
        Some("align-text") => validate_value(statement, language::TEXT_ALIGN, diagnostics),
        Some("decoration") => validate_value(statement, language::TEXT_DECORATIONS, diagnostics),
        Some("whitespace") => validate_value(statement, language::WHITE_SPACE, diagnostics),
        Some("word-break") => validate_value(statement, language::WORD_BREAKS, diagnostics),
        Some("hyphenate") => validate_value(statement, language::HYPHENS, diagnostics),
        Some("line") => validate_value(statement, language::LINES, diagnostics),
        Some("letter") => validate_value(statement, language::LETTERS, diagnostics),
        Some("control") => validate_value(statement, language::CONTROLS, diagnostics),
        Some("position") => validate_value(statement, language::POSITIONS, diagnostics),
        Some("anchor") => validate_value(statement, language::ANCHORS, diagnostics),
        Some("z") => validate_value(statement, language::Z_LAYERS, diagnostics),
        Some("color" | "text") => validate_color(statement, symbols, contract, diagnostics),
        Some("background") => validate_background(statement, symbols, contract, diagnostics),
        Some("columns") => validate_grid_columns(statement, diagnostics),
        Some("flow") => validate_value(statement, language::GRID_FLOWS, diagnostics),
        Some("transition") => validate_value(statement, language::TRANSITIONS, diagnostics),
        Some("duration") => validate_value(statement, language::DURATIONS, diagnostics),
        Some("ease") => validate_value(statement, language::EASES, diagnostics),
        Some("animation" | "animate") => {
            validate_value(statement, language::ANIMATIONS, diagnostics)
        }
        Some("lift" | "sink" | "shift" | "grow" | "shrink" | "tilt" | "press" | "pop") => {
            validate_motion_statement(statement, diagnostics)
        }
        _ => {}
    }
}

pub(crate) fn validate_tracks(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    match statement.words.get(1).map(String::as_str) {
        Some("columns" | "rows") => {}
        Some(value) => {
            diagnostics.push(Diagnostic::error(
                format!("tracks expects `columns` or `rows`, not `{value}`."),
                statement.span,
            ));
            return;
        }
        None => {
            diagnostics.push(Diagnostic::error(
                "tracks expects an axis, for example `tracks columns rail panel fill side`.",
                statement.span,
            ));
            return;
        }
    }

    if statement.words.len() <= 2 {
        diagnostics.push(Diagnostic::error(
            "tracks expects one or more track values.",
            statement.span,
        ));
        return;
    }

    for value in statement.words.iter().skip(2) {
        if !language::TRACKS.contains(&value.as_str()) && !is_valid_percentage(value) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown track value `{value}`.\n\nUse app layout tracks like `rail`, `panel`, `side`, `header`, `composer`, `fill`, `auto`, or percentages."
                ),
                statement.span,
            ));
        }
    }
}

pub(crate) fn validate_flex(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(subcommand) = statement.words.get(1).map(String::as_str) else {
        diagnostics.push(Diagnostic::error(
            "flex expects `direction`, `wrap`, `grow`, `shrink`, or `basis`.",
            statement.span,
        ));
        return;
    };

    match subcommand {
        "direction" => {
            let Some(value) = statement.words.get(2) else {
                diagnostics.push(Diagnostic::error(
                    "flex direction expects `row`, `column`, `row-reverse`, or `column-reverse`.",
                    statement.span,
                ));
                return;
            };
            if !language::FLEX_DIRECTIONS.contains(&value.as_str()) {
                diagnostics.push(Diagnostic::error(
                    format!("Unknown flex direction `{value}`.\n\nUse `row`, `column`, `row-reverse`, or `column-reverse`."),
                    statement.span,
                ));
            }
        }
        "wrap" => {
            let Some(value) = statement.words.get(2) else {
                diagnostics.push(Diagnostic::error(
                    "flex wrap expects `nowrap`, `wrap`, or `wrap-reverse`.",
                    statement.span,
                ));
                return;
            };
            if !language::FLEX_WRAPS.contains(&value.as_str()) {
                diagnostics.push(Diagnostic::error(
                    format!("Unknown flex wrap `{value}`.\n\nUse `nowrap`, `wrap`, or `wrap-reverse`."),
                    statement.span,
                ));
            }
        }
        "grow" | "shrink" => {
            let Some(value) = statement.words.get(2) else {
                diagnostics.push(Diagnostic::error(
                    format!("flex {subcommand} expects a number such as `0`, `1`, or `2`."),
                    statement.span,
                ));
                return;
            };
            if !value.chars().all(|character| character.is_ascii_digit()) {
                diagnostics.push(Diagnostic::error(
                    format!("flex {subcommand} expects a non-negative number, not `{value}`."),
                    statement.span,
                ));
            }
        }
        "basis" => {
            let Some(value) = statement.words.get(2) else {
                diagnostics.push(Diagnostic::error(
                    "flex basis expects a size value like `auto`, `fill`, `content`, `25%`, or `sidebar`.",
                    statement.span,
                ));
                return;
            };
            if !is_valid_percentage(value) && !language::SIZES.contains(&value.as_str()) {
                diagnostics.push(Diagnostic::error(
                    format!("`{value}` is not a valid flex basis value.\n\nUse size values like `auto`, `fill`, `content`, `sidebar`, or percentages."),
                    statement.span,
                ));
            }
        }
        value => diagnostics.push(Diagnostic::error(
            format!(
                "Unknown flex option `{value}`.\n\nUse `direction`, `wrap`, `grow`, `shrink`, or `basis`."
            ),
            statement.span,
        )),
    }
}

pub(crate) fn validate_areas(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    if statement.words.len() <= 1 {
        diagnostics.push(Diagnostic::error(
            "areas expects named grid sections for one template row.",
            statement.span,
        ));
    }
}

pub(crate) fn validate_effect_statement(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(effect) = first_word(statement) else {
        return;
    };

    if language::declaration_keywords().contains(&effect) {
        diagnostics.push(Diagnostic::error(
            format!(
                "`{effect}` cannot be used inside an interaction state.\n\nUse effect keywords here, such as:\n- `lift`\n- `glow`\n- `brighten`\n- `dim`"
            ),
            statement.span,
        ));
        return;
    }

    if !language::EFFECTS.contains(&effect) {
        let suggestion = closest(effect, language::EFFECTS)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!("Unknown effect `{effect}`.{suggestion}\n\nUse interaction effects like `lift`, `glow`, `brighten`, `dim`, `press`, and `ring`."),
            statement.span,
        ));
        return;
    }

    match effect {
        "lift" | "sink" | "shift" | "grow" | "shrink" | "tilt" | "press" | "pop" => {
            validate_motion_statement(statement, diagnostics)
        }
        "glow" | "ring" => validate_glow(statement, symbols, diagnostics),
        "transition" => validate_value(statement, language::TRANSITIONS, diagnostics),
        "duration" => validate_value(statement, language::DURATIONS, diagnostics),
        "ease" => validate_value(statement, language::EASES, diagnostics),
        "animation" | "animate" => validate_value(statement, language::ANIMATIONS, diagnostics),
        _ => {}
    }
}

pub(crate) fn validate_motion_statement(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(effect) = first_word(statement) else {
        return;
    };

    match effect {
        "lift" | "sink" => validate_tuned_amount_at(
            statement,
            1,
            language::MOVEMENT_AMOUNTS,
            "movement amount",
            diagnostics,
        ),
        "shift" => {
            match statement.words.get(1).map(String::as_str) {
                Some("left" | "right" | "up" | "down") => {}
                Some(direction) => {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "Unknown shift direction `{direction}`.\n\nUse `left`, `right`, `up`, or `down`."
                        ),
                        statement.span,
                    ));
                    return;
                }
                None => {
                    diagnostics.push(Diagnostic::error(
                        "shift expects a direction and movement amount, for example `shift right small`.",
                        statement.span,
                    ));
                    return;
                }
            }
            validate_tuned_amount_at(
                statement,
                2,
                language::MOVEMENT_AMOUNTS,
                "movement amount",
                diagnostics,
            );
        }
        "grow" | "shrink" => validate_tuned_amount_at(
            statement,
            1,
            language::VISUAL_AMOUNTS,
            "visual amount",
            diagnostics,
        ),
        "tilt" => {
            match statement.words.get(1).map(String::as_str) {
                Some("left" | "right") => {}
                Some(direction) => {
                    diagnostics.push(Diagnostic::error(
                        format!("Unknown tilt direction `{direction}`.\n\nUse `left` or `right`."),
                        statement.span,
                    ));
                    return;
                }
                None => {
                    diagnostics.push(Diagnostic::error(
                        "tilt expects a direction and visual amount, for example `tilt right subtle`.",
                        statement.span,
                    ));
                    return;
                }
            }
            validate_tuned_amount_at(
                statement,
                2,
                language::VISUAL_AMOUNTS,
                "visual amount",
                diagnostics,
            );
        }
        "press" | "pop" => {}
        _ => {}
    }
}

pub(crate) fn validate_tuned_amount_at(
    statement: &Statement,
    index: usize,
    scale: &[&str],
    label: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(index) else {
        diagnostics.push(Diagnostic::error(
            format!("{} expects a {label}.", statement.words[0]),
            statement.span,
        ));
        return;
    };

    let (amount, percent) = value
        .split_once('%')
        .map_or((value.as_str(), None), |(amount, percent)| {
            (amount, Some(percent))
        });

    // t-shirt aliases map onto each five-step scale by position.
    let amount = match amount {
        "xs" => scale[0],
        "sm" => scale[1],
        "md" => scale[2],
        "lg" => scale[3],
        "xl" => scale[4],
        other => other,
    };

    if !scale.contains(&amount) {
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown {label} `{value}`.\n\nUse `{}` (or t-shirt aliases `xs`..`xl`). Add `%0` through `%100` for fine tuning, such as `{}%44`.",
                scale.join("`, `"),
                scale[0]
            ),
            statement.span,
        ));
        return;
    }

    if let Some(percent) = percent {
        let valid = !percent.is_empty()
            && percent.chars().all(|character| character.is_ascii_digit())
            && percent.parse::<u8>().is_ok_and(|percent| percent <= 100);
        if !valid {
            diagnostics.push(Diagnostic::error(
                format!(
                    "`{value}` has invalid percent tuning.\n\nUse an integer from 0 to 100, for example `{amount}%44`."
                ),
                statement.span,
            ));
        }
    }
}

pub(crate) fn validate_surface(
    statement: &Statement,
    symbols: &SymbolIndex,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "surface expects a value.\n\nUse named surfaces like `panel`, `main`, `glass`, `raised`, or `surface gradient dusk`.",
            statement.span,
        ));
        return;
    };

    if symbols.gradients.contains_key(value)
        || symbols.colors.contains_key(value)
        || contract_value(contract, TokenKind::Surface, value)
        || contract_value(contract, TokenKind::Gradient, value)
        || contract_value(contract, TokenKind::Color, value)
    {
        return;
    }

    if !language::SURFACES.contains(&value.as_str()) {
        let suggestion = closest(value, language::SURFACES)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown surface `{value}`.{suggestion}\n\nUse `surface panel` for sidebars, cards, and tool regions. Use `surface main` for primary content backgrounds."
            ),
            statement.span,
        ));
    }

    if value == "gradient" {
        let Some(gradient) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                "surface gradient expects a gradient name",
                statement.span,
            ));
            return;
        };

        if !matches!(
            gradient.as_str(),
            "dusk" | "midnight" | "aurora" | "ember" | "ocean" | "forest"
        ) && !symbols.gradients.contains_key(gradient)
        {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown gradient `{gradient}`.\n\nUse named gradients like `dusk`, `midnight`, `aurora`, `ember`, `ocean`, or `forest`, or define a custom gradient token."
                ),
                statement.span,
            ));
        }
    }
}

pub(crate) fn validate_background(
    statement: &Statement,
    symbols: &SymbolIndex,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "background expects a value.\n\nUse a semantic color like `accent`, a surface like `panel`, a preset gradient, or a custom token from `tokens`.",
            statement.span,
        ));
        return;
    };

    if language::COLORS.contains(&value.as_str())
        || language::SURFACES.contains(&value.as_str())
        || symbols.colors.contains_key(value)
        || symbols.gradients.contains_key(value)
        || contract_value(contract, TokenKind::Surface, value)
        || contract_value(contract, TokenKind::Gradient, value)
        || contract_value(contract, TokenKind::Color, value)
    {
        return;
    }

    let mut candidates = language::COLORS
        .iter()
        .chain(language::SURFACES.iter())
        .copied()
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    let suggestion = closest(value, &candidates)
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown background `{value}`.{suggestion}\n\n`background` accepts built-in color intent, surface names, custom color tokens, and custom gradient tokens.\n\nUse `background panel` for a secondary region, `background accent` for emphasis, or define a token before referencing it."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_color(
    statement: &Statement,
    symbols: &SymbolIndex,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!(
                "{} expects a color value.\n\nUse semantic colors like `accent`, `muted`, `danger`, `success`, or a custom color token from `tokens`.",
                statement.words[0]
            ),
            statement.span,
        ));
        return;
    };

    if language::COLORS.contains(&value.as_str())
        || symbols.colors.contains_key(value)
        || contract_value(contract, TokenKind::Color, value)
    {
        return;
    }

    let suggestion = closest(value, language::COLORS)
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    let property = statement.words[0].as_str();
    let label = if property == "color" {
        "color".to_string()
    } else {
        format!("{property} color")
    };
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown {label} `{value}`.{suggestion}\n\nFrame color properties accept semantic color intent, not raw CSS property names. Use built-in values like `accent`, `muted`, `danger`, `success`, `warning`, or define a custom color token."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_border(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "border expects a value.\n\nUse border styles like `soft`, `strong`, `accent`, `muted`, `danger`, `none`, or custom color tokens.",
            statement.span,
        ));
        return;
    };

    if value == "width" {
        if statement
            .words
            .get(2)
            .is_some_and(|value| matches!(value.as_str(), "small" | "medium" | "large"))
        {
            return;
        }
        diagnostics.push(Diagnostic::error(
            "border width expects `small`, `medium`, or `large`",
            statement.span,
        ));
        return;
    }

    if value == "style" {
        let Some(style) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                "border style expects `solid`, `dashed`, `dotted`, `double`, `none`, or another CSS border line style.",
                statement.span,
            ));
            return;
        };
        if language::BORDER_LINE_STYLES.contains(&style.as_str()) {
            return;
        }
        let suggestion = closest(style, language::BORDER_LINE_STYLES)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!("Unknown border style `{style}`.{suggestion}\n\nUse `solid`, `dashed`, `dotted`, `double`, `groove`, `ridge`, `inset`, `outset`, or `none`."),
            statement.span,
        ));
        return;
    }

    if value == "radius" {
        let Some(radius) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                "border radius expects a radius value",
                statement.span,
            ));
            return;
        };
        if language::RADII.contains(&radius.as_str()) {
            return;
        }
        let suggestion = closest(radius, language::RADII)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown border radius `{radius}`.{suggestion}\n\nUse radius values like `small`, `medium`, `large`, `pill`, `full`, or `none`."
            ),
            statement.span,
        ));
        return;
    }

    if matches!(value.as_str(), "top" | "right" | "bottom" | "left") {
        let Some(edge_value) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                format!("border {value} expects a border color or style."),
                statement.span,
            ));
            return;
        };
        if language::BORDER_STYLES.contains(&edge_value.as_str())
            || language::COLORS.contains(&edge_value.as_str())
            || symbols.colors.contains_key(edge_value)
        {
            return;
        }
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown border {value} value `{edge_value}`.\n\nUse a semantic color, custom color token, or border style."
            ),
            statement.span,
        ));
        return;
    }

    if language::BORDER_STYLES.contains(&value.as_str())
        || language::COLORS.contains(&value.as_str())
        || symbols.colors.contains_key(value)
    {
        return;
    }

    let suggestion = closest(value, language::BORDER_STYLES)
        .or_else(|| closest(value, language::COLORS))
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown border value `{value}`.{suggestion}\n\nUse border intent like `soft`, `strong`, `accent`, `muted`, `danger`, or `none`. Use `border width medium` when changing thickness."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_outline(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1).map(String::as_str) else {
        diagnostics.push(Diagnostic::error(
            "outline expects `none`, `offset`, or a color intent such as `accent`.",
            statement.span,
        ));
        return;
    };

    if value == "none" || language::COLORS.contains(&value) || symbols.colors.contains_key(value) {
        return;
    }

    if value == "offset" {
        let Some(offset) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                "outline offset expects a spacing value like `small`, `medium`, or `large`.",
                statement.span,
            ));
            return;
        };
        if language::SPACING.contains(&offset.as_str()) {
            return;
        }
        let suggestion = closest(offset, language::SPACING)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!("Unknown outline offset `{offset}`.{suggestion}\n\nUse spacing tokens like `none`, `small`, `medium`, `large`, or `xlarge`."),
            statement.span,
        ));
        return;
    }

    let suggestion = closest(value, language::COLORS)
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown outline value `{value}`.{suggestion}\n\nUse `outline none`, `outline offset small`, a semantic color like `accent`, or a custom color token."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_glow(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        return;
    };

    if language::GLOWS.contains(&value.as_str())
        || language::COLORS.contains(&value.as_str())
        || symbols.colors.contains_key(value)
    {
        return;
    }

    let suggestion = closest(value, language::GLOWS)
        .or_else(|| closest(value, language::COLORS))
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown glow color `{value}`.{suggestion}\n\n`glow` accepts semantic colors like `accent`, `danger`, and `success`, or a custom color token."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_value(
    statement: &Statement,
    allowed: &[&str],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!(
                "{} expects a value.\n\nValid values include: {}.",
                statement.words[0],
                allowed
                    .iter()
                    .map(|value| format!("`{value}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            statement.span,
        ));
        return;
    };

    if !allowed.contains(&value.as_str()) {
        let suggestion = closest(value, allowed)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown {} value `{value}`.{suggestion}\n\nValid values include: {}.",
                statement.words[0],
                allowed
                    .iter()
                    .map(|value| format!("`{value}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            statement.span,
        ));
    }
}

/// Like [`validate_value`], but also accepts names declared in the token
/// contract for the given kind (e.g. a custom `space xs` token for `gap xs`).
pub(crate) fn validate_value_with_tokens(
    statement: &Statement,
    allowed: &[&str],
    kind: TokenKind,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(value) = statement.words.get(1) {
        if contract_value(contract, kind, value) {
            return;
        }
    }
    validate_value(statement, allowed, diagnostics);
}

pub(crate) fn validate_animation_time(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!(
                "{} expects a duration like `fast`, `240ms`, or `1s`.",
                statement.words[0]
            ),
            statement.span,
        ));
        return;
    };

    if language::DURATIONS.contains(&value.as_str()) || is_time_value(value) {
        return;
    }

    diagnostics.push(Diagnostic::error(
        format!(
            "`{value}` is not a valid animation time.\n\nUse named duration tokens like `fast`, `normal`, `slow`, or CSS time values like `240ms` and `1s`."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_animation_iteration(
    statement: &Statement,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "iteration expects a count like `1`, `3`, or `infinite`",
            statement.span,
        ));
        return;
    };

    if value == "infinite" || value.chars().all(|character| character.is_ascii_digit()) {
        return;
    }

    diagnostics.push(Diagnostic::error(
        format!(
            "`{value}` is not a valid animation iteration count.\n\nUse a number or `infinite`."
        ),
        statement.span,
    ));
}

pub(crate) fn validate_box_space(
    statement: &Statement,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!("{} expects a value", statement.words[0]),
            statement.span,
        ));
        return;
    };

    if language::SPACING.contains(&value.as_str())
        || contract_value(contract, TokenKind::Space, value)
    {
        return;
    }

    if language::EDGES.contains(&value.as_str()) {
        let Some(amount) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                format!("{} {value} expects a spacing value", statement.words[0]),
                statement.span,
            ));
            return;
        };
        if language::SPACING.contains(&amount.as_str())
            || contract_value(contract, TokenKind::Space, amount)
        {
            return;
        }
    }

    diagnostics.push(Diagnostic::error(
        format!(
            "invalid {} value `{value}`.\n\nUse spacing values like `medium`, or targeted spacing like `{} top medium`.",
            statement.words[0], statement.words[0]
        ),
        statement.span,
    ));
}

pub(crate) fn validate_size_value(
    statement: &Statement,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!("{} expects a value", statement.words[0]),
            statement.span,
        ));
        return;
    };

    if is_valid_percentage(value)
        || language::SIZES.contains(&value.as_str())
        || contract_value(contract, TokenKind::Space, value)
    {
        return;
    }

    // Allow 'none' for min- properties (maps to CSS 0)
    let property = statement.words[0].as_str();
    if property.starts_with("min-") && value == "none" {
        return;
    }

    diagnostics.push(Diagnostic::error(
        format!("`{value}` is not a valid {} value.\n\nUse size values like `fill`, `content`, `screen`, `auto`, or percentages like `25%`, `50%`, and `100%`.\n\nCompiler detail: use a percentage from `0%` to `100%`.", statement.words[0]),
        statement.span,
    ));
}

pub(crate) fn validate_grid_columns(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    if statement.words.len() <= 1 {
        diagnostics.push(Diagnostic::error(
            "`columns` expects one or more column values.\n\nUse named sections like `columns sidebar content` or percentages like `columns 25% 50% 25%`.",
            statement.span,
        ));
        return;
    }

    for value in statement.words.iter().skip(1) {
        if value.ends_with('%') && !is_valid_percentage(value) {
            diagnostics.push(Diagnostic::error(
                format!("`{value}` is not a valid percentage size.\n\nUse values like `25%`, `50%`, or `100%`.\n\nCompiler detail: invalid columns percentage `{value}`."),
                statement.span,
            ));
        }
    }
}
