use crate::{helpers::*, ui::*, Line, ParseError, Parser};
use frame_core::{
    ComponentDecl, DeclarationKind, Document, Identifier, PropType, PropValue, PropsDecl, SlotDecl,
    Span, StateDecl, StateType, StateValue, UiNode, ViewDecl,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_document(&mut self) -> Result<Document, ParseError> {
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

    pub(crate) fn parse_component(&mut self) -> Result<ComponentDecl, ParseError> {
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

    pub(crate) fn parse_state_decl(&mut self) -> Result<StateDecl, ParseError> {
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

    pub(crate) fn parse_state_value(
        &self,
        line: Line<'a>,
        content: &str,
    ) -> Result<StateValue, ParseError> {
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

    pub(crate) fn parse_props_decl(&mut self) -> Result<PropsDecl, ParseError> {
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

    pub(crate) fn parse_prop_value(
        &self,
        line: Line<'a>,
        content: &str,
    ) -> Result<PropValue, ParseError> {
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

    pub(crate) fn parse_slot_decl(&mut self) -> Result<SlotDecl, ParseError> {
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

    pub(crate) fn parse_view_decl(&mut self) -> Result<ViewDecl, ParseError> {
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

    pub(crate) fn parse_ui_nodes_until_close(&mut self) -> Result<Vec<UiNode>, ParseError> {
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
}
