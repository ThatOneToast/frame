use frame_core::{DeclarationKind, Document, Node, Statement};

pub fn generate_css(document: &Document) -> String {
    let mut css = String::new();

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
    css.push_str("  --frame-gradient-dusk: linear-gradient(135deg, #22162f, #123047);\n");
    css.push_str("  --frame-gradient-midnight: linear-gradient(135deg, #080b18, #1b2440);\n");
    css.push_str(
        "  --frame-gradient-aurora: linear-gradient(135deg, #164e63, #4c1d95, #166534);\n",
    );
    css.push_str("  --frame-color-main: #f5f5f5;\n");
    css.push_str("  --frame-color-bright: #ffffff;\n");
    css.push_str("  --frame-color-muted: #a3a3a3;\n");
    css.push_str("  --frame-color-accent: #8ab4ff;\n");
    css.push_str("  --frame-color-danger: #f87171;\n");
    css.push_str("  --frame-color-success: #34d399;\n");
    css.push_str("  --frame-color-warning: #fbbf24;\n");
    css.push_str("  --frame-shadow-none: none;\n");
    css.push_str("  --frame-shadow-soft: 0 4px 16px rgba(0, 0, 0, 0.14);\n");
    css.push_str("  --frame-shadow-small: 0 6px 18px rgba(0, 0, 0, 0.18);\n");
    css.push_str("  --frame-shadow-medium: 0 12px 30px rgba(0, 0, 0, 0.25);\n");
    css.push_str("  --frame-shadow-large: 0 18px 48px rgba(0, 0, 0, 0.32);\n");
    css.push_str("  --frame-shadow-deep: 0 24px 64px rgba(0, 0, 0, 0.42);\n");
    css.push_str("  --frame-glow-accent: 0 0 24px rgba(120, 160, 255, 0.35);\n");
    css.push_str("  --frame-glow-danger: 0 0 24px rgba(248, 113, 113, 0.35);\n");
    css.push_str("  --frame-glow-success: 0 0 24px rgba(52, 211, 153, 0.35);\n");
    css.push_str("}\n\n");

    for declaration in &document.declarations {
        let class_name = format!("fr-{}", declaration.name.text);

        match declaration.kind {
            DeclarationKind::Grid => {
                css.push_str(&format!(".{class_name} {{\n  display: grid;\n"));
                emit_grid(&mut css, &declaration.body);
                css.push_str("}\n\n");
            }
            DeclarationKind::Area => {
                css.push_str(&format!(".{class_name} {{\n"));
                emit_common(&mut css, &declaration.body);
                if let Some(value) = find_statement_value(&declaration.body, "place") {
                    css.push_str(&format!("  grid-area: {value};\n"));
                }
                css.push_str("}\n\n");
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
                emit_common(&mut css, &declaration.body);
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
            }
            _ => {}
        }
    }

    css
}

fn emit_grid(css: &mut String, body: &[Node]) {
    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("columns") => emit_columns(css, statement),
            Some("rows") => emit_rows(css, statement),
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
}

fn emit_columns(css: &mut String, statement: &Statement) {
    let names = &statement.words[1..];
    if names == ["responsive", "cards"] {
        css.push_str("  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));\n");
    } else if !names.is_empty() {
        let columns = names
            .iter()
            .map(|_| "minmax(0, 1fr)")
            .collect::<Vec<_>>()
            .join(" ");
        css.push_str(&format!("  grid-template-columns: {columns};\n"));
        css.push_str(&format!(
            "  grid-template-areas: \"{}\";\n",
            names.join(" ")
        ));
    }
}

fn emit_rows(css: &mut String, statement: &Statement) {
    let names = &statement.words[1..];
    if !names.is_empty() {
        let rows = names.iter().map(|_| "auto").collect::<Vec<_>>().join(" ");
        css.push_str(&format!("  grid-template-rows: {rows};\n"));
    }
}

fn emit_common(css: &mut String, body: &[Node]) {
    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("surface") => {
                if statement.words.get(1).map(String::as_str) == Some("gradient") {
                    if let Some(name) = statement.words.get(2) {
                        css.push_str(&format!("  background: var(--frame-gradient-{name});\n"));
                    }
                } else if let Some(name) = statement.words.get(1) {
                    css.push_str(&format!("  background: var(--frame-surface-{name});\n"));
                }
            }
            Some("padding") => emit_space_property(css, "padding", statement),
            Some("margin") => emit_space_property(css, "margin", statement),
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
            Some("offset") => {
                if let Some(value) = statement.words.get(1) {
                    css.push_str(&format!("  inset: var(--frame-space-{value});\n"));
                }
            }
            Some("z") => emit_z(css, statement),
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
            Some("size") => emit_type_size(css, statement),
            Some("weight") => emit_weight(css, statement),
            _ => {}
        }
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

fn emit_size_property(css: &mut String, property: &str, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let css_value = match value.as_str() {
            "screen" if property.contains("height") => "100vh".to_string(),
            "screen" if property.contains("width") => "100vw".to_string(),
            "fill" => "100%".to_string(),
            "content" => "max-content".to_string(),
            "sidebar" => "18rem".to_string(),
            value => format!("var(--frame-space-{value})"),
        };
        css.push_str(&format!("  {property}: {css_value};\n"));
    }
}

fn emit_border(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("none") => css.push_str("  border: 0;\n"),
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
        Some("accent" | "danger" | "success") => {
            let value = statement.words[1].as_str();
            css.push_str(&format!(
                "  border: 1px solid var(--frame-color-{value});\n"
            ));
        }
        Some("soft") => css.push_str("  border: 1px solid rgba(255, 255, 255, 0.14);\n"),
        _ => {}
    }
}

fn emit_position(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("relative") => css.push_str("  position: relative;\n"),
        Some("absolute") => {
            css.push_str("  position: absolute;\n");
            if statement.words.get(2).map(String::as_str) == Some("top-right") {
                css.push_str("  top: 0;\n  right: 0;\n");
            }
        }
        Some("sticky") => {
            css.push_str("  position: sticky;\n");
            if statement.words.get(2).map(String::as_str).unwrap_or("top") == "top" {
                css.push_str("  top: 0;\n");
            }
        }
        Some("center") => css.push_str("  place-items: center;\n"),
        _ => {}
    }
}

fn emit_z(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let z = match value.as_str() {
            "above" => 10,
            "overlay" => 50,
            "modal" => 100,
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
    fn generates_responsive_card_grid_and_hover_effects() {
        let document = Document {
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
}
