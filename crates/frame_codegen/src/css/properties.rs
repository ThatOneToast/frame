use super::*;
use frame_core::Statement;

pub(crate) fn emit_space_property(css: &mut String, property: &str, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  {property}: var(--frame-space-{value});\n"));
    }
}
pub(crate) fn emit_box_space_property(css: &mut String, property: &str, statement: &Statement) {
    match (
        statement.words.get(1).map(String::as_str),
        statement.words.get(2),
    ) {
        (Some(edge @ ("top" | "right" | "bottom" | "left")), Some(value)) => {
            css.push_str(&format!(
                "  {property}-{edge}: var(--frame-space-{value});\n"
            ));
        }
        (Some("x" | "inline"), Some(value)) => {
            css.push_str(&format!(
                "  {property}-inline: var(--frame-space-{value});\n"
            ));
        }
        (Some("y" | "block"), Some(value)) => {
            css.push_str(&format!(
                "  {property}-block: var(--frame-space-{value});\n"
            ));
        }
        (Some(value), _) => {
            css.push_str(&format!("  {property}: var(--frame-space-{value});\n"));
        }
        _ => {}
    }
}
pub(crate) fn emit_size_property(css: &mut String, property: &str, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let css_value = size_css_value(property, value);
        css.push_str(&format!("  {property}: {css_value};\n"));
    }
}
pub(crate) fn emit_display(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  display: {value};\n"));
    }
}
pub(crate) fn emit_visibility(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  visibility: {value};\n"));
    }
}
pub(crate) fn emit_flex(css: &mut String, statement: &Statement) {
    match (
        statement.words.get(1).map(String::as_str),
        statement.words.get(2).map(String::as_str),
    ) {
        (Some("direction"), Some(value)) => {
            css.push_str(&format!("  flex-direction: {value};\n"));
        }
        (Some("wrap"), Some(value)) => {
            css.push_str(&format!("  flex-wrap: {value};\n"));
        }
        (Some("grow"), Some(value)) => {
            css.push_str(&format!("  flex-grow: {value};\n"));
        }
        (Some("shrink"), Some(value)) => {
            css.push_str(&format!("  flex-shrink: {value};\n"));
        }
        (Some("basis"), Some(value)) => {
            css.push_str(&format!(
                "  flex-basis: {};\n",
                size_css_value("width", value)
            ));
        }
        _ => {}
    }
}
pub(crate) fn emit_layout(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("icon-content-action") | Some("composer") => {
            css.push_str("  display: grid;\n");
            css.push_str("  grid-template-columns: auto minmax(0, 1fr) auto;\n");
            css.push_str("  align-items: center;\n");
        }
        Some("avatar-content") => {
            css.push_str("  display: grid;\n");
            css.push_str("  grid-template-columns: 2.5rem minmax(0, 1fr);\n");
        }
        Some("header") => {
            css.push_str("  display: grid;\n");
            css.push_str("  grid-template-columns: 16rem minmax(0, 1fr) auto;\n");
            css.push_str("  align-items: center;\n");
        }
        Some("center") => {
            css.push_str("  display: grid;\n");
            css.push_str("  place-items: center;\n");
        }
        _ => {}
    }
}
pub(crate) fn emit_overflow(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  overflow: {value};\n"));
    }
}
pub(crate) fn emit_scroll(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("x") => css.push_str("  overflow-x: auto;\n"),
        Some("y") => css.push_str("  overflow-y: auto;\n"),
        Some("both") => css.push_str("  overflow: auto;\n"),
        _ => {}
    }
}
pub(crate) fn emit_scrollbar(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("dense") => {
            css.push_str("  scrollbar-width: thin;\n");
            css.push_str("  scrollbar-color: var(--frame-color-muted) transparent;\n");
        }
        Some("normal") => css.push_str("  scrollbar-width: auto;\n"),
        _ => {}
    }
}
pub(crate) fn emit_box(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("border") => css.push_str("  box-sizing: border-box;\n"),
        Some("content") => css.push_str("  box-sizing: content-box;\n"),
        _ => {}
    }
}
pub(crate) fn emit_square(css: &mut String, statement: &Statement) {
    let Some(value) = statement.words.get(1).map(String::as_str) else {
        return;
    };
    let size = match value {
        "server" => "3rem",
        "avatar" | "icon" => "2.5rem",
        "presence" => "0.65rem",
        "unread" => "0.55rem",
        _ => return,
    };
    css.push_str(&format!("  width: {size};\n  height: {size};\n"));
}
pub(crate) fn emit_self(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!(
            "  justify-self: {value};\n  align-self: {value};\n"
        ));
    }
}
pub(crate) fn emit_nudge(css: &mut String, statement: &Statement) {
    if statement.words.get(1).map(String::as_str) == Some("top-right") {
        css.push_str("  top: -0.1rem;\n  right: -0.1rem;\n");
    }
}
pub(crate) fn emit_wrap(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("anywhere") => css.push_str("  overflow-wrap: anywhere;\n"),
        Some("normal") => css.push_str("  overflow-wrap: normal;\n"),
        _ => {}
    }
}
pub(crate) fn emit_text_case(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("uppercase") => css.push_str("  text-transform: uppercase;\n"),
        Some("lowercase") => css.push_str("  text-transform: lowercase;\n"),
        Some("capitalize") => css.push_str("  text-transform: capitalize;\n"),
        Some("normal") => css.push_str("  text-transform: none;\n"),
        _ => {}
    }
}
pub(crate) fn emit_text_align(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  text-align: {value};\n"));
    }
}
pub(crate) fn emit_text_decoration(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  text-decoration-line: {value};\n"));
    }
}
pub(crate) fn emit_white_space(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  white-space: {value};\n"));
    }
}
pub(crate) fn emit_word_break(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  word-break: {value};\n"));
    }
}
pub(crate) fn emit_hyphenate(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        css.push_str(&format!("  hyphens: {value};\n"));
    }
}
pub(crate) fn emit_line(css: &mut String, statement: &Statement) {
    let Some(value) = statement.words.get(1).map(String::as_str) else {
        return;
    };
    let line_height = match value {
        "relaxed" => "1.45",
        "tight" => "1.15",
        "normal" => "1.3",
        _ => return,
    };
    css.push_str(&format!("  line-height: {line_height};\n"));
}
pub(crate) fn emit_letter(css: &mut String, statement: &Statement) {
    if statement.words.get(1).map(String::as_str) == Some("normal") {
        css.push_str("  letter-spacing: 0;\n");
    }
}
pub(crate) fn emit_control(css: &mut String, statement: &Statement) {
    if statement.words.get(1).map(String::as_str) == Some("reset") {
        css.push_str("  appearance: none;\n");
    }
}
pub(crate) fn emit_border(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("none") => css.push_str("  border: 0;\n"),
        Some(edge @ ("top" | "right" | "bottom" | "left")) => {
            let value = statement.words.get(2).map(String::as_str).unwrap_or("soft");
            match value {
                "soft" => {
                    css.push_str(&format!(
                        "  border-{edge}: 1px solid rgba(255, 255, 255, 0.14);\n"
                    ));
                }
                "strong" => {
                    css.push_str(&format!(
                        "  border-{edge}: 1px solid rgba(255, 255, 255, 0.32);\n"
                    ));
                }
                "none" => css.push_str(&format!("  border-{edge}: 0;\n")),
                value => css.push_str(&format!(
                    "  border-{edge}: 1px solid var(--frame-color-{value});\n"
                )),
            }
        }
        Some("radius") => {
            let value = statement
                .words
                .get(2)
                .map(String::as_str)
                .unwrap_or("medium");
            css.push_str(&format!("  border-radius: var(--frame-radius-{value});\n"));
        }
        Some("width") => {
            let width = statement
                .words
                .get(2)
                .map(String::as_str)
                .unwrap_or("small");
            let value = match width {
                "medium" => "2px",
                "large" => "3px",
                _ => "1px",
            };
            css.push_str(&format!(
                "  border-width: {value};\n  border-style: solid;\n"
            ));
        }
        Some("style") => {
            let style = statement
                .words
                .get(2)
                .map(String::as_str)
                .unwrap_or("solid");
            css.push_str(&format!("  border-style: {style};\n"));
        }
        Some("accent" | "muted" | "danger" | "success" | "warning") => {
            let value = statement.words[1].as_str();
            css.push_str(&format!(
                "  border: 1px solid var(--frame-color-{value});\n"
            ));
        }
        Some("soft") => css.push_str("  border: 1px solid rgba(255, 255, 255, 0.14);\n"),
        Some("strong") => css.push_str("  border: 1px solid rgba(255, 255, 255, 0.32);\n"),
        Some(value) => css.push_str(&format!(
            "  border: 1px solid var(--frame-color-{value});\n"
        )),
        _ => {}
    }
}
pub(crate) fn emit_outline(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("none") => css.push_str("  outline: 0;\n"),
        Some("offset") => {
            let value = statement
                .words
                .get(2)
                .map(String::as_str)
                .unwrap_or("small");
            css.push_str(&format!("  outline-offset: var(--frame-space-{value});\n"));
        }
        Some(value) => css.push_str(&format!(
            "  outline: 2px solid var(--frame-color-{value});\n"
        )),
        _ => {}
    }
}
pub(crate) fn emit_transition(css: &mut String, statement: &Statement) {
    let value = statement
        .words
        .get(1)
        .map(String::as_str)
        .unwrap_or("smooth");
    match value {
        "none" => css.push_str("  transition: none;\n"),
        "fast" => css.push_str("  transition: all 120ms ease-out;\n"),
        "slow" => css.push_str("  transition: all 360ms ease;\n"),
        _ => css.push_str("  transition: all 200ms ease;\n"),
    }
}
pub(crate) fn emit_duration(css: &mut String, statement: &Statement) {
    let value = statement
        .words
        .get(1)
        .map(String::as_str)
        .unwrap_or("normal");
    let duration = match value {
        "fast" => "120ms",
        "slow" => "360ms",
        _ => "200ms",
    };
    css.push_str(&format!("  transition-duration: {duration};\n"));
}
pub(crate) fn emit_ease(css: &mut String, statement: &Statement) {
    let value = statement
        .words
        .get(1)
        .map(String::as_str)
        .unwrap_or("smooth");
    let ease = match value {
        "linear" => "linear",
        "bounce" => "cubic-bezier(.2, 1.4, .4, 1)",
        "sharp" => "cubic-bezier(.4, 0, 1, 1)",
        _ => "ease",
    };
    css.push_str(&format!("  transition-timing-function: {ease};\n"));
}
pub(crate) fn emit_animation(css: &mut String, statement: &Statement) {
    let value = statement.words.get(1).map(String::as_str).unwrap_or("none");
    if value == "none" {
        css.push_str("  animation: none;\n");
    } else {
        css.push_str(&format!("  animation: frame-{value} 240ms ease both;\n"));
    }
}
pub(crate) fn emit_position(css: &mut String, statement: &Statement) {
    match statement.words.get(1).map(String::as_str) {
        Some("relative") => css.push_str("  position: relative;\n"),
        Some("absolute") => {
            css.push_str("  position: absolute;\n");
            emit_position_edge(css, statement.words.get(2).map(String::as_str));
        }
        Some("sticky") => {
            css.push_str("  position: sticky;\n");
            emit_position_edge(
                css,
                statement.words.get(2).map(String::as_str).or(Some("top")),
            );
        }
        Some("fixed") => {
            css.push_str("  position: fixed;\n");
            emit_position_edge(
                css,
                statement.words.get(2).map(String::as_str).or(Some("top")),
            );
        }
        Some("center") => css.push_str("  place-items: center;\n"),
        _ => {}
    }
}
pub(crate) fn emit_anchor(css: &mut String, statement: &Statement) {
    let value = statement.words.get(1).map(String::as_str).unwrap_or("top");
    css.push_str("  position: sticky;\n");
    match value {
        "top" => css.push_str("  top: 0;\n"),
        "bottom" => css.push_str("  bottom: 0;\n"),
        "left" => css.push_str("  left: 0;\n"),
        "right" => css.push_str("  right: 0;\n"),
        "top-left" => css.push_str("  top: 0;\n  left: 0;\n"),
        "top-right" => css.push_str("  top: 0;\n  right: 0;\n"),
        "bottom-left" => css.push_str("  bottom: 0;\n  left: 0;\n"),
        "bottom-right" => css.push_str("  bottom: 0;\n  right: 0;\n"),
        _ => css.push_str("  top: 0;\n"),
    }
}
pub(crate) fn emit_position_edge(css: &mut String, edge: Option<&str>) {
    match edge {
        Some("top") => css.push_str("  top: 0;\n"),
        Some("bottom") => css.push_str("  bottom: 0;\n"),
        Some("top-left") => css.push_str("  top: 0;\n  left: 0;\n"),
        Some("top-right") => css.push_str("  top: 0;\n  right: 0;\n"),
        Some("bottom-left") => css.push_str("  bottom: 0;\n  left: 0;\n"),
        Some("bottom-right") => css.push_str("  bottom: 0;\n  right: 0;\n"),
        _ => {}
    }
}
pub(crate) fn emit_z(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let z = match value.as_str() {
            "base" => 0,
            "above" => 10,
            "dropdown" => 40,
            "sticky" => 45,
            "overlay" => 50,
            "modal" => 100,
            "toast" => 110,
            _ => 1,
        };
        css.push_str(&format!("  z-index: {z};\n"));
    }
}
pub(crate) fn emit_type_size(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let size = match value.as_str() {
            "heading" => "2rem",
            "caption" => "0.875rem",
            "body" => "1rem",
            _ => "1rem",
        };
        css.push_str(&format!("  font-size: {size};\n"));
    }
}
pub(crate) fn emit_weight(css: &mut String, statement: &Statement) {
    if let Some(value) = statement.words.get(1) {
        let weight = match value.as_str() {
            "thin" => 300,
            "normal" => 400,
            "semibold" => 600,
            "bold" => 700,
            _ => 400,
        };
        css.push_str(&format!("  font-weight: {weight};\n"));
    }
}
