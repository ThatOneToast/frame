use std::collections::HashSet;

use super::helpers::*;
use frame_core::style::{
    normalize_declaration, properties, NormalizedStyle, StyleContext, StyleFact, FILTER_PART,
    TRANSFORM_PART,
};
use frame_core::{Declaration, DeclarationKind, Node, Statement};

/// Emit the reset layer for Frame primitives.
///
/// Resets live in a dedicated cascade layer and target the concrete classes
/// Frame generated, never broad `[class*="fr-"]` attribute scans.
pub(crate) fn emit_reset_layer(css: &mut String, document: &frame_core::Document) {
    css.push_str("@layer frame-reset {\n");
    css.push_str("  .fr-FrameText {\n");
    css.push_str("    display: inline;\n");
    css.push_str("    white-space: pre-wrap;\n");
    css.push_str("  }\n");

    let mut button_classes: Vec<String> = document
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Button)
        .map(|declaration| format!(".fr-{}", declaration.name.text))
        .collect();
    button_classes.dedup();

    if !button_classes.is_empty() {
        let selectors = button_classes.join(",\n  ");
        css.push_str(&format!("  {selectors} {{\n"));
        css.push_str("    appearance: none;\n");
        css.push_str("    background: none;\n");
        css.push_str("    border: none;\n");
        css.push_str("    cursor: pointer;\n");
        css.push_str("    font: inherit;\n");
        css.push_str("    color: inherit;\n");
        css.push_str("    padding: 0;\n");
        css.push_str("    display: inline-flex;\n");
        css.push_str("    align-items: center;\n");
        css.push_str("    gap: var(--frame-space-small);\n");
        css.push_str("    flex-direction: row;\n");
        css.push_str("  }\n");
    }
    css.push_str("}\n\n");
}

pub(crate) fn emit_declaration_css(
    css: &mut String,
    declaration: &Declaration,
    ctx: &StyleContext,
    all_declarations: &[Declaration],
) {
    let class_name = format!("fr-{}", declaration.name.text);

    match declaration.kind {
        DeclarationKind::Tokens | DeclarationKind::Theme | DeclarationKind::Motion => {}
        DeclarationKind::Layout => {
            let mut style = frame_core::style::lower_layout(&declaration.body, ctx);
            frame_core::style::expand_motion_references(&mut style, ctx.motions);
            css.push_str(&format!(".{class_name} {{\n"));
            write_facts(css, &style.facts, "  ");
            css.push_str("}\n\n");
            emit_grid_section_rules(css, &class_name, &style);
            emit_scopes(css, &class_name, &style, ctx);
        }
        DeclarationKind::Recipe => {
            let contract_ctx = ctx;
            let recipe = frame_core::style::recipes::lower_recipe(
                &declaration.name.text,
                &declaration.body,
                contract_ctx,
            );
            let mut base = recipe.base.clone();
            frame_core::style::expand_motion_references(&mut base, ctx.motions);
            css.push_str(&format!(".{} {{\n", recipe.base_class()));
            write_facts(css, &base.facts, "  ");
            css.push_str("}\n\n");
            emit_scopes(css, &recipe.base_class(), &base, ctx);

            for group in &recipe.variants {
                for (option, style) in &group.options {
                    let mut style = style.clone();
                    frame_core::style::expand_motion_references(&mut style, ctx.motions);
                    let variant_class = recipe.variant_class(&group.name, option);
                    css.push_str(&format!(".{variant_class} {{\n"));
                    write_facts(css, &style.facts, "  ");
                    css.push_str("}\n\n");
                    emit_scopes(css, &variant_class, &style, ctx);
                }
            }
        }
        DeclarationKind::Keyframes => {
            emit_custom_keyframes(css, &declaration.name.text, &declaration.body);
        }
        DeclarationKind::Supports => emit_supports(css, declaration, ctx),
        DeclarationKind::StyleOrder => emit_style_order(css, declaration),
        DeclarationKind::StyleGroup => emit_style_group(css, declaration, ctx),
        DeclarationKind::Html => {
            let style = normalize_declaration(&declaration.kind, &declaration.body, ctx);
            css.push_str("html {\n");
            write_facts(css, &style.facts, "  ");
            css.push_str("}\n\n");
        }
        DeclarationKind::Body => {
            let style = normalize_declaration(&declaration.kind, &declaration.body, ctx);
            css.push_str("body {\n");
            css.push_str("  min-height: 100vh;\n");
            write_facts(css, &style.facts, "  ");
            css.push_str("}\n\n");
        }
        DeclarationKind::Unknown(_) => {}
        _ => {
            let mut style = resolve_style(declaration, all_declarations, ctx);
            frame_core::style::normalize::apply_kind_defaults(&mut style, &declaration.kind);
            frame_core::style::expand_motion_references(&mut style, ctx.motions);

            css.push_str(&format!(".{class_name} {{\n"));
            write_facts(css, &style.facts, "  ");
            css.push_str("}\n\n");

            if declaration.kind == DeclarationKind::Grid {
                emit_grid_section_rules(css, &class_name, &style);
            }

            emit_scopes(css, &class_name, &style, ctx);
        }
    }
}

/// Emit state and condition scopes for a class.
pub(crate) fn emit_scopes(
    css: &mut String,
    class_name: &str,
    style: &frame_core::style::NormalizedStyle,
    ctx: &StyleContext,
) {
    for state in &style.states {
        let Some(selector) = properties::state_selector(&state.state) else {
            continue;
        };
        css.push_str(&format!(".{class_name}{selector} {{\n"));
        write_facts(css, &state.facts, "  ");
        css.push_str("}\n\n");
    }

    for condition in &style.conditions {
        let Some(rule) = properties::condition_rule(&condition.condition, ctx.contract) else {
            continue;
        };
        css.push_str(&format!("{rule} {{\n  .{class_name} {{\n"));
        write_facts(css, &condition.facts, "    ");
        css.push_str("  }\n}\n\n");
    }
}

/// Resolve a declaration's normalized style, walking `extends` chains with
/// property-path overrides (base first, child facts winning by path).
pub(crate) fn resolve_style(
    declaration: &Declaration,
    all_declarations: &[Declaration],
    ctx: &StyleContext,
) -> NormalizedStyle {
    let mut visited = HashSet::new();
    visited.insert(declaration.name.text.clone());
    resolve_style_inner(declaration, all_declarations, ctx, &mut visited)
}

fn resolve_style_inner(
    declaration: &Declaration,
    all_declarations: &[Declaration],
    ctx: &StyleContext,
    visited: &mut HashSet<String>,
) -> NormalizedStyle {
    let own = normalize_declaration(&declaration.kind, &declaration.body, ctx);
    let Some(ref base_name) = declaration.extends else {
        return own;
    };
    if !visited.insert(base_name.text.clone()) {
        return own;
    }
    let Some(base_decl) = all_declarations
        .iter()
        .find(|candidate| candidate.name.text == base_name.text)
    else {
        return own;
    };
    let base = resolve_style_inner(base_decl, all_declarations, ctx, visited);
    base.merge_child(own)
}

/// Render facts to `property: value;` lines, merging transform/filter
/// fragments. Shared by the semantic and atomic backends.
pub(crate) fn fact_lines(facts: &[StyleFact]) -> Vec<String> {
    let mut lines = Vec::new();
    let mut transforms: Vec<&str> = Vec::new();
    let mut filters: Vec<&str> = Vec::new();

    for fact in facts {
        for decl in &fact.decls {
            match decl.property.as_str() {
                TRANSFORM_PART => transforms.push(&decl.value),
                FILTER_PART => filters.push(&decl.value),
                property if property.starts_with('@') => {}
                property => lines.push(format!("{property}: {};", decl.value)),
            }
        }
    }

    if !transforms.is_empty() {
        lines.push(format!("transform: {};", transforms.join(" ")));
    }
    if !filters.is_empty() {
        lines.push(format!("filter: {};", filters.join(" ")));
    }
    lines
}

/// Write facts as CSS declarations, merging transform/filter fragments.
pub(crate) fn write_facts(css: &mut String, facts: &[StyleFact], indent: &str) {
    for line in fact_lines(facts) {
        css.push_str(&format!("{indent}{line}\n"));
    }
}

pub(crate) fn emit_grid_section_rules(css: &mut String, class_name: &str, style: &NormalizedStyle) {
    if style.section_names.is_empty() {
        return;
    }

    for (index, section) in style.section_names.iter().enumerate() {
        css.push_str(&format!(
            ".{class_name} > :nth-child({}), .{class_name} > [data-frame-section=\"{section}\"] {{\n  grid-area: {section};\n",
            index + 1
        ));
        if let Some(block) = style
            .sections
            .iter()
            .find(|candidate| candidate.name == *section)
        {
            write_facts(css, &block.facts, "  ");
        }
        css.push_str("}\n\n");
    }
}

pub(crate) fn emit_style_order(css: &mut String, declaration: &Declaration) {
    let names = style_order_names(&declaration.name.text);
    if !names.is_empty() {
        css.push_str(&format!("@layer {};\n\n", names.join(", ")));
    }
}

pub(crate) fn emit_style_group(css: &mut String, declaration: &Declaration, ctx: &StyleContext) {
    if declaration.name.text.is_empty() {
        return;
    }
    css.push_str(&format!("@layer {} {{\n", declaration.name.text));
    emit_nested_declarations(css, declaration, ctx);
    css.push_str("}\n\n");
}

pub(crate) fn emit_supports(css: &mut String, declaration: &Declaration, ctx: &StyleContext) {
    let Some(condition) = properties::supports_condition(&declaration.name.text) else {
        return;
    };
    css.push_str(&format!("@supports {condition} {{\n"));
    emit_nested_declarations(css, declaration, ctx);
    css.push_str("}\n\n");
}

fn emit_nested_declarations(css: &mut String, declaration: &Declaration, ctx: &StyleContext) {
    let mut nested_css = String::new();
    for node in &declaration.body {
        let Node::Block(block) = node else {
            continue;
        };
        let Some(nested) = declaration_from_block(block) else {
            continue;
        };
        emit_declaration_css(&mut nested_css, &nested, ctx, &[]);
    }
    for line in nested_css.lines() {
        if line.is_empty() {
            css.push('\n');
        } else {
            css.push_str("  ");
            css.push_str(line);
            css.push('\n');
        }
    }
}

pub(crate) fn emit_custom_keyframes(css: &mut String, name: &str, body: &[Node]) {
    css.push_str(&format!("@keyframes frame-{name} {{\n"));
    for node in body {
        let Node::Block(block) = node else {
            continue;
        };
        if !is_keyframe_selector(&block.name) {
            continue;
        }
        css.push_str(&format!("  {} {{\n", block.name));
        for statement in statements(&block.body) {
            emit_keyframe_statement(css, statement);
        }
        css.push_str("  }\n");
    }
    css.push_str("}\n\n");
}

pub(crate) fn emit_keyframe_statement(css: &mut String, statement: &Statement) {
    let Some(property) = statement.words.first().map(String::as_str) else {
        return;
    };
    let value = statement
        .words
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if value.is_empty() {
        return;
    }
    match property {
        "opacity" | "transform" | "filter" => {
            css.push_str(&format!("    {property}: {value};\n"));
        }
        "scale" | "translate" | "rotate" => {
            css.push_str(&format!("    transform: {property}({value});\n"));
        }
        _ => {}
    }
}

pub(crate) fn emit_keyframes(css: &mut String) {
    css.push_str("@keyframes frame-fade-in {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}\n\n");
    css.push_str("@keyframes frame-slide-up {\n  from { opacity: 0; transform: translateY(0.5rem); }\n  to { opacity: 1; transform: translateY(0); }\n}\n\n");
    css.push_str("@keyframes frame-pop-in {\n  from { opacity: 0; transform: scale(0.96); }\n  to { opacity: 1; transform: scale(1); }\n}\n\n");
    css.push_str(
        "@keyframes frame-pulse {\n  0%, 100% { opacity: 1; }\n  50% { opacity: 0.72; }\n}\n\n",
    );
}

pub(crate) fn statements(body: &[Node]) -> impl Iterator<Item = &Statement> {
    body.iter().filter_map(|node| {
        if let Node::Statement(statement) = node {
            Some(statement)
        } else {
            None
        }
    })
}
