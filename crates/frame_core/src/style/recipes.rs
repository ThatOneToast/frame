//! Recipes and variants.
//!
//! A recipe is a first-class component style: a base plus named variant
//! groups, compiled to static classes:
//!
//! ```frame
//! recipe Button {
//!   base {
//!     align center
//!     gap small
//!     radius medium
//!     motion Pressable
//!   }
//!   variant tone {
//!     primary { background token(color.accent) }
//!     ghost { background transparent }
//!   }
//!   variant size {
//!     sm { padding small }
//!     lg { padding large }
//!   }
//! }
//! ```
//!
//! lowers to `.fr-Button`, `.fr-Button--tone-primary`, `.fr-Button--size-sm`,
//! and friends. Variant classes are plain static CSS; runtimes toggle them
//! like any other class.

use crate::{DeclarationKind, Document, Node};

use super::normalize::normalize_declaration;
use super::schema::NormalizedStyle;
use super::StyleContext;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VariantGroup {
    pub name: String,
    pub options: Vec<(String, NormalizedStyle)>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recipe {
    pub name: String,
    pub base: NormalizedStyle,
    pub variants: Vec<VariantGroup>,
}

impl Recipe {
    pub fn base_class(&self) -> String {
        format!("fr-{}", self.name)
    }

    pub fn variant_class(&self, group: &str, option: &str) -> String {
        format!("fr-{}--{group}-{option}", self.name)
    }
}

/// Lower one recipe declaration body.
pub fn lower_recipe(name: &str, body: &[Node], ctx: &StyleContext) -> Recipe {
    let mut recipe = Recipe {
        name: name.to_string(),
        ..Recipe::default()
    };

    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if block.name == "base" {
            recipe.base = normalize_declaration(&DeclarationKind::Recipe, &block.body, ctx);
            continue;
        }
        if let Some(group_name) = block.name.strip_prefix("variant ") {
            let mut group = VariantGroup {
                name: group_name.trim().to_string(),
                options: Vec::new(),
            };
            for option in &block.body {
                let Node::Block(option_block) = option else {
                    continue;
                };
                group.options.push((
                    option_block.name.clone(),
                    normalize_declaration(&DeclarationKind::Recipe, &option_block.body, ctx),
                ));
            }
            recipe.variants.push(group);
        }
    }

    recipe
}

/// Collect every recipe declaration in a document.
pub fn document_recipes(document: &Document, ctx: &StyleContext) -> Vec<Recipe> {
    document
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Recipe)
        .map(|declaration| lower_recipe(&declaration.name.text, &declaration.body, ctx))
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
    fn lowers_base_and_variant_groups() {
        let contract = default_contract();
        let ctx = StyleContext::new(&contract);
        let recipe = lower_recipe(
            "Button",
            &[
                block("base", vec![statement(&["radius", "medium"])]),
                block(
                    "variant tone",
                    vec![
                        block("primary", vec![statement(&["background", "accent"])]),
                        block("ghost", vec![statement(&["background", "transparent"])]),
                    ],
                ),
            ],
            &ctx,
        );

        assert_eq!(recipe.base_class(), "fr-Button");
        assert_eq!(recipe.base.facts.len(), 1);
        assert_eq!(recipe.variants.len(), 1);
        assert_eq!(recipe.variants[0].name, "tone");
        assert_eq!(recipe.variants[0].options.len(), 2);
        assert_eq!(
            recipe.variant_class("tone", "primary"),
            "fr-Button--tone-primary"
        );
    }
}
