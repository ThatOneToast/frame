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
    css.push_str("  --frame-surface-panel: #171717;\n");
    css.push_str("  --frame-surface-main: #101010;\n");
    css.push_str("  --frame-surface-glass: rgba(255, 255, 255, 0.08);\n");
    css.push_str("  --frame-surface-flat: transparent;\n");
    css.push_str("  --frame-surface-raised: #202020;\n");
    css.push_str("  --frame-gradient-dusk: linear-gradient(135deg, #22162f, #123047);\n");
    css.push_str("  --frame-shadow-none: none;\n");
    css.push_str("  --frame-shadow-small: 0 6px 18px rgba(0, 0, 0, 0.18);\n");
    css.push_str("  --frame-shadow-medium: 0 12px 30px rgba(0, 0, 0, 0.25);\n");
    css.push_str("  --frame-shadow-large: 0 18px 48px rgba(0, 0, 0, 0.32);\n");
    css.push_str("  --frame-glow-accent: 0 0 24px rgba(120, 160, 255, 0.35);\n");
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
            | DeclarationKind::Text => {
                css.push_str(&format!(".{class_name} {{\n"));
                match declaration.kind {
                    DeclarationKind::Card | DeclarationKind::Stack => {
                        css.push_str("  display: flex;\n  flex-direction: column;\n")
                    }
                    DeclarationKind::Row => {
                        css.push_str("  display: flex;\n  flex-direction: row;\n")
                    }
                    _ => {}
                }
                emit_common(&mut css, &declaration.body);
                css.push_str("}\n\n");

                for node in &declaration.body {
                    if let Node::Block(block) = node {
                        if block.name == "hover" {
                            css.push_str(&format!(".{class_name}:hover {{\n"));
                            emit_hover(&mut css, &block.body);
                            css.push_str("}\n\n");
                        }
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
            _ => {}
        }
    }
}

fn emit_hover(css: &mut String, body: &[Node]) {
    let mut transforms = Vec::new();
    let mut filters = Vec::new();

    for statement in statements(body) {
        match statement.words.first().map(String::as_str) {
            Some("lift") => transforms.push(match statement.words.get(1).map(String::as_str) {
                Some("small") => "translateY(-2px)",
                Some("large") => "translateY(-8px)",
                _ => "translateY(-4px)",
            }),
            Some("glow") => css.push_str("  box-shadow: var(--frame-glow-accent);\n"),
            Some("brighten") => filters.push(match statement.words.get(1).map(String::as_str) {
                Some("subtle") => "brightness(1.04)",
                Some("large") => "brightness(1.12)",
                _ => "brightness(1.08)",
            }),
            Some("dim") => filters.push("brightness(0.92)"),
            Some("press") => transforms.push("translateY(1px)"),
            Some("blur") => filters.push("blur(2px)"),
            Some("ring") => {
                css.push_str("  outline: 2px solid currentColor;\n  outline-offset: 2px;\n")
            }
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
}
