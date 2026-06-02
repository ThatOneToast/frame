use std::collections::{HashMap, HashSet};

use crate::{tokens, Declaration, DeclarationKind, Diagnostic, Document, Node, Statement};

pub fn validate(document: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut names = HashSet::new();
    let grids = collect_grids(document);

    for declaration in &document.declarations {
        if !names.insert(declaration.name.text.clone()) {
            diagnostics.push(Diagnostic::error(
                format!("duplicate declaration `{}`", declaration.name.text),
                declaration.name.span,
            ));
        }

        if let DeclarationKind::Unknown(kind) = &declaration.kind {
            diagnostics.push(Diagnostic::error(
                format!("unknown declaration kind `{kind}`"),
                declaration.span,
            ));
        }

        validate_statements(declaration, &mut diagnostics);

        if declaration.kind == DeclarationKind::Area {
            validate_area(declaration, &grids, &mut diagnostics);
        }
    }

    diagnostics
}

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

fn validate_area(
    declaration: &Declaration,
    grids: &HashMap<String, HashSet<String>>,
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

    let Some(grid_places) = grids.get(grid_name) else {
        diagnostics.push(Diagnostic::error(
            format!(
                "area `{}` references unknown grid `{grid_name}`",
                declaration.name.text
            ),
            declaration.span,
        ));
        return;
    };

    if let Some(place) = find_statement_value(declaration, "place") {
        if !grid_places.is_empty() && !grid_places.contains(place) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "area `{}` places into unknown grid slot `{place}`",
                    declaration.name.text
                ),
                declaration.span,
            ));
        }
    }
}

fn validate_statements(declaration: &Declaration, diagnostics: &mut Vec<Diagnostic>) {
    for node in &declaration.body {
        match node {
            Node::Statement(statement) => validate_statement(statement, diagnostics),
            Node::Block(block) => {
                for node in &block.body {
                    if let Node::Statement(statement) = node {
                        validate_effect_statement(statement, diagnostics);
                    }
                }
            }
        }
    }
}

fn validate_statement(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    match first_word(statement) {
        Some("padding" | "gap" | "margin") => {
            validate_value(statement, tokens::SPACING, diagnostics)
        }
        Some("radius") => validate_value(statement, tokens::RADII, diagnostics),
        Some("surface") => validate_surface(statement, diagnostics),
        Some("shadow") => validate_value(statement, tokens::SHADOWS, diagnostics),
        Some("height" | "width" | "min-height" | "max-height" | "min-width" | "max-width") => {
            validate_value(statement, tokens::SIZES, diagnostics)
        }
        Some("align") => validate_value(statement, tokens::ALIGN, diagnostics),
        Some("justify") => validate_value(statement, tokens::JUSTIFY, diagnostics),
        Some("position") => validate_value(statement, tokens::POSITIONS, diagnostics),
        Some("theme" | "color" | "background" | "text") => {
            validate_value(statement, tokens::COLORS, diagnostics)
        }
        _ => {}
    }
}

fn validate_effect_statement(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(effect) = first_word(statement) else {
        return;
    };

    if !tokens::EFFECTS.contains(&effect) {
        diagnostics.push(Diagnostic::error(
            format!("unknown effect `{effect}`"),
            statement.span,
        ));
    }
}

fn validate_surface(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error("surface expects a value", statement.span));
        return;
    };

    if !tokens::SURFACES.contains(&value.as_str()) {
        diagnostics.push(Diagnostic::error(
            format!("invalid surface value `{value}`"),
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

        if !tokens::COLORS.contains(&gradient.as_str()) {
            diagnostics.push(Diagnostic::error(
                format!("unknown gradient `{gradient}`"),
                statement.span,
            ));
        }
    }
}

fn validate_value(statement: &Statement, allowed: &[&str], diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            format!("{} expects a value", statement.words[0]),
            statement.span,
        ));
        return;
    };

    if !allowed.contains(&value.as_str()) {
        diagnostics.push(Diagnostic::error(
            format!("invalid {} value `{value}`", statement.words[0]),
            statement.span,
        ));
    }
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

    #[test]
    fn validates_area_grid_references_and_places() {
        let document = Document {
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
}
