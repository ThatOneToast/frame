use serde::{Deserialize, Serialize};

use crate::{
    ComponentDecl, DataRef, Document, EventBinding, Identifier, Span, StateDefault, StateType,
    TextValue, UiElement, UiNode, UiProperty, UiPropertyValue, UiText,
};

pub const FRAME_IR_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrDocument {
    pub version: u32,
    pub components: Vec<FrameIrComponent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrComponent {
    pub name: String,
    pub state: Vec<FrameIrState>,
    pub nodes: Vec<FrameIrNode>,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrState {
    pub name: String,
    pub value_type: FrameIrStateType,
    pub default: FrameIrStateDefault,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrStateType {
    Text,
    Bool,
    Number,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrStateDefault {
    Text(String),
    Bool(bool),
    Number(String),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrNode {
    Element(FrameIrElement),
    Text(FrameIrText),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrElement {
    pub kind: String,
    pub name: String,
    pub style: FrameIrStyleBinding,
    pub attributes: Vec<FrameIrAttribute>,
    pub bindings: Vec<FrameIrBinding>,
    pub events: Vec<FrameIrEvent>,
    pub conditions: Vec<FrameIrCondition>,
    pub children: Vec<FrameIrNode>,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrText {
    pub value: FrameIrTextValue,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrTextValue {
    Literal(String),
    DataRef(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrAttribute {
    pub name: String,
    pub value: FrameIrAttributeValue,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrAttributeValue {
    Literal(String),
    DataRef(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrBinding {
    pub property: String,
    pub state: String,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrEvent {
    pub event: String,
    pub modifiers: Vec<String>,
    pub handler: String,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrStyleBinding {
    Explicit {
        style: String,
        source: FrameIrSourceSpan,
    },
    Automatic {
        style: String,
        source: FrameIrSourceSpan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrCondition {
    Flag {
        property: String,
        state: String,
        source: FrameIrSourceSpan,
    },
    Style {
        state: String,
        style: String,
        source: FrameIrSourceSpan,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrSourceSpan {
    pub start: usize,
    pub end: usize,
}

pub fn lower_document_to_ir(document: &Document) -> FrameIrDocument {
    FrameIrDocument {
        version: FRAME_IR_VERSION,
        components: document.components.iter().map(lower_component).collect(),
    }
}

fn lower_component(component: &ComponentDecl) -> FrameIrComponent {
    FrameIrComponent {
        name: component.name.text.clone(),
        state: component
            .state
            .as_ref()
            .map(|state| state.values.iter().map(lower_state).collect())
            .unwrap_or_default(),
        nodes: component
            .view
            .as_ref()
            .map(|view| view.nodes.iter().map(lower_node).collect())
            .unwrap_or_default(),
        source: span(component.span),
    }
}

fn lower_state(state: &crate::StateValue) -> FrameIrState {
    FrameIrState {
        name: state.name.text.clone(),
        value_type: match &state.value_type {
            StateType::Text => FrameIrStateType::Text,
            StateType::Bool => FrameIrStateType::Bool,
            StateType::Number => FrameIrStateType::Number,
            StateType::Unknown(value) => FrameIrStateType::Unknown(value.clone()),
        },
        default: match &state.default {
            StateDefault::Text(value) => FrameIrStateDefault::Text(value.clone()),
            StateDefault::Bool(value) => FrameIrStateDefault::Bool(*value),
            StateDefault::Number(value) => FrameIrStateDefault::Number(value.clone()),
            StateDefault::Invalid(value) => FrameIrStateDefault::Invalid(value.clone()),
        },
        source: span(state.span),
    }
}

fn lower_node(node: &UiNode) -> FrameIrNode {
    match node {
        UiNode::Element(element) => FrameIrNode::Element(lower_element(element)),
        UiNode::Text(text) => FrameIrNode::Text(lower_text(text)),
    }
}

fn lower_element(element: &UiElement) -> FrameIrElement {
    let mut attributes = Vec::new();
    let mut bindings = Vec::new();
    let mut conditions = Vec::new();

    for property in &element.properties {
        lower_property(
            property,
            &mut attributes,
            &mut bindings,
            &mut conditions,
        );
    }

    FrameIrElement {
        kind: element.kind.text.clone(),
        name: element.name.text.clone(),
        style: element.style.as_ref().map_or_else(
            || FrameIrStyleBinding::Automatic {
                style: element.name.text.clone(),
                source: span(element.name.span),
            },
            |style| FrameIrStyleBinding::Explicit {
                style: style.name.text.clone(),
                source: span(style.span),
            },
        ),
        attributes,
        bindings,
        events: element.events.iter().map(lower_event).collect(),
        conditions,
        children: element.children.iter().map(lower_node).collect(),
        source: span(element.span),
    }
}

fn lower_property(
    property: &UiProperty,
    attributes: &mut Vec<FrameIrAttribute>,
    bindings: &mut Vec<FrameIrBinding>,
    conditions: &mut Vec<FrameIrCondition>,
) {
    match &property.value {
        UiPropertyValue::Literal(value) => attributes.push(FrameIrAttribute {
            name: property.name.text.clone(),
            value: FrameIrAttributeValue::Literal(value.clone()),
            source: span(property.span),
        }),
        UiPropertyValue::Data(reference) => attributes.push(FrameIrAttribute {
            name: property.name.text.clone(),
            value: FrameIrAttributeValue::DataRef(reference_name(reference)),
            source: span(property.span),
        }),
        UiPropertyValue::Bind(reference) => bindings.push(FrameIrBinding {
            property: property.name.text.clone(),
            state: reference_name(reference),
            source: span(property.span),
        }),
        UiPropertyValue::Conditional(binding) => conditions.push(FrameIrCondition::Flag {
            property: property.name.text.clone(),
            state: reference_name(&binding.condition),
            source: span(binding.span),
        }),
        UiPropertyValue::StyleWhen { condition, style } => conditions.push(FrameIrCondition::Style {
            state: reference_name(condition),
            style: style.name.text.clone(),
            source: span(style.span),
        }),
        UiPropertyValue::Unknown(_) => {}
    }
}

fn lower_text(text: &UiText) -> FrameIrText {
    FrameIrText {
        value: match &text.value {
            TextValue::Literal(value) => FrameIrTextValue::Literal(value.clone()),
            TextValue::Data(reference) => FrameIrTextValue::DataRef(reference_name(reference)),
        },
        source: span(text.span),
    }
}

fn lower_event(event: &EventBinding) -> FrameIrEvent {
    FrameIrEvent {
        event: event.event.text.clone(),
        modifiers: event
            .modifiers
            .iter()
            .map(|modifier| modifier.text.clone())
            .collect(),
        handler: event.handler.name.text.clone(),
        source: span(event.span),
    }
}

fn reference_name(reference: &DataRef) -> String {
    reference.name.text.clone()
}

fn span(span: Span) -> FrameIrSourceSpan {
    FrameIrSourceSpan {
        start: span.start,
        end: span.end,
    }
}

#[allow(dead_code)]
fn identifier_name(identifier: &Identifier) -> String {
    identifier.text.clone()
}

#[cfg(test)]
mod tests {
    use crate::{
        ConditionalBinding, Document, HandlerRef, StateDecl, StateValue, StyleBinding, UiProperty,
    };

    use super::*;

    #[test]
    fn lowers_initial_ui_ast_to_ir() {
        let document = document_fixture();

        let ir = lower_document_to_ir(&document);
        assert_eq!(ir.version, FRAME_IR_VERSION);
        assert_eq!(ir.components.len(), 1);

        let component = &ir.components[0];
        assert_eq!(component.name, "ChatInput");
        assert_eq!(component.state.len(), 2);
        assert_eq!(component.state[0].name, "draft");
        assert_eq!(component.state[0].value_type, FrameIrStateType::Text);
        assert_eq!(
            component.state[0].default,
            FrameIrStateDefault::Text(String::new())
        );

        let input = match &component.nodes[0] {
            FrameIrNode::Element(element) => element,
            _ => panic!("expected input element"),
        };
        assert_eq!(input.kind, "input");
        assert_eq!(input.name, "MessageBox");
        assert!(matches!(
            input.style,
            FrameIrStyleBinding::Automatic { ref style, .. } if style == "MessageBox"
        ));
        assert_eq!(input.bindings[0].property, "value");
        assert_eq!(input.bindings[0].state, "draft");
        assert_eq!(input.attributes[0].name, "placeholder");
        assert_eq!(input.events[0].event, "keydown");
        assert_eq!(input.events[0].modifiers, vec!["enter"]);
        assert_eq!(input.events[0].handler, "sendMessage");

        let button = match &component.nodes[1] {
            FrameIrNode::Element(element) => element,
            _ => panic!("expected button element"),
        };
        assert!(matches!(
            button.style,
            FrameIrStyleBinding::Explicit { ref style, .. } if style == "PrimaryButton"
        ));
        assert!(matches!(
            button.children[0],
            FrameIrNode::Text(FrameIrText {
                value: FrameIrTextValue::Literal(ref value),
                ..
            }) if value == "Send"
        ));
        assert!(matches!(
            button.conditions[0],
            FrameIrCondition::Flag {
                ref property,
                ref state,
                ..
            } if property == "disabled" && state == "sending"
        ));
        assert!(matches!(
            button.conditions[1],
            FrameIrCondition::Style {
                ref state,
                ref style,
                ..
            } if state == "sending" && style == "LoadingButton"
        ));
    }

    fn document_fixture() -> Document {
        Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: ident("ChatInput"),
                state: Some(StateDecl {
                    values: vec![
                        StateValue {
                            name: ident("draft"),
                            value_type: StateType::Text,
                            default: StateDefault::Text(String::new()),
                            span: Span::default(),
                        },
                        StateValue {
                            name: ident("sending"),
                            value_type: StateType::Bool,
                            default: StateDefault::Bool(false),
                            span: Span::default(),
                        },
                    ],
                    span: Span::default(),
                }),
                view: Some(crate::ViewDecl {
                    nodes: vec![
                        UiNode::Element(UiElement {
                            kind: ident("input"),
                            name: ident("MessageBox"),
                            style: None,
                            properties: vec![
                                UiProperty {
                                    name: ident("value"),
                                    value: UiPropertyValue::Bind(data_ref("draft")),
                                    span: Span::default(),
                                },
                                UiProperty {
                                    name: ident("placeholder"),
                                    value: UiPropertyValue::Literal("Message".to_string()),
                                    span: Span::default(),
                                },
                            ],
                            events: vec![EventBinding {
                                event: ident("keydown"),
                                modifiers: vec![ident("enter")],
                                handler: HandlerRef {
                                    name: ident("sendMessage"),
                                    span: Span::default(),
                                },
                                span: Span::default(),
                            }],
                            children: Vec::new(),
                            span: Span::default(),
                        }),
                        UiNode::Element(UiElement {
                            kind: ident("button"),
                            name: ident("Send"),
                            style: Some(StyleBinding {
                                name: ident("PrimaryButton"),
                                span: Span::default(),
                            }),
                            properties: vec![
                                UiProperty {
                                    name: ident("disabled"),
                                    value: UiPropertyValue::Conditional(ConditionalBinding {
                                        condition: data_ref("sending"),
                                        span: Span::default(),
                                    }),
                                    span: Span::default(),
                                },
                                UiProperty {
                                    name: ident("style"),
                                    value: UiPropertyValue::StyleWhen {
                                        condition: data_ref("sending"),
                                        style: StyleBinding {
                                            name: ident("LoadingButton"),
                                            span: Span::default(),
                                        },
                                    },
                                    span: Span::default(),
                                },
                            ],
                            events: vec![EventBinding {
                                event: ident("click"),
                                modifiers: Vec::new(),
                                handler: HandlerRef {
                                    name: ident("sendMessage"),
                                    span: Span::default(),
                                },
                                span: Span::default(),
                            }],
                            children: vec![UiNode::Text(UiText {
                                value: TextValue::Literal("Send".to_string()),
                                span: Span::default(),
                            })],
                            span: Span::default(),
                        }),
                    ],
                    span: Span::default(),
                }),
                span: Span::default(),
            }],
        }
    }

    fn data_ref(name: &str) -> DataRef {
        DataRef {
            name: ident(name),
            span: Span::default(),
        }
    }

    fn ident(name: &str) -> Identifier {
        Identifier::new(name, Span::default())
    }
}
