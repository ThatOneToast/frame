//! Parser for the first Frame language slice.
//!
//! The parser is deliberately small and line-oriented for the MVP. It produces
//! a shared AST plus structured diagnostics that can be reused by the CLI and
//! future editor tooling.

use std::{error::Error, fmt};

use frame_core::{
    Block, Declaration, DeclarationKind, Diagnostic, Document, Identifier, Include, Node, Severity,
    Span, Statement,
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

        while let Some(line) = self.current_content() {
            if line.is_empty() {
                self.advance();
                continue;
            }

            if line.starts_with("#include") {
                includes.push(self.parse_include()?);
                continue;
            }

            declarations.push(self.parse_declaration()?);
        }

        Ok(Document {
            includes,
            declarations,
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

        let kind = match kind_text {
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
            other => DeclarationKind::Unknown(other.to_string()),
        };

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

    fn parse_nodes_until_close(&mut self) -> Result<Vec<Node>, ParseError> {
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
                if !is_allowed_nested_block(name) {
                    return Err(ParseError::one(
                        format!("unknown nested block `{name}`"),
                        Span {
                            start: line.start,
                            end: line.end,
                        },
                    ));
                }

                self.advance();
                let body = self.parse_nodes_until_close()?;
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

fn is_allowed_nested_block(name: &str) -> bool {
    matches!(name, "hover" | "focus" | "active" | "disabled" | "advanced")
        || name == "gradient"
        || name.starts_with("gradient ")
        || name.starts_with("section ")
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
"#;

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.declarations.len(), 12);
        assert_eq!(document.declarations[0].kind, DeclarationKind::Grid);
        assert_eq!(document.declarations[7].kind, DeclarationKind::Tokens);
        assert_eq!(document.declarations[8].kind, DeclarationKind::Center);
        assert_eq!(document.declarations[11].kind, DeclarationKind::Dock);
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
}
"#;

        let document = parse(source).expect("parse should succeed");
        let declaration = &document.declarations[0];

        assert_eq!(declaration.name.text, "QuickLinkCard");
        assert_eq!(declaration.body.len(), 2);
        assert!(matches!(declaration.body[1], Node::Block(_)));
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
}
