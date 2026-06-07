use crate::Line;
use frame_core::{DataRef, DeclarationKind, Identifier, Span, StateDefault};

pub fn declaration_kind(kind: &str) -> DeclarationKind {
    match kind {
        "grid" => DeclarationKind::Grid,
        "area" => DeclarationKind::Area,
        "card" => DeclarationKind::Card,
        "stack" => DeclarationKind::Stack,
        "row" => DeclarationKind::Row,
        "button" => DeclarationKind::Button,
        "text" => DeclarationKind::Text,
        "tokens" => DeclarationKind::Tokens,
        "center" => DeclarationKind::Center,
        "split" => DeclarationKind::Split,
        "overlay" => DeclarationKind::Overlay,
        "dock" => DeclarationKind::Dock,
        "keyframes" => DeclarationKind::Keyframes,
        "supports" => DeclarationKind::Supports,
        "style-group" => DeclarationKind::StyleGroup,
        "style-order" => DeclarationKind::StyleOrder,
        other => DeclarationKind::Unknown(other.to_string()),
    }
}

pub fn is_declaration_keyword(kind: &str) -> bool {
    !matches!(declaration_kind(kind), DeclarationKind::Unknown(_))
}

pub fn is_allowed_nested_block(name: &str) -> bool {
    matches!(
        name,
        "hover"
            | "focus"
            | "focus-visible"
            | "focus-within"
            | "active"
            | "disabled"
            | "checked"
            | "selected"
            | "invalid"
            | "required"
            | "target"
            | "advanced"
    ) || name == "gradient"
        || name.starts_with("gradient ")
        || name.starts_with("section ")
        || name.starts_with("animation ")
        || matches!(name, "from" | "to")
        || is_percentage_selector(name)
        || name.starts_with("below ")
        || name.starts_with("above ")
        || name.starts_with("between ")
        || name.starts_with("container ")
}

pub fn is_percentage_selector(name: &str) -> bool {
    name.strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

pub fn is_semantic_ui_primitive(kind: &str) -> bool {
    matches!(
        kind,
        "screen"
            | "panel"
            | "section"
            | "stack"
            | "row"
            | "grid"
            | "split"
            | "dock"
            | "overlay"
            | "scroll"
            | "action"
            | "link"
            | "menu"
            | "toolbar"
            | "tabs"
            | "input"
            | "editor"
            | "toggle"
            | "choice"
            | "select"
            | "composer"
            | "title"
            | "text"
            | "label"
            | "badge"
            | "avatar"
            | "icon"
            | "image"
            | "list"
            | "feed"
            | "data"
            | "item"
            | "empty"
            | "card"
            | "dialog"
            | "popover"
    )
}

pub fn default_ui_node_name(kind: &str) -> &str {
    match kind {
        "item" => "Item",
        "empty" => "Empty",
        "title" => "Title",
        "text" => "Text",
        "label" => "Label",
        _ => kind,
    }
}

pub fn parse_state_default(value: &str) -> StateDefault {
    if value.starts_with('"') && value.ends_with('"') {
        StateDefault::Text(unquote(value))
    } else if value == "true" {
        StateDefault::Bool(true)
    } else if value == "false" {
        StateDefault::Bool(false)
    } else if is_number_literal(value) {
        StateDefault::Number(value.to_string())
    } else if value == "[]" {
        StateDefault::List
    } else {
        StateDefault::Invalid(value.to_string())
    }
}

pub fn data_ref(line: Line<'_>, token: &str, name: &str) -> DataRef {
    let span = word_span_in_line(line, token);
    DataRef {
        name: Identifier::new(name, word_span_in_line(line, name)),
        span,
    }
}

pub fn split_frame_words(content: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    for ch in content.chars() {
        if ch == '"' {
            current.push(ch);
            in_string = !in_string;
            continue;
        }
        if ch.is_whitespace() && !in_string {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        current.push(ch);
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

pub fn unquote(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
        .to_string()
}

pub fn is_number_literal(value: &str) -> bool {
    let mut chars = value.chars();
    if matches!(chars.clone().next(), Some('-')) {
        chars.next();
    }
    let rest = chars.as_str();
    !rest.is_empty()
        && rest.chars().filter(|ch| *ch == '.').count() <= 1
        && rest.chars().all(|ch| ch.is_ascii_digit() || ch == '.')
}

pub fn word_span_in_line(line: Line<'_>, word: &str) -> Span {
    let relative = line.text.find(word).unwrap_or(0);
    Span {
        start: line.start + relative,
        end: line.start + relative + word.len(),
    }
}

pub fn start_line_span(line: Line<'_>) -> Span {
    Span {
        start: line.start,
        end: line.end,
    }
}

pub fn source_lines(source: &str) -> Vec<Line<'_>> {
    let mut offset = 0;
    source
        .lines()
        .map(|line| {
            let start = offset;
            let end = start + line.len();
            offset = end + 1;

            Line {
                text: line,
                start,
                end,
            }
        })
        .collect()
}

pub fn content_without_comment(line: &str) -> &str {
    line.split_once("//")
        .map(|(before_comment, _)| before_comment)
        .unwrap_or(line)
        .trim()
}

pub fn looks_like_component_invocation(content: &str) -> bool {
    let Some(open_paren) = content.find('(') else {
        return false;
    };
    content.ends_with(')')
        && content[..open_paren]
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_uppercase())
}

pub fn looks_like_semantic_shorthand(content: &str) -> bool {
    let words = split_frame_words(content);
    words.len() == 2 && is_semantic_ui_primitive(&words[0])
}
