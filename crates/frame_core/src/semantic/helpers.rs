use std::collections::{HashMap, HashSet};

use crate::{Declaration, DeclarationKind, Document, Identifier, Node, Statement};

pub(crate) fn closest<'a>(needle: &str, candidates: &'a [&str]) -> Option<&'a str> {
    candidates
        .iter()
        .copied()
        .map(|candidate| (candidate, edit_distance(needle, candidate)))
        .filter(|(_, distance)| *distance <= 2)
        .min_by_key(|(_, distance)| *distance)
        .map(|(candidate, _)| candidate)
}

pub(crate) fn edit_distance(left: &str, right: &str) -> usize {
    let mut costs = (0..=right.len()).collect::<Vec<_>>();

    for (left_index, left_char) in left.chars().enumerate() {
        let mut previous = left_index;
        costs[0] = left_index + 1;

        for (right_index, right_char) in right.chars().enumerate() {
            let old = costs[right_index + 1];
            costs[right_index + 1] = if left_char == right_char {
                previous
            } else {
                1 + previous.min(costs[right_index]).min(old)
            };
            previous = old;
        }
    }

    *costs.last().unwrap_or(&0)
}

pub(crate) fn semantic_alternative_for(browser_word: &str) -> Option<(&'static str, &'static str)> {
    crate::semantic::constants::BROWSER_TO_SEMANTIC
        .iter()
        .find(|(word, _, _)| *word == browser_word)
        .map(|(_, alt, explain)| (*alt, *explain))
}

pub(crate) fn is_valid_style_identifier(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && name.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

#[allow(dead_code)]
pub(crate) fn collect_grids(document: &Document) -> HashMap<String, HashSet<String>> {
    document
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Grid)
        .map(|declaration| {
            let columns = declaration
                .body
                .iter()
                .filter_map(statement)
                .find(|statement| first_word(statement) == Some("columns"))
                .map(|statement| {
                    statement
                        .words
                        .iter()
                        .skip(1)
                        .filter(|word| !matches!(word.as_str(), "responsive" | "cards" | "subgrid"))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();

            (declaration.name.text.clone(), columns)
        })
        .collect()
}

#[allow(dead_code)]
pub(crate) fn collect_custom_colors(document: &Document) -> HashSet<String> {
    let mut colors = HashSet::new();
    for declaration in &document.declarations {
        if declaration.kind != DeclarationKind::Tokens {
            continue;
        }
        for node in &declaration.body {
            let Some(statement) = statement(node) else {
                continue;
            };
            if first_word(statement) == Some("color") {
                if let Some(name) = statement.words.get(1) {
                    colors.insert(name.clone());
                }
            }
        }
    }
    colors
}

pub(crate) fn singular_item_name(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    lower
        .strip_suffix("ies")
        .map(|prefix| format!("{prefix}y"))
        .or_else(|| lower.strip_suffix('s').map(ToOwned::to_owned))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "item".to_string())
}

pub(crate) fn first_word(statement: &Statement) -> Option<&str> {
    statement.words.first().map(String::as_str)
}

pub(crate) fn statement(node: &Node) -> Option<&Statement> {
    if let Node::Statement(statement) = node {
        Some(statement)
    } else {
        None
    }
}

pub(crate) fn find_statement_value<'a>(
    declaration: &'a Declaration,
    keyword: &str,
) -> Option<&'a str> {
    declaration
        .body
        .iter()
        .filter_map(statement)
        .find(|statement| first_word(statement) == Some(keyword))
        .and_then(|statement| statement.words.get(1))
        .map(String::as_str)
}

pub(crate) fn is_valid_percentage(value: &str) -> bool {
    let Some(number) = value.strip_suffix('%') else {
        return false;
    };

    if number.is_empty()
        || number.starts_with('-')
        || number.contains('%')
        || !number.chars().all(|character| character.is_ascii_digit())
    {
        return false;
    }

    number.parse::<u8>().is_ok_and(|value| value <= 100)
}

pub(crate) fn is_time_value(value: &str) -> bool {
    value
        .strip_suffix("ms")
        .or_else(|| value.strip_suffix('s'))
        .is_some_and(|number| {
            !number.is_empty()
                && number
                    .chars()
                    .all(|character| character.is_ascii_digit() || character == '.')
        })
}

pub(crate) fn is_condition_block(name: &str) -> bool {
    name.starts_with("below ")
        || name.starts_with("above ")
        || name.starts_with("between ")
        || name.starts_with("container ")
}

pub(crate) fn is_keyframe_selector(name: &str) -> bool {
    matches!(name, "from" | "to")
        || name
            .strip_suffix('%')
            .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

pub(crate) fn is_valid_angle(value: &str) -> bool {
    value
        .strip_suffix("deg")
        .is_some_and(|number| !number.is_empty() && number.parse::<i16>().is_ok())
}

pub(crate) fn is_hex_color(value: &str) -> bool {
    let Some(hex) = value.strip_prefix('#') else {
        return false;
    };

    matches!(hex.len(), 3 | 6 | 8) && hex.chars().all(|character| character.is_ascii_hexdigit())
}

pub(crate) fn css_property_alias(property: &str) -> Option<&'static str> {
    match property {
        "justify-content" | "justify-items" | "place-content" => Some("justify"),
        "align-items" | "align-content" | "place-items" => Some("align"),
        "grid-template-columns" => Some("columns"),
        "grid-template-rows" => Some("rows"),
        "grid-area" => Some("place"),
        "grid-column" => Some("col"),
        "grid-row" => Some("row"),
        "box-sizing" => Some("box"),
        "inline-size" => Some("inline-size"),
        "block-size" => Some("block-size"),
        "min-inline-size" => Some("min-inline-size"),
        "max-inline-size" => Some("max-inline-size"),
        "min-block-size" => Some("min-block-size"),
        "max-block-size" => Some("max-block-size"),
        "display" | "visibility" | "flex-direction" | "flex-wrap" | "flex-grow" | "flex-shrink"
        | "flex-basis" => Some(match property {
            "display" => "display",
            "visibility" => "visibility",
            "flex-direction" => "flex direction",
            "flex-wrap" => "flex wrap",
            "flex-grow" => "flex grow",
            "flex-shrink" => "flex shrink",
            "flex-basis" => "flex basis",
            _ => "display",
        }),
        "background-color" => Some("background"),
        "color" => Some("text"),
        "border-radius" => Some("radius"),
        "box-shadow" => Some("shadow"),
        "font-size" => Some("size"),
        "font-weight" => Some("weight"),
        "text-decoration" | "text-decoration-line" => Some("decoration"),
        "white-space" => Some("whitespace"),
        "word-break" => Some("word-break"),
        "hyphens" => Some("hyphenate"),
        "z-index" => Some("z"),
        _ => None,
    }
}

pub(crate) fn alias_example_value(alias: &str) -> &'static str {
    match alias {
        "columns" => "sidebar content",
        "rows" => "header content footer",
        "place" => "content",
        "col" | "row" => "1",
        "background" => "panel",
        "radius" => "large",
        "shadow" => "medium",
        "size" => "heading",
        "weight" => "semibold",
        "decoration" => "underline",
        "whitespace" => "pre-wrap",
        "word-break" => "break-word",
        "hyphenate" => "auto",
        "z" => "modal",
        "align" | "justify" => "center",
        "display" => "block",
        "visibility" => "visible",
        "box" => "border",
        "inline-size" | "block-size" => "fill",
        "min-inline-size" | "min-block-size" => "zero",
        "max-inline-size" | "max-block-size" => "fill",
        "flex direction" => "row",
        "flex wrap" => "wrap",
        "flex grow" | "flex shrink" => "1",
        "flex basis" => "fill",
        "text" => "bright",
        _ => "center",
    }
}

pub(crate) fn declaration_from_block(block: &crate::Block) -> Option<Declaration> {
    let mut parts = block.name.split_whitespace();
    let kind_text = parts.next()?;
    let name_text = parts.next()?;
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
        "tokens" => DeclarationKind::Tokens,
        "keyframes" => DeclarationKind::Keyframes,
        "supports" => DeclarationKind::Supports,
        "style-group" => DeclarationKind::StyleGroup,
        "style-order" => DeclarationKind::StyleOrder,
        other => DeclarationKind::Unknown(other.to_string()),
    };

    Some(Declaration {
        kind,
        name: Identifier::new(name_text, block.span),
        extends: None,
        body: block.body.clone(),
        span: block.span,
    })
}

pub(crate) fn valid_properties_for_primitive(kind: &str) -> &'static [&'static str] {
    match kind {
        "input" | "editor" => &[
            "value",
            "placeholder",
            "label",
            "disabled",
            "readonly",
            "style",
            "show",
            "hidden",
        ],
        "toggle" | "choice" => &[
            "checked", "selected", "label", "disabled", "readonly", "style", "show", "hidden",
        ],
        "select" => &[
            "selected", "options", "label", "disabled", "readonly", "style", "show", "hidden",
        ],
        "field" => &[
            "label",
            "description",
            "hint",
            "style",
            "show",
            "hidden",
            "disabled",
        ],
        "action" | "link" => &[
            "text",
            "label",
            "goto",
            "disabled",
            "style",
            "show",
            "hidden",
            "new-window",
        ],
        "composer" => &[
            "label", "draft", "send", "disabled", "style", "show", "hidden",
        ],
        "media" | "image" | "avatar" => &[
            "source",
            "sources",
            "alt",
            "description",
            "decorative",
            "poster",
            "style",
            "show",
            "hidden",
        ],
        "icon" => &["label", "decorative", "style", "show", "hidden"],
        "list" | "feed" | "data" => &["source", "style", "show", "hidden"],
        "text" | "title" | "label" | "badge" => &["value", "style", "show", "hidden"],
        "card" | "panel" | "stack" | "row" | "screen" | "section" | "dialog" | "popover"
        | "split" | "dock" | "overlay" | "scroll" | "grid" | "menu" | "toolbar" | "tabs" => {
            &["style", "show", "hidden"]
        }
        "item" | "empty" => &["style", "show", "hidden"],
        _ => &[],
    }
}

pub(crate) fn primitive_kind_label(kind: &str) -> &'static str {
    match kind {
        "input" | "editor" => "input-like primitive",
        "toggle" | "choice" | "select" => "selection primitive",
        "field" => "field",
        "action" | "link" => "action-like primitive",
        "composer" => "composer",
        "media" | "image" | "avatar" => "media-like primitive",
        "icon" => "icon",
        "list" | "feed" | "data" => "collection primitive",
        "text" | "title" | "label" | "badge" => "text-like primitive",
        "card" | "panel" | "stack" | "row" | "screen" | "section" | "dialog" | "popover"
        | "split" | "dock" | "overlay" | "scroll" | "grid" | "menu" | "toolbar" | "tabs" => {
            "container primitive"
        }
        "item" | "empty" => "collection item",
        _ => "primitive",
    }
}

pub(crate) fn is_style_declaration_kind(kind: &DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::Grid
            | DeclarationKind::Area
            | DeclarationKind::Card
            | DeclarationKind::Stack
            | DeclarationKind::Row
            | DeclarationKind::Button
            | DeclarationKind::Text
            | DeclarationKind::Center
            | DeclarationKind::Split
            | DeclarationKind::Overlay
            | DeclarationKind::Dock
    )
}

pub(crate) fn style_order_names(order: &str) -> Vec<String> {
    order
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}
