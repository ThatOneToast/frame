use crate::{helpers::*, Line, ParseError, Parser};
use frame_core::{
    ConditionalBinding, DataRef, EventBinding, HandlerRef, Identifier, Include, Span, StyleBinding,
    TextValue, UiComponentArgument, UiComponentArgumentValue, UiComponentInvocation, UiElement,
    UiNode, UiProperty, UiPropertyValue, UiText,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_ui_element(&mut self) -> Result<UiElement, ParseError> {
        let line = self.current_line().expect("parse_ui_element needs a line");
        let content = content_without_comment(line.text);
        let header = content.trim_end_matches('{').trim();
        let parts = split_frame_words(header);
        if parts.len() > 2 || parts.is_empty() {
            return Err(ParseError::one(
                "UI primitives use `primitive Name {`, `primitive Name:StyleName {`, or `item {`",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }
        let kind_text = &parts[0];
        let default_name = default_ui_node_name(kind_text);
        let name_and_style = parts.get(1).map_or(default_name, String::as_str);
        let (name_text, style) = name_and_style
            .split_once(':')
            .map(|(name, style)| (name, Some(style)))
            .unwrap_or((name_and_style, None));
        let kind_span = word_span_in_line(line, kind_text);
        let name_span = word_span_in_line(line, name_text);
        let style_binding = style.map(|style_name| {
            let span = word_span_in_line(line, style_name);
            StyleBinding {
                name: Identifier::new(style_name, span),
                span,
            }
        });

        self.advance();
        let mut properties = Vec::new();
        let mut events = Vec::new();
        let mut children = Vec::new();
        while let Some(child_line) = self.current_line() {
            let child = content_without_comment(child_line.text);
            if child.is_empty() {
                self.advance();
                continue;
            }
            if child == "}" {
                self.advance();
                let end = self
                    .previous_line()
                    .map(|line| line.end)
                    .unwrap_or(child_line.end);
                return Ok(UiElement {
                    kind: Identifier::new(kind_text, kind_span),
                    name: Identifier::new(name_text, name_span),
                    style: style_binding,
                    properties,
                    events,
                    children,
                    span: Span {
                        start: line.start,
                        end,
                    },
                });
            }
            if child.starts_with("for ") && child.ends_with('{') {
                children.push(UiNode::Loop(self.parse_ui_for_loop()?));
                continue;
            }
            let words = split_frame_words(child);
            match words.first().map(String::as_str) {
                Some("text") => {
                    children.push(UiNode::Text(parse_ui_text(child_line, &words)?));
                    self.advance();
                }
                _ if child.ends_with('{') => {
                    children.push(UiNode::Element(self.parse_ui_element()?));
                }
                _ if looks_like_semantic_shorthand(child) => {
                    children.push(UiNode::Element(parse_ui_element_shorthand(
                        child_line, child,
                    )?));
                    self.advance();
                }
                _ if looks_like_component_invocation(child) => {
                    children.push(UiNode::Component(parse_component_invocation(
                        child_line, child,
                    )?));
                    self.advance();
                }
                Some("on") => {
                    events.push(parse_event_binding(child_line, &words)?);
                    self.advance();
                }
                Some(_) => {
                    properties.push(parse_ui_property(child_line, &words)?);
                    self.advance();
                }
                None => self.advance(),
            }
        }
        Err(ParseError::one(
            "missing closing `}`",
            start_line_span(line),
        ))
    }

    pub(crate) fn parse_ui_for_loop(&mut self) -> Result<frame_core::UiForLoop, ParseError> {
        let line = self.current_line().expect("parse_ui_for_loop needs a line");
        let content = content_without_comment(line.text);
        let header = content.trim_end_matches('{').trim();
        let words = split_frame_words(header);
        let valid_unkeyed =
            words.len() == 4 && words[0] == "for" && words[2] == "in" && words[3].starts_with('$');
        let valid_keyed = words.len() == 6
            && words[0] == "for"
            && words[2] == "in"
            && words[3].starts_with('$')
            && words[4] == "key"
            && words[5].starts_with('$');
        if !valid_unkeyed && !valid_keyed {
            return Err(ParseError::one(
                "for loops use `for item in $items {` or `for item in $items key $key {`",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let item_text = &words[1];
        let collection_text = words[3].trim_start_matches('$');
        let item_span = word_span_in_line(line, item_text);
        let collection_span = word_span_in_line(line, &words[3]);
        let key = if valid_keyed {
            let key_text = words[5].trim_start_matches('$');
            let key_span = word_span_in_line(line, &words[5]);
            Some(DataRef {
                name: Identifier::new(key_text, key_span),
                span: key_span,
            })
        } else {
            None
        };

        self.advance();
        let children = self.parse_ui_nodes_until_close()?;
        let end = self
            .previous_line()
            .map(|line| line.end)
            .unwrap_or(line.end);
        Ok(frame_core::UiForLoop {
            item: Identifier::new(item_text, item_span),
            collection: DataRef {
                name: Identifier::new(collection_text, collection_span),
                span: collection_span,
            },
            key,
            children,
            span: Span {
                start: line.start,
                end,
            },
        })
    }

    pub(crate) fn parse_include(&mut self) -> Result<Include, ParseError> {
        let line = self.current_line().expect("parse_include needs a line");
        let content = content_without_comment(line.text);
        let mut parts = content.split_whitespace();
        let keyword = parts.next().unwrap_or_default();

        if keyword != "#include" {
            return Err(ParseError::one(
                format!("expected `#include`, found `{keyword}`"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let target = parts.next().ok_or_else(|| {
            ParseError::one(
                "#include expects a style name or .frame path",
                Span {
                    start: line.start,
                    end: line.end,
                },
            )
        })?;

        if parts.next().is_some() {
            return Err(ParseError::one(
                "#include accepts exactly one target",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let target_start = line.start + line.text.find(target).unwrap_or(0);
        self.advance();

        Ok(Include {
            target: target.to_string(),
            span: Span {
                start: line.start,
                end: line.end,
            },
            target_span: Span {
                start: target_start,
                end: target_start + target.len(),
            },
        })
    }
}

pub fn parse_ui_text(line: Line<'_>, words: &[String]) -> Result<UiText, ParseError> {
    if words.len() != 2 {
        return Err(ParseError::one(
            "text nodes use `text \"literal\"` or `text $value`",
            Span {
                start: line.start,
                end: line.end,
            },
        ));
    }
    let value = &words[1];
    Ok(UiText {
        value: if let Some(name) = value.strip_prefix('$') {
            TextValue::Data(data_ref(line, value, name))
        } else {
            TextValue::Literal(unquote(value))
        },
        span: Span {
            start: line.start,
            end: line.end,
        },
    })
}

pub fn parse_ui_element_shorthand(line: Line<'_>, content: &str) -> Result<UiElement, ParseError> {
    let parts = split_frame_words(content);
    if parts.len() != 2 {
        return Err(ParseError::one(
            "semantic UI shorthand uses `primitive Name` or `title \"Text\"`",
            start_line_span(line),
        ));
    }

    let kind_text = &parts[0];
    let name_token = &parts[1];
    let (name_text, style) = name_token
        .split_once(':')
        .map(|(name, style)| (name, Some(style)))
        .unwrap_or((name_token.as_str(), None));
    let kind_span = word_span_in_line(line, kind_text);
    let name_span = word_span_in_line(line, name_text);
    let mut properties = Vec::new();
    let name = if name_token.starts_with('"') || name_token.starts_with('$') {
        properties.push(UiProperty {
            name: Identifier::new("value", name_span),
            value: if let Some(reference) = name_token.strip_prefix('$') {
                UiPropertyValue::Data(data_ref(line, name_token, reference))
            } else {
                UiPropertyValue::Literal(unquote(name_token))
            },
            span: Span {
                start: line.start,
                end: line.end,
            },
        });
        default_ui_node_name(kind_text)
    } else {
        name_text
    };

    let style_binding = style.map(|style_name| {
        let span = word_span_in_line(line, style_name);
        StyleBinding {
            name: Identifier::new(style_name, span),
            span,
        }
    });

    Ok(UiElement {
        kind: Identifier::new(kind_text, kind_span),
        name: Identifier::new(name, word_span_in_line(line, name)),
        style: style_binding,
        properties,
        events: Vec::new(),
        children: Vec::new(),
        span: Span {
            start: line.start,
            end: line.end,
        },
    })
}

pub fn parse_component_invocation(
    line: Line<'_>,
    content: &str,
) -> Result<UiComponentInvocation, ParseError> {
    let Some(open_paren) = content.find('(') else {
        return Err(ParseError::one(
            "component invocation uses `ComponentName()`",
            start_line_span(line),
        ));
    };
    let Some(args_text) = content.strip_suffix(')') else {
        return Err(ParseError::one(
            "component invocation must end with `)`",
            start_line_span(line),
        ));
    };
    let name = content[..open_paren].trim();
    if name.is_empty() {
        return Err(ParseError::one(
            "component invocation expects a component name",
            start_line_span(line),
        ));
    }
    let args_text = &args_text[open_paren + 1..];
    let arguments = if args_text.trim().is_empty() {
        Vec::new()
    } else {
        args_text
            .split(',')
            .map(str::trim)
            .map(|argument| parse_component_argument(line, argument))
            .collect::<Result<Vec<_>, _>>()?
    };

    Ok(UiComponentInvocation {
        name: Identifier::new(name, word_span_in_line(line, name)),
        arguments,
        span: Span {
            start: line.start,
            end: line.end,
        },
    })
}

fn parse_component_argument(
    line: Line<'_>,
    argument: &str,
) -> Result<UiComponentArgument, ParseError> {
    let words = split_frame_words(argument);
    match words.as_slice() {
        [name, bind, value] if bind == "bind" && value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            Ok(UiComponentArgument {
                name: Identifier::new(name, word_span_in_line(line, name)),
                value: UiComponentArgumentValue::Bind(data_ref(line, value, state)),
                span: word_span_in_line(line, argument),
            })
        }
        [name_colon, value] if name_colon.ends_with(':') && value.starts_with('$') => {
            let name = name_colon.trim_end_matches(':');
            let state = value.trim_start_matches('$');
            Ok(UiComponentArgument {
                name: Identifier::new(name, word_span_in_line(line, name)),
                value: UiComponentArgumentValue::Data(data_ref(line, value, state)),
                span: word_span_in_line(line, argument),
            })
        }
        [name_colon, value] if name_colon.ends_with(':') => {
            let name = name_colon.trim_end_matches(':');
            Ok(UiComponentArgument {
                name: Identifier::new(name, word_span_in_line(line, name)),
                value: UiComponentArgumentValue::Literal(unquote(value)),
                span: word_span_in_line(line, argument),
            })
        }
        _ => Err(ParseError::one(
            "component arguments use `name: $value`, `name: \"literal\"`, or `name bind $value`",
            start_line_span(line),
        )),
    }
}

fn parse_event_binding(line: Line<'_>, words: &[String]) -> Result<EventBinding, ParseError> {
    if words.len() != 3 || !words[2].starts_with('@') {
        return Err(ParseError::one(
            "events use `on event[.modifier...] @handler`",
            Span {
                start: line.start,
                end: line.end,
            },
        ));
    }
    let mut event_parts = words[1].split('.');
    let event_text = event_parts.next().unwrap_or_default();
    let event_span = word_span_in_line(line, event_text);
    let modifiers = event_parts
        .map(|modifier| Identifier::new(modifier, word_span_in_line(line, modifier)))
        .collect();
    let handler_token = &words[2];
    let handler_name = handler_token.trim_start_matches('@');
    let handler_span = word_span_in_line(line, handler_token);
    Ok(EventBinding {
        event: Identifier::new(event_text, event_span),
        modifiers,
        handler: HandlerRef {
            name: Identifier::new(handler_name, word_span_in_line(line, handler_name)),
            span: handler_span,
        },
        span: Span {
            start: line.start,
            end: line.end,
        },
    })
}

fn parse_ui_property(line: Line<'_>, words: &[String]) -> Result<UiProperty, ParseError> {
    let Some(name) = words.first() else {
        return Err(ParseError::one("missing property", start_line_span(line)));
    };
    let (name, value) = match words {
        [bind, value] if bind == "bind" && value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            ("value", UiPropertyValue::Bind(data_ref(line, value, state)))
        }
        [_, value] if value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            (
                name.as_str(),
                UiPropertyValue::Data(data_ref(line, value, state)),
            )
        }
        [_, value] if value.starts_with('@') => {
            let handler_name = value.trim_start_matches('@');
            let handler_span = word_span_in_line(line, value);
            (
                name.as_str(),
                UiPropertyValue::Handler(HandlerRef {
                    name: Identifier::new(handler_name, word_span_in_line(line, handler_name)),
                    span: handler_span,
                }),
            )
        }
        [_, value] => (name.as_str(), UiPropertyValue::Literal(unquote(value))),
        [_, bind, value] if bind == "bind" && value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            (
                name.as_str(),
                UiPropertyValue::Bind(data_ref(line, value, state)),
            )
        }
        [_, when, value] if when == "when" && value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            let reference = data_ref(line, value, state);
            (
                name.as_str(),
                UiPropertyValue::Conditional(ConditionalBinding {
                    condition: reference,
                    span: Span {
                        start: line.start,
                        end: line.end,
                    },
                }),
            )
        }
        [property, when, value, equals, style]
            if property == "style" && when == "when" && value.starts_with('$') && equals == "=" =>
        {
            let state = value.trim_start_matches('$');
            let style_span = word_span_in_line(line, style);
            (
                name.as_str(),
                UiPropertyValue::StyleWhen {
                    condition: data_ref(line, value, state),
                    style: StyleBinding {
                        name: Identifier::new(style, style_span),
                        span: style_span,
                    },
                },
            )
        }
        _ => (
            name.as_str(),
            UiPropertyValue::Unknown(words.iter().skip(1).cloned().collect()),
        ),
    };
    Ok(UiProperty {
        name: Identifier::new(name, word_span_in_line(line, name)),
        value,
        span: Span {
            start: line.start,
            end: line.end,
        },
    })
}
