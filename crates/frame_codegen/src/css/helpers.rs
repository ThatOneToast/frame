use frame_core::{Declaration, DeclarationKind, Identifier, Node, Statement};

pub(crate) fn supports_condition(predicate: &str) -> Option<&'static str> {
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
pub(crate) fn declaration_from_block(block: &frame_core::Block) -> Option<Declaration> {
    let mut parts = block.name.split_whitespace();
    let kind_text = parts.next()?;
    let kind = match kind_text {
        "grid" => DeclarationKind::Grid,
        "area" => DeclarationKind::Area,
        "card" => DeclarationKind::Card,
        "stack" => DeclarationKind::Stack,
        "row" => DeclarationKind::Row,
        "button" => DeclarationKind::Button,
        "text" => DeclarationKind::Text,
        "center" => DeclarationKind::Center,
        "split" => DeclarationKind::Split,
        "overlay" => DeclarationKind::Overlay,
        "dock" => DeclarationKind::Dock,
        "keyframes" => DeclarationKind::Keyframes,
        "html" => DeclarationKind::Html,
        "page-body" => DeclarationKind::Body,
        _ => return None,
    };

    let unnamed = matches!(kind, DeclarationKind::Html | DeclarationKind::Body);
    let name_text = if unnamed {
        kind_text.to_string()
    } else {
        parts.next()?.to_string()
    };

    Some(Declaration {
        kind,
        name: Identifier::new(&name_text, block.span),
        body: block.body.clone(),
        span: block.span,
    })
}
pub(crate) fn style_order_names(order: &str) -> Vec<String> {
    order
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}
pub(crate) fn grid_flow(body: &[Node]) -> Option<&str> {
    find_statement_value(body, "flow")
}
pub(crate) fn grid_section_names(body: &[Node]) -> Vec<String> {
    statements(body)
        .find(|statement| statement.words.first().map(String::as_str) == Some("columns"))
        .map(|statement| {
            statement
                .words
                .iter()
                .skip(1)
                .filter(|name| {
                    !matches!(name.as_str(), "responsive" | "cards" | "auto" | "fill")
                        && !name.ends_with('%')
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}
pub(crate) fn section_block<'a>(body: &'a [Node], section: &str) -> Option<&'a frame_core::Block> {
    body.iter().find_map(|node| {
        let Node::Block(block) = node else {
            return None;
        };
        (block.name == format!("section {section}")).then_some(block)
    })
}
pub(crate) fn is_keyframe_selector(name: &str) -> bool {
    matches!(name, "from" | "to")
        || name
            .strip_suffix('%')
            .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}
pub(crate) fn gradient_css(body: &[Node]) -> Option<String> {
    let mut angle = "180deg".to_string();
    let mut stops = Vec::new();
    let mut corners = Vec::new();

    for statement in statements(body) {
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
                    let color = if color.starts_with('#') {
                        color.clone()
                    } else {
                        format!("var(--frame-color-{color})")
                    };
                    stops.push(format!("{color} {position}"));
                }
            }
            Some("corner") => {
                if let (Some(corner), Some(color)) =
                    (statement.words.get(1), statement.words.get(2))
                {
                    let color = color_css_value(color);
                    let fade = statement.words.get(3).map(String::as_str).unwrap_or("70%");
                    corners.push(format!(
                        "radial-gradient(circle at {}, {color} 0%, transparent {fade})",
                        css_corner(corner)
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
pub(crate) fn color_css_value(color: &str) -> String {
    if color.starts_with('#') {
        color.to_string()
    } else {
        format!("var(--frame-color-{color})")
    }
}
pub(crate) fn css_corner(corner: &str) -> &str {
    match corner {
        "top-left" => "top left",
        "top-right" => "top right",
        "bottom-left" => "bottom left",
        "bottom-right" => "bottom right",
        value => value,
    }
}
pub(crate) fn animation_duration(value: Option<&str>) -> String {
    match value {
        Some("fast") => "120ms".to_string(),
        Some("normal") => "200ms".to_string(),
        Some("slow") => "360ms".to_string(),
        Some(value) if value.ends_with("ms") || value.ends_with('s') => value.to_string(),
        _ => "240ms".to_string(),
    }
}
pub(crate) fn animation_ease(value: Option<&str>) -> String {
    match value {
        Some("linear") => "linear".to_string(),
        Some("bounce") => "cubic-bezier(.2, 1.4, .4, 1)".to_string(),
        Some("sharp") => "cubic-bezier(.4, 0, 1, 1)".to_string(),
        _ => "ease".to_string(),
    }
}
pub(crate) fn condition_rule(name: &str) -> Option<String> {
    let words = name.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["below", breakpoint] => Some(format!(
            "@media (max-width: {})",
            breakpoint_max(breakpoint)
        )),
        ["above", breakpoint] => Some(format!(
            "@media (min-width: {})",
            breakpoint_min(breakpoint)
        )),
        ["between", start, end] => Some(format!(
            "@media (min-width: {}) and (max-width: {})",
            breakpoint_min(start),
            breakpoint_max(end)
        )),
        ["container", name] => Some(format!("@container (max-width: {})", container_width(name))),
        _ => None,
    }
}
pub(crate) fn breakpoint_min(name: &str) -> &str {
    match name {
        "mobile" => "0px",
        "tablet" => "768px",
        "desktop" => "1024px",
        "wide" => "1280px",
        _ => "0px",
    }
}
pub(crate) fn breakpoint_max(name: &str) -> &str {
    match name {
        "mobile" => "767px",
        "tablet" => "1023px",
        "desktop" => "1279px",
        "wide" => "1535px",
        _ => "1023px",
    }
}
pub(crate) fn container_width(name: &str) -> &str {
    match name {
        "narrow" => "42rem",
        "content" => "64rem",
        "wide" => "80rem",
        _ => name,
    }
}
pub(crate) fn tuned_value(value: Option<&str>, scale: &[(&str, f32)]) -> f32 {
    let value = value.unwrap_or(scale[1].0);
    let (name, percent) = value
        .split_once('%')
        .map_or((value, 0.0), |(name, percent)| {
            (
                name,
                percent.parse::<f32>().unwrap_or(0.0).clamp(0.0, 100.0) / 100.0,
            )
        });
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
pub(crate) fn format_px(value: f32) -> String {
    format!("{}px", format_number(value))
}
pub(crate) fn format_deg(value: f32) -> String {
    format!("{}deg", format_number(value))
}
pub(crate) fn format_number(value: f32) -> String {
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
pub(crate) fn size_css_value(property: &str, value: &str) -> String {
    match value {
        "screen" if is_block_axis_property(property) => "100vh".to_string(),
        "screen" if is_inline_axis_property(property) => "100vw".to_string(),
        "fill" => "100%".to_string(),
        "content" => "max-content".to_string(),
        "auto" => "auto".to_string(),
        "sidebar" => "18rem".to_string(),
        "narrow" => "12rem".to_string(),
        "wide" => "32rem".to_string(),
        "zero" if property.starts_with("min-") => "0".to_string(),
        "modal" if property == "width" || property == "inline-size" => {
            "min(42rem, 100%)".to_string()
        }
        "icon" => "2.5rem".to_string(),
        value if is_percentage(value) => value.to_string(),
        value => format!("var(--frame-space-{value})"),
    }
}
pub(crate) fn is_inline_axis_property(property: &str) -> bool {
    property.contains("width") || property.contains("inline-size")
}
pub(crate) fn is_block_axis_property(property: &str) -> bool {
    property.contains("height") || property.contains("block-size")
}
pub(crate) fn column_css_value(value: &str) -> &str {
    match value {
        value if is_percentage(value) => value,
        "auto" => "auto",
        "fill" => "minmax(0, 1fr)",
        "subgrid" => "subgrid",
        _ => "minmax(0, 1fr)",
    }
}
pub(crate) fn track_css_value(value: &str) -> String {
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
        _ => "minmax(0, 1fr)".to_string(),
    }
}
pub(crate) fn is_percentage(value: &str) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}
pub(crate) fn is_identifier_grid_name(value: &str) -> bool {
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
pub(crate) fn surface_value(value: &str) -> bool {
    matches!(
        value,
        "panel" | "main" | "glass" | "flat" | "raised" | "overlay" | "inset" | "sunken"
    )
}
pub(crate) fn css_alignment(value: &str) -> &str {
    match value {
        "start" => "flex-start",
        "end" => "flex-end",
        value => value,
    }
}
pub(crate) fn css_justify(value: &str) -> &str {
    match value {
        "start" => "flex-start",
        "end" => "flex-end",
        "between" => "space-between",
        "around" => "space-around",
        "evenly" => "space-evenly",
        value => value,
    }
}
pub(crate) fn state_selector(name: &str) -> Option<&'static str> {
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
pub(crate) fn find_statement_value<'a>(body: &'a [Node], keyword: &str) -> Option<&'a str> {
    statements(body)
        .find(|statement| statement.words.first().map(String::as_str) == Some(keyword))
        .and_then(|statement| statement.words.get(1))
        .map(String::as_str)
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
