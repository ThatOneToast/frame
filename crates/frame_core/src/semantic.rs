use std::collections::{HashMap, HashSet};

use crate::{
    knowledge,
    symbols::{index_document, SymbolIndex},
    tokens, Declaration, DeclarationKind, Diagnostic, Document, Node, Statement,
};

pub fn validate(document: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut names = HashSet::new();
    let symbols = index_document("", document);

    for declaration in &document.declarations {
        if !names.insert(declaration.name.text.clone()) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Duplicate declaration `{}`.\n\nEach Frame declaration exports one stable class name, so names must be unique within the compiled graph.\n\nRename one declaration, or merge the rules if they describe the same UI concept.",
                    declaration.name.text
                ),
                declaration.name.span,
            ));
        }

        if let DeclarationKind::Unknown(kind) = &declaration.kind {
            let suggestion = closest(kind, knowledge::declaration_keywords())
                .map(|value| format!("\n\nDid you mean `{value}`?"))
                .unwrap_or_default();
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown declaration `{kind}`.\n\nFrame uses design declarations like `card`, `grid`, `area`, `stack`, and `row`.{suggestion}\n\nCompiler detail: unknown declaration kind `{kind}`."
                ),
                declaration.span,
            ));
        }

        validate_statements(declaration, &symbols, &mut diagnostics);

        if declaration.kind == DeclarationKind::Area {
            validate_area(declaration, &symbols, &mut diagnostics);
        }

        if declaration.kind == DeclarationKind::Keyframes {
            validate_keyframes(declaration, &mut diagnostics);
        }
    }

    diagnostics
}

#[allow(dead_code)]
fn collect_grids(document: &Document) -> HashMap<String, HashSet<String>> {
    document
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Grid)
        .map(|declaration| {
            let columns = declaration
                .body
                .iter()
                .filter_map(statement)
                .find(|statement| first_word(statement) == Some("columns"))
                .map(|statement| {
                    statement
                        .words
                        .iter()
                        .skip(1)
                        .filter(|word| word.as_str() != "responsive" && word.as_str() != "cards")
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();

            (declaration.name.text.clone(), columns)
        })
        .collect()
}

#[allow(dead_code)]
fn collect_custom_colors(document: &Document) -> HashSet<String> {
    let mut colors = HashSet::new();
    for declaration in &document.declarations {
        if declaration.kind != DeclarationKind::Tokens {
            continue;
        }
        for node in &declaration.body {
            let Some(statement) = statement(node) else {
                continue;
            };
            if first_word(statement) == Some("color") {
                if let Some(name) = statement.words.get(1) {
                    colors.insert(name.clone());
                }
            }
        }
    }
    colors
}

fn validate_area(
    declaration: &Declaration,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let grid_name = find_statement_value(declaration, "in");

    let Some(grid_name) = grid_name else {
        diagnostics.push(Diagnostic::error(
            format!(
                "area `{}` must declare `in GridName`",
                declaration.name.text
            ),
            declaration.span,
        ));
        return;
    };

    let Some(_grid_symbol) = symbols.grids.get(grid_name) else {
        let grid_names = symbols.grids.keys().map(String::as_str).collect::<Vec<_>>();
        let suggestion = closest(grid_name, &grid_names)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown grid `{grid_name}`.{suggestion}\n\n`area` blocks must reference an existing `grid` using `in`.\n\nCompiler detail: unknown grid `{grid_name}`."
            ),
            declaration.span,
        ));
        return;
    };

    if let Some(place) = find_statement_value(declaration, "place") {
        let grid_places = symbols.grid_sections.get(grid_name);
        if let Some(grid_places) = grid_places {
            if grid_places.is_empty() || grid_places.contains_key(place) {
                return;
            }
            let mut known = grid_places.keys().cloned().collect::<Vec<_>>();
            known.sort();
            let known_list = known
                .iter()
                .map(|name| format!("- `{name}`"))
                .collect::<Vec<_>>()
                .join("\n");
            diagnostics.push(Diagnostic::error(
                format!(
                    "`{place}` is not a known section in grid `{grid_name}`.\n\nKnown sections:\n{known_list}\n\nUse `place {}` or update the parent grid columns.",
                    known.first().map(String::as_str).unwrap_or("section")
                ) + &format!("\n\nCompiler detail: unknown grid slot `{place}`."),
                declaration.span,
            ));
        }
    }

    if find_statement_value(declaration, "place").is_none()
        && find_statement_value(declaration, "col").is_none()
        && find_statement_value(declaration, "row").is_none()
    {
        let mut known = symbols
            .grid_sections
            .get(grid_name)
            .map(|sections| sections.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        known.sort();
        let example = known
            .first()
            .map(|section| format!("place {section}"))
            .unwrap_or_else(|| "col 1".to_string());
        diagnostics.push(Diagnostic::error(
            format!(
                "area `{}` references grid `{grid_name}` but does not claim a position.\n\nAdd `{example}`, `col 1`, or `row 1` so the generated class has explicit grid placement.\n\nWhy: `in {grid_name}` tells Frame which grid owns the area; `place`, `col`, or `row` tells Frame where it belongs.",
                declaration.name.text
            ),
            declaration.span,
        ));
    }
}

fn validate_statements(
    declaration: &Declaration,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for node in &declaration.body {
        match node {
            Node::Statement(statement) if declaration.kind == DeclarationKind::Tokens => {
                validate_token_statement(statement, symbols, diagnostics);
            }
            Node::Statement(statement) => validate_statement(statement, symbols, diagnostics),
            Node::Block(block) if declaration.kind == DeclarationKind::Tokens => {
                validate_token_block(block, symbols, diagnostics);
            }
            Node::Block(block)
                if declaration.kind == DeclarationKind::Grid
                    && block.name.starts_with("section ") =>
            {
                validate_section_block(block, diagnostics);
            }
            Node::Block(block) if block.name == "advanced" => {
                validate_advanced_block(block, diagnostics);
            }
            Node::Block(block) if block.name.starts_with("animation ") => {
                validate_animation_block(block, symbols, diagnostics);
            }
            Node::Block(block) if is_condition_block(&block.name) => {
                validate_condition_block(declaration, block, symbols, diagnostics);
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

fn validate_condition_block(
    declaration: &Declaration,
    block: &crate::Block,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    validate_condition_header(&block.name, diagnostics, block.span);
    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match declaration.kind {
            DeclarationKind::Grid => validate_statement(statement, symbols, diagnostics),
            DeclarationKind::Area => validate_statement(statement, symbols, diagnostics),
            _ => validate_statement(statement, symbols, diagnostics),
        }
    }
}

fn validate_condition_header(name: &str, diagnostics: &mut Vec<Diagnostic>, span: crate::Span) {
    let words = name.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["below" | "above", breakpoint] if tokens::BREAKPOINTS.contains(breakpoint) => {}
        ["between", start, end]
            if tokens::BREAKPOINTS.contains(start) && tokens::BREAKPOINTS.contains(end) => {}
        ["container", container] if tokens::CONTAINERS.contains(container) => {}
        ["below" | "above", breakpoint] => diagnostics.push(Diagnostic::error(
            format!(
                "Unknown breakpoint `{breakpoint}`.\n\nUse `mobile`, `tablet`, `desktop`, or `wide`."
            ),
            span,
        )),
        ["between", start, end] => diagnostics.push(Diagnostic::error(
            format!(
                "Unknown breakpoint range `{start} {end}`.\n\nUse `between tablet desktop` or another known breakpoint pair."
            ),
            span,
        )),
        ["container", container] => diagnostics.push(Diagnostic::error(
            format!(
                "Unknown container size `{container}`.\n\nUse `narrow`, `content`, or `wide`."
            ),
            span,
        )),
        _ => {}
    }
}

fn validate_animation_block(
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

    if !tokens::ANIMATIONS.contains(&name) && !symbols.keyframes.contains_key(name) {
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
            Some("ease") => validate_value(statement, tokens::EASES, diagnostics),
            Some("iteration") => validate_animation_iteration(statement, diagnostics),
            Some("direction") => validate_value(statement, tokens::ANIMATION_DIRECTIONS, diagnostics),
            Some("fill") => validate_value(statement, tokens::ANIMATION_FILLS, diagnostics),
            Some("play-state") => validate_value(statement, tokens::ANIMATION_PLAY_STATES, diagnostics),
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

fn validate_keyframes(declaration: &Declaration, diagnostics: &mut Vec<Diagnostic>) {
    let mut selectors = 0usize;
    for node in &declaration.body {
        if let Node::Block(block) = node {
            if is_keyframe_selector(&block.name) {
                selectors += 1;
            }
        }
    }
    if selectors == 0 {
        diagnostics.push(Diagnostic::error(
            format!(
                "keyframes `{}` needs at least one selector block like `from`, `to`, or `50%`.",
                declaration.name.text
            ),
            declaration.span,
        ));
    }
}

fn validate_keyframe_block(block: &crate::Block, diagnostics: &mut Vec<Diagnostic>) {
    if !is_keyframe_selector(&block.name) {
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown keyframe selector `{}`.\n\nUse `from`, `to`, or percentages like `50%`.",
                block.name
            ),
            block.span,
        ));
        return;
    }

    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match statement.words.first().map(String::as_str) {
            Some(property) if tokens::KEYFRAME_PROPERTIES.contains(&property) => {}
            Some(other) => diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown keyframe property `{other}`.\n\nFrame keyframes currently support `opacity`, `transform`, `filter`, `scale`, `translate`, and `rotate`."
                ),
                statement.span,
            )),
            None => {}
        }
    }
}

fn validate_section_block(block: &crate::Block, diagnostics: &mut Vec<Diagnostic>) {
    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match statement.words.first().map(String::as_str) {
            Some("padding" | "margin") => validate_box_space(statement, diagnostics),
            Some("align") => validate_value(statement, tokens::ALIGN, diagnostics),
            Some("justify") => validate_value(statement, tokens::JUSTIFY, diagnostics),
            Some("gap") => validate_value(statement, tokens::SPACING, diagnostics),
            Some(
                "width"
                | "height"
                | "min-height"
                | "max-height"
                | "min-width"
                | "max-width"
                | "inline-size"
                | "block-size"
                | "min-inline-size"
                | "max-inline-size"
                | "min-block-size"
                | "max-block-size",
            ) => validate_size_value(statement, diagnostics),
            Some(other) => diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown section property `{other}`.\n\nUse spacing and alignment properties like `padding top small`, `margin bottom medium`, `align center`, or `justify between`."
                ),
                statement.span,
            )),
            None => {}
        }
    }
}

fn validate_advanced_block(block: &crate::Block, diagnostics: &mut Vec<Diagnostic>) {
    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        if statement.words.first().map(String::as_str) != Some("css") {
            diagnostics.push(Diagnostic::error(
                "advanced blocks currently support `css \"property\" value` entries",
                statement.span,
            ));
            continue;
        }
        if statement.words.len() < 3 {
            diagnostics.push(Diagnostic::error(
                "advanced css expects `css \"property\" value`",
                statement.span,
            ));
        }
    }
}

fn validate_token_statement(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(keyword) = first_word(statement) else {
        return;
    };

    if keyword != "color" {
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown token kind `{keyword}`.\n\nSupported token definitions use `color name #hex` or `gradient name {{ ... }}`."
            ),
            statement.span,
        ));
        return;
    }

    if statement.words.len() < 3 {
        diagnostics.push(Diagnostic::error(
            "color token expects `color name #hex`",
            statement.span,
        ));
        return;
    }

    let value = &statement.words[2];
    if !is_hex_color(value) {
        diagnostics.push(Diagnostic::error(
            format!(
                "`{value}` is not a valid color token value.\n\nUse hex colors like `#fff`, `#ffffff`, or `#ffffffff`.\n\nFunction colors such as `rgb(...)` are planned for a later Frame release."
            ),
            statement.span,
        ));
    }

    if let Some(name) = statement.words.get(1) {
        if symbols.gradients.contains_key(name) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Duplicate token `{name}`.\n\nA color and gradient cannot share a token name."
                ),
                statement.span,
            ));
        }
    }
}

fn validate_token_block(
    block: &crate::Block,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if block.name == "gradient" {
        diagnostics.push(Diagnostic::error(
            "gradient token blocks expect a name, for example `gradient hero-gradient { ... }`",
            block.span,
        ));
        return;
    }

    if !block.name.starts_with("gradient ") {
        diagnostics.push(Diagnostic::error(
            format!("Unknown token block `{}`.", block.name),
            block.span,
        ));
        return;
    }

    let mut stops = 0usize;

    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match statement.words.first().map(String::as_str) {
            Some("type") => {
                let gradient_type = statement.words.get(1).map(String::as_str).unwrap_or("");
                if !tokens::GRADIENT_TYPES.contains(&gradient_type) {
                    diagnostics.push(Diagnostic::error(
                        format!("Unknown gradient type `{gradient_type}`.\n\nUse `linear`, `radial`, `conic`, or `layered`."),
                        statement.span,
                    ));
                }
            }
            Some("angle") => {
                let Some(angle) = statement.words.get(1) else {
                    diagnostics.push(Diagnostic::error("gradient angle expects a value like `135deg`", statement.span));
                    continue;
                };
                if !is_valid_angle(angle) {
                    diagnostics.push(Diagnostic::error(
                        format!("`{angle}` is not a valid gradient angle.\n\nUse degree values like `135deg`."),
                        statement.span,
                    ));
                }
            }
            Some("stop") => {
                stops += 1;
                let (Some(color), Some(position)) = (statement.words.get(1), statement.words.get(2)) else {
                    diagnostics.push(Diagnostic::error(
                        "gradient stop expects `stop color 0%`",
                        statement.span,
                    ));
                    continue;
                };
                if !is_hex_color(color)
                    && !tokens::COLORS.contains(&color.as_str())
                    && !symbols.colors.contains_key(color)
                {
                    diagnostics.push(Diagnostic::error(
                        format!("Unknown gradient stop color `{color}`.\n\nGradient stops must reference a built-in color, custom color token, or valid hex color."),
                        statement.span,
                    ));
                }
                if !is_valid_percentage(position) {
                    diagnostics.push(Diagnostic::error(
                        format!("`{position}` is not a valid gradient stop percentage."),
                        statement.span,
                    ));
                }
            }
            Some("corner") => {
                let (Some(corner), Some(color)) = (statement.words.get(1), statement.words.get(2))
                else {
                    diagnostics.push(Diagnostic::error(
                        "gradient corner expects `corner top-left color`",
                        statement.span,
                    ));
                    continue;
                };
                if !tokens::GRADIENT_CORNERS.contains(&corner.as_str()) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "Unknown gradient corner `{corner}`.\n\nUse top-left, top-right, bottom-left, or bottom-right."
                        ),
                        statement.span,
                    ));
                }
                if !is_hex_color(color)
                    && !tokens::COLORS.contains(&color.as_str())
                    && !symbols.colors.contains_key(color)
                {
                    diagnostics.push(Diagnostic::error(
                        format!("Unknown gradient corner color `{color}`."),
                        statement.span,
                    ));
                }
            }
            Some(other) => diagnostics.push(Diagnostic::error(
                format!("Unknown gradient property `{other}`.\n\nUse `type linear`, `angle 135deg`, `stop color 0%`, or `corner top-left color`."),
                statement.span,
            )),
            None => {}
        }
    }

    let corners = block
        .body
        .iter()
        .filter_map(|node| match node {
            Node::Statement(statement) => statement.words.first().map(String::as_str),
            Node::Block(_) => None,
        })
        .filter(|keyword| *keyword == "corner")
        .count();

    if stops < 2 && corners == 0 {
        diagnostics.push(Diagnostic::error(
            "gradient needs at least two `stop` entries or one `corner` entry",
            block.span,
        ));
    }
}

fn validate_statement(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(keyword) = first_word(statement) else {
        return;
    };

    if !knowledge::property_keywords().contains(&keyword) {
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

        let suggestion = closest(keyword, knowledge::property_keywords())
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
        Some("padding" | "margin") => validate_box_space(statement, diagnostics),
        Some("display") => validate_value(statement, tokens::DISPLAY, diagnostics),
        Some("visibility") => validate_value(statement, tokens::VISIBILITY, diagnostics),
        Some("flex") => validate_flex(statement, diagnostics),
        Some("gap") => validate_value(statement, tokens::SPACING, diagnostics),
        Some("radius") => validate_value(statement, tokens::RADII, diagnostics),
        Some("surface") => validate_surface(statement, symbols, diagnostics),
        Some("shadow") => validate_value(statement, tokens::SHADOWS, diagnostics),
        Some("border") => validate_border(statement, symbols, diagnostics),
        Some(
            "height" | "width" | "min-height" | "max-height" | "min-width" | "max-width"
            | "inline-size" | "block-size" | "min-inline-size" | "max-inline-size"
            | "min-block-size" | "max-block-size",
        ) => validate_size_value(statement, diagnostics),
        Some("align") => validate_value(statement, tokens::ALIGN, diagnostics),
        Some("justify") => validate_value(statement, tokens::JUSTIFY, diagnostics),
        Some("tracks") => validate_tracks(statement, diagnostics),
        Some("areas") => validate_areas(statement, diagnostics),
        Some("layout") => validate_value(statement, tokens::LAYOUTS, diagnostics),
        Some("overflow") => validate_value(statement, tokens::OVERFLOWS, diagnostics),
        Some("scroll") => validate_value(statement, tokens::SCROLL_AXES, diagnostics),
        Some("scrollbar") => validate_value(statement, tokens::SCROLLBARS, diagnostics),
        Some("box") => validate_value(statement, tokens::BOX_SIZING, diagnostics),
        Some("square") => validate_value(statement, tokens::SQUARES, diagnostics),
        Some("self") => validate_value(statement, tokens::SELF_ALIGN, diagnostics),
        Some("nudge") => validate_value(statement, tokens::NUDGES, diagnostics),
        Some("wrap") => validate_value(statement, tokens::TEXT_WRAPS, diagnostics),
        Some("case") => validate_value(statement, tokens::TEXT_CASES, diagnostics),
        Some("align-text") => validate_value(statement, tokens::TEXT_ALIGN, diagnostics),
        Some("decoration") => validate_value(statement, tokens::TEXT_DECORATIONS, diagnostics),
        Some("whitespace") => validate_value(statement, tokens::WHITE_SPACE, diagnostics),
        Some("word-break") => validate_value(statement, tokens::WORD_BREAKS, diagnostics),
        Some("hyphenate") => validate_value(statement, tokens::HYPHENS, diagnostics),
        Some("line") => validate_value(statement, tokens::LINES, diagnostics),
        Some("letter") => validate_value(statement, tokens::LETTERS, diagnostics),
        Some("control") => validate_value(statement, tokens::CONTROLS, diagnostics),
        Some("position") => validate_value(statement, tokens::POSITIONS, diagnostics),
        Some("anchor") => validate_value(statement, tokens::ANCHORS, diagnostics),
        Some("z") => validate_value(statement, tokens::Z_LAYERS, diagnostics),
        Some("theme" | "color" | "text") => validate_color(statement, symbols, diagnostics),
        Some("background") => validate_background(statement, symbols, diagnostics),
        Some("columns") => validate_grid_columns(statement, diagnostics),
        Some("flow") => validate_value(statement, tokens::GRID_FLOWS, diagnostics),
        Some("transition") => validate_value(statement, tokens::TRANSITIONS, diagnostics),
        Some("duration") => validate_value(statement, tokens::DURATIONS, diagnostics),
        Some("ease") => validate_value(statement, tokens::EASES, diagnostics),
        Some("animation" | "animate") => validate_value(statement, tokens::ANIMATIONS, diagnostics),
        _ => {}
    }
}

fn validate_tracks(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
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
        if !tokens::TRACKS.contains(&value.as_str()) && !is_valid_percentage(value) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown track value `{value}`.\n\nUse app layout tracks like `rail`, `panel`, `side`, `header`, `composer`, `fill`, `auto`, or percentages."
                ),
                statement.span,
            ));
        }
    }
}

fn validate_flex(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
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
            if !tokens::FLEX_DIRECTIONS.contains(&value.as_str()) {
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
            if !tokens::FLEX_WRAPS.contains(&value.as_str()) {
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
            if !is_valid_percentage(value) && !tokens::SIZES.contains(&value.as_str()) {
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

fn validate_areas(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    if statement.words.len() <= 1 {
        diagnostics.push(Diagnostic::error(
            "areas expects named grid sections for one template row.",
            statement.span,
        ));
    }
}

fn validate_effect_statement(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(effect) = first_word(statement) else {
        return;
    };

    if knowledge::declaration_keywords().contains(&effect) {
        diagnostics.push(Diagnostic::error(
            format!(
                "`{effect}` cannot be used inside an interaction state.\n\nUse effect keywords here, such as:\n- `lift`\n- `glow`\n- `brighten`\n- `dim`"
            ),
            statement.span,
        ));
        return;
    }

    if !tokens::EFFECTS.contains(&effect) {
        let suggestion = closest(effect, tokens::EFFECTS)
            .map(|value| format!("\n\nDid you mean `{value}`?"))
            .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!("Unknown effect `{effect}`.{suggestion}\n\nUse interaction effects like `lift`, `glow`, `brighten`, `dim`, `press`, and `ring`."),
            statement.span,
        ));
        return;
    }

    match effect {
        "glow" | "ring" => validate_glow(statement, symbols, diagnostics),
        "transition" => validate_value(statement, tokens::TRANSITIONS, diagnostics),
        "duration" => validate_value(statement, tokens::DURATIONS, diagnostics),
        "ease" => validate_value(statement, tokens::EASES, diagnostics),
        "animation" | "animate" => validate_value(statement, tokens::ANIMATIONS, diagnostics),
        _ => {}
    }
}

fn validate_surface(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "surface expects a value.\n\nUse named surfaces like `panel`, `main`, `glass`, `raised`, or `surface gradient dusk`.",
            statement.span,
        ));
        return;
    };

    if symbols.gradients.contains_key(value) || symbols.colors.contains_key(value) {
        return;
    }

    if !tokens::SURFACES.contains(&value.as_str()) {
        let suggestion = closest(value, tokens::SURFACES)
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

fn validate_background(
    statement: &Statement,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "background expects a value.\n\nUse a semantic color like `accent`, a surface like `panel`, a preset gradient, or a custom token from `tokens`.",
            statement.span,
        ));
        return;
    };

    if tokens::COLORS.contains(&value.as_str())
        || tokens::SURFACES.contains(&value.as_str())
        || symbols.colors.contains_key(value)
        || symbols.gradients.contains_key(value)
    {
        return;
    }

    let mut candidates = tokens::COLORS
        .iter()
        .chain(tokens::SURFACES.iter())
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

fn validate_color(statement: &Statement, symbols: &SymbolIndex, diagnostics: &mut Vec<Diagnostic>) {
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

    if tokens::COLORS.contains(&value.as_str()) || symbols.colors.contains_key(value) {
        return;
    }

    let suggestion = closest(value, tokens::COLORS)
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

fn validate_border(
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

    if value == "radius" {
        let Some(radius) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                "border radius expects a radius value",
                statement.span,
            ));
            return;
        };
        if tokens::RADII.contains(&radius.as_str()) {
            return;
        }
        let suggestion = closest(radius, tokens::RADII)
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
        if tokens::BORDER_STYLES.contains(&edge_value.as_str())
            || tokens::COLORS.contains(&edge_value.as_str())
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

    if tokens::BORDER_STYLES.contains(&value.as_str())
        || tokens::COLORS.contains(&value.as_str())
        || symbols.colors.contains_key(value)
    {
        return;
    }

    let suggestion = closest(value, tokens::BORDER_STYLES)
        .or_else(|| closest(value, tokens::COLORS))
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown border value `{value}`.{suggestion}\n\nUse border intent like `soft`, `strong`, `accent`, `muted`, `danger`, or `none`. Use `border width medium` when changing thickness."
        ),
        statement.span,
    ));
}

fn validate_glow(statement: &Statement, symbols: &SymbolIndex, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        return;
    };

    if tokens::GLOWS.contains(&value.as_str())
        || tokens::COLORS.contains(&value.as_str())
        || symbols.colors.contains_key(value)
    {
        return;
    }

    let suggestion = closest(value, tokens::GLOWS)
        .or_else(|| closest(value, tokens::COLORS))
        .map(|value| format!("\n\nDid you mean `{value}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown glow color `{value}`.{suggestion}\n\n`glow` accepts semantic colors like `accent`, `danger`, and `success`, or a custom color token."
        ),
        statement.span,
    ));
}

fn validate_value(statement: &Statement, allowed: &[&str], diagnostics: &mut Vec<Diagnostic>) {
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

fn validate_animation_time(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
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

    if tokens::DURATIONS.contains(&value.as_str()) || is_time_value(value) {
        return;
    }

    diagnostics.push(Diagnostic::error(
        format!(
            "`{value}` is not a valid animation time.\n\nUse named duration tokens like `fast`, `normal`, `slow`, or CSS time values like `240ms` and `1s`."
        ),
        statement.span,
    ));
}

fn validate_animation_iteration(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
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

fn validate_box_space(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!("{} expects a value", statement.words[0]),
            statement.span,
        ));
        return;
    };

    if tokens::SPACING.contains(&value.as_str()) {
        return;
    }

    if tokens::EDGES.contains(&value.as_str()) {
        let Some(amount) = statement.words.get(2) else {
            diagnostics.push(Diagnostic::error(
                format!("{} {value} expects a spacing value", statement.words[0]),
                statement.span,
            ));
            return;
        };
        if tokens::SPACING.contains(&amount.as_str()) {
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

fn validate_size_value(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!("{} expects a value", statement.words[0]),
            statement.span,
        ));
        return;
    };

    if is_valid_percentage(value) || tokens::SIZES.contains(&value.as_str()) {
        return;
    }

    diagnostics.push(Diagnostic::error(
        format!("`{value}` is not a valid {} value.\n\nUse size values like `fill`, `content`, `screen`, `auto`, or percentages like `25%`, `50%`, and `100%`.\n\nCompiler detail: use a percentage from `0%` to `100%`.", statement.words[0]),
        statement.span,
    ));
}

fn validate_grid_columns(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    if statement.words.len() <= 1 {
        diagnostics.push(Diagnostic::error("columns expects values", statement.span));
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

fn is_valid_percentage(value: &str) -> bool {
    let Some(number) = value.strip_suffix('%') else {
        return false;
    };

    if number.is_empty()
        || number.starts_with('-')
        || number.contains('%')
        || !number.chars().all(|character| character.is_ascii_digit())
    {
        return false;
    }

    number.parse::<u8>().is_ok_and(|value| value <= 100)
}

fn is_time_value(value: &str) -> bool {
    value
        .strip_suffix("ms")
        .or_else(|| value.strip_suffix('s'))
        .is_some_and(|number| {
            !number.is_empty()
                && number
                    .chars()
                    .all(|character| character.is_ascii_digit() || character == '.')
        })
}

fn is_condition_block(name: &str) -> bool {
    name.starts_with("below ")
        || name.starts_with("above ")
        || name.starts_with("between ")
        || name.starts_with("container ")
}

fn is_keyframe_selector(name: &str) -> bool {
    matches!(name, "from" | "to")
        || name
            .strip_suffix('%')
            .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn is_valid_angle(value: &str) -> bool {
    value
        .strip_suffix("deg")
        .is_some_and(|number| !number.is_empty() && number.parse::<i16>().is_ok())
}

fn is_hex_color(value: &str) -> bool {
    let Some(hex) = value.strip_prefix('#') else {
        return false;
    };

    matches!(hex.len(), 3 | 6 | 8) && hex.chars().all(|character| character.is_ascii_hexdigit())
}

fn find_statement_value<'a>(declaration: &'a Declaration, keyword: &str) -> Option<&'a str> {
    declaration
        .body
        .iter()
        .filter_map(statement)
        .find(|statement| first_word(statement) == Some(keyword))
        .and_then(|statement| statement.words.get(1))
        .map(String::as_str)
}

fn statement(node: &Node) -> Option<&Statement> {
    if let Node::Statement(statement) = node {
        Some(statement)
    } else {
        None
    }
}

fn first_word(statement: &Statement) -> Option<&str> {
    statement.words.first().map(String::as_str)
}

fn css_property_alias(property: &str) -> Option<&'static str> {
    match property {
        "justify-content" | "justify-items" | "place-content" => Some("justify"),
        "align-items" | "align-content" | "place-items" => Some("align"),
        "grid-template-columns" => Some("columns"),
        "grid-template-rows" => Some("rows"),
        "grid-area" => Some("place"),
        "grid-column" => Some("col"),
        "grid-row" => Some("row"),
        "box-sizing" => Some("box"),
        "inline-size" => Some("inline-size"),
        "block-size" => Some("block-size"),
        "min-inline-size" => Some("min-inline-size"),
        "max-inline-size" => Some("max-inline-size"),
        "min-block-size" => Some("min-block-size"),
        "max-block-size" => Some("max-block-size"),
        "display" | "visibility" | "flex-direction" | "flex-wrap" | "flex-grow" | "flex-shrink"
        | "flex-basis" => Some(match property {
            "display" => "display",
            "visibility" => "visibility",
            "flex-direction" => "flex direction",
            "flex-wrap" => "flex wrap",
            "flex-grow" => "flex grow",
            "flex-shrink" => "flex shrink",
            "flex-basis" => "flex basis",
            _ => "display",
        }),
        "background-color" => Some("background"),
        "color" => Some("text"),
        "border-radius" => Some("radius"),
        "box-shadow" => Some("shadow"),
        "font-size" => Some("size"),
        "font-weight" => Some("weight"),
        "text-decoration" | "text-decoration-line" => Some("decoration"),
        "white-space" => Some("whitespace"),
        "word-break" => Some("word-break"),
        "hyphens" => Some("hyphenate"),
        "z-index" => Some("z"),
        _ => None,
    }
}

fn alias_example_value(alias: &str) -> &'static str {
    match alias {
        "columns" => "sidebar content",
        "rows" => "header content footer",
        "place" => "content",
        "col" | "row" => "1",
        "background" => "panel",
        "radius" => "large",
        "shadow" => "medium",
        "size" => "heading",
        "weight" => "semibold",
        "decoration" => "underline",
        "whitespace" => "pre-wrap",
        "word-break" => "break-word",
        "hyphenate" => "auto",
        "z" => "modal",
        "align" | "justify" => "center",
        "display" => "block",
        "visibility" => "visible",
        "box" => "border",
        "inline-size" | "block-size" => "fill",
        "min-inline-size" | "min-block-size" => "zero",
        "max-inline-size" | "max-block-size" => "fill",
        "flex direction" => "row",
        "flex wrap" => "wrap",
        "flex grow" | "flex shrink" => "1",
        "flex basis" => "fill",
        "text" => "bright",
        _ => "center",
    }
}

fn closest<'a>(needle: &str, candidates: &'a [&str]) -> Option<&'a str> {
    candidates
        .iter()
        .copied()
        .map(|candidate| (candidate, edit_distance(needle, candidate)))
        .filter(|(_, distance)| *distance <= 2)
        .min_by_key(|(_, distance)| *distance)
        .map(|(candidate, _)| candidate)
}

fn edit_distance(left: &str, right: &str) -> usize {
    let mut costs = (0..=right.len()).collect::<Vec<_>>();

    for (left_index, left_char) in left.chars().enumerate() {
        let mut previous = left_index;
        costs[0] = left_index + 1;

        for (right_index, right_char) in right.chars().enumerate() {
            let old = costs[right_index + 1];
            costs[right_index + 1] = if left_char == right_char {
                previous
            } else {
                1 + previous.min(costs[right_index]).min(old)
            };
            previous = old;
        }
    }

    *costs.last().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use crate::{Identifier, Span};

    use super::*;

    fn declaration(kind: DeclarationKind, name: &str, body: Vec<Node>) -> Declaration {
        Declaration {
            kind,
            name: Identifier::new(name, Span::default()),
            body,
            span: Span::default(),
        }
    }

    fn statement(words: &[&str]) -> Node {
        Node::Statement(Statement {
            words: words.iter().map(|word| word.to_string()).collect(),
            span: Span::default(),
        })
    }

    fn block(name: &str, body: Vec<Node>) -> Node {
        Node::Block(crate::Block {
            name: name.to_string(),
            body,
            span: Span::default(),
        })
    }

    #[test]
    fn validates_area_grid_references_and_places() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![statement(&["columns", "sidebar", "content"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Sidebar",
                    vec![
                        statement(&["in", "AppShell"]),
                        statement(&["place", "footer"]),
                    ],
                ),
            ],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("unknown grid slot `footer`"));
    }

    #[test]
    fn explains_area_missing_placement() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "Dashboard",
                    vec![statement(&["columns", "sidebar", "content"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Sidebar",
                    vec![statement(&["in", "Dashboard"])],
                ),
            ],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("does not claim a position"));
        assert!(diagnostics[0].message.contains("place"));
    }

    #[test]
    fn explains_raw_css_property_aliases() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Dock,
                "Main",
                vec![statement(&["justify-content", "center"])],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("raw CSS property name"));
        assert!(diagnostics[0].message.contains("justify center"));
        assert!(diagnostics[0].message.contains("advanced"));
    }

    #[test]
    fn accepts_percent_size_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![statement(&["width", "50%"]), statement(&["height", "100%"])],
            )],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_app_driven_native_styling_vocabulary() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Theme",
                    vec![statement(&["color", "terminal-border", "#263241"])],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![
                        statement(&["columns", "header", "sidebar", "content", "users"]),
                        statement(&["tracks", "columns", "rail", "panel", "fill", "side"]),
                        statement(&["tracks", "rows", "header", "fill", "composer"]),
                        statement(&["areas", "header", "header", "header", "header"]),
                        statement(&["overflow", "hidden"]),
                        statement(&["box", "border"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Button,
                    "ChannelButton",
                    vec![
                        statement(&["layout", "icon-content-action"]),
                        statement(&["control", "reset"]),
                        statement(&["interactive"]),
                        statement(&["align-text", "left"]),
                        statement(&["border", "bottom", "terminal-border"]),
                        statement(&["scroll", "y"]),
                        statement(&["scrollbar", "dense"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Text,
                    "MessageText",
                    vec![
                        statement(&["truncate"]),
                        statement(&["wrap", "anywhere"]),
                        statement(&["case", "uppercase"]),
                        statement(&["line", "relaxed"]),
                        statement(&["letter", "normal"]),
                        statement(&["min-width", "zero"]),
                        statement(&["square", "avatar"]),
                        statement(&["self", "center"]),
                        statement(&["nudge", "top-right"]),
                    ],
                ),
            ],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_display_flex_visibility_and_logical_sizing() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["display", "flex"]),
                    statement(&["visibility", "hidden"]),
                    statement(&["flex", "direction", "column"]),
                    statement(&["flex", "wrap", "wrap"]),
                    statement(&["flex", "grow", "1"]),
                    statement(&["flex", "shrink", "0"]),
                    statement(&["flex", "basis", "fill"]),
                    statement(&["inline-size", "fill"]),
                    statement(&["block-size", "screen"]),
                    statement(&["min-inline-size", "zero"]),
                    statement(&["max-block-size", "100%"]),
                ],
            )],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_expanded_typography_controls() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Text,
                "MessageBody",
                vec![
                    statement(&["align-text", "justify"]),
                    statement(&["case", "capitalize"]),
                    statement(&["decoration", "underline"]),
                    statement(&["whitespace", "pre-wrap"]),
                    statement(&["word-break", "break-word"]),
                    statement(&["hyphenate", "auto"]),
                ],
            )],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_display_flex_and_visibility_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["display", "table"]),
                    statement(&["visibility", "gone"]),
                    statement(&["flex", "direction", "sideways"]),
                    statement(&["flex", "grow", "-1"]),
                    statement(&["flex", "basis", "huge"]),
                ],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 5);
        assert!(diagnostics[0].message.contains("Unknown display value"));
        assert!(diagnostics[1].message.contains("Unknown visibility value"));
        assert!(diagnostics[2].message.contains("Unknown flex direction"));
        assert!(diagnostics[3].message.contains("non-negative number"));
        assert!(diagnostics[4].message.contains("valid flex basis value"));
    }

    #[test]
    fn rejects_invalid_expanded_typography_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Text,
                "MessageBody",
                vec![
                    statement(&["align-text", "middle"]),
                    statement(&["case", "title"]),
                    statement(&["decoration", "blink"]),
                    statement(&["whitespace", "squish"]),
                    statement(&["word-break", "shatter"]),
                    statement(&["hyphenate", "always"]),
                ],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 6);
        assert!(diagnostics[0].message.contains("Unknown align-text value"));
        assert!(diagnostics[1].message.contains("Unknown case value"));
        assert!(diagnostics[2].message.contains("Unknown decoration value"));
        assert!(diagnostics[3].message.contains("Unknown whitespace value"));
        assert!(diagnostics[4].message.contains("Unknown word-break value"));
        assert!(diagnostics[5].message.contains("Unknown hyphenate value"));
    }

    #[test]
    fn rejects_invalid_percent_size_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![
                        statement(&["width", "-10%"]),
                        statement(&["height", "120%%"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "Dashboard",
                    vec![statement(&["columns", "25%", "abc%", "75%"])],
                ),
            ],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 3);
        assert!(diagnostics[0].message.contains("0%` to `100%"));
        assert!(diagnostics[2]
            .message
            .contains("invalid columns percentage"));
    }

    #[test]
    fn accepts_custom_color_tokens_and_usage() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand", "#7c3aed"]),
                        statement(&["color", "panel-bg", "#181820"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "BrandCard",
                    vec![
                        statement(&["background", "brand"]),
                        statement(&["color", "white"]),
                        statement(&["border", "panel-bg"]),
                    ],
                ),
            ],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_custom_gradient_tokens_and_usage() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand-purple", "#7c3aed"]),
                        Node::Block(crate::Block {
                            name: "gradient hero-gradient".to_string(),
                            body: vec![
                                statement(&["type", "linear"]),
                                statement(&["angle", "135deg"]),
                                statement(&["stop", "brand-purple", "0%"]),
                                statement(&["stop", "#181820", "100%"]),
                            ],
                            span: Span::default(),
                        }),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Hero",
                    vec![statement(&["background", "hero-gradient"])],
                ),
            ],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_corner_gradients_targeted_padding_and_anchor() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand-purple", "#7c3aed"]),
                        Node::Block(crate::Block {
                            name: "gradient four-corners".to_string(),
                            body: vec![
                                statement(&["type", "layered"]),
                                statement(&["corner", "top-left", "brand-purple", "65%"]),
                                statement(&["corner", "bottom-right", "#181820", "70%"]),
                            ],
                            span: Span::default(),
                        }),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "PinnedHero",
                    vec![
                        statement(&["background", "four-corners"]),
                        statement(&["padding", "top", "large"]),
                        statement(&["padding", "x", "medium"]),
                        statement(&["anchor", "top"]),
                    ],
                ),
            ],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_vertical_grid_flow_and_section_spacing() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "HoverCardInfo",
                vec![
                    statement(&["flow", "vertical"]),
                    statement(&["columns", "title", "description"]),
                    block(
                        "section title",
                        vec![statement(&["padding", "bottom", "small"])],
                    ),
                    block(
                        "section description",
                        vec![statement(&["margin", "top", "none"])],
                    ),
                ],
            )],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_grid_flow_and_section_properties() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "HoverCardInfo",
                vec![
                    statement(&["flow", "diagonal"]),
                    block("section title", vec![statement(&["surface", "panel"])]),
                ],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("Unknown flow value"));
        assert!(diagnostics[0].message.contains("Valid values include"));
        assert!(diagnostics[1]
            .message
            .contains("Unknown section property `surface`"));
    }

    #[test]
    fn explains_unknown_color_and_background_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["background", "accnt"]),
                    statement(&["color", "primry"]),
                ],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("Unknown background"));
        assert!(diagnostics[0].message.contains("Did you mean"));
        assert!(diagnostics[0].message.contains("custom color tokens"));
        assert!(diagnostics[1].message.contains("Unknown color"));
        assert!(diagnostics[1].message.contains("semantic color intent"));
    }

    #[test]
    fn explains_duplicate_declarations() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(DeclarationKind::Card, "Panel", Vec::new()),
                declaration(DeclarationKind::Card, "Panel", Vec::new()),
            ],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("Each Frame declaration exports one stable class name"));
    }

    #[test]
    fn rejects_invalid_corner_gradient_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![Node::Block(crate::Block {
                    name: "gradient bad".to_string(),
                    body: vec![statement(&["corner", "middle", "missing-color"])],
                    span: Span::default(),
                })],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("Unknown gradient corner"));
        assert!(diagnostics[1]
            .message
            .contains("Unknown gradient corner color"));
    }

    #[test]
    fn rejects_invalid_gradient_stop_color() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![Node::Block(crate::Block {
                    name: "gradient hero-gradient".to_string(),
                    body: vec![
                        statement(&["angle", "135deg"]),
                        statement(&["stop", "missing-color", "0%"]),
                        statement(&["stop", "#181820", "100%"]),
                    ],
                    span: Span::default(),
                })],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("Unknown gradient stop color"));
    }

    #[test]
    fn accepts_keyframes_animation_blocks_and_responsive_conditions() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Keyframes,
                    "FloatIn",
                    vec![
                        block(
                            "from",
                            vec![
                                statement(&["opacity", "0"]),
                                statement(&["transform", "translateY(12px)"]),
                            ],
                        ),
                        block(
                            "to",
                            vec![
                                statement(&["opacity", "1"]),
                                statement(&["transform", "translateY(0)"]),
                            ],
                        ),
                    ],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![
                        statement(&["columns", "sidebar", "content"]),
                        block("below tablet", vec![statement(&["columns", "content"])]),
                        block("container narrow", vec![statement(&["columns", "content"])]),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![block(
                        "animation FloatIn",
                        vec![
                            statement(&["duration", "240ms"]),
                            statement(&["delay", "0ms"]),
                            statement(&["ease", "smooth"]),
                            statement(&["iteration", "1"]),
                            statement(&["direction", "normal"]),
                            statement(&["fill", "both"]),
                            statement(&["play-state", "running"]),
                        ],
                    )],
                ),
            ],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_keyframes_animation_and_responsive_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Keyframes,
                    "FloatIn",
                    vec![block("middle", vec![statement(&["left", "0"])])],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![block(
                        "below phablet",
                        vec![statement(&["columns", "content"])],
                    )],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![block(
                        "animation MissingMotion",
                        vec![statement(&["fill", "sideways"])],
                    )],
                ),
            ],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown keyframe selector")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown breakpoint")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown animation")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown fill value")));
    }

    #[test]
    fn knowledge_catalog_documents_new_motion_and_responsive_concepts() {
        let keyframes = knowledge::concept("keyframes").expect("keyframes concept");
        let below = knowledge::concept("below").expect("below concept");
        let container = knowledge::concept("container").expect("container concept");

        assert!(keyframes.markdown().contains("@keyframes frame-Name"));
        assert!(below.markdown().contains("@media"));
        assert!(container.markdown().contains("@container"));
    }

    #[test]
    fn rejects_invalid_hex_color_tokens() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![statement(&["color", "brand", "#12"])],
            )],
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("valid color token"));
    }
}
