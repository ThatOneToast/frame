//! Semantic property policy: how Frame vocabulary maps to CSS values.
//!
//! This module owns *meaning* (which names exist, what they resolve to).
//! It never writes CSS rules — backends do that from normalized facts.

use super::tokens::{breakpoint_below, token_reference, TokenContract, TokenKind};

/// Resolve any value position: `token(kind.name)` references become
/// `var(--frame-kind-name)`, everything else passes through.
pub fn resolve_value(value: &str) -> String {
    if let Some((kind, name)) = token_reference(value) {
        return format!("var({})", TokenContract::css_variable(kind, name));
    }
    value.to_string()
}

/// Resolve a value that defaults to the spacing scale.
pub fn space_value(value: &str) -> String {
    if let Some((kind, name)) = token_reference(value) {
        return format!("var({})", TokenContract::css_variable(kind, name));
    }
    format!("var(--frame-space-{value})")
}

/// Resolve a value that defaults to the color scale.
pub fn color_value(value: &str) -> String {
    if let Some((kind, name)) = token_reference(value) {
        return format!("var({})", TokenContract::css_variable(kind, name));
    }
    if value.starts_with('#') {
        return value.to_string();
    }
    format!("var(--frame-color-{value})")
}

/// Resolve a background value: gradients, then surfaces, then colors.
pub fn background_value(value: &str, contract: &TokenContract) -> String {
    if let Some((kind, name)) = token_reference(value) {
        return format!("var({})", TokenContract::css_variable(kind, name));
    }
    if value == "transparent" || value.starts_with('#') {
        return value.to_string();
    }
    if contract.get(TokenKind::Gradient, value).is_some() {
        return format!("var(--frame-gradient-{value})");
    }
    if contract.get(TokenKind::Surface, value).is_some() {
        return format!("var(--frame-surface-{value})");
    }
    format!("var(--frame-color-{value})")
}

/// Resolve a `surface` value: gradients, then colors, then surfaces.
pub fn surface_value(value: &str, contract: &TokenContract) -> String {
    if let Some((kind, name)) = token_reference(value) {
        return format!("var({})", TokenContract::css_variable(kind, name));
    }
    if contract.get(TokenKind::Gradient, value).is_some() {
        return format!("var(--frame-gradient-{value})");
    }
    if contract.get(TokenKind::Surface, value).is_some() {
        return format!("var(--frame-surface-{value})");
    }
    format!("var(--frame-color-{value})")
}

pub fn size_css_value(property: &str, value: &str) -> String {
    if let Some((kind, name)) = token_reference(value) {
        return format!("var({})", TokenContract::css_variable(kind, name));
    }
    match value {
        "screen" if is_block_axis_property(property) => "100vh".to_string(),
        "screen" if is_inline_axis_property(property) => "100vw".to_string(),
        "fill" => "100%".to_string(),
        "content" => "max-content".to_string(),
        "auto" => "auto".to_string(),
        "chart" => "12rem".to_string(),
        "panel" => "16rem".to_string(),
        "sidebar" => "18rem".to_string(),
        "narrow" => "12rem".to_string(),
        "wide" => "32rem".to_string(),
        "input" => "32rem".to_string(),
        "dashboard" => "96rem".to_string(),
        "zero" if property.starts_with("min-") => "0".to_string(),
        "none" if property.starts_with("min-") => "0".to_string(),
        "modal" if property == "width" || property == "inline-size" => {
            "min(42rem, 100%)".to_string()
        }
        "icon" => "2.5rem".to_string(),
        value if is_percentage(value) => value.to_string(),
        value => format!("var(--frame-space-{value})"),
    }
}

pub fn is_inline_axis_property(property: &str) -> bool {
    property.contains("width") || property.contains("inline-size")
}

pub fn is_block_axis_property(property: &str) -> bool {
    property.contains("height") || property.contains("block-size")
}

pub fn column_css_value(value: &str) -> String {
    match value {
        value if is_percentage(value) => value.to_string(),
        "auto" => "auto".to_string(),
        "fill" => "minmax(0, 1fr)".to_string(),
        "subgrid" => "subgrid".to_string(),
        value if is_fr(value) => fr_to_css(value),
        _ => "minmax(0, 1fr)".to_string(),
    }
}

pub fn track_css_value(value: &str) -> String {
    match value {
        "rail" => "4.5rem".to_string(),
        "panel" => "18rem".to_string(),
        "side" => "16rem".to_string(),
        "header" => "3.25rem".to_string(),
        "composer" => "4.75rem".to_string(),
        "fill" => "minmax(0, 1fr)".to_string(),
        "auto" => "auto".to_string(),
        "content" => "max-content".to_string(),
        value if is_percentage(value) => value.to_string(),
        value if is_fr(value) => fr_to_css(value),
        _ => "minmax(0, 1fr)".to_string(),
    }
}

pub fn is_percentage(value: &str) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

pub fn is_fr(value: &str) -> bool {
    value.ends_with("fr")
        && value
            .strip_suffix("fr")
            .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

pub fn fr_to_css(value: &str) -> String {
    let number = value.strip_suffix("fr").unwrap_or("1");
    format!("minmax(0, {number}fr)")
}

pub fn is_identifier_grid_name(value: &str) -> bool {
    if value == "subgrid" {
        return false;
    }
    value
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

pub fn css_alignment(value: &str) -> &str {
    match value {
        "start" => "flex-start",
        "end" => "flex-end",
        value => value,
    }
}

pub fn css_justify(value: &str) -> &str {
    match value {
        "start" => "flex-start",
        "end" => "flex-end",
        "between" => "space-between",
        "around" => "space-around",
        "evenly" => "space-evenly",
        value => value,
    }
}

pub fn css_corner(corner: &str) -> &str {
    match corner {
        "top-left" => "top left",
        "top-right" => "top right",
        "bottom-left" => "bottom left",
        "bottom-right" => "bottom right",
        value => value,
    }
}

pub fn animation_duration(value: Option<&str>) -> String {
    match value {
        Some("fast") => "120ms".to_string(),
        Some("normal") => "200ms".to_string(),
        Some("slow") => "360ms".to_string(),
        Some(value) if value.ends_with("ms") || value.ends_with('s') => value.to_string(),
        _ => "240ms".to_string(),
    }
}

pub fn animation_ease(value: Option<&str>) -> String {
    match value {
        Some("linear") => "linear".to_string(),
        Some("bounce") => "cubic-bezier(.2, 1.4, .4, 1)".to_string(),
        Some("sharp") => "cubic-bezier(.4, 0, 1, 1)".to_string(),
        _ => "ease".to_string(),
    }
}

/// Resolve a breakpoint name (or raw length) against the token contract.
pub fn breakpoint_value<'a>(name: &'a str, contract: &'a TokenContract) -> &'a str {
    contract.get(TokenKind::Breakpoint, name).unwrap_or(name)
}

/// Resolve a container name (or raw length) against the token contract.
pub fn container_value<'a>(name: &'a str, contract: &'a TokenContract) -> &'a str {
    contract.get(TokenKind::Container, name).unwrap_or(name)
}

/// Translate a responsive/container condition block name into an at-rule.
pub fn condition_rule(name: &str, contract: &TokenContract) -> Option<String> {
    let words = name.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["below", breakpoint] => Some(format!(
            "@media (max-width: {})",
            breakpoint_below(breakpoint_value(breakpoint, contract))
        )),
        ["above", breakpoint] => Some(format!(
            "@media (min-width: {})",
            breakpoint_value(breakpoint, contract)
        )),
        ["between", start, end] => Some(format!(
            "@media (min-width: {}) and (max-width: {})",
            breakpoint_value(start, contract),
            breakpoint_below(breakpoint_value(end, contract))
        )),
        ["container", name] => Some(format!(
            "@container (max-width: {})",
            container_value(name, contract)
        )),
        _ => None,
    }
}

pub fn supports_condition(predicate: &str) -> Option<&'static str> {
    match predicate.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["display", "grid"] => Some("(display: grid)"),
        ["display", "flex"] => Some("(display: flex)"),
        ["backdrop", "blur"] => Some("(backdrop-filter: blur(1px))"),
        ["color", "oklch"] => Some("(color: oklch(50% 0.1 180))"),
        ["selector", "has"] => Some("selector(:has(*))"),
        ["container", "queries"] => Some("(container-type: inline-size)"),
        ["subgrid"] => Some("(grid-template-columns: subgrid)"),
        _ => None,
    }
}

pub fn state_selector(name: &str) -> Option<&'static str> {
    match name {
        "hover" => Some(":hover"),
        "focus" => Some(":focus-visible"),
        "focus-visible" => Some(":focus-visible"),
        "focus-within" => Some(":focus-within"),
        "active" => Some(":active"),
        "disabled" => Some(":disabled"),
        "checked" => Some(":checked"),
        "invalid" => Some(":invalid"),
        "required" => Some(":required"),
        "target" => Some(":target"),
        _ => None,
    }
}

pub const Z_LAYERS: [(&str, i32); 7] = [
    ("base", 0),
    ("above", 10),
    ("dropdown", 40),
    ("sticky", 45),
    ("overlay", 50),
    ("modal", 100),
    ("toast", 110),
];

pub const MOVEMENT_SCALE: [(&str, f32); 5] = [
    ("tiny", 1.0),
    ("small", 4.0),
    ("medium", 8.0),
    ("large", 12.0),
    ("huge", 16.0),
];

pub const GROW_SCALE: [(&str, f32); 5] = [
    ("slight", 1.02),
    ("subtle", 1.04),
    ("normal", 1.06),
    ("strong", 1.10),
    ("dramatic", 1.16),
];

pub const SHRINK_SCALE: [(&str, f32); 5] = [
    ("slight", 0.98),
    ("subtle", 0.96),
    ("normal", 0.94),
    ("strong", 0.90),
    ("dramatic", 0.84),
];

pub const TILT_SCALE: [(&str, f32); 5] = [
    ("slight", 0.5),
    ("subtle", 1.0),
    ("normal", 2.0),
    ("strong", 4.0),
    ("dramatic", 8.0),
];

pub fn tuned_value(value: Option<&str>, scale: &[(&str, f32)]) -> f32 {
    let value = value.unwrap_or(scale[1].0);
    let (name, percent) = value
        .split_once('%')
        .map_or((value, 0.0), |(name, percent)| {
            (
                name,
                percent.parse::<f32>().unwrap_or(0.0).clamp(0.0, 100.0) / 100.0,
            )
        });
    // t-shirt aliases map onto each five-step scale by position.
    let name = match name {
        "xs" => scale[0].0,
        "sm" => scale[1].0,
        "md" => scale[2].0,
        "lg" => scale[3].0,
        "xl" => scale[4].0,
        other => other,
    };
    let Some(index) = scale.iter().position(|(entry, _)| *entry == name) else {
        return scale[1].1;
    };
    let base = scale[index].1;
    let step = if let Some((_, next)) = scale.get(index + 1) {
        next - base
    } else if index > 0 {
        base - scale[index - 1].1
    } else {
        0.0
    };
    base + (step * percent)
}

pub fn format_px(value: f32) -> String {
    format!("{}px", format_number(value))
}

pub fn format_deg(value: f32) -> String {
    format!("{}deg", format_number(value))
}

pub fn format_number(value: f32) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    let mut formatted = format!("{rounded:.3}");
    while formatted.contains('.') && formatted.ends_with('0') {
        formatted.pop();
    }
    if formatted.ends_with('.') {
        formatted.pop();
    }
    formatted
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::tokens::default_contract;

    #[test]
    fn token_references_resolve_in_value_positions() {
        assert_eq!(
            resolve_value("token(surface.panel)"),
            "var(--frame-surface-panel)"
        );
        assert_eq!(
            space_value("token(space.medium)"),
            "var(--frame-space-medium)"
        );
        assert_eq!(space_value("medium"), "var(--frame-space-medium)");
        assert_eq!(color_value("accent"), "var(--frame-color-accent)");
    }

    #[test]
    fn condition_rules_use_breakpoint_tokens() {
        let contract = default_contract();
        assert_eq!(
            condition_rule("below tablet", &contract).as_deref(),
            Some("@media (max-width: 47.9375rem)")
        );
        assert_eq!(
            condition_rule("above desktop", &contract).as_deref(),
            Some("@media (min-width: 64rem)")
        );
        assert_eq!(
            condition_rule("container content", &contract).as_deref(),
            Some("@container (max-width: 64rem)")
        );
        // Raw lengths are an escape hatch.
        assert_eq!(
            condition_rule("below 40rem", &contract).as_deref(),
            Some("@media (max-width: 39.9375rem)")
        );
    }
}
