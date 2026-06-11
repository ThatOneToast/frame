use frame_core::{symbols::index_document, Document};

mod emit;
mod helpers;
mod properties;

#[cfg(test)]
mod tests;

pub(crate) use emit::*;
pub(crate) use helpers::*;
pub(crate) use properties::*;

pub fn generate_css(document: &Document) -> String {
    let mut css = String::new();
    let symbols = index_document("", document);

    css.push_str(":root {\n");
    css.push_str("  --frame-space-none: 0;\n");
    css.push_str("  --frame-space-small: 0.5rem;\n");
    css.push_str("  --frame-space-medium: 1rem;\n");
    css.push_str("  --frame-space-large: 1.5rem;\n");
    css.push_str("  --frame-space-xlarge: 2rem;\n");
    css.push_str("  --frame-radius-none: 0;\n");
    css.push_str("  --frame-radius-small: 0.375rem;\n");
    css.push_str("  --frame-radius-medium: 0.625rem;\n");
    css.push_str("  --frame-radius-large: 1rem;\n");
    css.push_str("  --frame-radius-xlarge: 1.5rem;\n");
    css.push_str("  --frame-radius-pill: 999px;\n");
    css.push_str("  --frame-radius-full: 999px;\n");
    css.push_str("  --frame-surface-panel: #171717;\n");
    css.push_str("  --frame-surface-main: #101010;\n");
    css.push_str("  --frame-surface-glass: rgba(255, 255, 255, 0.08);\n");
    css.push_str("  --frame-surface-flat: transparent;\n");
    css.push_str("  --frame-surface-raised: #202020;\n");
    css.push_str("  --frame-surface-overlay: rgba(10, 10, 12, 0.92);\n");
    css.push_str("  --frame-surface-inset: #0b0b0f;\n");
    css.push_str("  --frame-surface-sunken: #08080b;\n");
    css.push_str("  --frame-gradient-dusk: linear-gradient(135deg, #22162f, #123047);\n");
    css.push_str("  --frame-gradient-midnight: linear-gradient(135deg, #080b18, #1b2440);\n");
    css.push_str(
        "  --frame-gradient-aurora: linear-gradient(135deg, #164e63, #4c1d95, #166534);\n",
    );
    css.push_str("  --frame-gradient-ember: linear-gradient(135deg, #7f1d1d, #f97316);\n");
    css.push_str("  --frame-gradient-ocean: linear-gradient(135deg, #0f766e, #1d4ed8);\n");
    css.push_str("  --frame-gradient-forest: linear-gradient(135deg, #14532d, #84cc16);\n");
    css.push_str("  --frame-color-main: #f5f5f5;\n");
    css.push_str("  --frame-color-bright: #ffffff;\n");
    css.push_str("  --frame-color-muted: #a3a3a3;\n");
    css.push_str("  --frame-color-accent: #8ab4ff;\n");
    css.push_str("  --frame-color-primary: #93c5fd;\n");
    css.push_str("  --frame-color-secondary: #c4b5fd;\n");
    css.push_str("  --frame-color-danger: #f87171;\n");
    css.push_str("  --frame-color-success: #34d399;\n");
    css.push_str("  --frame-color-warning: #fbbf24;\n");
    css.push_str("  --frame-color-info: #38bdf8;\n");
    css.push_str("  --frame-color-white: #ffffff;\n");
    css.push_str("  --frame-color-black: #000000;\n");
    css.push_str("  --frame-color-gray: #9ca3af;\n");
    css.push_str("  --frame-color-slate: #64748b;\n");
    css.push_str("  --frame-color-red: #ef4444;\n");
    css.push_str("  --frame-color-orange: #fb923c;\n");
    css.push_str("  --frame-color-yellow: #facc15;\n");
    css.push_str("  --frame-color-green: #22c55e;\n");
    css.push_str("  --frame-color-blue: #60a5fa;\n");
    css.push_str("  --frame-color-purple: #a78bfa;\n");
    css.push_str("  --frame-color-pink: #f472b6;\n");
    css.push_str("  --frame-color-cyan: #22d3ee;\n");
    css.push_str("  --frame-color-transparent: transparent;\n");
    css.push_str("  --frame-shadow-none: none;\n");
    css.push_str("  --frame-shadow-soft: 0 4px 16px rgba(0, 0, 0, 0.14);\n");
    css.push_str("  --frame-shadow-small: 0 6px 18px rgba(0, 0, 0, 0.18);\n");
    css.push_str("  --frame-shadow-medium: 0 12px 30px rgba(0, 0, 0, 0.25);\n");
    css.push_str("  --frame-shadow-large: 0 18px 48px rgba(0, 0, 0, 0.32);\n");
    css.push_str("  --frame-shadow-deep: 0 24px 64px rgba(0, 0, 0, 0.42);\n");
    css.push_str("  --frame-shadow-floating: 0 30px 80px rgba(0, 0, 0, 0.48);\n");
    css.push_str("  --frame-glow-none: none;\n");
    css.push_str("  --frame-glow-accent: 0 0 24px rgba(120, 160, 255, 0.35);\n");
    css.push_str("  --frame-glow-danger: 0 0 24px rgba(248, 113, 113, 0.35);\n");
    css.push_str("  --frame-glow-success: 0 0 24px rgba(52, 211, 153, 0.35);\n");
    css.push_str("  --frame-glow-warning: 0 0 24px rgba(251, 191, 36, 0.35);\n");
    css.push_str("  --frame-glow-soft: 0 0 18px rgba(255, 255, 255, 0.16);\n");
    css.push_str("  --frame-glow-strong: 0 0 34px rgba(255, 255, 255, 0.28);\n");
    emit_custom_tokens(&mut css, document);
    css.push_str("}\n\n");

    css.push_str(".fr-FrameText {\n");
    css.push_str("  display: inline;\n");
    css.push_str("  white-space: pre-wrap;\n");
    css.push_str("}\n\n");

    css.push_str("[class*=\"fr-\"][type=\"button\"],\n");
    css.push_str("[class*=\"fr-\"][type=\"submit\"],\n");
    css.push_str("[class*=\"fr-\"][type=\"reset\"] {\n");
    css.push_str("  appearance: none;\n");
    css.push_str("  background: none;\n");
    css.push_str("  border: none;\n");
    css.push_str("  cursor: pointer;\n");
    css.push_str("  font: inherit;\n");
    css.push_str("  color: inherit;\n");
    css.push_str("  padding: 0;\n");
    css.push_str("}\n\n");

    css.push_str("button[class*=\"fr-\"],\n");
    css.push_str("a[class*=\"fr-\"] {\n");
    css.push_str("  display: inline-flex !important;\n");
    css.push_str("  align-items: center !important;\n");
    css.push_str("  gap: var(--frame-space-small) !important;\n");
    css.push_str("  flex-direction: row !important;\n");
    css.push_str("}\n\n");

    for declaration in &document.declarations {
        emit_declaration_css(&mut css, declaration, &symbols, &document.declarations);
    }

    emit_keyframes(&mut css);
    css
}
