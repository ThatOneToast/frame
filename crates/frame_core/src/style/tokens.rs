//! Token contracts.
//!
//! Tokens are typed, named design values. The default contract below is the
//! single source of the values that used to be hard-coded inside the CSS
//! generator. Documents extend or override it with `tokens <namespace> { ... }`
//! declarations, and themes scope overrides with `theme <name> uses <ns> { ... }`.

use crate::{DeclarationKind, Document, Node};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Color,
    Surface,
    Gradient,
    Space,
    Radius,
    Shadow,
    Glow,
    Breakpoint,
    Container,
}

impl TokenKind {
    pub fn keyword(self) -> &'static str {
        match self {
            TokenKind::Color => "color",
            TokenKind::Surface => "surface",
            TokenKind::Gradient => "gradient",
            TokenKind::Space => "space",
            TokenKind::Radius => "radius",
            TokenKind::Shadow => "shadow",
            TokenKind::Glow => "glow",
            TokenKind::Breakpoint => "breakpoint",
            TokenKind::Container => "container",
        }
    }

    pub fn from_keyword(keyword: &str) -> Option<Self> {
        Some(match keyword {
            "color" => TokenKind::Color,
            "surface" => TokenKind::Surface,
            "gradient" => TokenKind::Gradient,
            "space" => TokenKind::Space,
            "radius" => TokenKind::Radius,
            "shadow" => TokenKind::Shadow,
            "glow" => TokenKind::Glow,
            "breakpoint" => TokenKind::Breakpoint,
            "container" => TokenKind::Container,
            _ => return None,
        })
    }

    pub const ALL: &'static [TokenKind] = &[
        TokenKind::Color,
        TokenKind::Surface,
        TokenKind::Gradient,
        TokenKind::Space,
        TokenKind::Radius,
        TokenKind::Shadow,
        TokenKind::Glow,
        TokenKind::Breakpoint,
        TokenKind::Container,
    ];

    /// Whether tokens of this kind become CSS custom properties.
    /// Breakpoints and containers resolve at compile time instead.
    pub fn emits_css_variable(self) -> bool {
        !matches!(self, TokenKind::Breakpoint | TokenKind::Container)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenEntry {
    pub kind: TokenKind,
    pub name: String,
    pub value: String,
}

/// A resolved set of tokens: the default contract plus document overrides.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenContract {
    pub entries: Vec<TokenEntry>,
}

impl TokenContract {
    pub fn get(&self, kind: TokenKind, name: &str) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|entry| entry.kind == kind && entry.name == name)
            .map(|entry| entry.value.as_str())
    }

    pub fn names(&self, kind: TokenKind) -> Vec<&str> {
        let mut names: Vec<&str> = self
            .entries
            .iter()
            .filter(|entry| entry.kind == kind)
            .map(|entry| entry.name.as_str())
            .collect();
        names.sort();
        names.dedup();
        names
    }

    /// Insert or override an entry.
    pub fn set(&mut self, kind: TokenKind, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|entry| entry.kind == kind && entry.name == name)
        {
            existing.value = value;
        } else {
            self.entries.push(TokenEntry { kind, name, value });
        }
    }

    /// The CSS custom property name for a token.
    pub fn css_variable(kind: TokenKind, name: &str) -> String {
        format!("--frame-{}-{}", kind.keyword(), name)
    }

    /// The custom-property pairs to emit, in deterministic kind-major order.
    pub fn css_variables(&self) -> Vec<(String, String)> {
        let mut output = Vec::new();
        for &kind in TokenKind::ALL {
            if !kind.emits_css_variable() {
                continue;
            }
            for entry in self.entries.iter().filter(|entry| entry.kind == kind) {
                output.push((Self::css_variable(kind, &entry.name), entry.value.clone()));
            }
        }
        output
    }
}

fn push_default(entries: &mut Vec<TokenEntry>, kind: TokenKind, pairs: &[(&str, &str)]) {
    for (name, value) in pairs {
        entries.push(TokenEntry {
            kind,
            name: (*name).to_string(),
            value: (*value).to_string(),
        });
    }
}

/// The built-in default token contract.
///
/// These are starting points, not built-ins users are stuck with: any entry
/// can be overridden from a `tokens` declaration or scoped via a `theme`.
pub fn default_contract() -> TokenContract {
    let mut entries = Vec::new();
    push_default(
        &mut entries,
        TokenKind::Space,
        &[
            ("none", "0"),
            ("small", "0.5rem"),
            ("medium", "1rem"),
            ("large", "1.5rem"),
            ("xlarge", "2rem"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Radius,
        &[
            ("none", "0"),
            ("small", "0.375rem"),
            ("medium", "0.625rem"),
            ("large", "1rem"),
            ("xlarge", "1.5rem"),
            ("pill", "999px"),
            ("full", "999px"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Surface,
        &[
            ("panel", "#171717"),
            ("main", "#101010"),
            ("glass", "rgba(255, 255, 255, 0.08)"),
            ("flat", "transparent"),
            ("raised", "#202020"),
            ("overlay", "rgba(10, 10, 12, 0.92)"),
            ("inset", "#0b0b0f"),
            ("sunken", "#08080b"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Gradient,
        &[
            ("dusk", "linear-gradient(135deg, #22162f, #123047)"),
            ("midnight", "linear-gradient(135deg, #080b18, #1b2440)"),
            (
                "aurora",
                "linear-gradient(135deg, #164e63, #4c1d95, #166534)",
            ),
            ("ember", "linear-gradient(135deg, #7f1d1d, #f97316)"),
            ("ocean", "linear-gradient(135deg, #0f766e, #1d4ed8)"),
            ("forest", "linear-gradient(135deg, #14532d, #84cc16)"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Color,
        &[
            ("main", "#f5f5f5"),
            ("bright", "#ffffff"),
            ("muted", "#a3a3a3"),
            ("accent", "#8ab4ff"),
            ("primary", "#93c5fd"),
            ("secondary", "#c4b5fd"),
            ("danger", "#f87171"),
            ("success", "#34d399"),
            ("warning", "#fbbf24"),
            ("info", "#38bdf8"),
            ("white", "#ffffff"),
            ("black", "#000000"),
            ("gray", "#9ca3af"),
            ("slate", "#64748b"),
            ("red", "#ef4444"),
            ("orange", "#fb923c"),
            ("yellow", "#facc15"),
            ("green", "#22c55e"),
            ("blue", "#60a5fa"),
            ("purple", "#a78bfa"),
            ("pink", "#f472b6"),
            ("cyan", "#22d3ee"),
            ("transparent", "transparent"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Shadow,
        &[
            ("none", "none"),
            ("soft", "0 4px 16px rgba(0, 0, 0, 0.14)"),
            ("small", "0 6px 18px rgba(0, 0, 0, 0.18)"),
            ("medium", "0 12px 30px rgba(0, 0, 0, 0.25)"),
            ("large", "0 18px 48px rgba(0, 0, 0, 0.32)"),
            ("deep", "0 24px 64px rgba(0, 0, 0, 0.42)"),
            ("floating", "0 30px 80px rgba(0, 0, 0, 0.48)"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Glow,
        &[
            ("none", "none"),
            ("accent", "0 0 24px rgba(120, 160, 255, 0.35)"),
            ("danger", "0 0 24px rgba(248, 113, 113, 0.35)"),
            ("success", "0 0 24px rgba(52, 211, 153, 0.35)"),
            ("warning", "0 0 24px rgba(251, 191, 36, 0.35)"),
            ("soft", "0 0 18px rgba(255, 255, 255, 0.16)"),
            ("strong", "0 0 34px rgba(255, 255, 255, 0.28)"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Breakpoint,
        &[
            ("mobile", "30rem"),
            ("tablet", "48rem"),
            ("desktop", "64rem"),
            ("wide", "80rem"),
        ],
    );
    push_default(
        &mut entries,
        TokenKind::Container,
        &[("narrow", "42rem"), ("content", "64rem"), ("wide", "80rem")],
    );
    TokenContract { entries }
}

/// Collect the resolved token contract for a document:
/// the default contract plus every `tokens` declaration, in source order.
pub fn document_contract(document: &Document) -> TokenContract {
    let mut contract = default_contract();
    for declaration in &document.declarations {
        if declaration.kind != DeclarationKind::Tokens {
            continue;
        }
        collect_token_entries(&declaration.body, &mut contract);
    }
    contract
}

pub(crate) fn collect_token_entries(body: &[Node], contract: &mut TokenContract) {
    for node in body {
        match node {
            Node::Statement(statement) => {
                let Some(kind) = statement
                    .words
                    .first()
                    .and_then(|word| TokenKind::from_keyword(word))
                else {
                    continue;
                };
                let (Some(name), Some(_)) = (statement.words.get(1), statement.words.get(2)) else {
                    continue;
                };
                let value = statement.words[2..].join(" ");
                let value = resolve_alias(kind, &value, contract);
                contract.set(kind, name.clone(), value);
            }
            Node::Block(block) if block.name.starts_with("gradient ") => {
                let Some(name) = block.name.split_whitespace().nth(1) else {
                    continue;
                };
                if let Some(value) = crate::style::gradient_css(&block.body, contract) {
                    contract.set(TokenKind::Gradient, name.to_string(), value);
                }
            }
            _ => {}
        }
    }
}

/// Token values may reference an existing token of the same kind by name:
/// `shadow panel soft` aliases the built-in `soft` shadow.
fn resolve_alias(kind: TokenKind, value: &str, contract: &TokenContract) -> String {
    if !value.contains(' ')
        && !value.starts_with('#')
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !value.chars().next().is_some_and(|c| c.is_ascii_digit())
    {
        if let Some(resolved) = contract.get(kind, value) {
            return resolved.to_string();
        }
    }
    value.to_string()
}

/// Resolve a `token(kind.name)` reference to its CSS custom property.
/// Returns `None` when the value is not a token reference.
pub fn token_reference(value: &str) -> Option<(TokenKind, &str)> {
    let inner = value.strip_prefix("token(")?.strip_suffix(')')?;
    let (kind_text, name) = inner.split_once('.')?;
    let kind = TokenKind::from_keyword(kind_text.trim())?;
    Some((kind, name.trim()))
}

/// Compute the `max-width` value for `below <breakpoint>` queries:
/// one device pixel under the breakpoint threshold.
pub fn breakpoint_below(value: &str) -> String {
    if let Some(number) = value.strip_suffix("rem") {
        if let Ok(parsed) = number.trim().parse::<f64>() {
            return format!("{}rem", trim_float(parsed - 0.0625));
        }
    }
    if let Some(number) = value.strip_suffix("px") {
        if let Ok(parsed) = number.trim().parse::<f64>() {
            return format!("{}px", trim_float(parsed - 1.0));
        }
    }
    format!("calc({value} - 1px)")
}

fn trim_float(value: f64) -> String {
    let mut text = format!("{value:.4}");
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_contract_covers_legacy_builtins() {
        let contract = default_contract();
        assert_eq!(contract.get(TokenKind::Space, "medium"), Some("1rem"));
        assert_eq!(contract.get(TokenKind::Color, "accent"), Some("#8ab4ff"));
        assert_eq!(contract.get(TokenKind::Surface, "panel"), Some("#171717"));
        assert_eq!(contract.get(TokenKind::Breakpoint, "tablet"), Some("48rem"));
        assert_eq!(contract.get(TokenKind::Container, "content"), Some("64rem"));
    }

    #[test]
    fn token_reference_parses() {
        assert_eq!(
            token_reference("token(surface.panel)"),
            Some((TokenKind::Surface, "panel"))
        );
        assert_eq!(
            token_reference("token(color.accent)"),
            Some((TokenKind::Color, "accent"))
        );
        assert_eq!(token_reference("plain"), None);
        assert_eq!(token_reference("token(bogus.name)"), None);
    }

    #[test]
    fn breakpoint_below_subtracts_epsilon() {
        assert_eq!(breakpoint_below("48rem"), "47.9375rem");
        assert_eq!(breakpoint_below("768px"), "767px");
        assert_eq!(breakpoint_below("50vw"), "calc(50vw - 1px)");
    }
}
