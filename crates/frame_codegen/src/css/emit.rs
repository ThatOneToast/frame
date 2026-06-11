use super::*;
use frame_core::{symbols::SymbolIndex, Declaration, DeclarationKind, Document, Node, Statement};

pub(crate) fn emit_declaration_css(
    css: &mut String,
    declaration: &Declaration,
    symbols: &SymbolIndex,
    all_declarations: &[Declaration],
) {
    let class_name = format!("fr-{}", declaration.name.text);

    // Resolve inherited body: base properties first, child properties override.
    let resolved_body = resolve_inherited_body(declaration, all_declarations);

    match declaration.kind {
        DeclarationKind::Grid => {
            css.push_str(&format!(".{class_name} {{\n  display: grid;\n"));
            emit_grid(css, &resolved_body);
            emit_common(css, &resolved_body, symbols);
            css.push_str("}\n\n");
            emit_grid_section_rules(css, &class_name, &resolved_body);
            emit_condition_blocks(css, &class_name, declaration.kind.clone(), &resolved_body);
        }
        DeclarationKind::Area => {
            css.push_str(&format!(".{class_name} {{\n"));
            emit_common(css, &resolved_body, symbols);
            if let Some(value) = find_statement_value(&resolved_body, "place") {
                css.push_str(&format!("  grid-area: {value};\n"));
            }
            if let Some(value) = find_statement_value(&resolved_body, "col") {
                css.push_str(&format!("  grid-column: {value};\n"));
            }
            if let Some(value) = find_statement_value(&resolved_body, "row") {
                css.push_str(&format!("  grid-row: {value};\n"));
            }
            if let Some(value) = find_statement_value(&resolved_body, "span") {
                css.push_str(&format!("  grid-column: span {value};\n"));
            }
            css.push_str("}\n\n");
            emit_condition_blocks(css, &class_name, declaration.kind.clone(), &resolved_body);
        }
        DeclarationKind::Card
        | DeclarationKind::Stack
        | DeclarationKind::Row
        | DeclarationKind::Button
        | DeclarationKind::Text
        | DeclarationKind::Center
        | DeclarationKind::Split
        | DeclarationKind::Overlay
        | DeclarationKind::Dock => {
            css.push_str(&format!(".{class_name} {{\n"));
            match declaration.kind {
                DeclarationKind::Card | DeclarationKind::Stack => {
                    css.push_str("  display: flex;\n  flex-direction: column;\n")
                }
                DeclarationKind::Row => {
                    if has_columns_statement(&resolved_body) {
                        css.push_str("  display: grid;\n");
                        for statement in statements(&resolved_body) {
                            if statement.words.first().map(String::as_str) == Some("columns") {
                                emit_columns(css, statement, false);
                            }
                        }
                    } else {
                        css.push_str("  display: flex;\n  flex-direction: row;\n");
                    }
                }
                DeclarationKind::Center => {
                    css.push_str("  display: grid;\n  place-items: center;\n")
                }
                DeclarationKind::Split => css.push_str(
                    "  display: grid;\n  grid-template-columns: minmax(0, auto) minmax(0, 1fr);\n",
                ),
                DeclarationKind::Overlay => {
                    css.push_str("  position: fixed;\n  inset: 0;\n  display: grid;\n")
                }
                DeclarationKind::Dock => {
                    css.push_str("  position: fixed;\n  inset-inline: 0;\n  bottom: 0;\n")
                }
                _ => {}
            }
            emit_common(css, &resolved_body, symbols);
            css.push_str("}\n\n");

            for node in &resolved_body {
                if let Node::Block(block) = node {
                    let Some(selector) = state_selector(&block.name) else {
                        continue;
                    };
                    css.push_str(&format!(".{class_name}{selector} {{\n"));
                    emit_effects(css, &block.body);
                    css.push_str("}\n\n");
                }
            }
            emit_condition_blocks(css, &class_name, declaration.kind.clone(), &resolved_body);
        }
        DeclarationKind::Keyframes => {
            emit_custom_keyframes(css, &declaration.name.text, &resolved_body);
        }
        DeclarationKind::Supports => emit_supports(css, declaration, symbols),
        DeclarationKind::StyleOrder => emit_style_order(css, declaration),
        DeclarationKind::StyleGroup => emit_style_group(css, declaration, symbols),
        DeclarationKind::Html => {
            css.push_str("html {\n");
            emit_page_root_css(css, &resolved_body);
            css.push_str("}\n\n");
        }
        DeclarationKind::Body => {
            css.push_str("body {\n");
            css.push_str("  min-height: 100vh;\n");
            emit_page_root_css(css, &resolved_body);
            css.push_str("}\n\n");
        }
        _ => {}
    }
}

/// Resolve the effective body for a declaration with inheritance.
/// Base properties come first; child properties override by statement word[0].
fn resolve_inherited_body(
    declaration: &Declaration,
    all_declarations: &[Declaration],
) -> Vec<Node> {
    let Some(ref base_name) = declaration.extends else {
        return declaration.body.clone();
    };
    // Find the base declaration.
    let Some(base_decl) = all_declarations
        .iter()
        .find(|d| d.name.text == base_name.text)
    else {
        return declaration.body.clone();
    };
    // Recursively resolve the base's body (supports multi-level inheritance).
    let base_body = resolve_inherited_body(base_decl, all_declarations);

    // Merge: start with base, then override with child statements.
    // Child statements override base statements where word[0] matches.
    let mut merged = base_body;
    for node in &declaration.body {
        if let Node::Statement(stmt) = node {
            let key = stmt.words.first().cloned().unwrap_or_default();
            // Remove any base statement with the same key.
            merged.retain(|existing| {
                if let Node::Statement(existing_stmt) = existing {
                    existing_stmt.words.first() != Some(&key)
                } else {
                    true
                }
            });
        }
        // Also handle blocks (hover, focus, active, etc.) by name.
        if let Node::Block(block) = node {
            merged.retain(|existing| {
                if let Node::Block(existing_block) = existing {
                    existing_block.name != block.name
                } else {
                    true
                }
            });
        }
        merged.push(node.clone());
    }
    merged
}

pub(crate) fn emit_style_order(css: &mut String, declaration: &Declaration) {
    let names = style_order_names(&declaration.name.text);
    if !names.is_empty() {
        css.push_str(&format!("@layer {};\n\n", names.join(", ")));
    }
}
pub(crate) fn emit_style_group(css: &mut String, declaration: &Declaration, symbols: &SymbolIndex) {
    if declaration.name.text.is_empty() {
        return;
    }
    css.push_str(&format!("@layer {} {{\n", declaration.name.text));
    let mut nested_css = String::new();
    for node in &declaration.body {
        let Node::Block(block) = node else {
            continue;
        };
        let Some(nested) = declaration_from_block(block) else {
            continue;
        };
        emit_declaration_css(&mut nested_css, &nested, symbols, &[]);
    }
    for line in nested_css.lines() {
        if line.is_empty() {
            css.push('\n');
        } else {
            css.push_str("  ");
            css.push_str(line);
            css.push('\n');
        }
    }
    css.push_str("}\n\n");
}
pub(crate) fn emit_supports(css: &mut String, declaration: &Declaration, symbols: &SymbolIndex) {
    let Some(condition) = supports_condition(&declaration.name.text) else {
        return;
    };

    css.push_str(&format!("@supports {condition} {{\n"));
    let mut nested_css = String::new();
    for node in &declaration.body {
        let Node::Block(block) = node else {
            continue;
        };
        let Some(nested) = declaration_from_block(block) else {
            continue;
        };
        emit_declaration_css(&mut nested_css, &nested, symbols, &[]);
    }
    for line in nested_css.lines() {
        if line.is_empty() {
            css.push('\n');
        } else {
            css.push_str("  ");
            css.push_str(line);
            css.push('\n');
        }
    }
    css.push_str("}\n\n");
}
pub(crate) fn emit_grid(css: &mut String, body: &[Node]) {
    let vertical = grid_flow(body) == Some("vertical");
    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("columns") => emit_columns(css, statement, vertical),
            Some("rows") => emit_rows(css, statement),
            Some("tracks") => emit_tracks(css, statement),
            Some("areas") => {}
            Some("height") if statement.words.get(1).map(String::as_str) == Some("screen") => {
                css.push_str("  min-height: 100vh;\n");
            }
            Some("height") if statement.words.get(1).map(String::as_str) == Some("fill") => {
                css.push_str("  min-height: 100%;\n");
            }
            _ => {}
        }
    }
    emit_area_template(css, body);
}
pub(crate) fn emit_grid_section_rules(css: &mut String, class_name: &str, body: &[Node]) {
    let sections = grid_section_names(body);
    if sections.is_empty() {
        return;
    }

    for (index, section) in sections.iter().enumerate() {
        css.push_str(&format!(
            ".{class_name} > :nth-child({}), .{class_name} > [data-frame-section=\"{section}\"] {{\n  grid-area: {section};\n",
            index + 1
        ));
        if let Some(block) = section_block(body, section) {
            emit_common(
                css,
                &block.body,
                &frame_core::symbols::SymbolIndex::default(),
            );
        }
        css.push_str("}\n\n");
    }
}
pub(crate) fn emit_custom_tokens(css: &mut String, document: &Document) {
    for declaration in &document.declarations {
        if declaration.kind != DeclarationKind::Tokens {
            continue;
        }

        for node in &declaration.body {
            match node {
                Node::Statement(statement)
                    if statement.words.first().map(String::as_str) == Some("color") =>
                {
                    let (Some(name), Some(value)) =
                        (statement.words.get(1), statement.words.get(2))
                    else {
                        continue;
                    };
                    css.push_str(&format!("  --frame-color-{name}: {value};\n"));
                }
                Node::Block(block) if block.name.starts_with("gradient ") => {
                    let Some(name) = block.name.split_whitespace().nth(1) else {
                        continue;
                    };
                    if let Some(value) = gradient_css(&block.body) {
                        css.push_str(&format!("  --frame-gradient-{name}: {value};\n"));
                    }
                }
                _ => {}
            }
        }
    }
}
pub(crate) fn emit_custom_keyframes(css: &mut String, name: &str, body: &[Node]) {
    css.push_str(&format!("@keyframes frame-{name} {{\n"));
    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if !is_keyframe_selector(&block.name) {
            continue;
        }
        css.push_str(&format!("  {} {{\n", block.name));
        for statement in statements(&block.body) {
            emit_keyframe_statement(css, statement);
        }
        css.push_str("  }\n");
    }
    css.push_str("}\n\n");
}
pub(crate) fn emit_keyframe_statement(css: &mut String, statement: &Statement) {
    let Some(property) = statement.words.first().map(String::as_str) else {
        return;
    };
    let value = statement
        .words
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if value.is_empty() {
        return;
    }
    match property {
        "opacity" | "transform" | "filter" => {
            css.push_str(&format!("    {property}: {value};\n"));
        }
        "scale" | "translate" | "rotate" => {
            css.push_str(&format!("    transform: {property}({value});\n"));
        }
        _ => {}
    }
}
pub(crate) fn emit_keyframes(css: &mut String) {
    css.push_str("@keyframes frame-fade-in {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}\n\n");
    css.push_str("@keyframes frame-slide-up {\n  from { opacity: 0; transform: translateY(0.5rem); }\n  to { opacity: 1; transform: translateY(0); }\n}\n\n");
    css.push_str("@keyframes frame-pop-in {\n  from { opacity: 0; transform: scale(0.96); }\n  to { opacity: 1; transform: scale(1); }\n}\n\n");
    css.push_str(
        "@keyframes frame-pulse {\n  0%, 100% { opacity: 1; }\n  50% { opacity: 0.72; }\n}\n\n",
    );
}
pub(crate) fn emit_columns(css: &mut String, statement: &Statement, vertical: bool) {
    let names = &statement.words[1..];
    if names == ["responsive", "cards"] {
        css.push_str("  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));\n");
    } else if !names.is_empty() {
        if vertical && names.iter().all(|name| is_identifier_grid_name(name)) {
            css.push_str("  grid-template-columns: minmax(0, 1fr);\n");
            css.push_str(&format!(
                "  grid-template-rows: {};\n",
                names.iter().map(|_| "auto").collect::<Vec<_>>().join(" ")
            ));
            css.push_str(&format!(
                "  grid-template-areas: {};\n",
                names
                    .iter()
                    .map(|name| format!("\"{name}\""))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            return;
        }
        let columns = names
            .iter()
            .map(|value| column_css_value(value))
            .collect::<Vec<_>>()
            .join(" ");
        css.push_str(&format!("  grid-template-columns: {columns};\n"));
        if names.iter().all(|name| is_identifier_grid_name(name)) {
            css.push_str(&format!(
                "  grid-template-areas: \"{}\";\n",
                names.join(" ")
            ));
        }
    }
}
pub(crate) fn emit_rows(css: &mut String, statement: &Statement) {
    let names = &statement.words[1..];
    if !names.is_empty() {
        let rows = names.iter().map(|_| "auto").collect::<Vec<_>>().join(" ");
        css.push_str(&format!("  grid-template-rows: {rows};\n"));
    }
}
pub(crate) fn emit_tracks(css: &mut String, statement: &Statement) {
    let Some(axis) = statement.words.get(1).map(String::as_str) else {
        return;
    };
    let values = statement
        .words
        .iter()
        .skip(2)
        .map(|value| track_css_value(value))
        .collect::<Vec<_>>();
    if values.is_empty() {
        return;
    }
    match axis {
        "columns" => css.push_str(&format!("  grid-template-columns: {};\n", values.join(" "))),
        "rows" => css.push_str(&format!("  grid-template-rows: {};\n", values.join(" "))),
        _ => {}
    }
}
pub(crate) fn emit_area_template(css: &mut String, body: &[Node]) {
    let rows = statements(body)
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

    if !rows.is_empty() {
        css.push_str(&format!("  grid-template-areas: {};\n", rows.join(" ")));
    }
}
pub(crate) fn emit_page_root_css(css: &mut String, body: &[Node]) {
    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("background") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  background: {value};\n"));
                }
            }
            Some("color") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  color: {value};\n"));
                }
            }
            Some("margin") => {
                if let Some(value) = statement.words.get(1) {
                    let css_value = match value.as_str() {
                        "none" => "0",
                        "small" => "var(--frame-space-small)",
                        "medium" => "var(--frame-space-medium)",
                        "large" => "var(--frame-space-large)",
                        "xlarge" => "var(--frame-space-xlarge)",
                        other => other,
                    };
                    css.push_str(&format!("  margin: {css_value};\n"));
                }
            }
            Some("font-family") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  font-family: {value};\n"));
                }
            }
            Some("font-size") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  font-size: {value};\n"));
                }
            }
            Some("min-height") => {
                if let Some(value) = statement.words.get(1) {
                    let css_value = match value.as_str() {
                        "screen" => "100vh",
                        "fill" => "100%",
                        other => other,
                    };
                    css.push_str(&format!("  min-height: {css_value};\n"));
                }
            }
            _ => {}
        }
    }
}
pub(crate) fn emit_common(
    css: &mut String,
    body: &[Node],
    symbols: &frame_core::symbols::SymbolIndex,
) {
    let mut transforms = Vec::new();
    let mut filters = Vec::new();

    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("surface") => {
                if statement.words.get(1).map(String::as_str) == Some("gradient") {
                    if let Some(name) = statement.words.get(2) {
                        css.push_str(&format!("  background: var(--frame-gradient-{name});\n"));
                    }
                } else if let Some(name) = statement.words.get(1) {
                    if symbols.gradients.contains_key(name) {
                        css.push_str(&format!("  background: var(--frame-gradient-{name});\n"));
                    } else if symbols.colors.contains_key(name) {
                        css.push_str(&format!("  background: var(--frame-color-{name});\n"));
                    } else {
                        css.push_str(&format!("  background: var(--frame-surface-{name});\n"));
                    }
                }
            }
            Some("background") => {
                if let Some(name) = statement.words.get(1) {
                    if symbols.gradients.contains_key(name) {
                        css.push_str(&format!("  background: var(--frame-gradient-{name});\n"));
                    } else if surface_value(name) {
                        css.push_str(&format!("  background: var(--frame-surface-{name});\n"));
                    } else {
                        css.push_str(&format!("  background: var(--frame-color-{name});\n"));
                    }
                }
            }
            Some("padding") => emit_box_space_property(css, "padding", statement),
            Some("margin") => emit_box_space_property(css, "margin", statement),
            Some("display") => emit_display(css, statement),
            Some("visibility") => emit_visibility(css, statement),
            Some("flex") => emit_flex(css, statement),
            Some("gap") => emit_space_property(css, "gap", statement),
            Some("opacity") => emit_opacity(css, statement),
            Some("shadow") => emit_shadow(css, statement),
            Some("radius") => emit_radius(css, statement),
            Some("border") => emit_border(css, statement),
            Some("outline") => emit_outline(css, statement),
            Some("layout") => emit_layout(css, statement),
            Some("overflow") => emit_overflow(css, statement),
            Some("scroll") => emit_scroll(css, statement),
            Some("scrollbar") => emit_scrollbar(css, statement),
            Some("box") => emit_box(css, statement),
            Some("square") => emit_square(css, statement),
            Some("self") => emit_self(css, statement),
            Some("nudge") => emit_nudge(css, statement),
            Some("height") => emit_size_property(css, "height", statement),
            Some("width") => emit_size_property(css, "width", statement),
            Some("min-height") => emit_size_property(css, "min-height", statement),
            Some("max-height") => emit_size_property(css, "max-height", statement),
            Some("min-width") => emit_size_property(css, "min-width", statement),
            Some("max-width") => emit_size_property(css, "max-width", statement),
            Some("inline-size") => emit_size_property(css, "inline-size", statement),
            Some("block-size") => emit_size_property(css, "block-size", statement),
            Some("min-inline-size") => emit_size_property(css, "min-inline-size", statement),
            Some("max-inline-size") => emit_size_property(css, "max-inline-size", statement),
            Some("min-block-size") => emit_size_property(css, "min-block-size", statement),
            Some("max-block-size") => emit_size_property(css, "max-block-size", statement),
            Some("align") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  align-items: {};\n", css_alignment(value)));
                }
            }
            Some("justify") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  justify-content: {};\n", css_justify(value)));
                }
            }
            Some("position") => emit_position(css, statement),
            Some("anchor") => emit_anchor(css, statement),
            Some("offset") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  inset: var(--frame-space-{value});\n"));
                }
            }
            Some("z") => emit_z(css, statement),
            Some("transition") => emit_transition(css, statement),
            Some("duration") => emit_duration(css, statement),
            Some("ease") => emit_ease(css, statement),
            Some("animation" | "animate") => emit_animation(css, statement),
            Some("lift" | "sink" | "shift" | "grow" | "shrink" | "tilt" | "press" | "pop") => {
                collect_effect(css, statement, &mut transforms, &mut filters)
            }
            Some("text" | "color") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  color: var(--frame-color-{value});\n"));
                }
            }
            Some("theme") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  color: var(--frame-color-{value});\n"));
                    css.push_str(&format!("  border-color: var(--frame-color-{value});\n"));
                }
            }
            Some("font") => {
                if statement.words.get(1).map(String::as_str) == Some("mono") {
                    css.push_str(
                        "  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;\n",
                    );
                }
            }
            Some("truncate") => {
                css.push_str(
                    "  white-space: nowrap;\n  overflow: hidden;\n  text-overflow: ellipsis;\n",
                );
            }
            Some("wrap") => emit_wrap(css, statement),
            Some("case") => emit_text_case(css, statement),
            Some("align-text") => emit_text_align(css, statement),
            Some("decoration") => emit_text_decoration(css, statement),
            Some("whitespace") => emit_white_space(css, statement),
            Some("word-break") => emit_word_break(css, statement),
            Some("hyphenate") => emit_hyphenate(css, statement),
            Some("size") => emit_type_size(css, statement),
            Some("weight") => emit_weight(css, statement),
            Some("line") => emit_line(css, statement),
            Some("letter") => emit_letter(css, statement),
            Some("control") => emit_control(css, statement),
            Some("interactive") => css.push_str("  cursor: pointer;\n"),
            Some("css") => emit_advanced_css(css, statement),
            _ => {}
        }
    }

    emit_collected_effects(css, &transforms, &filters);

    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if block.name != "advanced" {
            if let Some(animation_name) = block.name.strip_prefix("animation ") {
                emit_animation_block(css, animation_name, &block.body);
            }
            continue;
        }
        for statement in statements(&block.body) {
            if statement.words.first().map(String::as_str) == Some("css") {
                emit_advanced_css(css, statement);
            }
        }
    }
}
pub(crate) fn emit_animation_block(css: &mut String, name: &str, body: &[Node]) {
    let mut duration = "240ms".to_string();
    let mut ease = "ease".to_string();
    let mut delay = "0ms".to_string();
    let mut iteration = "1".to_string();
    let mut direction = "normal".to_string();
    let mut fill = "both".to_string();
    let mut play_state = "running".to_string();

    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("duration") => {
                duration = animation_duration(statement.words.get(1).map(String::as_str));
            }
            Some("delay") => {
                delay = animation_duration(statement.words.get(1).map(String::as_str));
            }
            Some("ease") => {
                ease = animation_ease(statement.words.get(1).map(String::as_str));
            }
            Some("iteration") => {
                iteration = statement
                    .words
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "1".to_string());
            }
            Some("direction") => {
                direction = statement
                    .words
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "normal".to_string());
            }
            Some("fill") => {
                fill = statement
                    .words
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "both".to_string());
            }
            Some("play-state") => {
                play_state = statement
                    .words
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "running".to_string());
            }
            _ => {}
        }
    }

    css.push_str(&format!(
        "  animation: frame-{name} {duration} {ease} {delay} {iteration} {direction} {fill};\n"
    ));
    css.push_str(&format!("  animation-play-state: {play_state};\n"));
}
pub(crate) fn emit_condition_blocks(
    css: &mut String,
    class_name: &str,
    kind: DeclarationKind,
    body: &[Node],
) {
    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        let Some(rule) = condition_rule(&block.name) else {
            continue;
        };
        css.push_str(&format!("{rule} {{\n  .{class_name} {{\n"));
        emit_conditional_body(css, &kind, &block.body);
        css.push_str("  }\n}\n\n");
    }
}
pub(crate) fn emit_conditional_body(css: &mut String, kind: &DeclarationKind, body: &[Node]) {
    match kind {
        DeclarationKind::Grid => emit_grid(css, body),
        DeclarationKind::Area => {
            emit_common(css, body, &frame_core::symbols::SymbolIndex::default());
            if let Some(value) = find_statement_value(body, "place") {
                css.push_str(&format!("    grid-area: {value};\n"));
            }
            if let Some(value) = find_statement_value(body, "col") {
                css.push_str(&format!("    grid-column: {value};\n"));
            }
            if let Some(value) = find_statement_value(body, "row") {
                css.push_str(&format!("    grid-row: {value};\n"));
            }
        }
        _ => emit_common(css, body, &frame_core::symbols::SymbolIndex::default()),
    }
}
pub(crate) fn emit_advanced_css(css: &mut String, statement: &Statement) {
    let Some(property) = statement.words.get(1) else {
        return;
    };
    let property = property.trim_matches('"');
    let value = statement
        .words
        .iter()
        .skip(2)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if !property.is_empty() && !value.is_empty() {
        css.push_str(&format!("  {property}: {value};\n"));
    }
}
pub(crate) fn emit_effects(css: &mut String, body: &[Node]) {
    let mut transforms = Vec::new();
    let mut filters = Vec::new();

    for statement in statements(body) {
        collect_effect(css, statement, &mut transforms, &mut filters);
    }

    emit_collected_effects(css, &transforms, &filters);
}
pub(crate) fn collect_effect(
    css: &mut String,
    statement: &Statement,
    transforms: &mut Vec<String>,
    filters: &mut Vec<&'static str>,
) {
    match statement.words.first().map(String::as_str) {
        Some("lift") => transforms.push(format!(
            "translateY(-{})",
            format_px(tuned_value(
                statement.words.get(1).map(String::as_str),
                &MOVEMENT_SCALE
            ))
        )),
        Some("sink") => transforms.push(format!(
            "translateY({})",
            format_px(tuned_value(
                statement.words.get(1).map(String::as_str),
                &MOVEMENT_SCALE
            ))
        )),
        Some("shift") => {
            let amount = format_px(tuned_value(
                statement.words.get(2).map(String::as_str),
                &MOVEMENT_SCALE,
            ));
            match statement.words.get(1).map(String::as_str) {
                Some("left") => transforms.push(format!("translateX(-{amount})")),
                Some("right") => transforms.push(format!("translateX({amount})")),
                Some("up") => transforms.push(format!("translateY(-{amount})")),
                Some("down") => transforms.push(format!("translateY({amount})")),
                _ => {}
            }
        }
        Some("grow") => transforms.push(format!(
            "scale({})",
            format_number(tuned_value(
                statement.words.get(1).map(String::as_str),
                &GROW_SCALE
            ))
        )),
        Some("shrink") => transforms.push(format!(
            "scale({})",
            format_number(tuned_value(
                statement.words.get(1).map(String::as_str),
                &SHRINK_SCALE
            ))
        )),
        Some("tilt") => {
            let degrees = tuned_value(statement.words.get(2).map(String::as_str), &TILT_SCALE);
            match statement.words.get(1).map(String::as_str) {
                Some("left") => transforms.push(format!("rotate(-{})", format_deg(degrees))),
                Some("right") => transforms.push(format!("rotate({})", format_deg(degrees))),
                _ => {}
            }
        }
        Some("glow") => {
            let value = statement
                .words
                .get(1)
                .map(String::as_str)
                .unwrap_or("accent");
            if value == "none" {
                css.push_str("  box-shadow: none;\n");
            } else {
                css.push_str(&format!("  box-shadow: var(--frame-glow-{value});\n"));
            }
        }
        Some("brighten") => filters.push(match statement.words.get(1).map(String::as_str) {
            Some("subtle") => "brightness(1.04)",
            Some("large") => "brightness(1.12)",
            _ => "brightness(1.08)",
        }),
        Some("dim") => filters.push("brightness(0.92)"),
        Some("press") => transforms.push("translateY(1px)".to_string()),
        Some("pop") => transforms.push("scale(1.04)".to_string()),
        Some("blur") => filters.push(match statement.words.get(1).map(String::as_str) {
            Some("heavy") => "blur(8px)",
            Some("none") => "blur(0)",
            _ => "blur(2px)",
        }),
        Some("ring") => {
            let value = statement
                .words
                .get(1)
                .map(String::as_str)
                .unwrap_or("accent");
            css.push_str(&format!(
                "  outline: 2px solid var(--frame-color-{value});\n  outline-offset: 2px;\n"
            ))
        }
        Some("fade") => css.push_str("  opacity: 0.72;\n"),
        Some("scale") => transforms.push("scale(1.02)".to_string()),
        Some("transition") => emit_transition(css, statement),
        Some("duration") => emit_duration(css, statement),
        Some("ease") => emit_ease(css, statement),
        Some("animation" | "animate") => emit_animation(css, statement),
        _ => {}
    }
}
pub(crate) fn emit_collected_effects(css: &mut String, transforms: &[String], filters: &[&str]) {
    if !transforms.is_empty() {
        css.push_str(&format!("  transform: {};\n", transforms.join(" ")));
    }

    if !filters.is_empty() {
        css.push_str(&format!("  filter: {};\n", filters.join(" ")));
    }
}
pub(crate) const MOVEMENT_SCALE: [(&str, f32); 5] = [
    ("tiny", 1.0),
    ("small", 4.0),
    ("medium", 8.0),
    ("large", 12.0),
    ("huge", 16.0),
];
pub(crate) const GROW_SCALE: [(&str, f32); 5] = [
    ("slight", 1.02),
    ("subtle", 1.04),
    ("normal", 1.06),
    ("strong", 1.10),
    ("dramatic", 1.16),
];
pub(crate) const SHRINK_SCALE: [(&str, f32); 5] = [
    ("slight", 0.98),
    ("subtle", 0.96),
    ("normal", 0.94),
    ("strong", 0.90),
    ("dramatic", 0.84),
];
pub(crate) const TILT_SCALE: [(&str, f32); 5] = [
    ("slight", 0.5),
    ("subtle", 1.0),
    ("normal", 2.0),
    ("strong", 4.0),
    ("dramatic", 8.0),
];
