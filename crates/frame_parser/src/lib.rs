//! Parser for the first Frame language slice.
//!
//! The parser is deliberately small and line-oriented for the MVP. It produces
//! a shared AST plus structured diagnostics that can be reused by the CLI and
//! future editor tooling.

use std::{error::Error, fmt};

use frame_core::{
    Block, ComponentDecl, ConditionalBinding, DataRef, Declaration, DeclarationKind, Diagnostic,
    Document, EventBinding, HandlerRef, Identifier, Include, Node, Severity, Span, StateDecl,
    StateDefault, StateType, StateValue, Statement, StyleBinding, TextValue, UiElement, UiNode,
    UiProperty, UiPropertyValue, UiText, ViewDecl,
};

pub fn parse(source: &str) -> Result<Document, ParseError> {
    Parser::new(source).parse_document()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub diagnostics: Vec<Diagnostic>,
}

impl ParseError {
    fn one(message: impl Into<String>, span: Span) -> Self {
        Self {
            diagnostics: vec![Diagnostic::error(message, span)],
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = self
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.message.as_str())
            .unwrap_or("parse failed");

        write!(formatter, "{message}")
    }
}

impl Error for ParseError {}

struct Parser<'a> {
    lines: Vec<Line<'a>>,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
struct Line<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            lines: source_lines(source),
            index: 0,
        }
    }

    fn parse_document(&mut self) -> Result<Document, ParseError> {
        let mut includes = Vec::new();
        let mut declarations = Vec::new();
        let mut components = Vec::new();

        while let Some(line) = self.current_content() {
            if line.is_empty() {
                self.advance();
                continue;
            }

            if line.starts_with("#include") {
                includes.push(self.parse_include()?);
                continue;
            }

            if line.starts_with("supports ") {
                declarations.push(self.parse_supports_declaration()?);
                continue;
            }

            if line.starts_with("style-group ") {
                declarations.push(
                    self.parse_wrapped_style_declaration(
                        "style-group",
                        DeclarationKind::StyleGroup,
                    )?,
                );
                continue;
            }

            if line.starts_with("style-order ") {
                declarations.push(self.parse_style_order()?);
                continue;
            }

            if line.starts_with("component ") {
                components.push(self.parse_component()?);
                continue;
            }

            declarations.push(self.parse_declaration()?);
        }

        Ok(Document {
            includes,
            declarations,
            components,
        })
    }

    fn parse_component(&mut self) -> Result<ComponentDecl, ParseError> {
        let line = self.current_line().expect("parse_component needs a line");
        let content = content_without_comment(line.text);
        if !content.ends_with('{') {
            return Err(ParseError::one(
                format!("expected component ending with `{{`, found `{content}`"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let header = content.trim_end_matches('{').trim();
        let mut parts = header.split_whitespace();
        let keyword = parts.next().unwrap_or_default();
        if keyword != "component" {
            return Err(ParseError::one(
                format!("expected `component`, found `{keyword}`"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }
        let name_text = parts.next().ok_or_else(|| {
            ParseError::one(
                "component expects a name",
                Span {
                    start: line.start,
                    end: line.end,
                },
            )
        })?;
        if parts.next().is_some() {
            return Err(ParseError::one(
                "component accepts exactly one name",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let name_start = line.start + line.text.find(name_text).unwrap_or(0);
        self.advance();
        let mut state = None;
        let mut view = None;
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
                    .unwrap_or(line.end);
                return Ok(ComponentDecl {
                    name: Identifier::new(
                        name_text,
                        Span {
                            start: name_start,
                            end: name_start + name_text.len(),
                        },
                    ),
                    state,
                    view,
                    span: Span {
                        start: line.start,
                        end,
                    },
                });
            }
            match child {
                "state {" => state = Some(self.parse_state_decl()?),
                "view {" => view = Some(self.parse_view_decl()?),
                _ => {
                    return Err(ParseError::one(
                        format!(
                            "unknown component block `{child}`\n\nComponents currently support `state {{ ... }}` and `view {{ ... }}`."
                        ),
                        Span {
                            start: child_line.start,
                            end: child_line.end,
                        },
                    ));
                }
            }
        }

        Err(ParseError::one(
            "missing closing `}`",
            self.previous_line()
                .map(|line| Span {
                    start: line.start,
                    end: line.end,
                })
                .unwrap_or_default(),
        ))
    }

    fn parse_state_decl(&mut self) -> Result<StateDecl, ParseError> {
        let start_line = self.current_line().expect("parse_state_decl needs a line");
        self.advance();
        let mut values = Vec::new();
        while let Some(line) = self.current_line() {
            let content = content_without_comment(line.text);
            if content.is_empty() {
                self.advance();
                continue;
            }
            if content == "}" {
                self.advance();
                let end = self
                    .previous_line()
                    .map(|line| line.end)
                    .unwrap_or(line.end);
                return Ok(StateDecl {
                    values,
                    span: Span {
                        start: start_line.start,
                        end,
                    },
                });
            }
            values.push(self.parse_state_value(line, content)?);
            self.advance();
        }
        Err(ParseError::one(
            "missing closing `}`",
            start_line_span(start_line),
        ))
    }

    fn parse_state_value(&self, line: Line<'a>, content: &str) -> Result<StateValue, ParseError> {
        let tokens = split_frame_words(content);
        if tokens.len() != 4 || tokens.get(2).map(String::as_str) != Some("=") {
            return Err(ParseError::one(
                "state values use `name type = default`",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }
        let name_text = &tokens[0];
        let type_text = &tokens[1];
        let default_text = &tokens[3];
        let name_span = word_span_in_line(line, name_text);
        Ok(StateValue {
            name: Identifier::new(name_text, name_span),
            value_type: match type_text.as_str() {
                "text" => StateType::Text,
                "bool" => StateType::Bool,
                "number" => StateType::Number,
                other => StateType::Unknown(other.to_string()),
            },
            default: parse_state_default(default_text),
            span: Span {
                start: line.start,
                end: line.end,
            },
        })
    }

    fn parse_view_decl(&mut self) -> Result<ViewDecl, ParseError> {
        let start_line = self.current_line().expect("parse_view_decl needs a line");
        self.advance();
        let nodes = self.parse_ui_nodes_until_close()?;
        let end = self
            .previous_line()
            .map(|line| line.end)
            .unwrap_or(start_line.end);
        Ok(ViewDecl {
            nodes,
            span: Span {
                start: start_line.start,
                end,
            },
        })
    }

    fn parse_ui_nodes_until_close(&mut self) -> Result<Vec<UiNode>, ParseError> {
        let mut nodes = Vec::new();
        while let Some(line) = self.current_line() {
            let content = content_without_comment(line.text);
            if content.is_empty() {
                self.advance();
                continue;
            }
            if content == "}" {
                self.advance();
                return Ok(nodes);
            }
            if content.ends_with('{') {
                nodes.push(UiNode::Element(self.parse_ui_element()?));
                continue;
            }
            let words = split_frame_words(content);
            if words.first().map(String::as_str) == Some("text") {
                nodes.push(UiNode::Text(parse_ui_text(line, &words)?));
                self.advance();
                continue;
            }
            return Err(ParseError::one(
                format!("expected UI element or text node, found `{content}`"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }
        Err(ParseError::one(
            "missing closing `}`",
            self.previous_line()
                .map(|line| Span {
                    start: line.start,
                    end: line.end,
                })
                .unwrap_or_default(),
        ))
    }

    fn parse_ui_element(&mut self) -> Result<UiElement, ParseError> {
        let line = self.current_line().expect("parse_ui_element needs a line");
        let content = content_without_comment(line.text);
        let header = content.trim_end_matches('{').trim();
        let parts = split_frame_words(header);
        if parts.len() != 2 {
            return Err(ParseError::one(
                "UI elements use `element NodeName {` or `element NodeName:StyleName {`",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }
        let kind_text = &parts[0];
        let name_and_style = &parts[1];
        let (name_text, style) = name_and_style
            .split_once(':')
            .map(|(name, style)| (name, Some(style)))
            .unwrap_or((name_and_style.as_str(), None));
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
            if child.ends_with('{') {
                children.push(UiNode::Element(self.parse_ui_element()?));
                continue;
            }
            let words = split_frame_words(child);
            match words.first().map(String::as_str) {
                Some("text") => {
                    children.push(UiNode::Text(parse_ui_text(child_line, &words)?));
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

    fn parse_include(&mut self) -> Result<Include, ParseError> {
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

    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        let line = self.current_line().expect("parse_declaration needs a line");
        let content = content_without_comment(line.text);

        if !content.ends_with('{') {
            return Err(ParseError::one(
                format!("expected declaration ending with `{{`, found `{content}`"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let header = content.trim_end_matches('{').trim();
        let mut parts = header.split_whitespace();
        let kind_text = parts.next().ok_or_else(|| {
            ParseError::one(
                "missing declaration kind",
                Span {
                    start: line.start,
                    end: line.end,
                },
            )
        })?;
        let name_text = parts.next().ok_or_else(|| {
            ParseError::one(
                "missing declaration name",
                Span {
                    start: line.start,
                    end: line.end,
                },
            )
        })?;

        let kind = declaration_kind(kind_text);

        let name_start = line.start + line.text.find(name_text).unwrap_or(0);
        let name = Identifier::new(
            name_text,
            Span {
                start: name_start,
                end: name_start + name_text.len(),
            },
        );

        self.advance();
        let body = self.parse_nodes_until_close()?;
        let end = self
            .previous_line()
            .map(|line| line.end)
            .unwrap_or(line.end);

        Ok(Declaration {
            kind,
            name,
            body,
            span: Span {
                start: line.start,
                end,
            },
        })
    }

    fn parse_supports_declaration(&mut self) -> Result<Declaration, ParseError> {
        self.parse_wrapped_style_declaration("supports", DeclarationKind::Supports)
    }

    fn parse_wrapped_style_declaration(
        &mut self,
        keyword: &str,
        kind: DeclarationKind,
    ) -> Result<Declaration, ParseError> {
        let line = self
            .current_line()
            .expect("parse_wrapped_style_declaration needs a line");
        let content = content_without_comment(line.text);

        if !content.ends_with('{') {
            return Err(ParseError::one(
                format!("expected {keyword} block ending with `{{`, found `{content}`"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let header = content.trim_end_matches('{').trim();
        let name_text = header
            .strip_prefix(&format!("{keyword} "))
            .unwrap_or_default()
            .trim();
        if name_text.is_empty() {
            return Err(ParseError::one(
                format!("{keyword} expects a name or typed predicate"),
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let name_start = line.start + line.text.find(name_text).unwrap_or(0);
        self.advance();
        let body = self.parse_nodes_until_close_with_declaration_blocks(true)?;
        let end = self
            .previous_line()
            .map(|line| line.end)
            .unwrap_or(line.end);

        Ok(Declaration {
            kind,
            name: Identifier::new(
                name_text,
                Span {
                    start: name_start,
                    end: name_start + name_text.len(),
                },
            ),
            body,
            span: Span {
                start: line.start,
                end,
            },
        })
    }

    fn parse_style_order(&mut self) -> Result<Declaration, ParseError> {
        let line = self.current_line().expect("parse_style_order needs a line");
        let content = content_without_comment(line.text);
        let order = content
            .strip_prefix("style-order ")
            .unwrap_or_default()
            .trim();
        if order.is_empty() {
            return Err(ParseError::one(
                "style-order expects one or more style group names",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }

        let order_start = line.start + line.text.find(order).unwrap_or(0);
        self.advance();

        Ok(Declaration {
            kind: DeclarationKind::StyleOrder,
            name: Identifier::new(
                order,
                Span {
                    start: order_start,
                    end: order_start + order.len(),
                },
            ),
            body: Vec::new(),
            span: Span {
                start: line.start,
                end: line.end,
            },
        })
    }

    fn parse_nodes_until_close(&mut self) -> Result<Vec<Node>, ParseError> {
        self.parse_nodes_until_close_with_declaration_blocks(false)
    }

    fn parse_nodes_until_close_with_declaration_blocks(
        &mut self,
        allow_declaration_blocks: bool,
    ) -> Result<Vec<Node>, ParseError> {
        let mut nodes = Vec::new();

        while let Some(line) = self.current_line() {
            let content = content_without_comment(line.text);

            if content.is_empty() {
                self.advance();
                continue;
            }

            if content == "}" {
                self.advance();
                return Ok(nodes);
            }

            if content.ends_with('{') {
                let name = content.trim_end_matches('{').trim();
                let is_declaration_block = allow_declaration_blocks
                    && name
                        .split_whitespace()
                        .next()
                        .is_some_and(is_declaration_keyword);
                if !is_declaration_block && !is_allowed_nested_block(name) {
                    return Err(ParseError::one(
                        format!("unknown nested block `{name}`"),
                        Span {
                            start: line.start,
                            end: line.end,
                        },
                    ));
                }

                self.advance();
                let body = self.parse_nodes_until_close_with_declaration_blocks(false)?;
                let end = self
                    .previous_line()
                    .map(|line| line.end)
                    .unwrap_or(line.end);

                nodes.push(Node::Block(Block {
                    name: name.to_string(),
                    body,
                    span: Span {
                        start: line.start,
                        end,
                    },
                }));
                continue;
            }

            let words = content.split_whitespace().map(ToOwned::to_owned).collect();
            self.advance();

            nodes.push(Node::Statement(Statement {
                words,
                span: Span {
                    start: line.start,
                    end: line.end,
                },
            }));
        }

        Err(ParseError::one(
            "missing closing `}`",
            self.previous_line()
                .map(|line| Span {
                    start: line.start,
                    end: line.end,
                })
                .unwrap_or_default(),
        ))
    }

    fn current_line(&self) -> Option<Line<'a>> {
        self.lines.get(self.index).copied()
    }

    fn previous_line(&self) -> Option<Line<'a>> {
        self.index
            .checked_sub(1)
            .and_then(|index| self.lines.get(index))
            .copied()
    }

    fn current_content(&self) -> Option<&'a str> {
        self.current_line()
            .map(|line| content_without_comment(line.text))
    }

    fn advance(&mut self) {
        self.index += 1;
    }
}

fn declaration_kind(kind: &str) -> DeclarationKind {
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

fn is_declaration_keyword(kind: &str) -> bool {
    !matches!(declaration_kind(kind), DeclarationKind::Unknown(_))
}

fn is_allowed_nested_block(name: &str) -> bool {
    matches!(
        name,
        "hover"
            | "focus"
            | "focus-visible"
            | "focus-within"
            | "active"
            | "disabled"
            | "checked"
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

fn is_percentage_selector(name: &str) -> bool {
    name.strip_suffix('%')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn parse_ui_text(line: Line<'_>, words: &[String]) -> Result<UiText, ParseError> {
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
    let value = match words {
        [_, value] if value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            UiPropertyValue::Data(data_ref(line, value, state))
        }
        [_, value] => UiPropertyValue::Literal(unquote(value)),
        [_, bind, value] if bind == "bind" && value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            UiPropertyValue::Bind(data_ref(line, value, state))
        }
        [_, when, value] if when == "when" && value.starts_with('$') => {
            let state = value.trim_start_matches('$');
            let reference = data_ref(line, value, state);
            UiPropertyValue::Conditional(ConditionalBinding {
                condition: reference,
                span: Span {
                    start: line.start,
                    end: line.end,
                },
            })
        }
        [property, when, value, equals, style]
            if property == "style" && when == "when" && value.starts_with('$') && equals == "=" =>
        {
            let state = value.trim_start_matches('$');
            let style_span = word_span_in_line(line, style);
            UiPropertyValue::StyleWhen {
                condition: data_ref(line, value, state),
                style: StyleBinding {
                    name: Identifier::new(style, style_span),
                    span: style_span,
                },
            }
        }
        _ => UiPropertyValue::Unknown(words.iter().skip(1).cloned().collect()),
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

fn parse_state_default(value: &str) -> StateDefault {
    if value.starts_with('"') && value.ends_with('"') {
        StateDefault::Text(unquote(value))
    } else if value == "true" {
        StateDefault::Bool(true)
    } else if value == "false" {
        StateDefault::Bool(false)
    } else if is_number_literal(value) {
        StateDefault::Number(value.to_string())
    } else {
        StateDefault::Invalid(value.to_string())
    }
}

fn data_ref(line: Line<'_>, token: &str, name: &str) -> DataRef {
    let span = word_span_in_line(line, token);
    DataRef {
        name: Identifier::new(name, word_span_in_line(line, name)),
        span,
    }
}

fn split_frame_words(content: &str) -> Vec<String> {
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

fn unquote(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
        .to_string()
}

fn is_number_literal(value: &str) -> bool {
    let mut chars = value.chars();
    if matches!(chars.clone().next(), Some('-')) {
        chars.next();
    }
    let rest = chars.as_str();
    !rest.is_empty()
        && rest.chars().filter(|ch| *ch == '.').count() <= 1
        && rest.chars().all(|ch| ch.is_ascii_digit() || ch == '.')
}

fn word_span_in_line(line: Line<'_>, word: &str) -> Span {
    let relative = line.text.find(word).unwrap_or(0);
    Span {
        start: line.start + relative,
        end: line.start + relative + word.len(),
    }
}

fn start_line_span(line: Line<'_>) -> Span {
    Span {
        start: line.start,
        end: line.end,
    }
}

fn source_lines(source: &str) -> Vec<Line<'_>> {
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

fn content_without_comment(line: &str) -> &str {
    line.split_once("//")
        .map(|(before_comment, _)| before_comment)
        .unwrap_or(line)
        .trim()
}

pub fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_initial_declaration_kinds() {
        let source = r#"
grid AppShell {
}
area Sidebar {
}
card QuickLinkCard {
}
stack SettingsPanel {
}
row Toolbar {
}
button PrimaryButton {
}
text MutedText {
}
tokens AppTheme {
}
center EmptyState {
}
split AppLayout {
}
overlay ModalLayer {
}
dock AppDock {
}
keyframes FloatIn {
}
"#;

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.declarations.len(), 13);
        assert_eq!(document.declarations[0].kind, DeclarationKind::Grid);
        assert_eq!(document.declarations[7].kind, DeclarationKind::Tokens);
        assert_eq!(document.declarations[8].kind, DeclarationKind::Center);
        assert_eq!(document.declarations[11].kind, DeclarationKind::Dock);
        assert_eq!(document.declarations[12].kind, DeclarationKind::Keyframes);
    }

    #[test]
    fn parses_root_includes() {
        let source = "#include base\n#include ./styles/cards.frame\n\ncard Demo {\n}\n";

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.includes.len(), 2);
        assert_eq!(document.includes[0].target, "base");
        assert_eq!(document.includes[1].target, "./styles/cards.frame");
        assert_eq!(document.declarations.len(), 1);
    }

    #[test]
    fn parses_gradient_token_blocks_and_advanced_blocks() {
        let source = r##"
tokens Brand {
  color brand-purple #7c3aed

  gradient hero-gradient {
    type linear
    angle 135deg
    stop brand-purple 0%
    stop #181820 100%
  }
}

card GlassCard {
  advanced {
    css "backdrop-filter" blur(12px)
  }
}
"##;

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.declarations.len(), 2);
        assert!(matches!(
            document.declarations[0].body[1],
            Node::Block(ref block) if block.name == "gradient hero-gradient"
        ));
        assert!(matches!(
            document.declarations[1].body[0],
            Node::Block(ref block) if block.name == "advanced"
        ));
    }

    #[test]
    fn tolerates_partial_gradient_block_while_editing() {
        let source = "tokens Brand {\n  gradient {\n  }\n}\n";

        let document = parse(source).expect("partial gradient block should parse");

        assert!(matches!(
            document.declarations[0].body[0],
            Node::Block(ref block) if block.name == "gradient"
        ));
    }

    #[test]
    fn parses_grid_section_blocks() {
        let source = r#"
grid HoverCardInfo {
  flow vertical
  columns title description

  section title {
    padding bottom small
  }
}
"#;

        let document = parse(source).expect("section block should parse");

        assert!(matches!(
            document.declarations[0].body[2],
            Node::Block(ref block) if block.name == "section title"
        ));
    }

    #[test]
    fn parses_card_with_hover_block() {
        let source = r#"
card QuickLinkCard {
  surface gradient dusk

  hover {
    lift small
    glow accent
  }

  focus-within {
    ring accent
  }

  invalid {
    ring danger
  }
}
"#;

        let document = parse(source).expect("parse should succeed");
        let declaration = &document.declarations[0];

        assert_eq!(declaration.name.text, "QuickLinkCard");
        assert_eq!(declaration.body.len(), 4);
        assert!(matches!(
            declaration.body[1],
            Node::Block(ref block) if block.name == "hover"
        ));
        assert!(matches!(
            declaration.body[2],
            Node::Block(ref block) if block.name == "focus-within"
        ));
        assert!(matches!(
            declaration.body[3],
            Node::Block(ref block) if block.name == "invalid"
        ));
    }

    #[test]
    fn parses_typed_supports_blocks_with_nested_declarations() {
        let source = r#"
supports display grid {
  grid AppShell {
    columns sidebar content
  }
}
"#;

        let document = parse(source).expect("supports block should parse");
        let declaration = &document.declarations[0];

        assert_eq!(declaration.kind, DeclarationKind::Supports);
        assert_eq!(declaration.name.text, "display grid");
        assert!(matches!(
            declaration.body[0],
            Node::Block(ref block) if block.name == "grid AppShell"
        ));
    }

    #[test]
    fn parses_style_groups_and_style_order() {
        let source = r#"
style-order reset, base, components, utilities

style-group components {
  button PrimaryButton {
    surface accent
  }
}
"#;

        let document = parse(source).expect("style groups should parse");

        assert_eq!(document.declarations[0].kind, DeclarationKind::StyleOrder);
        assert_eq!(
            document.declarations[0].name.text,
            "reset, base, components, utilities"
        );
        assert_eq!(document.declarations[1].kind, DeclarationKind::StyleGroup);
        assert_eq!(document.declarations[1].name.text, "components");
        assert!(matches!(
            document.declarations[1].body[0],
            Node::Block(ref block) if block.name == "button PrimaryButton"
        ));
    }

    #[test]
    fn parses_keyframes_and_responsive_blocks() {
        let source = r#"
keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px)
  }

  50% {
    opacity 0.8
  }

  to {
    opacity 1
    transform translateY(0)
  }
}

grid AppShell {
  columns sidebar content

  below tablet {
    columns content
  }

  container narrow {
    columns content
  }
}

card Panel {
  animation FloatIn {
    duration fast
    ease smooth
    fill both
  }
}
"#;

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.declarations[0].kind, DeclarationKind::Keyframes);
        assert!(matches!(
            document.declarations[0].body[0],
            Node::Block(ref block) if block.name == "from"
        ));
        assert!(matches!(
            document.declarations[1].body[1],
            Node::Block(ref block) if block.name == "below tablet"
        ));
        assert!(matches!(
            document.declarations[2].body[0],
            Node::Block(ref block) if block.name == "animation FloatIn"
        ));
    }

    #[test]
    fn reports_unknown_nested_blocks() {
        let source = r#"
card QuickLinkCard {
  magic {
    lift small
  }
}
"#;

        let error = parse(source).expect_err("parse should fail");

        assert!(error.diagnostics[0]
            .message
            .contains("unknown nested block `magic`"));
    }

    #[test]
    fn parses_initial_ui_component_syntax() {
        let source = r#"
component ChatInput {
  state {
    draft text = ""
    sending bool = false
    count number = 1
  }

  view {
    input MessageBox {
      value bind $draft
      placeholder "Message"
      on keydown.ctrl.enter @sendMessage
    }

    button Send:PrimaryButton {
      text "Send"
      disabled when $sending
      on click @sendMessage
      style when $sending = LoadingButton
    }
  }
}
"#;

        let document = parse(source).expect("component should parse");

        assert_eq!(document.components.len(), 1);
        let component = &document.components[0];
        assert_eq!(component.name.text, "ChatInput");
        assert_eq!(component.state.as_ref().unwrap().values.len(), 3);
        let view = component.view.as_ref().unwrap();
        assert_eq!(view.nodes.len(), 2);
        assert!(matches!(
            view.nodes[1],
            UiNode::Element(ref element)
                if element.kind.text == "button"
                    && element.name.text == "Send"
                    && element.style.as_ref().unwrap().name.text == "PrimaryButton"
                    && element.events[0].handler.name.text == "sendMessage"
        ));
    }

    #[test]
    fn rejects_invalid_ui_event_syntax() {
        let source = r#"
component ChatInput {
  view {
    button Send {
      on click sendMessage
    }
  }
}
"#;

        let error = parse(source).expect_err("event syntax should fail");

        assert!(error.diagnostics[0]
            .message
            .contains("events use `on event[.modifier...] @handler`"));
    }
}
