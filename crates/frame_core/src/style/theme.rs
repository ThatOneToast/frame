//! Scoped themes.
//!
//! A theme is a named set of token overrides applied through a
//! `[data-frame-theme="name"]` scope. The first declared theme also binds to
//! `:root`, making it the document default.

use crate::{DeclarationKind, Document};

use super::tokens::{collect_token_entries, TokenContract, TokenEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub name: String,
    /// The token namespace this theme refines (`uses <namespace>`).
    pub uses: Option<String>,
    pub overrides: Vec<TokenEntry>,
}

impl Theme {
    /// The custom-property pairs this theme overrides.
    pub fn css_variables(&self) -> Vec<(String, String)> {
        self.overrides
            .iter()
            .filter(|entry| entry.kind.emits_css_variable())
            .map(|entry| {
                (
                    TokenContract::css_variable(entry.kind, &entry.name),
                    entry.value.clone(),
                )
            })
            .collect()
    }
}

/// Collect scoped themes from `theme <name> uses <namespace> { ... }` declarations.
pub fn document_themes(document: &Document) -> Vec<Theme> {
    let mut themes = Vec::new();
    for declaration in &document.declarations {
        if declaration.kind != DeclarationKind::Theme {
            continue;
        }
        let mut scratch = TokenContract::default();
        collect_token_entries(&declaration.body, &mut scratch);
        themes.push(Theme {
            name: declaration.name.text.clone(),
            uses: declaration.extends.as_ref().map(|base| base.text.clone()),
            overrides: scratch.entries,
        });
    }
    themes
}
