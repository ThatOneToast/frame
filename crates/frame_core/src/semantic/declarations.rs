use crate::{
    language, symbols::SymbolIndex, Declaration, DeclarationKind, Diagnostic, Node, Span, Statement,
};

use super::helpers::*;
use super::statements::*;

pub(crate) fn validate_supports_declaration(
    declaration: &Declaration,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    validate_supports_predicate(&declaration.name.text, declaration.name.span, diagnostics);

    if declaration.body.is_empty() {
        diagnostics.push(Diagnostic::error(
            "supports block is empty.\n\nAdd one or more style declarations inside, such as `grid AppShell { ... }`.",
            declaration.span,
        ));
        return;
    }

    for node in &declaration.body {
        match node {
            Node::Statement(statement) => diagnostics.push(Diagnostic::error(
                format!(
                    "supports blocks contain style declarations, not loose statements.\n\nWrap `{}` inside a declaration such as `card Feature {{ ... }}`.",
                    statement.words.join(" ")
                ),
                statement.span,
            )),
            Node::Block(block) => {
                let Some(nested) = declaration_from_block(block) else {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "supports block contains invalid declaration `{}`.\n\nUse style declarations like `grid AppShell`, `card GlassPanel`, or `button PrimaryButton`.",
                            block.name
                        ),
                        block.span,
                    ));
                    continue;
                };

                if !is_style_declaration_kind(&nested.kind) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "`{}` cannot be declared inside `supports`.\n\nSupports blocks are for generated style declarations, not tokens, keyframes, or nested feature gates.",
                            block.name
                        ),
                        block.span,
                    ));
                    continue;
                }

                validate_statements(&nested, symbols, diagnostics);
                if nested.kind == DeclarationKind::Area {
                    validate_area(&nested, symbols, diagnostics);
                }
            }
        }
    }
}

pub(crate) fn validate_style_group_declaration(
    declaration: &Declaration,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !is_valid_style_identifier(&declaration.name.text) {
        diagnostics.push(Diagnostic::error(
            format!(
                "Invalid style group `{}`.\n\nUse a simple identifier such as `base`, `components`, or `utilities`.",
                declaration.name.text
            ),
            declaration.name.span,
        ));
    }

    if declaration.body.is_empty() {
        diagnostics.push(Diagnostic::error(
            "style-group block is empty.\n\nAdd one or more style declarations inside, such as `card Panel { ... }`.",
            declaration.span,
        ));
        return;
    }

    for node in &declaration.body {
        match node {
            Node::Statement(statement) => diagnostics.push(Diagnostic::error(
                format!(
                    "style-group blocks contain style declarations, not loose statements.\n\nWrap `{}` inside a declaration such as `card Feature {{ ... }}`.",
                    statement.words.join(" ")
                ),
                statement.span,
            )),
            Node::Block(block) => {
                let Some(nested) = declaration_from_block(block) else {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "style-group contains invalid declaration `{}`.\n\nUse style declarations like `text Body`, `card Panel`, or `button PrimaryButton`.",
                            block.name
                        ),
                        block.span,
                    ));
                    continue;
                };

                if !is_style_declaration_kind(&nested.kind) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "`{}` cannot be declared inside `style-group`.\n\nStyle groups are for generated style declarations, not tokens, keyframes, or nested feature gates.",
                            block.name
                        ),
                        block.span,
                    ));
                    continue;
                }

                validate_statements(&nested, symbols, diagnostics);
                if nested.kind == DeclarationKind::Area {
                    validate_area(&nested, symbols, diagnostics);
                }
            }
        }
    }
}

pub(crate) fn validate_style_order_declaration(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for name in style_order_names(&declaration.name.text) {
        if !is_valid_style_identifier(&name) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Invalid style-order group `{name}`.\n\nUse comma-separated identifiers such as `reset, base, components, utilities`."
                ),
                declaration.name.span,
            ));
        }
    }
}

pub(crate) fn validate_supports_predicate(
    predicate: &str,
    span: Span,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let words = predicate.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["display", "grid" | "flex"]
        | ["backdrop", "blur"]
        | ["color", "oklch"]
        | ["selector", "has"]
        | ["container", "queries"]
        | ["subgrid"] => {}
        ["display", value] => diagnostics.push(Diagnostic::error(
            format!("Unknown display support value `{value}`.\n\nUse `supports display grid` or `supports display flex`."),
            span,
        )),
        [category, ..] => diagnostics.push(Diagnostic::error(
            format!("Unknown support predicate `{predicate}`.\n\nUse typed predicates like `display grid`, `backdrop blur`, `color oklch`, `selector has`, `container queries`, or `subgrid`. Unknown category: `{category}`."),
            span,
        )),
        [] => diagnostics.push(Diagnostic::error(
            "supports expects a typed predicate such as `supports display grid`.",
            span,
        )),
    }
}

pub(crate) fn validate_area(
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

pub(crate) fn validate_keyframes(declaration: &Declaration, diagnostics: &mut Vec<Diagnostic>) {
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

pub(crate) fn validate_keyframe_block(block: &crate::Block, diagnostics: &mut Vec<Diagnostic>) {
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
            Some(property) if language::KEYFRAME_PROPERTIES.contains(&property) => {}
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

pub(crate) fn validate_section_block(block: &crate::Block, diagnostics: &mut Vec<Diagnostic>) {
    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match statement.words.first().map(String::as_str) {
            Some("padding" | "margin") => validate_box_space(statement, diagnostics),
            Some("align") => validate_value(statement, language::ALIGN, diagnostics),
            Some("justify") => validate_value(statement, language::JUSTIFY, diagnostics),
            Some("gap") => validate_value(statement, language::SPACING, diagnostics),
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

pub(crate) fn validate_advanced_block(block: &crate::Block, diagnostics: &mut Vec<Diagnostic>) {
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

pub(crate) fn validate_token_statement(
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
            "Color token definition is incomplete.\n\nUse the form `color name #hex` where `#hex` is a valid hex color like `#fff`, `#ffffff`, or `#ffffffff`.",
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

pub(crate) fn validate_token_block(
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
                if !language::GRADIENT_TYPES.contains(&gradient_type) {
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
                    && !language::COLORS.contains(&color.as_str())
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
                if !language::GRADIENT_CORNERS.contains(&corner.as_str()) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "Unknown gradient corner `{corner}`.\n\nUse top-left, top-right, bottom-left, or bottom-right."
                        ),
                        statement.span,
                    ));
                }
                if !is_hex_color(color)
                    && !language::COLORS.contains(&color.as_str())
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
