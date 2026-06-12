//! Normalized style system.
//!
//! Sits between semantic validation and CSS emission:
//!
//! ```text
//! AST -> semantic validation -> normalized style facts -> CSS backend(s)
//! ```
//!
//! - [`schema`] defines style facts and property-path override merging.
//! - [`tokens`] defines token contracts (the default manifest plus document
//!   `tokens` declarations).
//! - [`theme`] defines scoped themes (`theme dark uses default { ... }`).
//! - [`properties`] owns semantic value policy (what names mean).
//! - [`normalize`] lowers statements into facts.
//! - [`diagnostics`] holds shared did-you-mean helpers.

pub mod diagnostics;
pub mod layout;
pub mod motion;
pub mod normalize;
pub mod properties;
pub mod recipes;
pub mod schema;
pub mod theme;
pub mod tokens;

pub use diagnostics::closest_name;
pub use layout::lower_layout;
pub use motion::{document_motions, expand_motion_references, Motion};
pub use normalize::{normalize_declaration, normalize_statements};
pub use recipes::{document_recipes, Recipe};
pub use schema::{
    merge_facts, ConditionScope, CssDecl, NormalizedStyle, StateScope, StyleFact, FILTER_PART,
    TRANSFORM_PART,
};
pub use theme::{document_themes, Theme};
pub use tokens::{
    breakpoint_below, default_contract, document_contract, token_reference, TokenContract,
    TokenEntry, TokenKind,
};

use crate::Node;

/// Shared context for normalization: the resolved token contract plus the
/// document's semantic motions.
#[derive(Debug, Clone)]
pub struct StyleContext<'a> {
    pub contract: &'a TokenContract,
    pub motions: &'a [Motion],
}

impl<'a> StyleContext<'a> {
    pub fn new(contract: &'a TokenContract) -> Self {
        Self {
            contract,
            motions: &[],
        }
    }

    pub fn with_motions(contract: &'a TokenContract, motions: &'a [Motion]) -> Self {
        Self { contract, motions }
    }
}

/// Lower a `gradient` block body to a CSS gradient value.
pub fn gradient_css(body: &[Node], _contract: &TokenContract) -> Option<String> {
    let mut angle = "180deg".to_string();
    let mut stops = Vec::new();
    let mut corners = Vec::new();

    for statement in normalize::statements(body) {
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
                    stops.push(format!("{} {position}", properties::color_value(color)));
                }
            }
            Some("corner") => {
                if let (Some(corner), Some(color)) =
                    (statement.words.get(1), statement.words.get(2))
                {
                    let color = properties::color_value(color);
                    let fade = statement.words.get(3).map(String::as_str).unwrap_or("70%");
                    corners.push(format!(
                        "radial-gradient(circle at {}, {color} 0%, transparent {fade})",
                        properties::css_corner(corner)
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
