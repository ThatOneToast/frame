use crate::Line;
use frame_core::{DataRef, DeclarationKind, Identifier, Span, StateDefault};

pub fn declaration_kind(kind: &str) -> DeclarationKind {
    // `layout` is also a style property, so it cannot be classified as a
    // declaration in the registry; at declaration position it always means
    // a semantic layout.
    match kind {
        "layout" => return DeclarationKind::Layout,
        "motion" => return DeclarationKind::Motion,
        "recipe" => return DeclarationKind::Recipe,
        _ => {}
    }
    frame_core::language::item(kind)
        .filter(|i| {
            i.kind == frame_core::language::LanguageItemKind::Declaration
                || i.kind == frame_core::language::LanguageItemKind::Primitive
        })
        .map(|_| match kind {
            "grid" => DeclarationKind::Grid,
            "area" => DeclarationKind::Area,
            "card" => DeclarationKind::Card,
            "stack" => DeclarationKind::Stack,
            "row" => DeclarationKind::Row,
            "button" | "action" => DeclarationKind::Button,
            "text" => DeclarationKind::Text,
            "tokens" => DeclarationKind::Tokens,
            "theme" => DeclarationKind::Theme,
            "center" => DeclarationKind::Center,
            "split" => DeclarationKind::Split,
            "overlay" => DeclarationKind::Overlay,
            "dock" => DeclarationKind::Dock,
            "keyframes" => DeclarationKind::Keyframes,
            "supports" => DeclarationKind::Supports,
            "style-group" => DeclarationKind::StyleGroup,
            "style-order" => DeclarationKind::StyleOrder,
            "html" => DeclarationKind::Html,
            "page-body" => DeclarationKind::Body,
            other => DeclarationKind::Unknown(other.to_string()),
        })
        .unwrap_or_else(|| DeclarationKind::Unknown(kind.to_string()))
}

pub fn is_declaration_keyword(kind: &str) -> bool {
    !matches!(declaration_kind(kind), DeclarationKind::Unknown(_))
}

pub fn is_allowed_nested_block(name: &str) -> bool {
    if frame_core::language::state_keywords().contains(&name) {
        return true;
    }

    if matches!(name, "selected" | "advanced" | "from" | "to") {
        return frame_core::language::item(name).is_some();
    }

    let first = name.split_whitespace().next().unwrap_or(name);
    if matches!(
        first,
        "gradient" | "section" | "animation" | "below" | "above" | "between" | "container"
    ) {
        return frame_core::language::item(first).is_some();
    }

    is_percentage_selector(name)
}

pub fn is_percentage_selector(name: &str) -> bool {
    name.strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

pub fn is_semantic_ui_primitive(kind: &str) -> bool {
    frame_core::language::is_ui_primitive(kind)
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
    let stripped = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value);
    decode_string_escapes(stripped)
}

fn decode_string_escapes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('0') => result.push('\0'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if hex.len() == 4 {
                        if let Ok(code_point) = u32::from_str_radix(&hex, 16) {
                            if let Some(ch) = char::from_u32(code_point) {
                                result.push(ch);
                            } else {
                                result.push_str("\\u");
                                result.push_str(&hex);
                            }
                        } else {
                            result.push_str("\\u");
                            result.push_str(&hex);
                        }
                    } else {
                        result.push_str("\\u");
                        result.push_str(&hex);
                    }
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    result
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
