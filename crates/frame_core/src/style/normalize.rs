//! Statement → style-fact normalization.
//!
//! This is the only place that interprets style statement words. Everything
//! downstream (semantic CSS backend, atomic CSS backend, TypeScript metadata)
//! consumes the normalized facts this module produces.

use crate::{DeclarationKind, Node, Statement};

use super::properties::*;
use super::schema::{
    ConditionScope, CssDecl, GridSection, NormalizedStyle, StateScope, StyleFact, FILTER_PART,
    TRANSFORM_PART,
};
use super::StyleContext;

pub(crate) fn statements(body: &[Node]) -> impl Iterator<Item = &Statement> {
    body.iter().filter_map(|node| match node {
        Node::Statement(statement) => Some(statement),
        Node::Block(_) => None,
    })
}

fn word(statement: &Statement, index: usize) -> Option<&str> {
    statement.words.get(index).map(String::as_str)
}

fn fact(path: &str, property: &str, value: impl Into<String>) -> StyleFact {
    StyleFact::single(path, property, value)
}

/// Normalize a full styled declaration, including kind defaults, body
/// statements, state blocks, and responsive/container condition blocks.
pub fn normalize_declaration(
    kind: &DeclarationKind,
    body: &[Node],
    ctx: &StyleContext,
) -> NormalizedStyle {
    let mut style = NormalizedStyle::default();

    match kind {
        DeclarationKind::Grid => {
            style.facts.extend(normalize_grid_body(body));
            style.facts.extend(normalize_statements(body, ctx));
            style.section_names = grid_section_names(body);
            for node in body {
                let Node::Block(block) = node else {
                    continue;
                };
                if let Some(name) = block.name.strip_prefix("section ") {
                    style.sections.push(GridSection {
                        name: name.trim().to_string(),
                        facts: normalize_statements(&block.body, ctx),
                    });
                }
            }
        }
        DeclarationKind::Area => {
            style.facts.extend(normalize_statements(body, ctx));
            style.facts.extend(normalize_area_placement(body));
        }
        DeclarationKind::Html | DeclarationKind::Body => {
            style.facts.extend(normalize_page_root(body));
        }
        DeclarationKind::Row => {
            for statement in statements(body) {
                if statement.words.first().map(String::as_str) == Some("columns") {
                    style.facts.extend(normalize_columns(statement, false));
                }
            }
            style.facts.extend(normalize_statements(body, ctx));
        }
        _ => {
            style.facts.extend(normalize_statements(body, ctx));
        }
    }

    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if state_selector(&block.name).is_some() {
            style.states.push(StateScope {
                state: block.name.clone(),
                facts: normalize_effect_body(&block.body, ctx),
            });
        } else if condition_rule(&block.name, ctx.contract).is_some() {
            style.conditions.push(ConditionScope {
                condition: block.name.clone(),
                facts: normalize_condition_body(kind, &block.body, ctx),
            });
        }
    }

    style
}

/// Prepend the display defaults a declaration kind contributes.
///
/// Called after inheritance resolution so that, for example, a `row` that
/// inherits grid columns from its base stays a grid.
pub fn apply_kind_defaults(style: &mut NormalizedStyle, kind: &DeclarationKind) {
    let has_grid_columns = style
        .facts
        .iter()
        .any(|fact| fact.path == "layout.grid.columns");
    let defaults = kind_default_facts(kind, has_grid_columns);
    if defaults.is_empty() {
        return;
    }
    let facts = std::mem::take(&mut style.facts);
    style.facts = defaults.into_iter().chain(facts).collect();
}

/// The facts a declaration kind contributes before its body is considered.
fn kind_default_facts(kind: &DeclarationKind, has_grid_columns: bool) -> Vec<StyleFact> {
    match kind {
        DeclarationKind::Grid => vec![fact("layout.display", "display", "grid")],
        DeclarationKind::Card | DeclarationKind::Stack => vec![StyleFact::new(
            "layout.display",
            vec![
                CssDecl::new("display", "flex"),
                CssDecl::new("flex-direction", "column"),
            ],
        )],
        DeclarationKind::Row => {
            if has_grid_columns {
                vec![fact("layout.display", "display", "grid")]
            } else {
                vec![StyleFact::new(
                    "layout.display",
                    vec![
                        CssDecl::new("display", "flex"),
                        CssDecl::new("flex-direction", "row"),
                    ],
                )]
            }
        }
        DeclarationKind::Center => vec![StyleFact::new(
            "layout.display",
            vec![
                CssDecl::new("display", "grid"),
                CssDecl::new("place-items", "center"),
            ],
        )],
        DeclarationKind::Split => vec![StyleFact::new(
            "layout.display",
            vec![
                CssDecl::new("display", "grid"),
                CssDecl::new("grid-template-columns", "minmax(0, auto) minmax(0, 1fr)"),
            ],
        )],
        DeclarationKind::Overlay => vec![StyleFact::new(
            "layout.display",
            vec![
                CssDecl::new("position", "fixed"),
                CssDecl::new("inset", "0"),
                CssDecl::new("display", "grid"),
            ],
        )],
        DeclarationKind::Dock => vec![StyleFact::new(
            "layout.display",
            vec![
                CssDecl::new("position", "fixed"),
                CssDecl::new("inset-inline", "0"),
                CssDecl::new("bottom", "0"),
            ],
        )],
        _ => Vec::new(),
    }
}

pub fn has_columns_statement(body: &[Node]) -> bool {
    statements(body).any(|s| s.words.first().map(String::as_str) == Some("columns"))
}

/// The named grid sections introduced by a `columns` statement, in order.
pub fn grid_section_names(body: &[Node]) -> Vec<String> {
    statements(body)
        .find(|statement| statement.words.first().map(String::as_str) == Some("columns"))
        .map(|statement| {
            statement
                .words
                .iter()
                .skip(1)
                .filter(|name| {
                    !matches!(name.as_str(), "responsive" | "cards" | "auto" | "fill")
                        && !name.ends_with('%')
                        && !is_fr(name)
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

pub fn grid_flow(body: &[Node]) -> Option<&str> {
    statements(body)
        .find(|statement| statement.words.first().map(String::as_str) == Some("flow"))
        .and_then(|statement| statement.words.get(1))
        .map(String::as_str)
}

/// Normalize grid track/area statements (`columns`, `rows`, `tracks`, `areas`,
/// `height screen|fill`).
pub fn normalize_grid_body(body: &[Node]) -> Vec<StyleFact> {
    let vertical = grid_flow(body) == Some("vertical");
    let mut facts = Vec::new();
    for statement in statements(body) {
        match word(statement, 0) {
            Some("columns") => {
                facts.extend(normalize_columns(statement, vertical));
            }
            Some("rows") => {
                let names = &statement.words[1..];
                if !names.is_empty() {
                    let rows = names.iter().map(|_| "auto").collect::<Vec<_>>().join(" ");
                    facts.push(fact("layout.grid.rows", "grid-template-rows", rows));
                }
            }
            Some("tracks") => {
                let Some(axis) = word(statement, 1) else {
                    continue;
                };
                let values = statement
                    .words
                    .iter()
                    .skip(2)
                    .map(|value| track_css_value(value))
                    .collect::<Vec<_>>();
                if values.is_empty() {
                    continue;
                }
                match axis {
                    "columns" => facts.push(fact(
                        "layout.grid.columns",
                        "grid-template-columns",
                        values.join(" "),
                    )),
                    "rows" => facts.push(fact(
                        "layout.grid.rows",
                        "grid-template-rows",
                        values.join(" "),
                    )),
                    _ => {}
                }
            }
            Some("height") if word(statement, 1) == Some("screen") => {
                facts.push(fact("size.min-height", "min-height", "100vh"));
            }
            Some("height") if word(statement, 1) == Some("fill") => {
                facts.push(fact("size.min-height", "min-height", "100%"));
            }
            _ => {}
        }
    }

    let area_rows = statements(body)
        .filter(|statement| statement.words.first().map(String::as_str) == Some("areas"))
        .map(|statement| {
            format!(
                "\"{}\"",
                statement
                    .words
                    .iter()
                    .skip(1)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect::<Vec<_>>();
    if !area_rows.is_empty() {
        facts.push(fact(
            "layout.grid.areas",
            "grid-template-areas",
            area_rows.join(" "),
        ));
    }

    facts
}

pub fn normalize_columns(statement: &Statement, vertical: bool) -> Vec<StyleFact> {
    let names = &statement.words[1..];
    if names == ["responsive", "cards"] {
        return vec![fact(
            "layout.grid.columns",
            "grid-template-columns",
            "repeat(auto-fit, minmax(220px, 1fr))",
        )];
    }
    if names.is_empty() {
        return Vec::new();
    }
    if vertical && names.iter().all(|name| is_identifier_grid_name(name)) {
        return vec![StyleFact::new(
            "layout.grid.columns",
            vec![
                CssDecl::new("grid-template-columns", "minmax(0, 1fr)"),
                CssDecl::new(
                    "grid-template-rows",
                    names.iter().map(|_| "auto").collect::<Vec<_>>().join(" "),
                ),
                CssDecl::new(
                    "grid-template-areas",
                    names
                        .iter()
                        .map(|name| format!("\"{name}\""))
                        .collect::<Vec<_>>()
                        .join(" "),
                ),
            ],
        )];
    }
    let columns = names
        .iter()
        .map(|value| column_css_value(value))
        .collect::<Vec<_>>()
        .join(" ");
    let mut decls = vec![CssDecl::new("grid-template-columns", columns)];
    if names.iter().all(|name| is_identifier_grid_name(name)) {
        decls.push(CssDecl::new(
            "grid-template-areas",
            format!("\"{}\"", names.join(" ")),
        ));
    }
    vec![StyleFact::new("layout.grid.columns", decls)]
}

pub fn normalize_area_placement(body: &[Node]) -> Vec<StyleFact> {
    let mut facts = Vec::new();
    let find = |keyword: &str| {
        statements(body)
            .find(|statement| statement.words.first().map(String::as_str) == Some(keyword))
            .and_then(|statement| statement.words.get(1))
            .cloned()
    };
    if let Some(value) = find("place") {
        facts.push(fact("layout.area.place", "grid-area", value));
    }
    if let Some(value) = find("col") {
        facts.push(fact("layout.area.col", "grid-column", value));
    }
    if let Some(value) = find("row") {
        facts.push(fact("layout.area.row", "grid-row", value));
    }
    if let Some(value) = find("span") {
        facts.push(fact(
            "layout.area.span",
            "grid-column",
            format!("span {value}"),
        ));
    }
    facts
}

pub fn normalize_page_root(body: &[Node]) -> Vec<StyleFact> {
    let mut facts = Vec::new();
    for statement in statements(body) {
        match word(statement, 0) {
            Some("background") => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("background", "background", resolve_value(value)));
                }
            }
            Some("color") => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("color", "color", resolve_value(value)));
                }
            }
            Some("margin") => {
                if let Some(value) = statement.words.get(1) {
                    let css_value = match value.as_str() {
                        "none" => "0".to_string(),
                        "small" | "medium" | "large" | "xlarge" => space_value(value),
                        other => resolve_value(other),
                    };
                    facts.push(fact("margin", "margin", css_value));
                }
            }
            Some("font-family") => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("typography.font", "font-family", value.clone()));
                }
            }
            Some("font-size") => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("typography.size", "font-size", value.clone()));
                }
            }
            Some("min-height") => {
                if let Some(value) = statement.words.get(1) {
                    let css_value = match value.as_str() {
                        "screen" => "100vh",
                        "fill" => "100%",
                        other => other,
                    };
                    facts.push(fact("size.min-height", "min-height", css_value));
                }
            }
            _ => {}
        }
    }
    facts
}

fn normalize_condition_body(
    kind: &DeclarationKind,
    body: &[Node],
    ctx: &StyleContext,
) -> Vec<StyleFact> {
    match kind {
        DeclarationKind::Grid => {
            let mut facts = normalize_grid_body(body);
            facts.extend(normalize_statements(body, ctx));
            facts
        }
        DeclarationKind::Area => {
            let mut facts = normalize_statements(body, ctx);
            facts.extend(normalize_area_placement(body));
            facts
        }
        _ => normalize_statements(body, ctx),
    }
}

/// Normalize a state block body: effect statements plus transitions.
pub fn normalize_effect_body(body: &[Node], _ctx: &StyleContext) -> Vec<StyleFact> {
    let mut facts = Vec::new();
    for statement in statements(body) {
        if let Some(effect) = normalize_effect(statement) {
            facts.push(effect);
        }
    }
    facts
}

/// Normalize one effect statement (`lift small`, `glow accent`, `blur`, ...).
pub fn normalize_effect(statement: &Statement) -> Option<StyleFact> {
    let keyword = word(statement, 0)?;
    let transform = |path: &str, value: String| -> Option<StyleFact> {
        Some(fact(path, TRANSFORM_PART, value))
    };
    match keyword {
        "lift" => transform(
            "effect.lift",
            format!(
                "translateY(-{})",
                format_px(tuned_value(word(statement, 1), &MOVEMENT_SCALE))
            ),
        ),
        "sink" => transform(
            "effect.sink",
            format!(
                "translateY({})",
                format_px(tuned_value(word(statement, 1), &MOVEMENT_SCALE))
            ),
        ),
        "shift" => {
            let amount = format_px(tuned_value(word(statement, 2), &MOVEMENT_SCALE));
            match word(statement, 1) {
                Some("left") => transform("effect.shift", format!("translateX(-{amount})")),
                Some("right") => transform("effect.shift", format!("translateX({amount})")),
                Some("up") => transform("effect.shift", format!("translateY(-{amount})")),
                Some("down") => transform("effect.shift", format!("translateY({amount})")),
                _ => None,
            }
        }
        "grow" => transform(
            "effect.grow",
            format!(
                "scale({})",
                format_number(tuned_value(word(statement, 1), &GROW_SCALE))
            ),
        ),
        "shrink" => transform(
            "effect.shrink",
            format!(
                "scale({})",
                format_number(tuned_value(word(statement, 1), &SHRINK_SCALE))
            ),
        ),
        "tilt" => {
            let degrees = tuned_value(word(statement, 2), &TILT_SCALE);
            match word(statement, 1) {
                Some("left") => {
                    transform("effect.tilt", format!("rotate(-{})", format_deg(degrees)))
                }
                Some("right") => {
                    transform("effect.tilt", format!("rotate({})", format_deg(degrees)))
                }
                _ => None,
            }
        }
        "glow" => {
            let value = word(statement, 1).unwrap_or("accent");
            if value == "none" {
                Some(fact("shadow", "box-shadow", "none"))
            } else {
                Some(fact(
                    "shadow",
                    "box-shadow",
                    format!("var(--frame-glow-{value})"),
                ))
            }
        }
        "brighten" => Some(fact(
            "effect.brighten",
            FILTER_PART,
            match word(statement, 1) {
                Some("subtle") => "brightness(1.04)",
                Some("large") => "brightness(1.12)",
                _ => "brightness(1.08)",
            },
        )),
        "dim" => Some(fact("effect.dim", FILTER_PART, "brightness(0.92)")),
        "press" => transform("effect.press", "translateY(1px)".to_string()),
        "pop" => transform("effect.pop", "scale(1.04)".to_string()),
        "blur" => Some(fact(
            "effect.blur",
            FILTER_PART,
            match word(statement, 1) {
                Some("heavy") => "blur(8px)",
                Some("none") => "blur(0)",
                _ => "blur(2px)",
            },
        )),
        "ring" => {
            let value = word(statement, 1).unwrap_or("accent");
            Some(StyleFact::new(
                "outline",
                vec![
                    CssDecl::new("outline", format!("2px solid var(--frame-color-{value})")),
                    CssDecl::new("outline-offset", "2px"),
                ],
            ))
        }
        "fade" => Some(fact("opacity", "opacity", "0.72")),
        "scale" => transform("effect.scale", "scale(1.02)".to_string()),
        "transition" => Some(normalize_transition(statement)),
        "duration" => Some(normalize_duration(statement)),
        "ease" => Some(normalize_ease(statement)),
        "animation" | "animate" => Some(normalize_animation(statement)),
        _ => None,
    }
}

fn normalize_transition(statement: &Statement) -> StyleFact {
    let value = word(statement, 1).unwrap_or("smooth");
    let css = match value {
        "none" => "none",
        "fast" => "all 120ms ease-out",
        "slow" => "all 360ms ease",
        _ => "all 200ms ease",
    };
    fact("motion.transition", "transition", css)
}

fn normalize_duration(statement: &Statement) -> StyleFact {
    let value = word(statement, 1).unwrap_or("normal");
    let duration = match value {
        "fast" => "120ms",
        "slow" => "360ms",
        _ => "200ms",
    };
    fact(
        "motion.transition.duration",
        "transition-duration",
        duration,
    )
}

fn normalize_ease(statement: &Statement) -> StyleFact {
    let value = word(statement, 1).unwrap_or("smooth");
    let ease = match value {
        "linear" => "linear",
        "bounce" => "cubic-bezier(.2, 1.4, .4, 1)",
        "sharp" => "cubic-bezier(.4, 0, 1, 1)",
        _ => "ease",
    };
    fact("motion.transition.ease", "transition-timing-function", ease)
}

fn normalize_animation(statement: &Statement) -> StyleFact {
    let value = word(statement, 1).unwrap_or("none");
    if value == "none" {
        fact("motion.animation", "animation", "none")
    } else {
        fact(
            "motion.animation",
            "animation",
            format!("frame-{value} 240ms ease both"),
        )
    }
}

fn normalize_animation_block(name: &str, body: &[Node]) -> StyleFact {
    let mut duration = "240ms".to_string();
    let mut ease = "ease".to_string();
    let mut delay = "0ms".to_string();
    let mut iteration = "1".to_string();
    let mut direction = "normal".to_string();
    let mut fill = "both".to_string();
    let mut play_state = "running".to_string();

    for statement in statements(body) {
        match word(statement, 0) {
            Some("duration") => duration = animation_duration(word(statement, 1)),
            Some("delay") => delay = animation_duration(word(statement, 1)),
            Some("ease") => ease = animation_ease(word(statement, 1)),
            Some("iteration") => {
                iteration = word(statement, 1).unwrap_or("1").to_string();
            }
            Some("direction") => {
                direction = word(statement, 1).unwrap_or("normal").to_string();
            }
            Some("fill") => {
                fill = word(statement, 1).unwrap_or("both").to_string();
            }
            Some("play-state") => {
                play_state = word(statement, 1).unwrap_or("running").to_string();
            }
            _ => {}
        }
    }

    StyleFact::new(
        "motion.animation",
        vec![
            CssDecl::new(
                "animation",
                format!("frame-{name} {duration} {ease} {delay} {iteration} {direction} {fill}"),
            ),
            CssDecl::new("animation-play-state", play_state),
        ],
    )
}

fn normalize_advanced_css(statement: &Statement) -> Option<StyleFact> {
    let property = statement.words.get(1)?;
    let property = property.trim_matches('"');
    let value = statement
        .words
        .iter()
        .skip(2)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if property.is_empty() || value.is_empty() {
        return None;
    }
    Some(fact(&format!("advanced.{property}"), property, value))
}

/// Normalize common style statements shared by every styled declaration.
pub fn normalize_statements(body: &[Node], ctx: &StyleContext) -> Vec<StyleFact> {
    let mut facts = Vec::new();

    for statement in statements(body) {
        let Some(keyword) = word(statement, 0) else {
            continue;
        };
        match keyword {
            "surface" => {
                if word(statement, 1) == Some("gradient") {
                    if let Some(name) = statement.words.get(2) {
                        facts.push(fact(
                            "background",
                            "background",
                            format!("var(--frame-gradient-{name})"),
                        ));
                    }
                } else if let Some(name) = statement.words.get(1) {
                    facts.push(fact(
                        "background",
                        "background",
                        surface_value(name, ctx.contract),
                    ));
                }
            }
            "background" => {
                if let Some(name) = statement.words.get(1) {
                    facts.push(fact(
                        "background",
                        "background",
                        background_value(name, ctx.contract),
                    ));
                }
            }
            "padding" => facts.extend(normalize_box_space("padding", statement)),
            "margin" => facts.extend(normalize_box_space("margin", statement)),
            "display" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("layout.display", "display", value.clone()));
                }
            }
            "visibility" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("visibility", "visibility", value.clone()));
                }
            }
            "flex" => facts.extend(normalize_flex(statement)),
            "gap" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("layout.gap", "gap", space_value(value)));
                }
            }
            "opacity" => {
                if let Some(value) = statement.words.get(1) {
                    let opacity = match value.as_str() {
                        "none" => "0",
                        "slight" => "0.1",
                        "subtle" => "0.25",
                        "half" => "0.5",
                        "strong" => "0.75",
                        "full" => "1.0",
                        other => other,
                    };
                    facts.push(fact("opacity", "opacity", opacity));
                }
            }
            "shadow" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact(
                        "shadow",
                        "box-shadow",
                        if value.starts_with("token(") {
                            resolve_value(value)
                        } else {
                            format!("var(--frame-shadow-{value})")
                        },
                    ));
                }
            }
            "radius" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact(
                        "radius",
                        "border-radius",
                        if value.starts_with("token(") {
                            resolve_value(value)
                        } else {
                            format!("var(--frame-radius-{value})")
                        },
                    ));
                }
            }
            "border" => facts.extend(normalize_border(statement)),
            "outline" => facts.extend(normalize_outline(statement)),
            "layout" => facts.extend(normalize_layout_preset(statement)),
            "overflow" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("overflow", "overflow", value.clone()));
                }
            }
            "scroll" => match word(statement, 1) {
                Some("x") => facts.push(fact("overflow.x", "overflow-x", "auto")),
                Some("y") => facts.push(fact("overflow.y", "overflow-y", "auto")),
                Some("both") => facts.push(fact("overflow", "overflow", "auto")),
                _ => {}
            },
            "scrollbar" => match word(statement, 1) {
                Some("dense") => facts.push(StyleFact::new(
                    "scrollbar",
                    vec![
                        CssDecl::new("scrollbar-width", "thin"),
                        CssDecl::new("scrollbar-color", "var(--frame-color-muted) transparent"),
                    ],
                )),
                Some("normal") => facts.push(fact("scrollbar", "scrollbar-width", "auto")),
                _ => {}
            },
            "box" => match word(statement, 1) {
                Some("border") => facts.push(fact("layout.box-sizing", "box-sizing", "border-box")),
                Some("content") => {
                    facts.push(fact("layout.box-sizing", "box-sizing", "content-box"))
                }
                _ => {}
            },
            "square" => {
                let size = match word(statement, 1) {
                    Some("server") => "3rem",
                    Some("avatar") | Some("icon") => "2.5rem",
                    Some("presence") => "0.65rem",
                    Some("unread") => "0.55rem",
                    _ => continue,
                };
                facts.push(StyleFact::new(
                    "size.square",
                    vec![CssDecl::new("width", size), CssDecl::new("height", size)],
                ));
            }
            "self" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(StyleFact::new(
                        "layout.self",
                        vec![
                            CssDecl::new("justify-self", value.clone()),
                            CssDecl::new("align-self", value.clone()),
                        ],
                    ));
                }
            }
            "nudge" => {
                if word(statement, 1) == Some("top-right") {
                    facts.push(StyleFact::new(
                        "position.nudge",
                        vec![
                            CssDecl::new("top", "-0.1rem"),
                            CssDecl::new("right", "-0.1rem"),
                        ],
                    ));
                }
            }
            "height" | "width" | "min-height" | "max-height" | "min-width" | "max-width"
            | "inline-size" | "block-size" | "min-inline-size" | "max-inline-size"
            | "min-block-size" | "max-block-size" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact(
                        &format!("size.{keyword}"),
                        keyword,
                        size_css_value(keyword, value),
                    ));
                }
            }
            "align" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("layout.align", "align-items", css_alignment(value)));
                }
            }
            "justify" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact(
                        "layout.justify",
                        "justify-content",
                        css_justify(value),
                    ));
                }
            }
            "position" => facts.extend(normalize_position(statement)),
            "anchor" => facts.push(normalize_anchor(statement)),
            "offset" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("position.offset", "inset", space_value(value)));
                }
            }
            "z" => {
                if let Some(value) = statement.words.get(1) {
                    let z = Z_LAYERS
                        .iter()
                        .find(|(name, _)| name == value)
                        .map(|(_, z)| *z)
                        .unwrap_or(1);
                    facts.push(fact("z", "z-index", z.to_string()));
                }
            }
            "transition" => facts.push(normalize_transition(statement)),
            "duration" => facts.push(normalize_duration(statement)),
            "ease" => facts.push(normalize_ease(statement)),
            "animation" | "animate" => facts.push(normalize_animation(statement)),
            "lift" | "sink" | "shift" | "grow" | "shrink" | "tilt" | "press" | "pop" | "glow"
            | "brighten" | "dim" | "blur" | "ring" => {
                if let Some(effect) = normalize_effect(statement) {
                    facts.push(effect);
                }
            }
            "motion" => {
                if let Some(name) = statement.words.get(1) {
                    // Resolved by the caller against document motions; the
                    // reference fact survives so backends can report misses.
                    facts.push(fact("motion.reference", "@motion", name.clone()));
                }
            }
            "text" | "color" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("color", "color", color_value(value)));
                }
            }
            "font" => {
                if word(statement, 1) == Some("mono") {
                    facts.push(fact(
                        "typography.font",
                        "font-family",
                        "ui-monospace, SFMono-Regular, Menlo, monospace",
                    ));
                }
            }
            "truncate" => {
                facts.push(StyleFact::new(
                    "typography.truncate",
                    vec![
                        CssDecl::new("white-space", "nowrap"),
                        CssDecl::new("overflow", "hidden"),
                        CssDecl::new("text-overflow", "ellipsis"),
                    ],
                ));
            }
            "wrap" => match word(statement, 1) {
                Some("anywhere") => {
                    facts.push(fact("typography.wrap", "overflow-wrap", "anywhere"))
                }
                Some("normal") => facts.push(fact("typography.wrap", "overflow-wrap", "normal")),
                _ => {}
            },
            "case" => {
                let value = match word(statement, 1) {
                    Some("uppercase") => "uppercase",
                    Some("lowercase") => "lowercase",
                    Some("capitalize") => "capitalize",
                    Some("normal") => "none",
                    _ => continue,
                };
                facts.push(fact("typography.case", "text-transform", value));
            }
            "align-text" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("typography.align", "text-align", value.clone()));
                }
            }
            "decoration" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact(
                        "typography.decoration",
                        "text-decoration-line",
                        value.clone(),
                    ));
                }
            }
            "whitespace" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("typography.whitespace", "white-space", value.clone()));
                }
            }
            "word-break" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("typography.word-break", "word-break", value.clone()));
                }
            }
            "hyphenate" => {
                if let Some(value) = statement.words.get(1) {
                    facts.push(fact("typography.hyphens", "hyphens", value.clone()));
                }
            }
            "size" => {
                if let Some(value) = statement.words.get(1) {
                    let size = match value.as_str() {
                        "heading" => "2rem",
                        "caption" => "0.875rem",
                        _ => "1rem",
                    };
                    facts.push(fact("typography.size", "font-size", size));
                }
            }
            "weight" => {
                if let Some(value) = statement.words.get(1) {
                    let weight = match value.as_str() {
                        "thin" => 300,
                        "semibold" => 600,
                        "bold" => 700,
                        _ => 400,
                    };
                    facts.push(fact("typography.weight", "font-weight", weight.to_string()));
                }
            }
            "line" => {
                let line_height = match word(statement, 1) {
                    Some("relaxed") => "1.45",
                    Some("tight") => "1.15",
                    Some("normal") => "1.3",
                    _ => continue,
                };
                facts.push(fact("typography.line", "line-height", line_height));
            }
            "letter" => {
                if word(statement, 1) == Some("normal") {
                    facts.push(fact("typography.letter", "letter-spacing", "0"));
                }
            }
            "control" => {
                if word(statement, 1) == Some("reset") {
                    facts.push(fact("control", "appearance", "none"));
                }
            }
            "interactive" => facts.push(fact("interactive", "cursor", "pointer")),
            "css" => {
                if let Some(advanced) = normalize_advanced_css(statement) {
                    facts.push(advanced);
                }
            }
            _ => {}
        }
    }

    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if block.name == "advanced" {
            for statement in statements(&block.body) {
                if statement.words.first().map(String::as_str) == Some("css") {
                    if let Some(advanced) = normalize_advanced_css(statement) {
                        facts.push(advanced);
                    }
                }
            }
        } else if let Some(animation_name) = block.name.strip_prefix("animation ") {
            facts.push(normalize_animation_block(animation_name, &block.body));
        }
    }

    facts
}

fn normalize_box_space(property: &str, statement: &Statement) -> Vec<StyleFact> {
    match (word(statement, 1), statement.words.get(2)) {
        (Some(edge @ ("top" | "right" | "bottom" | "left")), Some(value)) => vec![fact(
            &format!("{property}.{edge}"),
            &format!("{property}-{edge}"),
            space_value(value),
        )],
        (Some("x" | "inline"), Some(value)) => vec![fact(
            &format!("{property}.inline"),
            &format!("{property}-inline"),
            space_value(value),
        )],
        (Some("y" | "block"), Some(value)) => vec![fact(
            &format!("{property}.block"),
            &format!("{property}-block"),
            space_value(value),
        )],
        (Some(value), _) => vec![fact(property, property, space_value(value))],
        _ => Vec::new(),
    }
}

fn normalize_flex(statement: &Statement) -> Vec<StyleFact> {
    match (word(statement, 1), word(statement, 2)) {
        (Some("direction"), Some(value)) => {
            vec![fact("layout.flex.direction", "flex-direction", value)]
        }
        (Some("wrap"), Some(value)) => vec![fact("layout.flex.wrap", "flex-wrap", value)],
        (Some("grow"), Some(value)) => vec![fact("layout.flex.grow", "flex-grow", value)],
        (Some("shrink"), Some(value)) => vec![fact("layout.flex.shrink", "flex-shrink", value)],
        (Some("basis"), Some(value)) => vec![fact(
            "layout.flex.basis",
            "flex-basis",
            size_css_value("width", value),
        )],
        _ => Vec::new(),
    }
}

fn normalize_border(statement: &Statement) -> Vec<StyleFact> {
    match word(statement, 1) {
        Some("none") => vec![fact("border", "border", "0")],
        Some(edge @ ("top" | "right" | "bottom" | "left")) => {
            let value = word(statement, 2).unwrap_or("soft");
            let css_value = match value {
                "soft" => "1px solid rgba(255, 255, 255, 0.14)".to_string(),
                "strong" => "1px solid rgba(255, 255, 255, 0.32)".to_string(),
                "none" => "0".to_string(),
                value => format!("1px solid {}", color_value(value)),
            };
            vec![fact(
                &format!("border.{edge}"),
                &format!("border-{edge}"),
                css_value,
            )]
        }
        Some("radius") => {
            let value = word(statement, 2).unwrap_or("medium");
            vec![fact(
                "radius",
                "border-radius",
                format!("var(--frame-radius-{value})"),
            )]
        }
        Some("width") => {
            let width = word(statement, 2).unwrap_or("small");
            let value = match width {
                "medium" => "2px",
                "large" => "3px",
                _ => "1px",
            };
            vec![StyleFact::new(
                "border.width",
                vec![
                    CssDecl::new("border-width", value),
                    CssDecl::new("border-style", "solid"),
                ],
            )]
        }
        Some("style") => {
            let style = word(statement, 2).unwrap_or("solid");
            vec![fact("border.style", "border-style", style)]
        }
        Some("soft") => vec![fact(
            "border",
            "border",
            "1px solid rgba(255, 255, 255, 0.14)",
        )],
        Some("strong") => vec![fact(
            "border",
            "border",
            "1px solid rgba(255, 255, 255, 0.32)",
        )],
        Some(value) => vec![fact(
            "border",
            "border",
            format!("1px solid {}", color_value(value)),
        )],
        _ => Vec::new(),
    }
}

fn normalize_outline(statement: &Statement) -> Vec<StyleFact> {
    match word(statement, 1) {
        Some("none") => vec![fact("outline", "outline", "0")],
        Some("offset") => {
            let value = word(statement, 2).unwrap_or("small");
            vec![fact("outline.offset", "outline-offset", space_value(value))]
        }
        Some(value) => vec![fact(
            "outline",
            "outline",
            format!("2px solid {}", color_value(value)),
        )],
        _ => Vec::new(),
    }
}

fn normalize_layout_preset(statement: &Statement) -> Vec<StyleFact> {
    let decls = match word(statement, 1) {
        Some("icon-content-action") | Some("composer") => vec![
            CssDecl::new("display", "grid"),
            CssDecl::new("grid-template-columns", "auto minmax(0, 1fr) auto"),
            CssDecl::new("align-items", "center"),
        ],
        Some("avatar-content") => vec![
            CssDecl::new("display", "grid"),
            CssDecl::new("grid-template-columns", "2.5rem minmax(0, 1fr)"),
        ],
        Some("header") => vec![
            CssDecl::new("display", "grid"),
            CssDecl::new("grid-template-columns", "16rem minmax(0, 1fr) auto"),
            CssDecl::new("align-items", "center"),
        ],
        Some("center") => vec![
            CssDecl::new("display", "grid"),
            CssDecl::new("place-items", "center"),
        ],
        _ => return Vec::new(),
    };
    vec![StyleFact::new("layout.preset", decls)]
}

fn normalize_position(statement: &Statement) -> Vec<StyleFact> {
    let mut decls = Vec::new();
    match word(statement, 1) {
        Some("relative") => decls.push(CssDecl::new("position", "relative")),
        Some("absolute") => {
            decls.push(CssDecl::new("position", "absolute"));
            decls.extend(position_edge_decls(word(statement, 2)));
        }
        Some("sticky") => {
            decls.push(CssDecl::new("position", "sticky"));
            decls.extend(position_edge_decls(word(statement, 2).or(Some("top"))));
        }
        Some("fixed") => {
            decls.push(CssDecl::new("position", "fixed"));
            decls.extend(position_edge_decls(word(statement, 2).or(Some("top"))));
        }
        Some("center") => decls.push(CssDecl::new("place-items", "center")),
        _ => return Vec::new(),
    }
    vec![StyleFact::new("position", decls)]
}

fn normalize_anchor(statement: &Statement) -> StyleFact {
    let value = word(statement, 1).unwrap_or("top");
    let mut decls = vec![CssDecl::new("position", "sticky")];
    match value {
        "bottom" => decls.push(CssDecl::new("bottom", "0")),
        "left" => decls.push(CssDecl::new("left", "0")),
        "right" => decls.push(CssDecl::new("right", "0")),
        "top-left" => {
            decls.push(CssDecl::new("top", "0"));
            decls.push(CssDecl::new("left", "0"));
        }
        "top-right" => {
            decls.push(CssDecl::new("top", "0"));
            decls.push(CssDecl::new("right", "0"));
        }
        "bottom-left" => {
            decls.push(CssDecl::new("bottom", "0"));
            decls.push(CssDecl::new("left", "0"));
        }
        "bottom-right" => {
            decls.push(CssDecl::new("bottom", "0"));
            decls.push(CssDecl::new("right", "0"));
        }
        _ => decls.push(CssDecl::new("top", "0")),
    }
    StyleFact::new("position", decls)
}

fn position_edge_decls(edge: Option<&str>) -> Vec<CssDecl> {
    match edge {
        Some("top") => vec![CssDecl::new("top", "0")],
        Some("bottom") => vec![CssDecl::new("bottom", "0")],
        Some("top-left") => vec![CssDecl::new("top", "0"), CssDecl::new("left", "0")],
        Some("top-right") => vec![CssDecl::new("top", "0"), CssDecl::new("right", "0")],
        Some("bottom-left") => vec![CssDecl::new("bottom", "0"), CssDecl::new("left", "0")],
        Some("bottom-right") => vec![CssDecl::new("bottom", "0"), CssDecl::new("right", "0")],
        _ => Vec::new(),
    }
}
