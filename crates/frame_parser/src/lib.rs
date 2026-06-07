//! Parser for the first Frame language slice.
//!
//! The parser is deliberately small and line-oriented for the MVP. It produces
//! a shared AST plus structured diagnostics that can be reused by the CLI and
//! future editor tooling.

use std::{error::Error, fmt};

use frame_core::{
    Block, ComponentDecl, ConditionalBinding, DataRef, Declaration, DeclarationKind, Diagnostic,
    Document, EventBinding, HandlerRef, Identifier, Include, Node, PropType, PropValue, PropsDecl,
    Severity, SlotDecl, Span, StateDecl, StateDefault, StateType, StateValue, Statement,
    StyleBinding, TextValue, UiComponentArgument, UiComponentArgumentValue, UiComponentInvocation,
    UiElement, UiNode, UiProperty, UiPropertyValue, UiText, ViewDecl,
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
        let mut props = None;
        let mut state = None;
        let mut view = None;
        let mut slots = Vec::new();
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
                    props,
                    state,
                    view,
                    slots,
                    span: Span {
                        start: line.start,
                        end,
                    },
                });
            }
            match child {
                "props {" => props = Some(self.parse_props_decl()?),
                "state {" => state = Some(self.parse_state_decl()?),
                "view {" => view = Some(self.parse_view_decl()?),
                _ if child.starts_with("slot ") && child.ends_with('{') => {
                    slots.push(self.parse_slot_decl()?)
                }
                _ => {
                    return Err(ParseError::one(
                        format!(
                            "unknown component block `{child}`\n\nComponents currently support `props {{ ... }}`, `state {{ ... }}`, `view {{ ... }}`, and `slot Name {{ ... }}`."
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
                "text" | "string" => StateType::Text,
                "bool" => StateType::Bool,
                "number" => StateType::Number,
                "list" => StateType::List,
                other => StateType::Unknown(other.to_string()),
            },
            default: parse_state_default(default_text),
            span: Span {
                start: line.start,
                end: line.end,
            },
        })
    }

    fn parse_props_decl(&mut self) -> Result<PropsDecl, ParseError> {
        let start_line = self.current_line().expect("parse_props_decl needs a line");
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
                return Ok(PropsDecl {
                    values,
                    span: Span {
                        start: start_line.start,
                        end,
                    },
                });
            }
            values.push(self.parse_prop_value(line, content)?);
            self.advance();
        }
        Err(ParseError::one(
            "missing closing `}`",
            start_line_span(start_line),
        ))
    }

    fn parse_prop_value(&self, line: Line<'a>, content: &str) -> Result<PropValue, ParseError> {
        let tokens = split_frame_words(content);
        if tokens.len() != 2 {
            return Err(ParseError::one(
                "props values use `name type`",
                Span {
                    start: line.start,
                    end: line.end,
                },
            ));
        }
        let name_text = &tokens[0];
        let type_text = &tokens[1];
        let name_span = word_span_in_line(line, name_text);
        Ok(PropValue {
            name: Identifier::new(name_text, name_span),
            value_type: match type_text.as_str() {
                "text" | "string" => PropType::Text,
                "bool" => PropType::Bool,
                "number" => PropType::Number,
                "list" => PropType::List,
                other => PropType::Unknown(other.to_string()),
            },
            span: Span {
                start: line.start,
                end: line.end,
            },
        })
    }

    fn parse_slot_decl(&mut self) -> Result<SlotDecl, ParseError> {
        let start_line = self.current_line().expect("parse_slot_decl needs a line");
        let content = content_without_comment(start_line.text);
        let header = content.trim_end_matches('{').trim();
        let mut parts = header.split_whitespace();
        let _keyword = parts.next().unwrap_or_default();
        let name_text = parts.next().ok_or_else(|| {
            ParseError::one(
                "slot expects a name",
                Span {
                    start: start_line.start,
                    end: start_line.end,
                },
            )
        })?;
        if parts.next().is_some() {
            return Err(ParseError::one(
                "slot accepts exactly one name",
                Span {
                    start: start_line.start,
                    end: start_line.end,
                },
            ));
        }
        let name_start = start_line.start + start_line.text.find(name_text).unwrap_or(0);
        self.advance();
        let nodes = self.parse_ui_nodes_until_close()?;
        let end = self
            .previous_line()
            .map(|line| line.end)
            .unwrap_or(start_line.end);
        Ok(SlotDecl {
            name: Identifier::new(
                name_text,
                Span {
                    start: name_start,
                    end: name_start + name_text.len(),
                },
            ),
            nodes,
            span: Span {
                start: start_line.start,
                end,
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
            if content.starts_with("for ") && content.ends_with('{') {
                nodes.push(UiNode::Loop(self.parse_ui_for_loop()?));
                continue;
            }
            let words = split_frame_words(content);
            if words.first().map(String::as_str) == Some("text") {
                nodes.push(UiNode::Text(parse_ui_text(line, &words)?));
                self.advance();
                continue;
            }
            if content.ends_with('{') {
                nodes.push(UiNode::Element(self.parse_ui_element()?));
                continue;
            }
            if looks_like_semantic_shorthand(content) {
                nodes.push(UiNode::Element(parse_ui_element_shorthand(line, content)?));
                self.advance();
                continue;
            }
            if looks_like_component_invocation(content) {
                nodes.push(UiNode::Component(parse_component_invocation(
                    line, content,
                )?));
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

    fn parse_ui_for_loop(&mut self) -> Result<frame_core::UiForLoop, ParseError> {
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

fn parse_ui_element_shorthand(line: Line<'_>, content: &str) -> Result<UiElement, ParseError> {
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

fn parse_component_invocation(
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

fn looks_like_component_invocation(content: &str) -> bool {
    let Some(open_paren) = content.find('(') else {
        return false;
    };
    content.ends_with(')')
        && content[..open_paren]
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_uppercase())
}

fn looks_like_semantic_shorthand(content: &str) -> bool {
    let words = split_frame_words(content);
    words.len() == 2 && is_semantic_ui_primitive(&words[0])
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

fn is_semantic_ui_primitive(kind: &str) -> bool {
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

fn default_ui_node_name(kind: &str) -> &str {
    match kind {
        "item" => "Item",
        "empty" => "Empty",
        "title" => "Title",
        "text" => "Text",
        "label" => "Label",
        _ => kind,
    }
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
    } else if value == "[]" {
        StateDefault::List
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
    fn parses_component_loops_with_optional_keys() {
        let source = r#"
component MessageList {
  props {
    messages list
    selectedId text
  }

  view {
    for message in $messages {
      text $message
    }
    for selected in $messages key $selectedId {
      MessageItem(text: $selected)
    }
  }
}
"#;

        let document = parse(source).expect("parse should succeed");
        let nodes = &document.components[0].view.as_ref().expect("view").nodes;

        assert!(matches!(
            nodes[0],
            UiNode::Loop(ref loop_node)
                if loop_node.item.text == "message"
                    && loop_node.collection.name.text == "messages"
                    && loop_node.key.is_none()
        ));
        assert!(matches!(
            nodes[1],
            UiNode::Loop(ref loop_node)
                if loop_node.item.text == "selected"
                    && loop_node.key.as_ref().map(|key| key.name.text.as_str()) == Some("selectedId")
        ));
    }

    #[test]
    fn parses_semantic_ui_primitives_and_shorthand() {
        let source = r#"
component Chat {
  state {
    draft text = ""
    messages list = []
  }

  view {
    screen Chat {
      title "Messages"

      list Messages {
        source $messages

        item {
          text $message.text
        }

        empty {
          text "No messages"
        }
      }

      composer ChatBox {
        label "Message"
        draft bind $draft
        send @sendMessage
      }

      action Save
    }
  }
}
"#;

        let document = parse(source).expect("semantic UI should parse");
        let screen = match &document.components[0].view.as_ref().unwrap().nodes[0] {
            UiNode::Element(element) => element,
            _ => panic!("expected screen"),
        };

        assert_eq!(screen.kind.text, "screen");
        assert_eq!(screen.children.len(), 4);
        assert!(matches!(
            screen.children[0],
            UiNode::Element(ref element)
                if element.kind.text == "title"
                    && matches!(element.properties[0].value, UiPropertyValue::Literal(ref value) if value == "Messages")
        ));
        assert!(matches!(
            screen.children[1],
            UiNode::Element(ref element)
                if element.kind.text == "list"
                    && matches!(element.children[0], UiNode::Element(ref child) if child.kind.text == "item")
        ));
        assert!(matches!(
            screen.children[2],
            UiNode::Element(ref element)
                if element.kind.text == "composer"
                    && matches!(element.properties[0].value, UiPropertyValue::Bind(ref reference) if reference.name.text == "draft")
                    && matches!(element.properties[1].value, UiPropertyValue::Handler(ref handler) if handler.name.text == "sendMessage")
        ));
        assert!(matches!(
            screen.children[3],
            UiNode::Element(ref element)
                if element.kind.text == "action" && element.name.text == "Save"
        ));
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

    #[test]
    fn parses_component_invocations_in_view() {
        let source = r#"
component ChatApp {
  state {
    activeChannel text = "general"
    draft text = ""
  }

  view {
    ChannelSidebar()
    ChatPanel(channel: $activeChannel)
    MessageComposer(draft bind $draft)
  }
}
"#;

        let document = parse(source).expect("component invocations should parse");
        let view = document.components[0].view.as_ref().unwrap();

        assert!(matches!(
            view.nodes[0],
            UiNode::Component(ref invocation)
                if invocation.name.text == "ChannelSidebar"
                    && invocation.arguments.is_empty()
        ));
        assert!(matches!(
            view.nodes[1],
            UiNode::Component(ref invocation)
                if invocation.name.text == "ChatPanel"
                    && invocation.arguments[0].name.text == "channel"
                    && matches!(
                        invocation.arguments[0].value,
                        UiComponentArgumentValue::Data(ref reference)
                            if reference.name.text == "activeChannel"
                    )
        ));
        assert!(matches!(
            view.nodes[2],
            UiNode::Component(ref invocation)
                if invocation.name.text == "MessageComposer"
                    && invocation.arguments[0].name.text == "draft"
                    && matches!(
                        invocation.arguments[0].value,
                        UiComponentArgumentValue::Bind(ref reference)
                            if reference.name.text == "draft"
                    )
        ));
    }

    #[test]
    fn parses_props_and_slots_in_component() {
        let source = r#"
component ChannelButton {
  props {
    channel text
    active bool
    unreadCount number
  }

  state {
    hovered bool = false
  }

  view {
    action Channel {
      text $channel
      disabled when $active
      on press @selectChannel
    }
  }

  slot Default {
    text "No content"
  }
}
"#;

        let document = parse(source).expect("component should parse");
        assert_eq!(document.components.len(), 1);
        let component = &document.components[0];

        let props = component.props.as_ref().unwrap();
        assert_eq!(props.values.len(), 3);
        assert_eq!(props.values[0].name.text, "channel");
        assert!(matches!(props.values[0].value_type, PropType::Text));
        assert_eq!(props.values[2].name.text, "unreadCount");
        assert!(matches!(props.values[2].value_type, PropType::Number));

        let view = component.view.as_ref().unwrap();
        let button = match &view.nodes[0] {
            UiNode::Element(element) => element,
            _ => panic!("expected action element"),
        };
        // text $channel is parsed as a child text node, not a property
        assert!(matches!(
            button.children[0],
            UiNode::Text(ref text)
                if matches!(text.value, TextValue::Data(ref reference) if reference.name.text == "channel")
        ));
        assert!(matches!(
            button.properties[0],
            UiProperty {
                name: ref property_name,
                value: UiPropertyValue::Conditional(ref binding),
                ..
            }
            if property_name.text == "disabled" && binding.condition.name.text == "active"
        ));

        assert_eq!(component.slots.len(), 1);
        assert_eq!(component.slots[0].name.text, "Default");
        assert!(matches!(
            component.slots[0].nodes[0],
            UiNode::Text(ref text)
                if matches!(text.value, TextValue::Literal(ref value) if value == "No content")
        ));
    }

    #[test]
    fn parses_show_when_conditional() {
        let source = r#"
component ChatApp {
  state {
    loggedIn bool = false
  }

  view {
    panel Main {
      show when $loggedIn
      text "Welcome"
    }
  }
}
"#;

        let document = parse(source).expect("component should parse");
        let view = document.components[0].view.as_ref().unwrap();
        let element = match &view.nodes[0] {
            UiNode::Element(element) => element,
            _ => panic!("expected element"),
        };
        assert!(matches!(
            element.properties[0],
            UiProperty {
                name: ref property_name,
                value: UiPropertyValue::Conditional(ref binding),
                ..
            }
            if property_name.text == "show" && binding.condition.name.text == "loggedIn"
        ));
    }

    #[test]
    fn rejects_unknown_component_block() {
        let source = r#"
component ChatApp {
  unknown {
    text "Hello"
  }
}
"#;

        let error = parse(source).expect_err("parse should fail");
        assert!(error.diagnostics[0]
            .message
            .contains("Components currently support"));
    }
}
