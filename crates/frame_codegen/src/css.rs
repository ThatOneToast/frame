use frame_core::{symbols::index_document, DeclarationKind, Document, Node, Statement};

pub fn generate_css(document: &Document) -> String {
    let mut css = String::new();
    let symbols = index_document("", document);

    css.push_str(":root {\n");
    css.push_str("  --frame-space-none: 0;\n");
    css.push_str("  --frame-space-small: 0.5rem;\n");
    css.push_str("  --frame-space-medium: 1rem;\n");
    css.push_str("  --frame-space-large: 1.5rem;\n");
    css.push_str("  --frame-space-xlarge: 2rem;\n");
    css.push_str("  --frame-radius-none: 0;\n");
    css.push_str("  --frame-radius-small: 0.375rem;\n");
    css.push_str("  --frame-radius-medium: 0.625rem;\n");
    css.push_str("  --frame-radius-large: 1rem;\n");
    css.push_str("  --frame-radius-xlarge: 1.5rem;\n");
    css.push_str("  --frame-radius-pill: 999px;\n");
    css.push_str("  --frame-radius-full: 999px;\n");
    css.push_str("  --frame-surface-panel: #171717;\n");
    css.push_str("  --frame-surface-main: #101010;\n");
    css.push_str("  --frame-surface-glass: rgba(255, 255, 255, 0.08);\n");
    css.push_str("  --frame-surface-flat: transparent;\n");
    css.push_str("  --frame-surface-raised: #202020;\n");
    css.push_str("  --frame-surface-overlay: rgba(10, 10, 12, 0.92);\n");
    css.push_str("  --frame-surface-inset: #0b0b0f;\n");
    css.push_str("  --frame-surface-sunken: #08080b;\n");
    css.push_str("  --frame-gradient-dusk: linear-gradient(135deg, #22162f, #123047);\n");
    css.push_str("  --frame-gradient-midnight: linear-gradient(135deg, #080b18, #1b2440);\n");
    css.push_str(
        "  --frame-gradient-aurora: linear-gradient(135deg, #164e63, #4c1d95, #166534);\n",
    );
    css.push_str("  --frame-gradient-ember: linear-gradient(135deg, #7f1d1d, #f97316);\n");
    css.push_str("  --frame-gradient-ocean: linear-gradient(135deg, #0f766e, #1d4ed8);\n");
    css.push_str("  --frame-gradient-forest: linear-gradient(135deg, #14532d, #84cc16);\n");
    css.push_str("  --frame-color-main: #f5f5f5;\n");
    css.push_str("  --frame-color-bright: #ffffff;\n");
    css.push_str("  --frame-color-muted: #a3a3a3;\n");
    css.push_str("  --frame-color-accent: #8ab4ff;\n");
    css.push_str("  --frame-color-primary: #93c5fd;\n");
    css.push_str("  --frame-color-secondary: #c4b5fd;\n");
    css.push_str("  --frame-color-danger: #f87171;\n");
    css.push_str("  --frame-color-success: #34d399;\n");
    css.push_str("  --frame-color-warning: #fbbf24;\n");
    css.push_str("  --frame-color-info: #38bdf8;\n");
    css.push_str("  --frame-color-white: #ffffff;\n");
    css.push_str("  --frame-color-black: #000000;\n");
    css.push_str("  --frame-color-gray: #9ca3af;\n");
    css.push_str("  --frame-color-slate: #64748b;\n");
    css.push_str("  --frame-color-red: #ef4444;\n");
    css.push_str("  --frame-color-orange: #fb923c;\n");
    css.push_str("  --frame-color-yellow: #facc15;\n");
    css.push_str("  --frame-color-green: #22c55e;\n");
    css.push_str("  --frame-color-blue: #60a5fa;\n");
    css.push_str("  --frame-color-purple: #a78bfa;\n");
    css.push_str("  --frame-color-pink: #f472b6;\n");
    css.push_str("  --frame-color-cyan: #22d3ee;\n");
    css.push_str("  --frame-color-transparent: transparent;\n");
    css.push_str("  --frame-shadow-none: none;\n");
    css.push_str("  --frame-shadow-soft: 0 4px 16px rgba(0, 0, 0, 0.14);\n");
    css.push_str("  --frame-shadow-small: 0 6px 18px rgba(0, 0, 0, 0.18);\n");
    css.push_str("  --frame-shadow-medium: 0 12px 30px rgba(0, 0, 0, 0.25);\n");
    css.push_str("  --frame-shadow-large: 0 18px 48px rgba(0, 0, 0, 0.32);\n");
    css.push_str("  --frame-shadow-deep: 0 24px 64px rgba(0, 0, 0, 0.42);\n");
    css.push_str("  --frame-shadow-floating: 0 30px 80px rgba(0, 0, 0, 0.48);\n");
    css.push_str("  --frame-glow-none: none;\n");
    css.push_str("  --frame-glow-accent: 0 0 24px rgba(120, 160, 255, 0.35);\n");
    css.push_str("  --frame-glow-danger: 0 0 24px rgba(248, 113, 113, 0.35);\n");
    css.push_str("  --frame-glow-success: 0 0 24px rgba(52, 211, 153, 0.35);\n");
    css.push_str("  --frame-glow-warning: 0 0 24px rgba(251, 191, 36, 0.35);\n");
    css.push_str("  --frame-glow-soft: 0 0 18px rgba(255, 255, 255, 0.16);\n");
    css.push_str("  --frame-glow-strong: 0 0 34px rgba(255, 255, 255, 0.28);\n");
    emit_custom_tokens(&mut css, document);
    css.push_str("}\n\n");

    for declaration in &document.declarations {
        let class_name = format!("fr-{}", declaration.name.text);

        match declaration.kind {
            DeclarationKind::Grid => {
                css.push_str(&format!(".{class_name} {{\n  display: grid;\n"));
                emit_grid(&mut css, &declaration.body);
                emit_common(&mut css, &declaration.body, &symbols);
                css.push_str("}\n\n");
                emit_grid_section_rules(&mut css, &class_name, &declaration.body);
                emit_condition_blocks(
                    &mut css,
                    &class_name,
                    declaration.kind.clone(),
                    &declaration.body,
                );
            }
            DeclarationKind::Area => {
                css.push_str(&format!(".{class_name} {{\n"));
                emit_common(&mut css, &declaration.body, &symbols);
                if let Some(value) = find_statement_value(&declaration.body, "place") {
                    css.push_str(&format!("  grid-area: {value};\n"));
                }
                if let Some(value) = find_statement_value(&declaration.body, "col") {
                    css.push_str(&format!("  grid-column: {value};\n"));
                }
                if let Some(value) = find_statement_value(&declaration.body, "row") {
                    css.push_str(&format!("  grid-row: {value};\n"));
                }
                if let Some(value) = find_statement_value(&declaration.body, "span") {
                    css.push_str(&format!("  grid-column: span {value};\n"));
                }
                css.push_str("}\n\n");
                emit_condition_blocks(
                    &mut css,
                    &class_name,
                    declaration.kind.clone(),
                    &declaration.body,
                );
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
                        css.push_str("  display: flex;\n  flex-direction: row;\n")
                    }
                    DeclarationKind::Center => {
                        css.push_str("  display: grid;\n  place-items: center;\n")
                    }
                    DeclarationKind::Split => {
                        css.push_str("  display: grid;\n  grid-template-columns: minmax(0, auto) minmax(0, 1fr);\n")
                    }
                    DeclarationKind::Overlay => {
                        css.push_str("  position: fixed;\n  inset: 0;\n  display: grid;\n")
                    }
                    DeclarationKind::Dock => {
                        css.push_str("  position: fixed;\n  inset-inline: 0;\n  bottom: 0;\n")
                    }
                    _ => {}
                }
                emit_common(&mut css, &declaration.body, &symbols);
                css.push_str("}\n\n");

                for node in &declaration.body {
                    if let Node::Block(block) = node {
                        let Some(selector) = state_selector(&block.name) else {
                            continue;
                        };
                        css.push_str(&format!(".{class_name}{selector} {{\n"));
                        emit_effects(&mut css, &block.body);
                        css.push_str("}\n\n");
                    }
                }
                emit_condition_blocks(
                    &mut css,
                    &class_name,
                    declaration.kind.clone(),
                    &declaration.body,
                );
            }
            DeclarationKind::Keyframes => {
                emit_custom_keyframes(&mut css, &declaration.name.text, &declaration.body);
            }
            _ => {}
        }
    }

    emit_keyframes(&mut css);
    css
}

fn emit_grid(css: &mut String, body: &[Node]) {
    let vertical = grid_flow(body) == Some("vertical");
    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("columns") => emit_columns(css, statement, vertical),
            Some("rows") => emit_rows(css, statement),
            Some("tracks") => emit_tracks(css, statement),
            Some("areas") => {}
            Some("gap") => emit_space_property(css, "gap", statement),
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

fn emit_grid_section_rules(css: &mut String, class_name: &str, body: &[Node]) {
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

fn grid_flow(body: &[Node]) -> Option<&str> {
    find_statement_value(body, "flow")
}

fn grid_section_names(body: &[Node]) -> Vec<String> {
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
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

fn section_block<'a>(body: &'a [Node], section: &str) -> Option<&'a frame_core::Block> {
    body.iter().find_map(|node| {
        let Node::Block(block) = node else {
            return None;
        };
        (block.name == format!("section {section}")).then_some(block)
    })
}

fn emit_custom_tokens(css: &mut String, document: &Document) {
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

fn emit_custom_keyframes(css: &mut String, name: &str, body: &[Node]) {
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

fn emit_keyframe_statement(css: &mut String, statement: &Statement) {
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

fn is_keyframe_selector(name: &str) -> bool {
    matches!(name, "from" | "to")
        || name
            .strip_suffix('%')
            .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn gradient_css(body: &[Node]) -> Option<String> {
    let mut angle = "180deg".to_string();
    let mut stops = Vec::new();
    let mut corners = Vec::new();

    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("angle") => {
                if let Some(value) = statement.words.get(1) {
                    angle = value.clone();
                }
            }
            Some("stop") => {
                if let (Some(color), Some(position)) =
                    (statement.words.get(1), statement.words.get(2))
                {
                    let color = if color.starts_with('#') {
                        color.clone()
                    } else {
                        format!("var(--frame-color-{color})")
                    };
                    stops.push(format!("{color} {position}"));
                }
            }
            Some("corner") => {
                if let (Some(corner), Some(color)) =
                    (statement.words.get(1), statement.words.get(2))
                {
                    let color = color_css_value(color);
                    let fade = statement.words.get(3).map(String::as_str).unwrap_or("70%");
                    corners.push(format!(
                        "radial-gradient(circle at {}, {color} 0%, transparent {fade})",
                        css_corner(corner)
                    ));
                }
            }
            _ => {}
        }
    }

    if !corners.is_empty() {
        if stops.len() >= 2 {
            corners.push(format!("linear-gradient({angle}, {})", stops.join(", ")));
        }
        return Some(corners.join(", "));
    }

    (stops.len() >= 2).then(|| format!("linear-gradient({angle}, {})", stops.join(", ")))
}

fn color_css_value(color: &str) -> String {
    if color.starts_with('#') {
        color.to_string()
    } else {
        format!("var(--frame-color-{color})")
    }
}

fn css_corner(corner: &str) -> &str {
    match corner {
        "top-left" => "top left",
        "top-right" => "top right",
        "bottom-left" => "bottom left",
        "bottom-right" => "bottom right",
        value => value,
    }
}

fn emit_keyframes(css: &mut String) {
    css.push_str("@keyframes frame-fade-in {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}\n\n");
    css.push_str("@keyframes frame-slide-up {\n  from { opacity: 0; transform: translateY(0.5rem); }\n  to { opacity: 1; transform: translateY(0); }\n}\n\n");
    css.push_str("@keyframes frame-pop-in {\n  from { opacity: 0; transform: scale(0.96); }\n  to { opacity: 1; transform: scale(1); }\n}\n\n");
    css.push_str(
        "@keyframes frame-pulse {\n  0%, 100% { opacity: 1; }\n  50% { opacity: 0.72; }\n}\n\n",
    );
}

fn emit_columns(css: &mut String, statement: &Statement, vertical: bool) {
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

fn emit_rows(css: &mut String, statement: &Statement) {
    let names = &statement.words[1..];
    if !names.is_empty() {
        let rows = names.iter().map(|_| "auto").collect::<Vec<_>>().join(" ");
        css.push_str(&format!("  grid-template-rows: {rows};\n"));
    }
}

fn emit_tracks(css: &mut String, statement: &Statement) {
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

fn emit_area_template(css: &mut String, body: &[Node]) {
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

fn emit_common(css: &mut String, body: &[Node], symbols: &frame_core::symbols::SymbolIndex) {
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
            Some("gap") => emit_space_property(css, "gap", statement),
            Some("radius") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  border-radius: var(--frame-radius-{value});\n"));
                }
            }
            Some("shadow") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  box-shadow: var(--frame-shadow-{value});\n"));
                }
            }
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

fn emit_animation_block(css: &mut String, name: &str, body: &[Node]) {
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

fn animation_duration(value: Option<&str>) -> String {
    match value {
        Some("fast") => "120ms".to_string(),
        Some("normal") => "200ms".to_string(),
        Some("slow") => "360ms".to_string(),
        Some(value) if value.ends_with("ms") || value.ends_with('s') => value.to_string(),
        _ => "240ms".to_string(),
    }
}

fn animation_ease(value: Option<&str>) -> String {
    match value {
        Some("linear") => "linear".to_string(),
        Some("bounce") => "cubic-bezier(.2, 1.4, .4, 1)".to_string(),
        Some("sharp") => "cubic-bezier(.4, 0, 1, 1)".to_string(),
        _ => "ease".to_string(),
    }
}

fn emit_condition_blocks(css: &mut String, class_name: &str, kind: DeclarationKind, body: &[Node]) {
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

fn emit_conditional_body(css: &mut String, kind: &DeclarationKind, body: &[Node]) {
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

fn condition_rule(name: &str) -> Option<String> {
    let words = name.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["below", breakpoint] => Some(format!(
            "@media (max-width: {})",
            breakpoint_max(breakpoint)
        )),
        ["above", breakpoint] => Some(format!(
            "@media (min-width: {})",
            breakpoint_min(breakpoint)
        )),
        ["between", start, end] => Some(format!(
            "@media (min-width: {}) and (max-width: {})",
            breakpoint_min(start),
            breakpoint_max(end)
        )),
        ["container", name] => Some(format!("@container (max-width: {})", container_width(name))),
        _ => None,
    }
}

fn breakpoint_min(name: &str) -> &str {
    match name {
        "mobile" => "0px",
        "tablet" => "768px",
        "desktop" => "1024px",
        "wide" => "1280px",
        _ => "0px",
    }
}

fn breakpoint_max(name: &str) -> &str {
    match name {
        "mobile" => "767px",
        "tablet" => "1023px",
        "desktop" => "1279px",
        "wide" => "1535px",
        _ => "1023px",
    }
}

fn container_width(name: &str) -> &str {
    match name {
        "narrow" => "42rem",
        "content" => "64rem",
        "wide" => "80rem",
        _ => name,
    }
}

fn emit_advanced_css(css: &mut String, statement: &Statement) {
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

fn emit_effects(css: &mut String, body: &[Node]) {
    let mut transforms = Vec::new();
    let mut filters = Vec::new();

    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("lift") => transforms.push(match statement.words.get(1).map(String::as_str) {
                Some("small") => "translateY(-2px)",
                Some("large") => "translateY(-8px)",
                _ => "translateY(-4px)",
            }),
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
            Some("press") => transforms.push("translateY(1px)"),
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
            Some("scale") => transforms.push("scale(1.02)"),
            Some("transition") => emit_transition(css, statement),
            Some("duration") => emit_duration(css, statement),
            Some("ease") => emit_ease(css, statement),
            Some("animation" | "animate") => emit_animation(css, statement),
            _ => {}
        }
    }

    if !transforms.is_empty() {
        css.push_str(&format!("  transform: {};\n", transforms.join(" ")));
    }

    if !filters.is_empty() {
        css.push_str(&format!("  filter: {};\n", filters.join(" ")));
    }
}

fn emit_space_property(css: &mut String, property: &str, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  {property}: var(--frame-space-{value});\n"));
    }
}

fn emit_box_space_property(css: &mut String, property: &str, statement: &Statement) {
    match (
        statement.words.get(1).map(String::as_str),
        statement.words.get(2),
    ) {
        (Some(edge @ ("top" | "right" | "bottom" | "left")), Some(value)) => {
            css.push_str(&format!(
                "  {property}-{edge}: var(--frame-space-{value});\n"
            ));
        }
        (Some("x" | "inline"), Some(value)) => {
            css.push_str(&format!(
                "  {property}-inline: var(--frame-space-{value});\n"
            ));
        }
        (Some("y" | "block"), Some(value)) => {
            css.push_str(&format!(
                "  {property}-block: var(--frame-space-{value});\n"
            ));
        }
        (Some(value), _) => {
            css.push_str(&format!("  {property}: var(--frame-space-{value});\n"));
        }
        _ => {}
    }
}

fn emit_size_property(css: &mut String, property: &str, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let css_value = match value.as_str() {
            "screen" if property.contains("height") => "100vh".to_string(),
            "screen" if property.contains("width") => "100vw".to_string(),
            "fill" => "100%".to_string(),
            "content" => "max-content".to_string(),
            "auto" => "auto".to_string(),
            "sidebar" => "18rem".to_string(),
            "narrow" => "12rem".to_string(),
            "wide" => "32rem".to_string(),
            "zero" if property.starts_with("min-") => "0".to_string(),
            "modal" if property == "width" => "min(42rem, 100%)".to_string(),
            "icon" => "2.5rem".to_string(),
            value if is_percentage(value) => value.to_string(),
            value => format!("var(--frame-space-{value})"),
        };
        css.push_str(&format!("  {property}: {css_value};\n"));
    }
}

fn emit_layout(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("icon-content-action") | Some("composer") => {
            css.push_str("  display: grid;\n");
            css.push_str("  grid-template-columns: auto minmax(0, 1fr) auto;\n");
            css.push_str("  align-items: center;\n");
        }
        Some("avatar-content") => {
            css.push_str("  display: grid;\n");
            css.push_str("  grid-template-columns: 2.5rem minmax(0, 1fr);\n");
        }
        Some("header") => {
            css.push_str("  display: grid;\n");
            css.push_str("  grid-template-columns: 16rem minmax(0, 1fr) auto;\n");
            css.push_str("  align-items: center;\n");
        }
        Some("center") => {
            css.push_str("  display: grid;\n");
            css.push_str("  place-items: center;\n");
        }
        _ => {}
    }
}

fn emit_overflow(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  overflow: {value};\n"));
    }
}

fn emit_scroll(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("x") => css.push_str("  overflow-x: auto;\n"),
        Some("y") => css.push_str("  overflow-y: auto;\n"),
        Some("both") => css.push_str("  overflow: auto;\n"),
        _ => {}
    }
}

fn emit_scrollbar(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("dense") => {
            css.push_str("  scrollbar-width: thin;\n");
            css.push_str("  scrollbar-color: var(--frame-color-muted) transparent;\n");
        }
        Some("normal") => css.push_str("  scrollbar-width: auto;\n"),
        _ => {}
    }
}

fn emit_box(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("border") => css.push_str("  box-sizing: border-box;\n"),
        Some("content") => css.push_str("  box-sizing: content-box;\n"),
        _ => {}
    }
}

fn emit_square(css: &mut String, statement: &Statement) {
    let Some(value) = statement.words.get(1).map(String::as_str) else {
        return;
    };
    let size = match value {
        "server" => "3rem",
        "avatar" | "icon" => "2.5rem",
        "presence" => "0.65rem",
        "unread" => "0.55rem",
        _ => return,
    };
    css.push_str(&format!("  width: {size};\n  height: {size};\n"));
}

fn emit_self(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!(
            "  justify-self: {value};\n  align-self: {value};\n"
        ));
    }
}

fn emit_nudge(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("top-right") => css.push_str("  top: -0.1rem;\n  right: -0.1rem;\n"),
        _ => {}
    }
}

fn emit_wrap(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("anywhere") => css.push_str("  overflow-wrap: anywhere;\n"),
        Some("normal") => css.push_str("  overflow-wrap: normal;\n"),
        _ => {}
    }
}

fn emit_text_case(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("uppercase") => css.push_str("  text-transform: uppercase;\n"),
        Some("normal") => css.push_str("  text-transform: none;\n"),
        _ => {}
    }
}

fn emit_text_align(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  text-align: {value};\n"));
    }
}

fn emit_line(css: &mut String, statement: &Statement) {
    let Some(value) = statement.words.get(1).map(String::as_str) else {
        return;
    };
    let line_height = match value {
        "relaxed" => "1.45",
        "tight" => "1.15",
        "normal" => "1.3",
        _ => return,
    };
    css.push_str(&format!("  line-height: {line_height};\n"));
}

fn emit_letter(css: &mut String, statement: &Statement) {
    if statement.words.get(1).map(String::as_str) == Some("normal") {
        css.push_str("  letter-spacing: 0;\n");
    }
}

fn emit_control(css: &mut String, statement: &Statement) {
    if statement.words.get(1).map(String::as_str) == Some("reset") {
        css.push_str("  appearance: none;\n");
    }
}

fn column_css_value(value: &str) -> &str {
    match value {
        value if is_percentage(value) => value,
        "auto" => "auto",
        "fill" => "minmax(0, 1fr)",
        _ => "minmax(0, 1fr)",
    }
}

fn track_css_value(value: &str) -> String {
    match value {
        "rail" => "4.5rem".to_string(),
        "panel" => "18rem".to_string(),
        "side" => "16rem".to_string(),
        "header" => "3.25rem".to_string(),
        "composer" => "4.75rem".to_string(),
        "fill" => "minmax(0, 1fr)".to_string(),
        "auto" => "auto".to_string(),
        "content" => "max-content".to_string(),
        value if is_percentage(value) => value.to_string(),
        _ => "minmax(0, 1fr)".to_string(),
    }
}

fn is_percentage(value: &str) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn is_identifier_grid_name(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

fn surface_value(value: &str) -> bool {
    matches!(
        value,
        "panel" | "main" | "glass" | "flat" | "raised" | "overlay" | "inset" | "sunken"
    )
}

fn emit_border(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("none") => css.push_str("  border: 0;\n"),
        Some(edge @ ("top" | "right" | "bottom" | "left")) => {
            let value = statement.words.get(2).map(String::as_str).unwrap_or("soft");
            match value {
                "soft" => {
                    css.push_str(&format!(
                        "  border-{edge}: 1px solid rgba(255, 255, 255, 0.14);\n"
                    ));
                }
                "strong" => {
                    css.push_str(&format!(
                        "  border-{edge}: 1px solid rgba(255, 255, 255, 0.32);\n"
                    ));
                }
                "none" => css.push_str(&format!("  border-{edge}: 0;\n")),
                value => css.push_str(&format!(
                    "  border-{edge}: 1px solid var(--frame-color-{value});\n"
                )),
            }
        }
        Some("radius") => {
            let value = statement
                .words
                .get(2)
                .map(String::as_str)
                .unwrap_or("medium");
            css.push_str(&format!("  border-radius: var(--frame-radius-{value});\n"));
        }
        Some("width") => {
            let width = statement
                .words
                .get(2)
                .map(String::as_str)
                .unwrap_or("small");
            let value = match width {
                "medium" => "2px",
                "large" => "3px",
                _ => "1px",
            };
            css.push_str(&format!(
                "  border-width: {value};\n  border-style: solid;\n"
            ));
        }
        Some("accent" | "muted" | "danger" | "success" | "warning") => {
            let value = statement.words[1].as_str();
            css.push_str(&format!(
                "  border: 1px solid var(--frame-color-{value});\n"
            ));
        }
        Some("soft") => css.push_str("  border: 1px solid rgba(255, 255, 255, 0.14);\n"),
        Some("strong") => css.push_str("  border: 1px solid rgba(255, 255, 255, 0.32);\n"),
        Some(value) => css.push_str(&format!(
            "  border: 1px solid var(--frame-color-{value});\n"
        )),
        _ => {}
    }
}

fn emit_outline(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        if value == "none" {
            css.push_str("  outline: 0;\n");
        } else {
            css.push_str(&format!(
                "  outline: 2px solid var(--frame-color-{value});\n"
            ));
        }
    }
}

fn emit_transition(css: &mut String, statement: &Statement) {
    let value = statement
        .words
        .get(1)
        .map(String::as_str)
        .unwrap_or("smooth");
    match value {
        "none" => css.push_str("  transition: none;\n"),
        "fast" => css.push_str("  transition: all 120ms ease-out;\n"),
        "slow" => css.push_str("  transition: all 360ms ease;\n"),
        _ => css.push_str("  transition: all 200ms ease;\n"),
    }
}

fn emit_duration(css: &mut String, statement: &Statement) {
    let value = statement
        .words
        .get(1)
        .map(String::as_str)
        .unwrap_or("normal");
    let duration = match value {
        "fast" => "120ms",
        "slow" => "360ms",
        _ => "200ms",
    };
    css.push_str(&format!("  transition-duration: {duration};\n"));
}

fn emit_ease(css: &mut String, statement: &Statement) {
    let value = statement
        .words
        .get(1)
        .map(String::as_str)
        .unwrap_or("smooth");
    let ease = match value {
        "linear" => "linear",
        "bounce" => "cubic-bezier(.2, 1.4, .4, 1)",
        "sharp" => "cubic-bezier(.4, 0, 1, 1)",
        _ => "ease",
    };
    css.push_str(&format!("  transition-timing-function: {ease};\n"));
}

fn emit_animation(css: &mut String, statement: &Statement) {
    let value = statement.words.get(1).map(String::as_str).unwrap_or("none");
    if value == "none" {
        css.push_str("  animation: none;\n");
    } else {
        css.push_str(&format!("  animation: frame-{value} 240ms ease both;\n"));
    }
}

fn emit_position(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("relative") => css.push_str("  position: relative;\n"),
        Some("absolute") => {
            css.push_str("  position: absolute;\n");
            emit_position_edge(css, statement.words.get(2).map(String::as_str));
        }
        Some("sticky") => {
            css.push_str("  position: sticky;\n");
            emit_position_edge(
                css,
                statement.words.get(2).map(String::as_str).or(Some("top")),
            );
        }
        Some("fixed") => {
            css.push_str("  position: fixed;\n");
            emit_position_edge(
                css,
                statement.words.get(2).map(String::as_str).or(Some("top")),
            );
        }
        Some("center") => css.push_str("  place-items: center;\n"),
        _ => {}
    }
}

fn emit_anchor(css: &mut String, statement: &Statement) {
    let value = statement.words.get(1).map(String::as_str).unwrap_or("top");
    css.push_str("  position: sticky;\n");
    match value {
        "top" => css.push_str("  top: 0;\n"),
        "bottom" => css.push_str("  bottom: 0;\n"),
        "left" => css.push_str("  left: 0;\n"),
        "right" => css.push_str("  right: 0;\n"),
        "top-left" => css.push_str("  top: 0;\n  left: 0;\n"),
        "top-right" => css.push_str("  top: 0;\n  right: 0;\n"),
        "bottom-left" => css.push_str("  bottom: 0;\n  left: 0;\n"),
        "bottom-right" => css.push_str("  bottom: 0;\n  right: 0;\n"),
        _ => css.push_str("  top: 0;\n"),
    }
}

fn emit_position_edge(css: &mut String, edge: Option<&str>) {
    match edge {
        Some("top") => css.push_str("  top: 0;\n"),
        Some("bottom") => css.push_str("  bottom: 0;\n"),
        Some("top-left") => css.push_str("  top: 0;\n  left: 0;\n"),
        Some("top-right") => css.push_str("  top: 0;\n  right: 0;\n"),
        Some("bottom-left") => css.push_str("  bottom: 0;\n  left: 0;\n"),
        Some("bottom-right") => css.push_str("  bottom: 0;\n  right: 0;\n"),
        _ => {}
    }
}

fn emit_z(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let z = match value.as_str() {
            "base" => 0,
            "above" => 10,
            "dropdown" => 40,
            "sticky" => 45,
            "overlay" => 50,
            "modal" => 100,
            "toast" => 110,
            _ => 1,
        };
        css.push_str(&format!("  z-index: {z};\n"));
    }
}

fn emit_type_size(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let size = match value.as_str() {
            "heading" => "2rem",
            "caption" => "0.875rem",
            "body" => "1rem",
            _ => "1rem",
        };
        css.push_str(&format!("  font-size: {size};\n"));
    }
}

fn emit_weight(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let weight = match value.as_str() {
            "thin" => 300,
            "normal" => 400,
            "semibold" => 600,
            "bold" => 700,
            _ => 400,
        };
        css.push_str(&format!("  font-weight: {weight};\n"));
    }
}

fn css_alignment(value: &str) -> &str {
    match value {
        "start" => "flex-start",
        "end" => "flex-end",
        value => value,
    }
}

fn css_justify(value: &str) -> &str {
    match value {
        "start" => "flex-start",
        "end" => "flex-end",
        "between" => "space-between",
        "around" => "space-around",
        "evenly" => "space-evenly",
        value => value,
    }
}

fn state_selector(name: &str) -> Option<&'static str> {
    match name {
        "hover" => Some(":hover"),
        "focus" => Some(":focus-visible"),
        "active" => Some(":active"),
        "disabled" => Some(":disabled"),
        _ => None,
    }
}

fn find_statement_value<'a>(body: &'a [Node], keyword: &str) -> Option<&'a str> {
    statements(body)
        .find(|statement| statement.words.first().map(String::as_str) == Some(keyword))
        .and_then(|statement| statement.words.get(1))
        .map(String::as_str)
}

fn statements(body: &[Node]) -> impl Iterator<Item = &Statement> {
    body.iter().filter_map(|node| {
        if let Node::Statement(statement) = node {
            Some(statement)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_core::{Declaration, Identifier, Span};

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

    #[test]
    fn generates_named_grid_columns_and_area_placement() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![statement(&["columns", "sidebar", "content", "inspector"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Sidebar",
                    vec![statement(&["place", "sidebar"])],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("grid-template-areas: \"sidebar content inspector\";"));
        assert!(css.contains("grid-area: sidebar;"));
    }

    #[test]
    fn grids_emit_common_and_advanced_properties() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "AppShell",
                vec![
                    statement(&["columns", "sidebar", "content"]),
                    statement(&["background", "panel"]),
                    Node::Block(frame_core::Block {
                        name: "advanced".to_string(),
                        body: vec![statement(&[
                            "css",
                            "\"grid-template-areas\"",
                            "\"header",
                            "header\"",
                            "\"sidebar",
                            "content\"",
                        ])],
                        span: Span::default(),
                    }),
                ],
            )],
        };

        let css = generate_css(&document);

        assert!(css.contains("background: var(--frame-surface-panel);"));
        assert!(css.contains("grid-template-areas: \"header header\" \"sidebar content\";"));
    }

    #[test]
    fn generates_app_driven_layout_vocabulary() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![
                        statement(&["columns", "header", "sidebar", "content", "users"]),
                        statement(&["tracks", "columns", "rail", "panel", "fill", "side"]),
                        statement(&["tracks", "rows", "header", "fill", "composer"]),
                        statement(&["areas", "header", "header", "header", "header"]),
                        statement(&["areas", "sidebar", "channels", "chat", "users"]),
                        statement(&["areas", "composer", "composer", "composer", "composer"]),
                        statement(&["overflow", "hidden"]),
                        statement(&["box", "border"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Button,
                    "ChannelButton",
                    vec![
                        statement(&["layout", "icon-content-action"]),
                        statement(&["gap", "small"]),
                        statement(&["control", "reset"]),
                        statement(&["interactive"]),
                        statement(&["align-text", "left"]),
                        statement(&["width", "fill"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Text,
                    "ChannelName",
                    vec![statement(&["truncate"])],
                ),
                declaration(
                    DeclarationKind::Text,
                    "MessageText",
                    vec![
                        statement(&["margin", "none"]),
                        statement(&["wrap", "anywhere"]),
                        statement(&["line", "relaxed"]),
                        statement(&["letter", "normal"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Center,
                    "PresenceDot",
                    vec![statement(&["square", "presence"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Panel",
                    vec![
                        statement(&["border", "right", "accent"]),
                        statement(&["scroll", "y"]),
                        statement(&["scrollbar", "dense"]),
                    ],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("grid-template-columns: 4.5rem 18rem minmax(0, 1fr) 16rem;"));
        assert!(css.contains("grid-template-rows: 3.25rem minmax(0, 1fr) 4.75rem;"));
        assert!(css.contains(
            "grid-template-areas: \"header header header header\" \"sidebar channels chat users\" \"composer composer composer composer\";"
        ));
        assert!(css.contains("grid-template-columns: auto minmax(0, 1fr) auto;"));
        assert!(css.contains("appearance: none;"));
        assert!(css.contains("cursor: pointer;"));
        assert!(css.contains("text-align: left;"));
        assert!(css.contains("white-space: nowrap;"));
        assert!(css.contains("overflow-wrap: anywhere;"));
        assert!(css.contains("line-height: 1.45;"));
        assert!(css.contains("letter-spacing: 0;"));
        assert!(css.contains("width: 0.65rem;\n  height: 0.65rem;"));
        assert!(css.contains("border-right: 1px solid var(--frame-color-accent);"));
        assert!(css.contains("overflow-y: auto;"));
        assert!(css.contains("scrollbar-width: thin;"));
    }

    #[test]
    fn generates_responsive_card_grid_and_hover_effects() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "QuickLinks",
                    vec![statement(&["columns", "responsive", "cards"])],
                ),
                declaration(
                    DeclarationKind::Card,
                    "QuickLinkCard",
                    vec![Node::Block(frame_core::Block {
                        name: "hover".to_string(),
                        body: vec![
                            statement(&["lift", "small"]),
                            statement(&["glow", "accent"]),
                            statement(&["brighten", "subtle"]),
                        ],
                        span: Span::default(),
                    })],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("repeat(auto-fit, minmax(220px, 1fr))"));
        assert!(css.contains(".fr-QuickLinkCard:hover"));
        assert!(css.contains("transform: translateY(-2px);"));
        assert!(css.contains("filter: brightness(1.04);"));
    }

    #[test]
    fn generates_expanded_layout_and_type_concepts() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Center,
                    "EmptyState",
                    vec![
                        statement(&["height", "screen"]),
                        statement(&["surface", "glass"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Row,
                    "Toolbar",
                    vec![
                        statement(&["align", "center"]),
                        statement(&["justify", "between"]),
                        statement(&["border", "accent"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Text,
                    "PageTitle",
                    vec![
                        statement(&["size", "heading"]),
                        statement(&["weight", "bold"]),
                        statement(&["color", "bright"]),
                    ],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains(".fr-EmptyState"));
        assert!(css.contains("place-items: center;"));
        assert!(css.contains("height: 100vh;"));
        assert!(css.contains("justify-content: space-between;"));
        assert!(css.contains("border: 1px solid var(--frame-color-accent);"));
        assert!(css.contains("font-size: 2rem;"));
        assert!(css.contains("font-weight: 700;"));
    }

    #[test]
    fn generates_percentage_columns_and_sizes() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "Dashboard",
                    vec![statement(&["columns", "25%", "50%", "25%"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Sidebar",
                    vec![statement(&["width", "25%"]), statement(&["height", "100%"])],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("grid-template-columns: 25% 50% 25%;"));
        assert!(!css.contains("grid-template-areas: \"25% 50% 25%\";"));
        assert!(css.contains("width: 25%;"));
        assert!(css.contains("height: 100%;"));
    }

    #[test]
    fn generates_vertical_grid_flow_and_section_spacing() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "HoverCardInfo",
                vec![
                    statement(&["flow", "vertical"]),
                    statement(&["columns", "title", "description"]),
                    statement(&["gap", "small"]),
                    Node::Block(frame_core::Block {
                        name: "section title".to_string(),
                        body: vec![statement(&["padding", "bottom", "small"])],
                        span: Span::default(),
                    }),
                    Node::Block(frame_core::Block {
                        name: "section description".to_string(),
                        body: vec![statement(&["padding", "top", "none"])],
                        span: Span::default(),
                    }),
                ],
            )],
        };

        let css = generate_css(&document);

        assert!(css.contains("grid-template-columns: minmax(0, 1fr);"));
        assert!(css.contains("grid-template-areas: \"title\" \"description\";"));
        assert!(css.contains(".fr-HoverCardInfo > :nth-child(1)"));
        assert!(css.contains("grid-area: title;"));
        assert!(css.contains("padding-bottom: var(--frame-space-small);"));
        assert!(css.contains("padding-top: var(--frame-space-none);"));
    }

    #[test]
    fn generates_numeric_area_placement() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Area,
                "Sidebar",
                vec![statement(&["col", "1"]), statement(&["row", "2"])],
            )],
        };

        let css = generate_css(&document);

        assert!(css.contains("grid-column: 1;"));
        assert!(css.contains("grid-row: 2;"));
    }

    #[test]
    fn generates_expanded_color_and_surface_tokens() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Status",
                vec![
                    statement(&["surface", "raised"]),
                    statement(&["text", "primary"]),
                    statement(&["background", "danger"]),
                ],
            )],
        };

        let css = generate_css(&document);

        assert!(css.contains("--frame-color-primary"));
        assert!(css.contains("--frame-color-secondary"));
        assert!(css.contains("--frame-color-info"));
        assert!(css.contains("background: var(--frame-color-danger);"));
        assert!(css.contains("color: var(--frame-color-primary);"));
    }

    #[test]
    fn generates_custom_colors_borders_and_animation() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![statement(&["color", "brand", "#7c3aed"])],
                ),
                declaration(
                    DeclarationKind::Card,
                    "BrandCard",
                    vec![
                        statement(&["background", "brand"]),
                        statement(&["border", "brand"]),
                        statement(&["border", "width", "medium"]),
                        statement(&["transition", "smooth"]),
                        statement(&["animation", "fade-in"]),
                    ],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("--frame-color-brand: #7c3aed;"));
        assert!(css.contains("background: var(--frame-color-brand);"));
        assert!(css.contains("border: 1px solid var(--frame-color-brand);"));
        assert!(css.contains("border-width: 2px;"));
        assert!(css.contains("transition: all 200ms ease;"));
        assert!(css.contains("animation: frame-fade-in 240ms ease both;"));
        assert!(css.contains("@keyframes frame-fade-in"));
    }

    #[test]
    fn generates_custom_gradient_tokens_and_advanced_css() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand-purple", "#7c3aed"]),
                        statement(&["color", "brand-bg", "#0f172a"]),
                        Node::Block(frame_core::Block {
                            name: "gradient hero-gradient".to_string(),
                            body: vec![
                                statement(&["type", "linear"]),
                                statement(&["angle", "135deg"]),
                                statement(&["stop", "brand-purple", "0%"]),
                                statement(&["stop", "brand-bg", "100%"]),
                            ],
                            span: Span::default(),
                        }),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "HeroCard",
                    vec![
                        statement(&["background", "hero-gradient"]),
                        Node::Block(frame_core::Block {
                            name: "advanced".to_string(),
                            body: vec![statement(&["css", "\"backdrop-filter\"", "blur(12px)"])],
                            span: Span::default(),
                        }),
                    ],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("--frame-gradient-hero-gradient: linear-gradient(135deg, var(--frame-color-brand-purple) 0%, var(--frame-color-brand-bg) 100%);"));
        assert!(css.contains("background: var(--frame-gradient-hero-gradient);"));
        assert!(css.contains("backdrop-filter: blur(12px);"));
    }

    #[test]
    fn generates_custom_keyframes_and_animation_blocks() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Keyframes,
                    "FloatIn",
                    vec![
                        Node::Block(frame_core::Block {
                            name: "from".to_string(),
                            body: vec![
                                statement(&["opacity", "0"]),
                                statement(&["transform", "translateY(12px)", "scale(0.98)"]),
                            ],
                            span: Span::default(),
                        }),
                        Node::Block(frame_core::Block {
                            name: "to".to_string(),
                            body: vec![
                                statement(&["opacity", "1"]),
                                statement(&["transform", "translateY(0)", "scale(1)"]),
                            ],
                            span: Span::default(),
                        }),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![Node::Block(frame_core::Block {
                        name: "animation FloatIn".to_string(),
                        body: vec![
                            statement(&["duration", "240ms"]),
                            statement(&["ease", "smooth"]),
                            statement(&["fill", "both"]),
                        ],
                        span: Span::default(),
                    })],
                ),
            ],
        };

        let css = generate_css(&document);

        assert!(css.contains("@keyframes frame-FloatIn"));
        assert!(css.contains("from {"));
        assert!(css.contains("opacity: 0;"));
        assert!(css.contains("transform: translateY(12px) scale(0.98);"));
        assert!(css.contains("animation: frame-FloatIn 240ms ease 0ms 1 normal both;"));
    }

    #[test]
    fn generates_responsive_and_container_rules() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "AppShell",
                vec![
                    statement(&["columns", "sidebar", "content", "inspector"]),
                    Node::Block(frame_core::Block {
                        name: "below tablet".to_string(),
                        body: vec![statement(&["columns", "content"])],
                        span: Span::default(),
                    }),
                    Node::Block(frame_core::Block {
                        name: "container narrow".to_string(),
                        body: vec![statement(&["columns", "content"])],
                        span: Span::default(),
                    }),
                ],
            )],
        };

        let css = generate_css(&document);

        assert!(css.contains("@media (max-width: 1023px)"));
        assert!(css.contains("@container (max-width: 42rem)"));
        assert!(css.contains(".fr-AppShell"));
    }

    #[test]
    fn generates_corner_gradient_layers_targeted_padding_and_anchor() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand-purple", "#7c3aed"]),
                        statement(&["color", "brand-panel", "#181820"]),
                        Node::Block(frame_core::Block {
                            name: "gradient four-corners".to_string(),
                            body: vec![
                                statement(&["type", "layered"]),
                                statement(&["corner", "top-left", "brand-purple", "65%"]),
                                statement(&["corner", "bottom-right", "brand-panel", "70%"]),
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

        let css = generate_css(&document);

        assert!(css.contains("radial-gradient(circle at top left"));
        assert!(css.contains("radial-gradient(circle at bottom right"));
        assert!(css.contains("padding-top: var(--frame-space-large);"));
        assert!(css.contains("padding-inline: var(--frame-space-medium);"));
        assert!(css.contains("position: sticky;"));
        assert!(css.contains("top: 0;"));
    }
}
