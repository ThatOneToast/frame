use std::collections::{HashMap, HashSet};

use crate::{
    knowledge, tokens, Declaration, DeclarationKind, Diagnostic, Document, Node, Statement,
};

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
        let grid_names = grids.keys().map(String::as_str).collect::<Vec<_>>();
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
        if !grid_places.is_empty() && !grid_places.contains(place) {
            let mut known = grid_places.iter().cloned().collect::<Vec<_>>();
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
    let Some(keyword) = first_word(statement) else {
        return;
    };

    if !knowledge::property_keywords().contains(&keyword) {
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
        Some("padding" | "gap" | "margin") => {
            validate_value(statement, tokens::SPACING, diagnostics)
        }
        Some("radius") => validate_value(statement, tokens::RADII, diagnostics),
        Some("surface") => validate_surface(statement, diagnostics),
        Some("shadow") => validate_value(statement, tokens::SHADOWS, diagnostics),
        Some("height" | "width" | "min-height" | "max-height" | "min-width" | "max-width") => {
            validate_size_value(statement, diagnostics)
        }
        Some("align") => validate_value(statement, tokens::ALIGN, diagnostics),
        Some("justify") => validate_value(statement, tokens::JUSTIFY, diagnostics),
        Some("position") => validate_value(statement, tokens::POSITIONS, diagnostics),
        Some("theme" | "color" | "text") => validate_value(statement, tokens::COLORS, diagnostics),
        Some("background") => validate_background(statement, diagnostics),
        Some("columns") => validate_grid_columns(statement, diagnostics),
        _ => {}
    }
}

fn validate_effect_statement(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
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
    }
}

fn validate_surface(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error("surface expects a value", statement.span));
        return;
    };

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

        if !tokens::COLORS.contains(&gradient.as_str()) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown gradient `{gradient}`.\n\nUse named gradients like `dusk`, `midnight`, or `aurora`."
                ),
                statement.span,
            ));
        }
    }
}

fn validate_background(statement: &Statement, diagnostics: &mut Vec<Diagnostic>) {
    let Some(value) = statement.words.get(1) else {
        diagnostics.push(Diagnostic::error(
            "background expects a value",
            statement.span,
        ));
        return;
    };

    if tokens::COLORS.contains(&value.as_str()) || tokens::SURFACES.contains(&value.as_str()) {
        return;
    }

    diagnostics.push(Diagnostic::error(
        format!("invalid background value `{value}`"),
        statement.span,
    ));
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

    #[test]
    fn accepts_percent_size_values() {
        let document = Document {
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![statement(&["width", "50%"]), statement(&["height", "100%"])],
            )],
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_percent_size_values() {
        let document = Document {
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
}
