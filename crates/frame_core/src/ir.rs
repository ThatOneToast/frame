use serde::{Deserialize, Serialize};

use crate::{
    ComponentDecl, DataRef, Document, EventBinding, Identifier, PropType, Span, StateDefault,
    StateType, TextValue, UiComponentArgumentValue, UiComponentInvocation, UiElement, UiForLoop,
    UiNode, UiProperty, UiPropertyValue, UiText,
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
    pub props: Vec<FrameIrProp>,
    pub state: Vec<FrameIrState>,
    pub slots: Vec<FrameIrSlot>,
    pub nodes: Vec<FrameIrNode>,
    pub capabilities: Vec<FrameIrCapability>,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrProp {
    pub name: String,
    pub value_type: FrameIrPropType,
    pub readonly: bool,
    pub binding: FrameIrPropBinding,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrPropBinding {
    Input,
    TwoWayAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrPropType {
    Text,
    Bool,
    Number,
    List,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrCapability {
    ConditionalRendering,
    ConditionalStyles,
    EventBinding,
    TwoWayBinding,
    ComponentComposition,
    SlotContent,
    ListRendering,
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
    List,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrStateDefault {
    Text(String),
    Bool(bool),
    Number(String),
    List,
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrNode {
    Element(FrameIrElement),
    Text(FrameIrText),
    Component(FrameIrComponentInvocation),
    List(FrameIrList),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrSlot {
    pub name: String,
    pub fallback: Vec<FrameIrNode>,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrComponentInvocation {
    pub name: String,
    pub arguments: Vec<FrameIrComponentArgument>,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrList {
    pub item: String,
    pub collection: String,
    pub key: Option<String>,
    pub children: Vec<FrameIrNode>,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrComponentArgument {
    pub name: String,
    pub value: FrameIrComponentArgumentValue,
    pub source: FrameIrSourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameIrComponentArgumentValue {
    DataRef(String),
    Bind(String),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIrElement {
    pub kind: String,
    pub semantic_kind: String,
    pub render_kind: String,
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
    Show {
        state: String,
        source: FrameIrSourceSpan,
    },
    Hidden {
        state: String,
        source: FrameIrSourceSpan,
    },
    Property {
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
    let mut capabilities = Vec::new();
    let nodes: Vec<FrameIrNode> = component
        .view
        .as_ref()
        .map(|view| view.nodes.iter().map(lower_node).collect())
        .unwrap_or_default();
    let props: Vec<FrameIrProp> = component
        .props
        .as_ref()
        .map(|props| props.values.iter().map(lower_prop).collect())
        .unwrap_or_default();
    let state: Vec<FrameIrState> = component
        .state
        .as_ref()
        .map(|state| state.values.iter().map(lower_state).collect())
        .unwrap_or_default();
    let slots: Vec<FrameIrSlot> = component
        .slots
        .iter()
        .map(|slot| FrameIrSlot {
            name: slot.name.text.clone(),
            fallback: slot.nodes.iter().map(lower_node).collect(),
            source: span(slot.span),
        })
        .collect();

    // Collect capabilities from nodes
    collect_capabilities(&nodes, &mut capabilities);
    for slot in &slots {
        collect_capabilities(&slot.fallback, &mut capabilities);
    }
    if !slots.is_empty() {
        capabilities.push(FrameIrCapability::SlotContent);
    }
    if !props.is_empty() {
        capabilities.push(FrameIrCapability::ComponentComposition);
    }

    FrameIrComponent {
        name: component.name.text.clone(),
        props,
        state,
        slots,
        nodes,
        capabilities,
        source: span(component.span),
    }
}

fn lower_prop(prop: &crate::PropValue) -> FrameIrProp {
    FrameIrProp {
        name: prop.name.text.clone(),
        value_type: match &prop.value_type {
            PropType::Text => FrameIrPropType::Text,
            PropType::Bool => FrameIrPropType::Bool,
            PropType::Number => FrameIrPropType::Number,
            PropType::List => FrameIrPropType::List,
            PropType::Unknown(value) => FrameIrPropType::Unknown(value.clone()),
        },
        readonly: true,
        binding: FrameIrPropBinding::Input,
        source: span(prop.span),
    }
}

fn collect_capabilities(nodes: &[FrameIrNode], capabilities: &mut Vec<FrameIrCapability>) {
    for node in nodes {
        match node {
            FrameIrNode::Element(element) => {
                if !element.events.is_empty()
                    && !capabilities.contains(&FrameIrCapability::EventBinding)
                {
                    capabilities.push(FrameIrCapability::EventBinding);
                }
                if !element.bindings.is_empty()
                    && !capabilities.contains(&FrameIrCapability::TwoWayBinding)
                {
                    capabilities.push(FrameIrCapability::TwoWayBinding);
                }
                if !element.conditions.is_empty() {
                    for condition in &element.conditions {
                        match condition {
                            FrameIrCondition::Show { .. }
                            | FrameIrCondition::Hidden { .. }
                            | FrameIrCondition::Property { .. } => {
                                if !capabilities.contains(&FrameIrCapability::ConditionalRendering)
                                {
                                    capabilities.push(FrameIrCapability::ConditionalRendering);
                                }
                            }
                            FrameIrCondition::Style { .. } => {
                                if !capabilities.contains(&FrameIrCapability::ConditionalStyles) {
                                    capabilities.push(FrameIrCapability::ConditionalStyles);
                                }
                            }
                        }
                    }
                }
                collect_capabilities(&element.children, capabilities);
            }
            FrameIrNode::Component(_) => {
                if !capabilities.contains(&FrameIrCapability::ComponentComposition) {
                    capabilities.push(FrameIrCapability::ComponentComposition);
                }
            }
            FrameIrNode::List(list) => {
                if !capabilities.contains(&FrameIrCapability::ListRendering) {
                    capabilities.push(FrameIrCapability::ListRendering);
                }
                collect_capabilities(&list.children, capabilities);
            }
            FrameIrNode::Text(_) => {}
        }
    }
}

fn lower_state(state: &crate::StateValue) -> FrameIrState {
    FrameIrState {
        name: state.name.text.clone(),
        value_type: match &state.value_type {
            StateType::Text => FrameIrStateType::Text,
            StateType::Bool => FrameIrStateType::Bool,
            StateType::Number => FrameIrStateType::Number,
            StateType::List => FrameIrStateType::List,
            StateType::Unknown(value) => FrameIrStateType::Unknown(value.clone()),
        },
        default: match &state.default {
            StateDefault::Text(value) => FrameIrStateDefault::Text(value.clone()),
            StateDefault::Bool(value) => FrameIrStateDefault::Bool(*value),
            StateDefault::Number(value) => FrameIrStateDefault::Number(value.clone()),
            StateDefault::List => FrameIrStateDefault::List,
            StateDefault::Invalid(value) => FrameIrStateDefault::Invalid(value.clone()),
        },
        source: span(state.span),
    }
}

fn lower_node(node: &UiNode) -> FrameIrNode {
    match node {
        UiNode::Element(element) => FrameIrNode::Element(lower_element(element)),
        UiNode::Text(text) => FrameIrNode::Text(lower_text(text)),
        UiNode::Component(invocation) => {
            FrameIrNode::Component(lower_component_invocation(invocation))
        }
        UiNode::Loop(loop_node) => FrameIrNode::List(lower_list(loop_node)),
    }
}

fn lower_list(loop_node: &UiForLoop) -> FrameIrList {
    FrameIrList {
        item: loop_node.item.text.clone(),
        collection: reference_name(&loop_node.collection),
        key: loop_node.key.as_ref().map(reference_name),
        children: loop_node.children.iter().map(lower_node).collect(),
        source: span(loop_node.span),
    }
}

fn lower_component_invocation(invocation: &UiComponentInvocation) -> FrameIrComponentInvocation {
    FrameIrComponentInvocation {
        name: invocation.name.text.clone(),
        arguments: invocation
            .arguments
            .iter()
            .map(|argument| FrameIrComponentArgument {
                name: argument.name.text.clone(),
                value: match &argument.value {
                    UiComponentArgumentValue::Data(reference) => {
                        FrameIrComponentArgumentValue::DataRef(reference_name(reference))
                    }
                    UiComponentArgumentValue::Bind(reference) => {
                        FrameIrComponentArgumentValue::Bind(reference_name(reference))
                    }
                    UiComponentArgumentValue::Literal(value) => {
                        FrameIrComponentArgumentValue::Literal(value.clone())
                    }
                },
                source: span(argument.span),
            })
            .collect(),
        source: span(invocation.span),
    }
}

fn lower_element(element: &UiElement) -> FrameIrElement {
    let mut attributes = Vec::new();
    let mut bindings = Vec::new();
    let mut conditions = Vec::new();
    let mut events: Vec<FrameIrEvent> = element.events.iter().map(lower_event).collect();

    for property in &element.properties {
        lower_property(
            property,
            &mut attributes,
            &mut bindings,
            &mut conditions,
            &mut events,
        );
    }

    FrameIrElement {
        kind: element.kind.text.clone(),
        semantic_kind: element.kind.text.clone(),
        render_kind: default_render_kind(&element.kind.text).to_string(),
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
        events,
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
    events: &mut Vec<FrameIrEvent>,
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
        UiPropertyValue::Bind(reference) => {
            let property_name = if property.name.text == "bind" {
                "value"
            } else {
                property.name.text.as_str()
            };
            bindings.push(FrameIrBinding {
                property: property_name.to_string(),
                state: reference_name(reference),
                source: span(property.span),
            })
        }
        UiPropertyValue::Handler(handler) => events.push(FrameIrEvent {
            event: property.name.text.clone(),
            modifiers: Vec::new(),
            handler: handler.name.text.clone(),
            source: span(property.span),
        }),
        UiPropertyValue::Conditional(binding) => {
            let state = reference_name(&binding.condition);
            let source = span(binding.span);
            match property.name.text.as_str() {
                "show" => conditions.push(FrameIrCondition::Show { state, source }),
                "hidden" => conditions.push(FrameIrCondition::Hidden { state, source }),
                _ => conditions.push(FrameIrCondition::Property {
                    property: property.name.text.clone(),
                    state,
                    source,
                }),
            }
        }
        UiPropertyValue::StyleWhen { condition, style } => {
            conditions.push(FrameIrCondition::Style {
                state: reference_name(condition),
                style: style.name.text.clone(),
                source: span(style.span),
            })
        }
        UiPropertyValue::Unknown(_) => {}
    }
}

fn default_render_kind(kind: &str) -> &str {
    match kind {
        "screen" | "panel" | "section" | "stack" | "row" | "grid" | "split" | "dock"
        | "overlay" | "scroll" | "toolbar" | "tabs" | "card" | "popover" | "item" | "empty"
        | "data" => "div",
        "action" => "button",
        "link" => "a",
        "menu" => "nav",
        "input" => "input",
        "editor" | "composer" => "textarea",
        "toggle" => "input",
        "choice" | "select" => "select",
        "title" => "h2",
        "text" | "label" | "badge" | "icon" => "span",
        "avatar" | "image" => "img",
        "list" | "feed" => "ul",
        "dialog" => "dialog",
        other => other,
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
            FrameIrCondition::Property {
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

        let invocation = match &component.nodes[2] {
            FrameIrNode::Component(invocation) => invocation,
            _ => panic!("expected component invocation"),
        };
        assert_eq!(invocation.name, "MessageComposer");
        assert_eq!(invocation.arguments[0].name, "draft");
        assert_eq!(
            invocation.arguments[0].value,
            FrameIrComponentArgumentValue::Bind("draft".to_string())
        );
    }

    #[test]
    fn lowers_slots_and_lists_to_renderer_neutral_ir() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: ident("MessageList"),
                props: None,
                state: Some(StateDecl {
                    values: vec![
                        StateValue {
                            name: ident("messages"),
                            value_type: StateType::List,
                            default: StateDefault::List,
                            span: Span::default(),
                        },
                        StateValue {
                            name: ident("messageId"),
                            value_type: StateType::Text,
                            default: StateDefault::Text(String::new()),
                            span: Span::default(),
                        },
                    ],
                    span: Span::default(),
                }),
                view: Some(crate::ViewDecl {
                    nodes: vec![UiNode::Loop(crate::UiForLoop {
                        item: ident("message"),
                        collection: data_ref("messages"),
                        key: Some(data_ref("messageId")),
                        children: vec![UiNode::Text(UiText {
                            value: TextValue::Data(data_ref("message")),
                            span: Span::default(),
                        })],
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }),
                slots: vec![crate::SlotDecl {
                    name: ident("Header"),
                    nodes: vec![UiNode::Text(UiText {
                        value: TextValue::Literal("Fallback".to_string()),
                        span: Span::default(),
                    })],
                    span: Span::default(),
                }],
                span: Span::default(),
            }],
        };

        let ir = lower_document_to_ir(&document);
        let component = &ir.components[0];

        assert!(component
            .capabilities
            .contains(&FrameIrCapability::ListRendering));
        assert!(component
            .capabilities
            .contains(&FrameIrCapability::SlotContent));
        assert_eq!(component.slots[0].name, "Header");
        assert!(matches!(
            component.nodes[0],
            FrameIrNode::List(FrameIrList {
                ref item,
                ref collection,
                ref key,
                ..
            }) if item == "message" && collection == "messages" && key.as_deref() == Some("messageId")
        ));
    }

    fn document_fixture() -> Document {
        Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: ident("ChatInput"),
                props: None,
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
                        UiNode::Component(crate::UiComponentInvocation {
                            name: ident("MessageComposer"),
                            arguments: vec![crate::UiComponentArgument {
                                name: ident("draft"),
                                value: crate::UiComponentArgumentValue::Bind(data_ref("draft")),
                                span: Span::default(),
                            }],
                            span: Span::default(),
                        }),
                    ],
                    span: Span::default(),
                }),
                slots: Vec::new(),
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
