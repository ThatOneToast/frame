use std::collections::HashMap;

use crate::{DeclarationKind, Document, Node, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Declaration(DeclarationKind),
    Color,
    Gradient,
    Keyframes,
    GridSection { grid: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolIndex {
    pub declarations: HashMap<String, FrameSymbol>,
    pub components: HashMap<String, FrameSymbol>,
    pub colors: HashMap<String, FrameSymbol>,
    pub gradients: HashMap<String, FrameSymbol>,
    pub keyframes: HashMap<String, FrameSymbol>,
    pub grids: HashMap<String, FrameSymbol>,
    pub grid_sections: HashMap<String, HashMap<String, FrameSymbol>>,
}

impl SymbolIndex {
    pub fn merge(&mut self, other: SymbolIndex) {
        self.declarations.extend(other.declarations);
        self.components.extend(other.components);
        self.colors.extend(other.colors);
        self.gradients.extend(other.gradients);
        self.keyframes.extend(other.keyframes);
        self.grids.extend(other.grids);
        for (grid, sections) in other.grid_sections {
            self.grid_sections.entry(grid).or_default().extend(sections);
        }
    }

    pub fn color_names(&self) -> Vec<String> {
        sorted_keys(&self.colors)
    }

    pub fn gradient_names(&self) -> Vec<String> {
        sorted_keys(&self.gradients)
    }

    pub fn grid_names(&self) -> Vec<String> {
        sorted_keys(&self.grids)
    }

    pub fn keyframe_names(&self) -> Vec<String> {
        sorted_keys(&self.keyframes)
    }
}

pub fn index_document(source: &str, document: &Document) -> SymbolIndex {
    let mut index = SymbolIndex::default();

    for declaration in &document.declarations {
        let declaration_symbol = FrameSymbol {
            name: declaration.name.text.clone(),
            kind: SymbolKind::Declaration(declaration.kind.clone()),
            span: declaration.name.span,
            value: None,
        };
        index
            .declarations
            .insert(declaration.name.text.clone(), declaration_symbol.clone());

        if declaration.kind == DeclarationKind::Grid {
            index
                .grids
                .insert(declaration.name.text.clone(), declaration_symbol);
            collect_grid_sections(
                source,
                declaration.name.text.as_str(),
                &declaration.body,
                &mut index,
            );
        }

        if declaration.kind == DeclarationKind::Keyframes {
            index.keyframes.insert(
                declaration.name.text.clone(),
                FrameSymbol {
                    name: declaration.name.text.clone(),
                    kind: SymbolKind::Keyframes,
                    span: declaration.name.span,
                    value: Some(format!("@keyframes frame-{}", declaration.name.text)),
                },
            );
        }

        if declaration.kind == DeclarationKind::Tokens {
            collect_token_symbols(source, &declaration.body, &mut index);
        }
    }

    for component in &document.components {
        index.components.insert(
            component.name.text.clone(),
            FrameSymbol {
                name: component.name.text.clone(),
                kind: SymbolKind::Declaration(DeclarationKind::Unknown("component".to_string())),
                span: component.name.span,
                value: None,
            },
        );
    }

    index
}

fn collect_token_symbols(source: &str, body: &[Node], index: &mut SymbolIndex) {
    for node in body {
        match node {
            Node::Statement(statement)
                if statement.words.first().map(String::as_str) == Some("color") =>
            {
                let Some(name) = statement.words.get(1) else {
                    continue;
                };
                let span = word_span(source, statement.span, name).unwrap_or(statement.span);
                index.colors.insert(
                    name.clone(),
                    FrameSymbol {
                        name: name.clone(),
                        kind: SymbolKind::Color,
                        span,
                        value: statement.words.get(2).cloned(),
                    },
                );
            }
            Node::Block(block) if block.name.starts_with("gradient ") => {
                let Some(name) = block.name.split_whitespace().nth(1) else {
                    continue;
                };
                let span = word_span(source, block.span, name).unwrap_or(block.span);
                index.gradients.insert(
                    name.to_string(),
                    FrameSymbol {
                        name: name.to_string(),
                        kind: SymbolKind::Gradient,
                        span,
                        value: gradient_css_preview(&block.body),
                    },
                );
            }
            _ => {}
        }
    }
}

fn collect_grid_sections(source: &str, grid: &str, body: &[Node], index: &mut SymbolIndex) {
    for node in body {
        let Node::Statement(statement) = node else {
            continue;
        };
        if statement.words.first().map(String::as_str) != Some("columns") {
            continue;
        }
        for name in statement.words.iter().skip(1) {
            if matches!(name.as_str(), "responsive" | "cards" | "auto" | "fill")
                || name.ends_with('%')
            {
                continue;
            }
            let span = word_span(source, statement.span, name).unwrap_or(statement.span);
            index
                .grid_sections
                .entry(grid.to_string())
                .or_default()
                .insert(
                    name.clone(),
                    FrameSymbol {
                        name: name.clone(),
                        kind: SymbolKind::GridSection {
                            grid: grid.to_string(),
                        },
                        span,
                        value: None,
                    },
                );
        }
    }
}

fn gradient_css_preview(body: &[Node]) -> Option<String> {
    let mut angle = "180deg".to_string();
    let mut stops = Vec::new();

    for statement in body.iter().filter_map(|node| match node {
        Node::Statement(statement) => Some(statement),
        Node::Block(_) => None,
    }) {
        match statement.words.first().map(String::as_str) {
            Some("angle") => {
                if let Some(value) = statement.words.get(1) {
                    angle = value.clone();
                }
            }
            Some("stop") => {
                if let (Some(color), Some(position)) =
                    (statement.words.get(1), statement.words.get(2))
                {
                    stops.push(format!("{color} {position}"));
                }
            }
            _ => {}
        }
    }

    (stops.len() >= 2).then(|| format!("linear-gradient({angle}, {})", stops.join(", ")))
}

fn word_span(source: &str, span: Span, word: &str) -> Option<Span> {
    if source.is_empty() || span.end > source.len() || span.start > span.end {
        return None;
    }
    let relative = source[span.start..span.end].find(word)?;
    Some(Span {
        start: span.start + relative,
        end: span.start + relative + word.len(),
    })
}

fn sorted_keys<T>(map: &HashMap<String, T>) -> Vec<String> {
    let mut values = map.keys().cloned().collect::<Vec<_>>();
    values.sort();
    values
}
