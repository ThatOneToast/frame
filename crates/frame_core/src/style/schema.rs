//! Normalized style facts.
//!
//! A style fact is the unit of meaning between Frame's semantic statements and
//! any CSS backend. Each fact carries a stable property path (used for
//! inheritance overrides and variant merging) plus the CSS declarations it
//! lowers to. Backends consume facts; they never re-parse statement words.

use serde::{Deserialize, Serialize};

/// Pseudo-property used for transform fragments that backends must merge
/// into a single `transform:` declaration.
pub const TRANSFORM_PART: &str = "@transform-part";
/// Pseudo-property used for filter fragments that backends must merge
/// into a single `filter:` declaration.
pub const FILTER_PART: &str = "@filter-part";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CssDecl {
    pub property: String,
    pub value: String,
}

impl CssDecl {
    pub fn new(property: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

/// One normalized style fact: a semantic property path and the CSS it lowers to.
///
/// Property paths are dot-separated and hierarchical: `border`, `border.width`,
/// `layout.display`, `motion.transition.duration`, `effect.transform.lift`.
/// A fact at path `border` supersedes facts at `border.*` when overriding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleFact {
    pub path: String,
    pub decls: Vec<CssDecl>,
}

impl StyleFact {
    pub fn new(path: impl Into<String>, decls: Vec<CssDecl>) -> Self {
        Self {
            path: path.into(),
            decls,
        }
    }

    pub fn single(
        path: impl Into<String>,
        property: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            decls: vec![CssDecl::new(property, value)],
        }
    }
}

/// Facts scoped to an interaction state (`hover`, `focus`, `active`, ...).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateScope {
    /// The state block name as written (`hover`, `focus-within`, ...).
    pub state: String,
    pub facts: Vec<StyleFact>,
}

/// Facts scoped to a responsive or container condition
/// (`below tablet`, `between tablet desktop`, `container content`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionScope {
    /// The condition block name as written.
    pub condition: String,
    pub facts: Vec<StyleFact>,
}

/// Facts for one named grid section (`section sidebar { ... }`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridSection {
    pub name: String,
    pub facts: Vec<StyleFact>,
}

/// The fully normalized style for one declaration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedStyle {
    pub facts: Vec<StyleFact>,
    pub states: Vec<StateScope>,
    pub conditions: Vec<ConditionScope>,
    /// Named grid sections, in track order (grids only).
    pub section_names: Vec<String>,
    pub sections: Vec<GridSection>,
}

impl NormalizedStyle {
    /// Merge `child` over `self` using property-path override semantics.
    ///
    /// - A child fact replaces a base fact with the same path in place.
    /// - A child fact at path `p` also removes base facts under `p.` —
    ///   re-declaring `border` supersedes an inherited `border.width`.
    /// - Child facts with new paths append after base facts, so the CSS
    ///   cascade resolves partial overrides (`border.width` after `border`).
    /// - State and condition scopes merge recursively by scope name.
    pub fn merge_child(mut self, child: NormalizedStyle) -> NormalizedStyle {
        self.facts = merge_facts(self.facts, child.facts);
        for child_scope in child.states {
            if let Some(base) = self
                .states
                .iter_mut()
                .find(|scope| scope.state == child_scope.state)
            {
                base.facts = merge_facts(std::mem::take(&mut base.facts), child_scope.facts);
            } else {
                self.states.push(child_scope);
            }
        }
        for child_scope in child.conditions {
            if let Some(base) = self
                .conditions
                .iter_mut()
                .find(|scope| scope.condition == child_scope.condition)
            {
                base.facts = merge_facts(std::mem::take(&mut base.facts), child_scope.facts);
            } else {
                self.conditions.push(child_scope);
            }
        }
        if !child.section_names.is_empty() {
            self.section_names = child.section_names;
        }
        for child_section in child.sections {
            if let Some(base) = self
                .sections
                .iter_mut()
                .find(|section| section.name == child_section.name)
            {
                base.facts = merge_facts(std::mem::take(&mut base.facts), child_section.facts);
            } else {
                self.sections.push(child_section);
            }
        }
        self
    }
}

pub fn merge_facts(base: Vec<StyleFact>, child: Vec<StyleFact>) -> Vec<StyleFact> {
    let mut merged = base;
    for fact in child {
        let prefix = format!("{}.", fact.path);
        merged.retain(|existing| existing.path != fact.path && !existing.path.starts_with(&prefix));
        merged.push(fact);
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(path: &str, property: &str, value: &str) -> StyleFact {
        StyleFact::single(path, property, value)
    }

    #[test]
    fn child_replaces_same_path() {
        let merged = merge_facts(
            vec![fact("background", "background", "red")],
            vec![fact("background", "background", "blue")],
        );
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].decls[0].value, "blue");
    }

    #[test]
    fn child_shorthand_supersedes_base_longhand() {
        let merged = merge_facts(
            vec![fact("border.width", "border-width", "2px")],
            vec![fact("border", "border", "1px solid red")],
        );
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].path, "border");
    }

    #[test]
    fn child_longhand_keeps_base_shorthand_and_wins_cascade() {
        let merged = merge_facts(
            vec![fact("border", "border", "1px solid red")],
            vec![fact("border.width", "border-width", "2px")],
        );
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].path, "border");
        assert_eq!(merged[1].path, "border.width");
    }

    #[test]
    fn unrelated_paths_are_preserved() {
        let merged = merge_facts(
            vec![fact("padding", "padding", "1rem")],
            vec![fact("padding.inline", "padding-inline", "2rem")],
        );
        assert_eq!(merged.len(), 2);
    }
}
