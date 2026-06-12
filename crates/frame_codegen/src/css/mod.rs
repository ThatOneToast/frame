use frame_core::style::{document_contract, document_motions, document_themes, StyleContext};
use frame_core::Document;

mod atomic;
mod emit;
mod helpers;

#[cfg(test)]
mod tests;

pub(crate) use emit::*;

/// The CSS generation strategy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CssBackend {
    /// One rule per generated class (the default).
    #[default]
    Semantic,
    /// Experimental: deduplicate identical declarations across classes by
    /// grouping selectors. Same class names, smaller output, but intra-rule
    /// cascade ordering may differ; see `css/atomic.rs`.
    Atomic,
}

impl CssBackend {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "semantic" => Some(Self::Semantic),
            "atomic" => Some(Self::Atomic),
            _ => None,
        }
    }
}

pub fn generate_css(document: &Document) -> String {
    generate_css_with_backend(document, CssBackend::Semantic)
}

pub fn generate_css_with_backend(document: &Document, backend: CssBackend) -> String {
    let mut css = String::new();
    let contract = document_contract(document);
    let themes = document_themes(document);
    let motions = document_motions(document);
    let ctx = StyleContext::with_motions(&contract, &motions);

    css.push_str(":root {\n");
    for (variable, value) in contract.css_variables() {
        css.push_str(&format!("  {variable}: {value};\n"));
    }
    css.push_str("}\n\n");

    for (index, theme) in themes.iter().enumerate() {
        let variables = theme.css_variables();
        if variables.is_empty() {
            continue;
        }
        if index == 0 {
            css.push_str(&format!(
                ":root,\n[data-frame-theme=\"{}\"] {{\n",
                theme.name
            ));
        } else {
            css.push_str(&format!("[data-frame-theme=\"{}\"] {{\n", theme.name));
        }
        for (variable, value) in variables {
            css.push_str(&format!("  {variable}: {value};\n"));
        }
        css.push_str("}\n\n");
    }

    emit_reset_layer(&mut css, document);

    match backend {
        CssBackend::Semantic => {
            for declaration in &document.declarations {
                emit_declaration_css(&mut css, declaration, &ctx, &document.declarations);
            }
        }
        CssBackend::Atomic => {
            atomic::emit_atomic_declarations(&mut css, document, &ctx);
        }
    }

    emit_keyframes(&mut css);
    css
}
