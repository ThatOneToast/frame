use crate::{helpers::*, ParseError, Parser};
use frame_core::{Block, Declaration, DeclarationKind, Identifier, Node, Span, Statement};

impl<'a> Parser<'a> {
    pub(crate) fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        let line = self.current_line().expect("parse_declaration needs a line");
        let content = content_without_comment(line.text);

        // Support both `card Foo {` (multi-line) and `card Foo { }` (empty single-line).
        let empty_body = content.ends_with("{ }") || content.ends_with("{}");
        if !content.ends_with('{') && !empty_body {
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

        let kind = declaration_kind(kind_text);

        let unnamed = matches!(kind, DeclarationKind::Html | DeclarationKind::Body);
        let name_text = if unnamed {
            kind_text.to_string()
        } else {
            parts
                .next()
                .ok_or_else(|| {
                    ParseError::one(
                        "missing declaration name",
                        Span {
                            start: line.start,
                            end: line.end,
                        },
                    )
                })?
                .to_string()
        };

        let name_start = line.start + line.text.find(&name_text).unwrap_or(0);
        let name = Identifier::new(
            &name_text,
            Span {
                start: name_start,
                end: name_start + name_text.len(),
            },
        );

        // Parse optional `extends BaseName` after the declaration name.
        let extends = if parts.next() == Some("extends") {
            let base_text = parts.next().ok_or_else(|| {
                ParseError::one(
                    "missing base style name after `extends`",
                    Span {
                        start: line.start,
                        end: line.end,
                    },
                )
            })?;
            let base_start = line.start + line.text.find(base_text).unwrap_or(0);
            Some(Identifier::new(
                base_text,
                Span {
                    start: base_start,
                    end: base_start + base_text.len(),
                },
            ))
        } else {
            None
        };

        self.advance();
        let body = if empty_body {
            Vec::new()
        } else {
            self.parse_nodes_until_close()?
        };
        let end = if empty_body {
            line.end
        } else {
            self.previous_line()
                .map(|line| line.end)
                .unwrap_or(line.end)
        };

        Ok(Declaration {
            kind,
            name,
            extends,
            body,
            span: Span {
                start: line.start,
                end,
            },
        })
    }

    pub(crate) fn parse_supports_declaration(&mut self) -> Result<Declaration, ParseError> {
        self.parse_wrapped_style_declaration("supports", DeclarationKind::Supports)
    }

    pub(crate) fn parse_wrapped_style_declaration(
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
            extends: None,
            body,
            span: Span {
                start: line.start,
                end,
            },
        })
    }

    pub(crate) fn parse_style_order(&mut self) -> Result<Declaration, ParseError> {
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
            extends: None,
            body: Vec::new(),
            span: Span {
                start: line.start,
                end: line.end,
            },
        })
    }

    pub(crate) fn parse_nodes_until_close(&mut self) -> Result<Vec<Node>, ParseError> {
        self.parse_nodes_until_close_with_declaration_blocks(false)
    }

    pub(crate) fn parse_nodes_until_close_with_declaration_blocks(
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
}
