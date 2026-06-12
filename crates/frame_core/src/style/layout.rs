//! Semantic app-shell layouts.
//!
//! A `layout` declaration describes regions by intent instead of grid track
//! micromanagement:
//!
//! ```frame
//! layout DashboardShell {
//!   shell {
//!     sidebar left fixed 18rem
//!     main fluid
//!     inspector right clamp(20rem, 28vw, 28rem)
//!   }
//!   gap lg
//!   density comfortable
//!   below tablet { shell stacked }
//! }
//! ```
//!
//! It lowers to a CSS grid with named areas; children attach with source
//! order or `data-frame-section="name"`, exactly like grids.

use crate::{DeclarationKind, Document, Node};

use super::normalize::{normalize_statements, statements};
use super::schema::{ConditionScope, CssDecl, NormalizedStyle, StyleFact};
use super::StyleContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionPosition {
    Left,
    Main,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellRegion {
    pub name: String,
    pub position: RegionPosition,
    /// The CSS track size (`18rem`, `minmax(0, 1fr)`, `clamp(...)`).
    pub track: String,
}

/// Parse one shell region statement:
/// `<name> [left|right] [fixed <size> | fluid | <size-expr>]`.
pub fn parse_region(words: &[String]) -> Option<ShellRegion> {
    let name = words.first()?.clone();
    let mut position = RegionPosition::Main;
    let mut track = "minmax(0, 1fr)".to_string();
    let mut index = 1;
    while index < words.len() {
        match words[index].as_str() {
            "left" => position = RegionPosition::Left,
            "right" => position = RegionPosition::Right,
            "fluid" => track = "minmax(0, 1fr)".to_string(),
            "fixed" => {
                if let Some(size) = words.get(index + 1) {
                    track = size.clone();
                    index += 1;
                }
            }
            other => track = other.to_string(),
        }
        index += 1;
    }
    if position == RegionPosition::Main && track == "minmax(0, 1fr)" && words.len() == 1 {
        track = "minmax(0, 1fr)".to_string();
    }
    Some(ShellRegion {
        name,
        position,
        track,
    })
}

/// Collect shell regions in left → main → right order.
pub fn shell_regions(body: &[Node]) -> Vec<ShellRegion> {
    let Some(shell) = body.iter().find_map(|node| match node {
        Node::Block(block) if block.name == "shell" => Some(block),
        _ => None,
    }) else {
        return Vec::new();
    };

    let mut regions: Vec<ShellRegion> = statements(&shell.body)
        .filter_map(|statement| parse_region(&statement.words))
        .collect();
    regions.sort_by_key(|region| match region.position {
        RegionPosition::Left => 0,
        RegionPosition::Main => 1,
        RegionPosition::Right => 2,
    });
    regions
}

fn density_fact(value: &str) -> Option<StyleFact> {
    let padding = match value {
        "compact" => "var(--frame-space-small)",
        "comfortable" => "var(--frame-space-medium)",
        "spacious" => "var(--frame-space-large)",
        _ => return None,
    };
    Some(StyleFact::single("padding", "padding", padding))
}

/// Lower a layout declaration to a normalized style (a grid with named areas).
pub fn lower_layout(body: &[Node], ctx: &StyleContext) -> NormalizedStyle {
    let regions = shell_regions(body);
    let mut style = NormalizedStyle::default();

    style
        .facts
        .push(StyleFact::single("layout.display", "display", "grid"));
    if !regions.is_empty() {
        let columns = regions
            .iter()
            .map(|region| region.track.clone())
            .collect::<Vec<_>>()
            .join(" ");
        let areas = regions
            .iter()
            .map(|region| region.name.clone())
            .collect::<Vec<_>>()
            .join(" ");
        style.facts.push(StyleFact::new(
            "layout.grid.columns",
            vec![
                CssDecl::new("grid-template-columns", columns),
                CssDecl::new("grid-template-areas", format!("\"{areas}\"")),
            ],
        ));
        style
            .facts
            .push(StyleFact::single("size.min-height", "min-height", "100vh"));
        style.section_names = regions.iter().map(|region| region.name.clone()).collect();
    }

    for statement in statements(body) {
        if statement.words.first().map(String::as_str) == Some("density") {
            if let Some(fact) = statement.words.get(1).and_then(|value| density_fact(value)) {
                style.facts.push(fact);
            }
        }
    }
    // Common statements (gap, padding, background, height, ...) apply as-is.
    style.facts.extend(normalize_statements(body, ctx));

    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if block.name == "shell" {
            continue;
        }
        if super::properties::condition_rule(&block.name, ctx.contract).is_none() {
            continue;
        }
        let mut facts = Vec::new();
        let stacked = statements(&block.body).any(|statement| {
            statement.words.first().map(String::as_str) == Some("shell")
                && statement.words.get(1).map(String::as_str) == Some("stacked")
        });
        if stacked && !regions.is_empty() {
            let areas = regions
                .iter()
                .map(|region| format!("\"{}\"", region.name))
                .collect::<Vec<_>>()
                .join(" ");
            facts.push(StyleFact::new(
                "layout.grid.columns",
                vec![
                    CssDecl::new("grid-template-columns", "minmax(0, 1fr)"),
                    CssDecl::new(
                        "grid-template-rows",
                        regions.iter().map(|_| "auto").collect::<Vec<_>>().join(" "),
                    ),
                    CssDecl::new("grid-template-areas", areas),
                ],
            ));
        }
        facts.extend(normalize_statements(&block.body, ctx));
        style.conditions.push(ConditionScope {
            condition: block.name.clone(),
            facts,
        });
    }

    style
}

/// Names of every layout declaration in the document.
pub fn document_layout_names(document: &Document) -> Vec<String> {
    document
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Layout)
        .map(|declaration| declaration.name.text.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::tokens::default_contract;
    use crate::{Block, Span, Statement};

    fn statement(words: &[&str]) -> Node {
        Node::Statement(Statement {
            words: words.iter().map(|word| word.to_string()).collect(),
            span: Span::default(),
        })
    }

    fn block(name: &str, body: Vec<Node>) -> Node {
        Node::Block(Block {
            name: name.to_string(),
            body,
            span: Span::default(),
        })
    }

    #[test]
    fn shell_regions_order_left_main_right() {
        let body = vec![block(
            "shell",
            vec![
                statement(&["inspector", "right", "clamp(20rem,", "28vw,", "28rem)"]),
                statement(&["main", "fluid"]),
                statement(&["sidebar", "left", "fixed", "18rem"]),
            ],
        )];

        let regions = shell_regions(&body);
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0].name, "sidebar");
        assert_eq!(regions[0].track, "18rem");
        assert_eq!(regions[1].name, "main");
        assert_eq!(regions[1].track, "minmax(0, 1fr)");
        assert_eq!(regions[2].name, "inspector");
    }

    #[test]
    fn lowers_shell_to_grid_with_stacked_condition() {
        let contract = default_contract();
        let ctx = StyleContext::new(&contract);
        let body = vec![
            block(
                "shell",
                vec![
                    statement(&["sidebar", "left", "fixed", "18rem"]),
                    statement(&["main", "fluid"]),
                ],
            ),
            statement(&["gap", "large"]),
            block("below tablet", vec![statement(&["shell", "stacked"])]),
        ];

        let style = lower_layout(&body, &StyleContext::new(ctx.contract));

        let columns = style
            .facts
            .iter()
            .find(|fact| fact.path == "layout.grid.columns")
            .unwrap();
        assert_eq!(columns.decls[0].value, "18rem minmax(0, 1fr)");
        assert_eq!(columns.decls[1].value, "\"sidebar main\"");
        assert_eq!(style.section_names, vec!["sidebar", "main"]);
        assert_eq!(style.conditions.len(), 1);
        let stacked = &style.conditions[0];
        assert_eq!(stacked.facts[0].decls[0].value, "minmax(0, 1fr)");
        assert_eq!(stacked.facts[0].decls[2].value, "\"sidebar\" \"main\"");
    }
}
