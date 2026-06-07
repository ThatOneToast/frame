use std::collections::HashSet;

use crate::{
    knowledge,
    symbols::{index_document, SymbolIndex},
    ComponentDecl, DeclarationKind, Diagnostic, Document, PropType,
};

mod constants;
mod declarations;
mod helpers;
mod statements;
mod ui;

use declarations::*;
use helpers::*;
use statements::*;
use ui::*;

pub fn validate(document: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut names = HashSet::new();
    let symbols = index_document("", document);

    for declaration in &document.declarations {
        if !matches!(
            declaration.kind,
            DeclarationKind::Supports | DeclarationKind::StyleGroup | DeclarationKind::StyleOrder
        ) && !names.insert(declaration.name.text.clone())
        {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Duplicate declaration `{}`.\n\nEach Frame declaration exports one stable class name, so names must be unique within the compiled graph.\n\nRename one declaration, or merge the rules if they describe the same UI concept.",
                    declaration.name.text
                ),
                declaration.name.span,
            ));
        }

        if let DeclarationKind::Unknown(kind) = &declaration.kind {
            let suggestion = closest(kind, knowledge::declaration_keywords())
                .map(|value| format!("\n\nDid you mean `{value}`?"))
                .unwrap_or_default();
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown declaration `{kind}`.\n\nFrame uses design declarations like `card`, `grid`, `area`, `stack`, and `row`.{suggestion}\n\nCompiler detail: unknown declaration kind `{kind}`."
                ),
                declaration.span,
            ));
        }

        if declaration.kind == DeclarationKind::Supports {
            validate_supports_declaration(declaration, &symbols, &mut diagnostics);
            continue;
        }

        if declaration.kind == DeclarationKind::StyleGroup {
            validate_style_group_declaration(declaration, &symbols, &mut diagnostics);
            continue;
        }

        if declaration.kind == DeclarationKind::StyleOrder {
            validate_style_order_declaration(declaration, &mut diagnostics);
            continue;
        }

        validate_statements(declaration, &symbols, &mut diagnostics);

        if declaration.kind == DeclarationKind::Area {
            validate_area(declaration, &symbols, &mut diagnostics);
        }

        if declaration.kind == DeclarationKind::Keyframes {
            validate_keyframes(declaration, &mut diagnostics);
        }
    }

    validate_components(document, &symbols, &mut diagnostics);

    diagnostics
}

fn validate_components(
    document: &Document,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut names = HashSet::new();
    let component_names = document
        .components
        .iter()
        .map(|component| component.name.text.clone())
        .collect::<HashSet<_>>();
    for component in &document.components {
        if !is_valid_style_identifier(&component.name.text) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Invalid component name `{}`.\n\nUse a simple identifier such as `ChatInput` or `MessageList`.",
                    component.name.text
                ),
                component.name.span,
            ));
        }
        if !names.insert(component.name.text.clone()) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Duplicate component `{}`.\n\nEach Frame component name must be unique in a file.",
                    component.name.text
                ),
                component.name.span,
            ));
        }
        validate_component(component, &component_names, symbols, diagnostics);
    }
}

fn validate_component(
    component: &ComponentDecl,
    component_names: &HashSet<String>,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut prop_names = HashSet::new();
    if let Some(props) = &component.props {
        for value in &props.values {
            if !is_valid_style_identifier(&value.name.text) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Invalid prop name `{}`.\n\nUse a simple identifier such as `title`, `active`, or `count`.",
                        value.name.text
                    ),
                    value.name.span,
                ));
            }
            if !prop_names.insert(value.name.text.clone()) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Duplicate prop `{}`.\n\nProp names must be unique within a component.",
                        value.name.text
                    ),
                    value.name.span,
                ));
            }
            if let PropType::Unknown(kind) = &value.value_type {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Unknown prop type `{kind}`.\n\nSupported prop types are `text`, `string`, `bool`, `number`, and `list`."
                    ),
                    value.span,
                ));
            }
        }
    }

    let mut state_names = HashSet::new();
    if let Some(state) = &component.state {
        for value in &state.values {
            if !is_valid_style_identifier(&value.name.text) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Invalid state value `{}`.\n\nUse a simple identifier such as `draft`, `sending`, or `count`.",
                        value.name.text
                    ),
                    value.name.span,
                ));
            }
            if !state_names.insert(value.name.text.clone()) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Duplicate state value `{}`.\n\nState names must be unique within a component.",
                        value.name.text
                    ),
                    value.name.span,
                ));
            }
            validate_state_default(value, diagnostics);
        }
    }

    // Check for prop/state name collisions
    for prop_name in &prop_names {
        if state_names.contains(prop_name) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "`{prop_name}` is declared as both a prop and state.\n\nUse distinct names for props and state within a component."
                ),
                component.span,
            ));
        }
    }

    let all_names: HashSet<String> = prop_names.union(&state_names).cloned().collect();

    if let Some(view) = &component.view {
        for node in &view.nodes {
            validate_ui_node(
                node,
                &all_names,
                component_names,
                symbols,
                diagnostics,
                &prop_names,
            );
        }
    }

    for slot in &component.slots {
        if !is_valid_style_identifier(&slot.name.text) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Invalid slot name `{}`.\n\nUse a simple identifier such as `Header`, `Content`, or `Footer`.",
                    slot.name.text
                ),
                slot.name.span,
            ));
        }
        for node in &slot.nodes {
            validate_ui_node(
                node,
                &all_names,
                component_names,
                symbols,
                diagnostics,
                &prop_names,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ComponentDecl, DataRef, Declaration, DeclarationKind, Document, Identifier, Node, PropType,
        PropValue, PropsDecl, Span, StateDefault, StateType, Statement, TextValue,
        UiComponentArgumentValue, UiElement, UiForLoop, UiNode, UiProperty, UiPropertyValue,
        UiText, ViewDecl,
    };

    use super::*;

    fn declaration(kind: DeclarationKind, name: &str, body: Vec<Node>) -> Declaration {
        Declaration {
            kind,
            name: Identifier::new(name, Span::default()),
            body,
            span: Span::default(),
        }
    }

    fn statement(words: &[&str]) -> Node {
        Node::Statement(Statement {
            words: words.iter().map(|word| word.to_string()).collect(),
            span: Span::default(),
        })
    }

    fn block(name: &str, body: Vec<Node>) -> Node {
        Node::Block(crate::Block {
            name: name.to_string(),
            body,
            span: Span::default(),
        })
    }

    #[test]
    fn validates_area_grid_references_and_places() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![statement(&["columns", "sidebar", "content"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Sidebar",
                    vec![
                        statement(&["in", "AppShell"]),
                        statement(&["place", "footer"]),
                    ],
                ),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("unknown grid slot `footer`"));
    }

    #[test]
    fn explains_area_missing_placement() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Grid,
                    "Dashboard",
                    vec![statement(&["columns", "sidebar", "content"])],
                ),
                declaration(
                    DeclarationKind::Area,
                    "Sidebar",
                    vec![statement(&["in", "Dashboard"])],
                ),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("does not claim a position"));
        assert!(diagnostics[0].message.contains("place"));
    }

    #[test]
    fn explains_raw_css_property_aliases() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Dock,
                "Main",
                vec![statement(&["justify-content", "center"])],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("raw CSS property name"));
        assert!(diagnostics[0].message.contains("justify center"));
        assert!(diagnostics[0].message.contains("advanced"));
    }

    #[test]
    fn accepts_percent_size_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![statement(&["width", "50%"]), statement(&["height", "100%"])],
            )],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn validates_loop_collection_and_scoped_item_references() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("MessageList", Span::default()),
                props: Some(PropsDecl {
                    values: vec![PropValue {
                        name: Identifier::new("messages", Span::default()),
                        value_type: PropType::List,
                        span: Span::default(),
                    }],
                    span: Span::default(),
                }),
                state: None,
                view: Some(ViewDecl {
                    nodes: vec![UiNode::Loop(UiForLoop {
                        item: Identifier::new("message", Span::default()),
                        collection: DataRef {
                            name: Identifier::new("messages", Span::default()),
                            span: Span::default(),
                        },
                        key: None,
                        children: vec![UiNode::Text(UiText {
                            value: TextValue::Data(DataRef {
                                name: Identifier::new("message", Span::default()),
                                span: Span::default(),
                            }),
                            span: Span::default(),
                        })],
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .all(|diagnostic| { !diagnostic.message.contains("Unknown reference `$message`") }));
    }

    #[test]
    fn rejects_unsafe_html_attributes_and_warns_about_url_sinks() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("UnsafePanel", Span::default()),
                props: None,
                state: None,
                view: Some(ViewDecl {
                    nodes: vec![UiNode::Element(UiElement {
                        kind: Identifier::new("link", Span::default()),
                        name: Identifier::new("DocsLink", Span::default()),
                        style: None,
                        properties: vec![
                            UiProperty {
                                name: Identifier::new("href", Span::default()),
                                value: UiPropertyValue::Literal("https://example.com".to_string()),
                                span: Span::default(),
                            },
                            UiProperty {
                                name: Identifier::new("html", Span::default()),
                                value: UiPropertyValue::Literal("<b>unsafe</b>".to_string()),
                                span: Span::default(),
                            },
                        ],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("browser-centric")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("unsafe HTML injection sink")));
    }

    #[test]
    fn diagnoses_accessibility_and_security_for_phase_three_dom_nodes() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("DiagnosticsDemo", Span::default()),
                props: None,
                state: None,
                view: Some(ViewDecl {
                    nodes: vec![
                        UiNode::Element(UiElement {
                            kind: Identifier::new("image", Span::default()),
                            name: Identifier::new("Avatar", Span::default()),
                            style: None,
                            properties: vec![UiProperty {
                                name: Identifier::new("source", Span::default()),
                                value: UiPropertyValue::Literal("javascript:alert(1)".to_string()),
                                span: Span::default(),
                            }],
                            events: Vec::new(),
                            children: Vec::new(),
                            span: Span::default(),
                        }),
                        UiNode::Element(UiElement {
                            kind: Identifier::new("link", Span::default()),
                            name: Identifier::new("External", Span::default()),
                            style: None,
                            properties: vec![
                                UiProperty {
                                    name: Identifier::new("href", Span::default()),
                                    value: UiPropertyValue::Literal(
                                        "https://example.com".to_string(),
                                    ),
                                    span: Span::default(),
                                },
                                UiProperty {
                                    name: Identifier::new("target", Span::default()),
                                    value: UiPropertyValue::Literal("_blank".to_string()),
                                    span: Span::default(),
                                },
                            ],
                            events: Vec::new(),
                            children: Vec::new(),
                            span: Span::default(),
                        }),
                        UiNode::Element(UiElement {
                            kind: Identifier::new("input", Span::default()),
                            name: Identifier::new("input", Span::default()),
                            style: None,
                            properties: vec![UiProperty {
                                name: Identifier::new("onclick", Span::default()),
                                value: UiPropertyValue::Literal("alert(1)".to_string()),
                                span: Span::default(),
                            }],
                            events: Vec::new(),
                            children: Vec::new(),
                            span: Span::default(),
                        }),
                        UiNode::Element(UiElement {
                            kind: Identifier::new("dialog", Span::default()),
                            name: Identifier::new("dialog", Span::default()),
                            style: None,
                            properties: Vec::new(),
                            events: Vec::new(),
                            children: Vec::new(),
                            span: Span::default(),
                        }),
                    ],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("requires alternate text")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("unsafe URL scheme")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("browser-centric")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("inline event attribute")));
        assert!(diagnostics.iter().any(|diagnostic| diagnostic
            .message
            .contains("`input` requires a Frame label")));
        assert!(diagnostics.iter().any(|diagnostic| diagnostic
            .message
            .contains("`dialog` requires a Frame label")));
    }

    #[test]
    fn accepts_app_driven_native_styling_vocabulary() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Theme",
                    vec![statement(&["color", "terminal-border", "#263241"])],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![
                        statement(&["columns", "header", "sidebar", "content", "users"]),
                        statement(&["tracks", "columns", "rail", "panel", "fill", "side"]),
                        statement(&["tracks", "rows", "header", "fill", "composer"]),
                        statement(&["areas", "header", "header", "header", "header"]),
                        statement(&["overflow", "hidden"]),
                        statement(&["box", "border"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Button,
                    "ChannelButton",
                    vec![
                        statement(&["layout", "icon-content-action"]),
                        statement(&["control", "reset"]),
                        statement(&["interactive"]),
                        statement(&["align-text", "left"]),
                        statement(&["border", "bottom", "terminal-border"]),
                        statement(&["scroll", "y"]),
                        statement(&["scrollbar", "dense"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Text,
                    "MessageText",
                    vec![
                        statement(&["truncate"]),
                        statement(&["wrap", "anywhere"]),
                        statement(&["case", "uppercase"]),
                        statement(&["line", "relaxed"]),
                        statement(&["letter", "normal"]),
                        statement(&["min-width", "zero"]),
                        statement(&["square", "avatar"]),
                        statement(&["self", "center"]),
                        statement(&["nudge", "top-right"]),
                    ],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_display_flex_visibility_and_logical_sizing() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["display", "flex"]),
                    statement(&["visibility", "hidden"]),
                    statement(&["flex", "direction", "column"]),
                    statement(&["flex", "wrap", "wrap"]),
                    statement(&["flex", "grow", "1"]),
                    statement(&["flex", "shrink", "0"]),
                    statement(&["flex", "basis", "fill"]),
                    statement(&["inline-size", "fill"]),
                    statement(&["block-size", "screen"]),
                    statement(&["min-inline-size", "zero"]),
                    statement(&["max-block-size", "100%"]),
                ],
            )],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_expanded_typography_controls() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Text,
                "MessageBody",
                vec![
                    statement(&["align-text", "justify"]),
                    statement(&["case", "capitalize"]),
                    statement(&["decoration", "underline"]),
                    statement(&["whitespace", "pre-wrap"]),
                    statement(&["word-break", "break-word"]),
                    statement(&["hyphenate", "auto"]),
                ],
            )],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_border_styles_and_outline_offsets() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Theme",
                    vec![statement(&["color", "focus-ring", "#93c5fd"])],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![
                        statement(&["border", "style", "dashed"]),
                        statement(&["border", "width", "large"]),
                        statement(&["outline", "focus-ring"]),
                        statement(&["outline", "offset", "small"]),
                    ],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_display_flex_and_visibility_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["display", "table"]),
                    statement(&["visibility", "gone"]),
                    statement(&["flex", "direction", "sideways"]),
                    statement(&["flex", "grow", "-1"]),
                    statement(&["flex", "basis", "huge"]),
                ],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 5);
        assert!(diagnostics[0].message.contains("Unknown display value"));
        assert!(diagnostics[1].message.contains("Unknown visibility value"));
        assert!(diagnostics[2].message.contains("Unknown flex direction"));
        assert!(diagnostics[3].message.contains("non-negative number"));
        assert!(diagnostics[4].message.contains("valid flex basis value"));
    }

    #[test]
    fn rejects_invalid_expanded_typography_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Text,
                "MessageBody",
                vec![
                    statement(&["align-text", "middle"]),
                    statement(&["case", "title"]),
                    statement(&["decoration", "blink"]),
                    statement(&["whitespace", "squish"]),
                    statement(&["word-break", "shatter"]),
                    statement(&["hyphenate", "always"]),
                ],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 6);
        assert!(diagnostics[0].message.contains("Unknown align-text value"));
        assert!(diagnostics[1].message.contains("Unknown case value"));
        assert!(diagnostics[2].message.contains("Unknown decoration value"));
        assert!(diagnostics[3].message.contains("Unknown whitespace value"));
        assert!(diagnostics[4].message.contains("Unknown word-break value"));
        assert!(diagnostics[5].message.contains("Unknown hyphenate value"));
    }

    #[test]
    fn rejects_invalid_border_styles_and_outline_offsets() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["border", "style", "wiggly"]),
                    statement(&["outline", "missing"]),
                    statement(&["outline", "offset", "huge"]),
                ],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 3);
        assert!(diagnostics[0].message.contains("Unknown border style"));
        assert!(diagnostics[1].message.contains("Unknown outline value"));
        assert!(diagnostics[2].message.contains("Unknown outline offset"));
    }

    #[test]
    fn accepts_intent_motion_helpers_and_tuned_amounts() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "FloatingCard",
                vec![
                    statement(&["lift", "small%44"]),
                    statement(&["sink", "huge%50"]),
                    statement(&["shift", "right", "medium"]),
                    statement(&["grow", "slight%5"]),
                    statement(&["shrink", "subtle"]),
                    statement(&["tilt", "left", "dramatic%100"]),
                    block(
                        "hover",
                        vec![
                            statement(&["lift", "small"]),
                            statement(&["grow", "slight"]),
                        ],
                    ),
                    block("active", vec![statement(&["press"])]),
                    block("checked", vec![statement(&["pop"])]),
                ],
            )],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_intent_motion_amounts() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "FloatingCard",
                vec![
                    statement(&["lift", "giant"]),
                    statement(&["lift", "small%101"]),
                    statement(&["lift", "small%half"]),
                    statement(&["shift", "diagonal", "small"]),
                    statement(&["tilt", "up", "subtle"]),
                    statement(&["grow", "medium"]),
                ],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 6);
        assert!(diagnostics[0].message.contains("Unknown movement amount"));
        assert!(diagnostics[1].message.contains("invalid percent tuning"));
        assert!(diagnostics[2].message.contains("invalid percent tuning"));
        assert!(diagnostics[3].message.contains("Unknown shift direction"));
        assert!(diagnostics[4].message.contains("Unknown tilt direction"));
        assert!(diagnostics[5].message.contains("Unknown visual amount"));
    }

    #[test]
    fn rejects_invalid_percent_size_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![
                        statement(&["width", "-10%"]),
                        statement(&["height", "120%%"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "Dashboard",
                    vec![statement(&["columns", "25%", "abc%", "75%"])],
                ),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 3);
        assert!(diagnostics[0].message.contains("0%` to `100%"));
        assert!(diagnostics[2]
            .message
            .contains("invalid columns percentage"));
    }

    #[test]
    fn accepts_custom_color_tokens_and_usage() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand", "#7c3aed"]),
                        statement(&["color", "panel-bg", "#181820"]),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "BrandCard",
                    vec![
                        statement(&["background", "brand"]),
                        statement(&["color", "white"]),
                        statement(&["border", "panel-bg"]),
                    ],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_custom_gradient_tokens_and_usage() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand-purple", "#7c3aed"]),
                        Node::Block(crate::Block {
                            name: "gradient hero-gradient".to_string(),
                            body: vec![
                                statement(&["type", "linear"]),
                                statement(&["angle", "135deg"]),
                                statement(&["stop", "brand-purple", "0%"]),
                                statement(&["stop", "#181820", "100%"]),
                            ],
                            span: Span::default(),
                        }),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Hero",
                    vec![statement(&["background", "hero-gradient"])],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_corner_gradients_targeted_padding_and_anchor() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Tokens,
                    "Brand",
                    vec![
                        statement(&["color", "brand-purple", "#7c3aed"]),
                        Node::Block(crate::Block {
                            name: "gradient four-corners".to_string(),
                            body: vec![
                                statement(&["type", "layered"]),
                                statement(&["corner", "top-left", "brand-purple", "65%"]),
                                statement(&["corner", "bottom-right", "#181820", "70%"]),
                            ],
                            span: Span::default(),
                        }),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "PinnedHero",
                    vec![
                        statement(&["background", "four-corners"]),
                        statement(&["padding", "top", "large"]),
                        statement(&["padding", "x", "medium"]),
                        statement(&["anchor", "top"]),
                    ],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_vertical_grid_flow_and_section_spacing() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "HoverCardInfo",
                vec![
                    statement(&["flow", "vertical"]),
                    statement(&["columns", "title", "description"]),
                    block(
                        "section title",
                        vec![statement(&["padding", "bottom", "small"])],
                    ),
                    block(
                        "section description",
                        vec![statement(&["margin", "top", "none"])],
                    ),
                ],
            )],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_grid_flow_and_section_properties() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Grid,
                "HoverCardInfo",
                vec![
                    statement(&["flow", "diagonal"]),
                    block("section title", vec![statement(&["surface", "panel"])]),
                ],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("Unknown flow value"));
        assert!(diagnostics[0].message.contains("Valid values include"));
        assert!(diagnostics[1]
            .message
            .contains("Unknown section property `surface`"));
    }

    #[test]
    fn explains_unknown_color_and_background_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Card,
                "Panel",
                vec![
                    statement(&["background", "accnt"]),
                    statement(&["color", "primry"]),
                ],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("Unknown background"));
        assert!(diagnostics[0].message.contains("Did you mean"));
        assert!(diagnostics[0].message.contains("custom color tokens"));
        assert!(diagnostics[1].message.contains("Unknown color"));
        assert!(diagnostics[1].message.contains("semantic color intent"));
    }

    #[test]
    fn explains_duplicate_declarations() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(DeclarationKind::Card, "Panel", Vec::new()),
                declaration(DeclarationKind::Card, "Panel", Vec::new()),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("Each Frame declaration exports one stable class name"));
    }

    #[test]
    fn rejects_invalid_corner_gradient_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![Node::Block(crate::Block {
                    name: "gradient bad".to_string(),
                    body: vec![statement(&["corner", "middle", "missing-color"])],
                    span: Span::default(),
                })],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("Unknown gradient corner"));
        assert!(diagnostics[1]
            .message
            .contains("Unknown gradient corner color"));
    }

    #[test]
    fn rejects_invalid_gradient_stop_color() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![Node::Block(crate::Block {
                    name: "gradient hero-gradient".to_string(),
                    body: vec![
                        statement(&["angle", "135deg"]),
                        statement(&["stop", "missing-color", "0%"]),
                        statement(&["stop", "#181820", "100%"]),
                    ],
                    span: Span::default(),
                })],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("Unknown gradient stop color"));
    }

    #[test]
    fn accepts_keyframes_animation_blocks_and_responsive_conditions() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Keyframes,
                    "FloatIn",
                    vec![
                        block(
                            "from",
                            vec![
                                statement(&["opacity", "0"]),
                                statement(&["transform", "translateY(12px)"]),
                            ],
                        ),
                        block(
                            "to",
                            vec![
                                statement(&["opacity", "1"]),
                                statement(&["transform", "translateY(0)"]),
                            ],
                        ),
                    ],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![
                        statement(&["columns", "sidebar", "content"]),
                        block("below tablet", vec![statement(&["columns", "content"])]),
                        block("container narrow", vec![statement(&["columns", "content"])]),
                    ],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![block(
                        "animation FloatIn",
                        vec![
                            statement(&["duration", "240ms"]),
                            statement(&["delay", "0ms"]),
                            statement(&["ease", "smooth"]),
                            statement(&["iteration", "1"]),
                            statement(&["direction", "normal"]),
                            statement(&["fill", "both"]),
                            statement(&["play-state", "running"]),
                        ],
                    )],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_typed_supports_blocks() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Supports,
                    "display grid",
                    vec![block(
                        "grid AppShell",
                        vec![statement(&["columns", "sidebar", "content"])],
                    )],
                ),
                declaration(
                    DeclarationKind::Supports,
                    "backdrop blur",
                    vec![block(
                        "card GlassPanel",
                        vec![statement(&["surface", "glass"])],
                    )],
                ),
                declaration(
                    DeclarationKind::Supports,
                    "subgrid",
                    vec![block(
                        "grid NestedGrid",
                        vec![statement(&["columns", "subgrid"])],
                    )],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn accepts_style_groups_and_style_order() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::StyleOrder,
                    "reset, base, components, utilities",
                    Vec::new(),
                ),
                declaration(
                    DeclarationKind::StyleGroup,
                    "components",
                    vec![block(
                        "button PrimaryButton",
                        vec![statement(&["surface", "panel"])],
                    )],
                ),
            ],
            components: Vec::new(),
        };

        assert!(validate(&document).is_empty());
    }

    #[test]
    fn rejects_invalid_style_groups_and_style_order() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(DeclarationKind::StyleOrder, "base, 123bad", Vec::new()),
                declaration(
                    DeclarationKind::StyleGroup,
                    "123bad",
                    vec![
                        statement(&["surface", "accent"]),
                        block("tokens Brand", vec![statement(&["color", "brand", "#fff"])]),
                    ],
                ),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 4);
        assert!(diagnostics[0].message.contains("Invalid style-order group"));
        assert!(diagnostics[1].message.contains("Invalid style group"));
        assert!(diagnostics[2]
            .message
            .contains("style-group blocks contain style declarations"));
        assert!(diagnostics[3]
            .message
            .contains("cannot be declared inside `style-group`"));
    }

    #[test]
    fn rejects_invalid_typed_supports_blocks() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Supports,
                    "display table",
                    vec![block(
                        "grid AppShell",
                        vec![statement(&["columns", "content"])],
                    )],
                ),
                declaration(
                    DeclarationKind::Supports,
                    "magic sparkle",
                    vec![statement(&["columns", "content"])],
                ),
                declaration(
                    DeclarationKind::Supports,
                    "color oklch",
                    vec![block(
                        "tokens Brand",
                        vec![statement(&["color", "brand", "#fff"])],
                    )],
                ),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 4);
        assert!(diagnostics[0]
            .message
            .contains("Unknown display support value"));
        assert!(diagnostics[1].message.contains("Unknown support predicate"));
        assert!(diagnostics[2]
            .message
            .contains("supports blocks contain style declarations"));
        assert!(diagnostics[3]
            .message
            .contains("cannot be declared inside `supports`"));
    }

    #[test]
    fn rejects_invalid_keyframes_animation_and_responsive_values() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                declaration(
                    DeclarationKind::Keyframes,
                    "FloatIn",
                    vec![block("middle", vec![statement(&["left", "0"])])],
                ),
                declaration(
                    DeclarationKind::Grid,
                    "AppShell",
                    vec![block(
                        "below phablet",
                        vec![statement(&["columns", "content"])],
                    )],
                ),
                declaration(
                    DeclarationKind::Card,
                    "Panel",
                    vec![block(
                        "animation MissingMotion",
                        vec![statement(&["fill", "sideways"])],
                    )],
                ),
            ],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown keyframe selector")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown breakpoint")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown animation")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown fill value")));
    }

    #[test]
    fn validates_initial_ui_component_semantics() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Button,
                "PrimaryButton",
                Vec::new(),
            )],
            components: vec![ComponentDecl {
                name: Identifier::new("ChatInput", Span::default()),
                props: None,
                state: Some(crate::StateDecl {
                    values: vec![
                        crate::StateValue {
                            name: Identifier::new("draft", Span::default()),
                            value_type: StateType::Text,
                            default: StateDefault::Number("123".to_string()),
                            span: Span::default(),
                        },
                        crate::StateValue {
                            name: Identifier::new("sending", Span::default()),
                            value_type: StateType::Bool,
                            default: StateDefault::Bool(false),
                            span: Span::default(),
                        },
                    ],
                    span: Span::default(),
                }),
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("action", Span::default()),
                        name: Identifier::new("Send", Span::default()),
                        style: Some(crate::StyleBinding {
                            name: Identifier::new("MissingButton", Span::default()),
                            span: Span::default(),
                        }),
                        properties: vec![
                            crate::UiProperty {
                                name: Identifier::new("disabled", Span::default()),
                                value: UiPropertyValue::Conditional(crate::ConditionalBinding {
                                    condition: DataRef {
                                        name: Identifier::new("message", Span::default()),
                                        span: Span::default(),
                                    },
                                    span: Span::default(),
                                }),
                                span: Span::default(),
                            },
                            crate::UiProperty {
                                name: Identifier::new("style", Span::default()),
                                value: UiPropertyValue::StyleWhen {
                                    condition: DataRef {
                                        name: Identifier::new("sending", Span::default()),
                                        span: Span::default(),
                                    },
                                    style: crate::StyleBinding {
                                        name: Identifier::new("PrimaryButton", Span::default()),
                                        span: Span::default(),
                                    },
                                },
                                span: Span::default(),
                            },
                        ],
                        events: vec![crate::EventBinding {
                            event: Identifier::new("press", Span::default()),
                            modifiers: vec![Identifier::new("magic", Span::default())],
                            handler: crate::HandlerRef {
                                name: Identifier::new("sendMessage", Span::default()),
                                span: Span::default(),
                            },
                            span: Span::default(),
                        }],
                        children: vec![UiNode::Text(crate::UiText {
                            value: TextValue::Data(DataRef {
                                name: Identifier::new("draft", Span::default()),
                                span: Span::default(),
                            }),
                            span: Span::default(),
                        })],
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("does not match declared type")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown reference `$message`")));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.message.contains("Unknown event `press`")));
        assert!(diagnostics.iter().any(|diagnostic| diagnostic
            .message
            .contains("Unknown event modifier `magic`")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Style `MissingButton`")));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("@sendMessage references")));
    }

    #[test]
    fn validates_component_invocation_semantics() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![
                ComponentDecl {
                    name: Identifier::new("ChatApp", Span::default()),
                    props: None,
                    state: Some(crate::StateDecl {
                        values: vec![crate::StateValue {
                            name: Identifier::new("activeChannel", Span::default()),
                            value_type: StateType::Text,
                            default: StateDefault::Text("general".to_string()),
                            span: Span::default(),
                        }],
                        span: Span::default(),
                    }),
                    view: Some(crate::ViewDecl {
                        nodes: vec![
                            UiNode::Component(crate::UiComponentInvocation {
                                name: Identifier::new("ChatPanel", Span::default()),
                                arguments: vec![crate::UiComponentArgument {
                                    name: Identifier::new("channel", Span::default()),
                                    value: UiComponentArgumentValue::Data(DataRef {
                                        name: Identifier::new("missing", Span::default()),
                                        span: Span::default(),
                                    }),
                                    span: Span::default(),
                                }],
                                span: Span::default(),
                            }),
                            UiNode::Component(crate::UiComponentInvocation {
                                name: Identifier::new("MissingPanel", Span::default()),
                                arguments: Vec::new(),
                                span: Span::default(),
                            }),
                        ],
                        span: Span::default(),
                    }),
                    slots: Vec::new(),
                    span: Span::default(),
                },
                ComponentDecl {
                    name: Identifier::new("ChatPanel", Span::default()),
                    props: None,
                    state: None,
                    view: None,
                    slots: Vec::new(),
                    span: Span::default(),
                },
            ],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("Unknown reference `$missing`")));
        assert!(diagnostics.iter().any(|diagnostic| diagnostic
            .message
            .contains("Unknown component `MissingPanel`")));
    }

    #[test]
    fn knowledge_catalog_documents_new_motion_and_responsive_concepts() {
        let keyframes = knowledge::concept("keyframes").expect("keyframes concept");
        let below = knowledge::concept("below").expect("below concept");
        let container = knowledge::concept("container").expect("container concept");

        assert!(keyframes.markdown().contains("@keyframes frame-Name"));
        assert!(below.markdown().contains("@media"));
        assert!(container.markdown().contains("@container"));
    }

    #[test]
    fn rejects_invalid_hex_color_tokens() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![statement(&["color", "brand", "#12"])],
            )],
            components: Vec::new(),
        };

        let diagnostics = validate(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("valid color token"));
    }

    #[test]
    fn rejects_value_bind_on_text_primitive() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Demo", Span::default()),
                props: None,
                state: Some(crate::StateDecl {
                    values: vec![crate::StateValue {
                        name: Identifier::new("draft", Span::default()),
                        value_type: StateType::Text,
                        default: StateDefault::Text("".to_string()),
                        span: Span::default(),
                    }],
                    span: Span::default(),
                }),
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("text", Span::default()),
                        name: Identifier::new("Message", Span::default()),
                        style: None,
                        properties: vec![crate::UiProperty {
                            name: Identifier::new("value", Span::default()),
                            value: UiPropertyValue::Bind(DataRef {
                                name: Identifier::new("draft", Span::default()),
                                span: Span::default(),
                            }),
                            span: Span::default(),
                        }],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics.iter().any(|d| d
            .message
            .contains("`value bind` is only valid on input-like primitives")));
    }

    #[test]
    fn rejects_placeholder_on_action_primitive() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Demo", Span::default()),
                props: None,
                state: None,
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("action", Span::default()),
                        name: Identifier::new("Send", Span::default()),
                        style: None,
                        properties: vec![crate::UiProperty {
                            name: Identifier::new("placeholder", Span::default()),
                            value: UiPropertyValue::Literal("Type here".to_string()),
                            span: Span::default(),
                        }],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics.iter().any(|d| d
            .message
            .contains("Action-like primitives do not accept placeholder text")));
    }

    #[test]
    fn accepts_valid_input_properties() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Demo", Span::default()),
                props: None,
                state: Some(crate::StateDecl {
                    values: vec![crate::StateValue {
                        name: Identifier::new("draft", Span::default()),
                        value_type: StateType::Text,
                        default: StateDefault::Text("".to_string()),
                        span: Span::default(),
                    }],
                    span: Span::default(),
                }),
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("input", Span::default()),
                        name: Identifier::new("MessageBox", Span::default()),
                        style: None,
                        properties: vec![
                            crate::UiProperty {
                                name: Identifier::new("value", Span::default()),
                                value: UiPropertyValue::Bind(DataRef {
                                    name: Identifier::new("draft", Span::default()),
                                    span: Span::default(),
                                }),
                                span: Span::default(),
                            },
                            crate::UiProperty {
                                name: Identifier::new("placeholder", Span::default()),
                                value: UiPropertyValue::Literal("Message".to_string()),
                                span: Span::default(),
                            },
                            crate::UiProperty {
                                name: Identifier::new("label", Span::default()),
                                value: UiPropertyValue::Literal("Draft".to_string()),
                                span: Span::default(),
                            },
                        ],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(!diagnostics
            .iter()
            .any(|d| d.message.contains("is not a valid property for `input`")));
    }

    #[test]
    fn rejects_value_bind_on_container_primitive() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Demo", Span::default()),
                props: None,
                state: Some(crate::StateDecl {
                    values: vec![crate::StateValue {
                        name: Identifier::new("draft", Span::default()),
                        value_type: StateType::Text,
                        default: StateDefault::Text("".to_string()),
                        span: Span::default(),
                    }],
                    span: Span::default(),
                }),
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("panel", Span::default()),
                        name: Identifier::new("Main", Span::default()),
                        style: None,
                        properties: vec![crate::UiProperty {
                            name: Identifier::new("value", Span::default()),
                            value: UiPropertyValue::Bind(DataRef {
                                name: Identifier::new("draft", Span::default()),
                                span: Span::default(),
                            }),
                            span: Span::default(),
                        }],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics.iter().any(|d| d
            .message
            .contains("Container primitives group children. They do not own form state")));
    }

    #[test]
    fn rejects_source_on_non_media_primitive() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Demo", Span::default()),
                props: None,
                state: None,
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("action", Span::default()),
                        name: Identifier::new("Play", Span::default()),
                        style: None,
                        properties: vec![crate::UiProperty {
                            name: Identifier::new("source", Span::default()),
                            value: UiPropertyValue::Literal("https://example.com".to_string()),
                            span: Span::default(),
                        }],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(diagnostics.iter().any(|d| d
            .message
            .contains("`source` is not a valid property for `action`")));
    }

    #[test]
    fn accepts_media_source_on_media_primitive() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Demo", Span::default()),
                props: None,
                state: None,
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Element(crate::UiElement {
                        kind: Identifier::new("media", Span::default()),
                        name: Identifier::new("Preview", Span::default()),
                        style: None,
                        properties: vec![crate::UiProperty {
                            name: Identifier::new("source", Span::default()),
                            value: UiPropertyValue::Literal(
                                "https://example.com/video.mp4".to_string(),
                            ),
                            span: Span::default(),
                        }],
                        events: Vec::new(),
                        children: Vec::new(),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: Vec::new(),
                span: Span::default(),
            }],
        };

        let diagnostics = validate(&document);

        assert!(!diagnostics
            .iter()
            .any(|d| d.message.contains("is not a valid property for `media`")));
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("navigation or media destination")));
    }
}
