//! Experimental atomic CSS backend.
//!
//! Consumes the same normalized style facts as the semantic backend, but
//! deduplicates identical declarations across classes by grouping selectors:
//!
//! ```css
//! .fr-Panel,
//! .fr-Sidebar {
//!   padding: var(--frame-space-medium);
//! }
//! ```
//!
//! Class names and runtime behavior are unchanged — only the generated
//! stylesheet shape differs. Experimental: declaration grouping follows
//! global first-seen order, so classes that rely on intra-rule cascade
//! ordering (such as a longhand override after a shorthand from
//! inheritance) may resolve differently. Use `--css-backend semantic`
//! (the default) when in doubt.

use frame_core::style::{expand_motion_references, normalize::apply_kind_defaults, StyleContext};
use frame_core::{DeclarationKind, Document};

use super::emit::{emit_declaration_css, emit_grid_section_rules, fact_lines, resolve_style};
use frame_core::style::properties;

type GroupKey = (Option<String>, String);

#[derive(Default)]
struct Groups {
    /// (at-rule, declaration line) -> selectors, in first-seen order.
    entries: Vec<(GroupKey, Vec<String>)>,
}

impl Groups {
    fn add(&mut self, at_rule: Option<&str>, selector: &str, lines: &[String]) {
        for line in lines {
            let key = (at_rule.map(ToOwned::to_owned), line.clone());
            if let Some((_, selectors)) = self
                .entries
                .iter_mut()
                .find(|(existing, _)| *existing == key)
            {
                if !selectors.iter().any(|existing| existing == selector) {
                    selectors.push(selector.to_string());
                }
            } else {
                self.entries.push((key, vec![selector.to_string()]));
            }
        }
    }

    fn write(&self, css: &mut String) {
        // Emit unconditional groups first, then at-rule groups in order.
        for (key, selectors) in &self.entries {
            if key.0.is_some() {
                continue;
            }
            css.push_str(&format!(
                "{} {{\n  {}\n}}\n\n",
                selectors.join(",\n"),
                key.1
            ));
        }
        let mut seen_rules: Vec<&str> = Vec::new();
        for (key, _) in &self.entries {
            if let Some(rule) = key.0.as_deref() {
                if !seen_rules.contains(&rule) {
                    seen_rules.push(rule);
                }
            }
        }
        for rule in seen_rules {
            css.push_str(&format!("{rule} {{\n"));
            for (key, selectors) in &self.entries {
                if key.0.as_deref() != Some(rule) {
                    continue;
                }
                css.push_str(&format!(
                    "  {} {{\n    {}\n  }}\n",
                    selectors.join(",\n  "),
                    key.1
                ));
            }
            css.push_str("}\n\n");
        }
    }
}

pub(crate) fn emit_atomic_declarations(css: &mut String, document: &Document, ctx: &StyleContext) {
    let mut groups = Groups::default();
    let mut passthrough = String::new();

    for declaration in &document.declarations {
        match declaration.kind {
            DeclarationKind::Tokens | DeclarationKind::Theme | DeclarationKind::Motion => {}
            DeclarationKind::Keyframes
            | DeclarationKind::Supports
            | DeclarationKind::StyleOrder
            | DeclarationKind::StyleGroup
            | DeclarationKind::Html
            | DeclarationKind::Body => {
                emit_declaration_css(&mut passthrough, declaration, ctx, &document.declarations);
            }
            DeclarationKind::Unknown(_) => {}
            DeclarationKind::Layout => {
                let mut style = frame_core::style::lower_layout(&declaration.body, ctx);
                expand_motion_references(&mut style, ctx.motions);
                let class = format!(".fr-{}", declaration.name.text);
                add_style(&mut groups, &class, &style, ctx);
                emit_grid_section_rules(&mut passthrough, &class[1..], &style);
            }
            DeclarationKind::Recipe => {
                let recipe = frame_core::style::recipes::lower_recipe(
                    &declaration.name.text,
                    &declaration.body,
                    ctx,
                );
                let mut base = recipe.base.clone();
                expand_motion_references(&mut base, ctx.motions);
                add_style(
                    &mut groups,
                    &format!(".{}", recipe.base_class()),
                    &base,
                    ctx,
                );
                for group in &recipe.variants {
                    for (option, style) in &group.options {
                        let mut style = style.clone();
                        expand_motion_references(&mut style, ctx.motions);
                        add_style(
                            &mut groups,
                            &format!(".{}", recipe.variant_class(&group.name, option)),
                            &style,
                            ctx,
                        );
                    }
                }
            }
            _ => {
                let mut style = resolve_style(declaration, &document.declarations, ctx);
                apply_kind_defaults(&mut style, &declaration.kind);
                expand_motion_references(&mut style, ctx.motions);
                let class = format!(".fr-{}", declaration.name.text);
                add_style(&mut groups, &class, &style, ctx);
                if declaration.kind == DeclarationKind::Grid {
                    emit_grid_section_rules(&mut passthrough, &class[1..], &style);
                }
            }
        }
    }

    groups.write(css);
    css.push_str(&passthrough);
}

fn add_style(
    groups: &mut Groups,
    class_selector: &str,
    style: &frame_core::style::NormalizedStyle,
    ctx: &StyleContext,
) {
    groups.add(None, class_selector, &fact_lines(&style.facts));
    for state in &style.states {
        let Some(suffix) = properties::state_selector(&state.state) else {
            continue;
        };
        groups.add(
            None,
            &format!("{class_selector}{suffix}"),
            &fact_lines(&state.facts),
        );
    }
    for condition in &style.conditions {
        let Some(rule) = properties::condition_rule(&condition.condition, ctx.contract) else {
            continue;
        };
        groups.add(Some(&rule), class_selector, &fact_lines(&condition.facts));
    }
}
