use crate::style::tokens::TokenContract;
use crate::{
    language, symbols::SymbolIndex, Declaration, DeclarationKind, Diagnostic, Node, Span, Statement,
};

use super::helpers::*;
use super::statements::*;

pub(crate) fn validate_supports_declaration(
    declaration: &Declaration,
    symbols: &SymbolIndex,
    contract: &TokenContract,
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

                validate_statements(&nested, symbols, contract, diagnostics);
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
    contract: &TokenContract,
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

                validate_statements(&nested, symbols, contract, diagnostics);
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

pub(crate) fn validate_section_block(
    block: &crate::Block,
    contract: &TokenContract,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for node in &block.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        match statement.words.first().map(String::as_str) {
            Some("padding" | "margin") => validate_box_space(statement, contract, diagnostics),
            Some("align") => validate_value(statement, language::ALIGN, diagnostics),
            Some("justify") => validate_value(statement, language::JUSTIFY, diagnostics),
            Some("gap") => validate_value_with_tokens(
                statement,
                language::SPACING,
                crate::style::tokens::TokenKind::Space,
                contract,
                diagnostics,
            ),
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
            ) => validate_size_value(statement, contract, diagnostics),
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
    use crate::style::tokens::TokenKind;

    let Some(keyword) = first_word(statement) else {
        return;
    };

    let Some(kind) = TokenKind::from_keyword(keyword) else {
        let kinds = TokenKind::ALL
            .iter()
            .map(|kind| format!("`{}`", kind.keyword()))
            .collect::<Vec<_>>()
            .join(", ");
        let suggestion =
            crate::style::closest_name(keyword, TokenKind::ALL.iter().map(|kind| kind.keyword()))
                .map(|value| format!(" Did you mean `{value}`?"))
                .unwrap_or_default();
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown token kind `{keyword}`.{suggestion}\n\nTokens are typed contracts. Supported kinds: {kinds}. Gradients use `gradient name {{ ... }}` blocks."
            ),
            statement.span,
        ));
        return;
    };

    if kind == TokenKind::Gradient {
        diagnostics.push(Diagnostic::error(
            "Gradient tokens are defined as blocks.\n\nUse `gradient name { angle 135deg stop #22162f 0% stop #123047 100% }`.",
            statement.span,
        ));
        return;
    }

    if statement.words.len() < 3 {
        diagnostics.push(Diagnostic::error(
            format!(
                "{keyword} token definition is incomplete.\n\nUse the form `{keyword} name value`, for example `{}`.",
                token_kind_example(kind)
            ),
            statement.span,
        ));
        return;
    }

    let value = statement.words[2..].join(" ");
    match kind {
        TokenKind::Color | TokenKind::Surface => {
            if !is_hex_color(&value) && !is_css_color_function(&value) && value != "transparent" {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "`{value}` is not a valid {keyword} token value.\n\nUse hex colors like `#fff` and `#ffffff`, `transparent`, or color functions like `rgba(...)` and `oklch(...)`."
                    ),
                    statement.span,
                ));
            }
        }
        TokenKind::Space | TokenKind::Radius | TokenKind::Breakpoint | TokenKind::Container => {
            if !super::statements::is_css_length(&value) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "`{value}` is not a valid {keyword} token value.\n\nUse a CSS length like `0.5rem`, `16px`, or `0`."
                    ),
                    statement.span,
                ));
            }
        }
        // Shadows and glows accept raw box-shadow values or an alias to an
        // existing token of the same kind (`shadow panel soft`).
        TokenKind::Shadow | TokenKind::Glow | TokenKind::Gradient => {}
    }

    if kind == TokenKind::Color {
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
}

fn token_kind_example(kind: crate::style::tokens::TokenKind) -> &'static str {
    use crate::style::tokens::TokenKind;
    match kind {
        TokenKind::Color => "color accent #8ab4ff",
        TokenKind::Surface => "surface panel #171722",
        TokenKind::Gradient => "gradient dusk { ... }",
        TokenKind::Space => "space md 1rem",
        TokenKind::Radius => "radius lg 1rem",
        TokenKind::Shadow => "shadow panel soft",
        TokenKind::Glow => "glow accent soft",
        TokenKind::Breakpoint => "breakpoint tablet 48rem",
        TokenKind::Container => "container content 64rem",
    }
}

fn is_css_color_function(value: &str) -> bool {
    value.ends_with(')')
        && [
            "rgb(", "rgba(", "hsl(", "hsla(", "oklch(", "oklab(", "color(",
        ]
        .iter()
        .any(|prefix| value.starts_with(prefix))
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

pub(crate) fn validate_grid_conflicts(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut has_columns = false;
    let mut has_tracks = false;
    let mut column_names: Vec<String> = Vec::new();

    for node in &declaration.body {
        let Node::Statement(statement) = node else {
            continue;
        };
        let Some(first) = statement.words.first().map(String::as_str) else {
            continue;
        };
        match first {
            "columns" => {
                has_columns = true;
                column_names = statement.words.iter().skip(1).cloned().collect();
            }
            "tracks" => {
                has_tracks = true;
            }
            _ => {}
        }
    }

    if has_columns && has_tracks {
        diagnostics.push(Diagnostic::error(
            "grid uses both `columns` and `tracks` which both set `grid-template-columns`.\n\nRemove one. Use `columns` for named sections, or `tracks` for explicit grid template definitions.",
            declaration.span,
        ));
    }

    let mut seen = std::collections::HashMap::new();
    for (i, name) in column_names.iter().enumerate() {
        if matches!(name.as_str(), "responsive" | "cards" | "auto" | "fill") || name.ends_with('%')
        {
            continue;
        }
        if let Some(_prev) = seen.insert(name.clone(), i) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "duplicate column name `{name}` in grid `{}`.\n\nEach column must have a unique name so child areas can claim distinct slots.",
                    declaration.name.text
                ),
                declaration.span,
            ));
        }
    }
}
