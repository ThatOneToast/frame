use std::collections::BTreeSet;

use frame_core::{
    ir::{
        lower_document_to_ir, FrameIrComponent, FrameIrNode, FrameIrPropType, FrameIrStateDefault,
        FrameIrStateType,
    },
    Document,
};

pub fn generate_contracts(document: &Document) -> String {
    let ir = lower_document_to_ir(document);
    generate_contracts_from_components(&ir.components)
}

fn generate_contracts_from_components(components: &[FrameIrComponent]) -> String {
    let mut output = String::from(
        "export type FrameEventContext<TState, TProps> = {\n  state: TState;\n  props: TProps;\n  event: Event;\n};\n\n",
    );

    for component in components {
        if !component.props.is_empty() {
            output.push_str(&format!("export type {}Props = {{\n", component.name));
            for prop in &component.props {
                output.push_str(&format!(
                    "  {}: {};\n",
                    property_name(&prop.name),
                    ts_prop_type(&prop.value_type)
                ));
            }
            output.push_str("};\n\n");
        }

        output.push_str(&format!("export type {}State = {{\n", component.name));
        for state in &component.state {
            output.push_str(&format!(
                "  {}: {};\n",
                property_name(&state.name),
                ts_state_type(&state.value_type)
            ));
        }
        output.push_str("};\n\n");

        let handlers = component_handlers(component);
        output.push_str(&format!("export type {}Handlers = {{\n", component.name));
        for handler in handlers {
            let context_type = if component.props.is_empty() {
                format!("FrameEventContext<{}State, {{}}>", component.name)
            } else {
                format!(
                    "FrameEventContext<{}State, {}Props>",
                    component.name, component.name
                )
            };
            output.push_str(&format!(
                "  {}(ctx: {}): void | Promise<void>;\n",
                property_name(&handler),
                context_type
            ));
        }
        output.push_str("};\n\n");
    }

    output
}

fn component_handlers(component: &FrameIrComponent) -> Vec<String> {
    let mut handlers = BTreeSet::new();
    for node in &component.nodes {
        collect_handlers(node, &mut handlers);
    }
    for slot in &component.slots {
        for node in &slot.fallback {
            collect_handlers(node, &mut handlers);
        }
    }
    handlers.into_iter().collect()
}

fn collect_handlers(node: &FrameIrNode, handlers: &mut BTreeSet<String>) {
    match node {
        FrameIrNode::Element(element) => {
            for event in &element.events {
                handlers.insert(event.handler.clone());
            }
            for child in &element.children {
                collect_handlers(child, handlers);
            }
        }
        FrameIrNode::List(list) => {
            for child in &list.children {
                collect_handlers(child, handlers);
            }
        }
        FrameIrNode::Text(_) | FrameIrNode::Component(_) => {}
    }
}

fn ts_state_type(value_type: &FrameIrStateType) -> &'static str {
    match value_type {
        FrameIrStateType::Text => "string",
        FrameIrStateType::Bool => "boolean",
        FrameIrStateType::Number => "number",
        FrameIrStateType::List => "unknown[]",
        FrameIrStateType::Unknown(_) => "unknown",
    }
}

fn ts_prop_type(value_type: &FrameIrPropType) -> &'static str {
    match value_type {
        FrameIrPropType::Text => "string",
        FrameIrPropType::Bool => "boolean",
        FrameIrPropType::Number => "number",
        FrameIrPropType::List => "unknown[]",
        FrameIrPropType::Unknown(_) => "unknown",
    }
}

fn property_name(name: &str) -> String {
    if is_typescript_identifier(name) {
        name.to_string()
    } else {
        format!("{name:?}")
    }
}

fn is_typescript_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    (first == '_' || first == '$' || first.is_ascii_alphabetic())
        && chars.all(|char| char == '_' || char == '$' || char.is_ascii_alphanumeric())
}

#[allow(dead_code)]
fn default_type(default: &FrameIrStateDefault) -> &'static str {
    match default {
        FrameIrStateDefault::Text(_) => "string",
        FrameIrStateDefault::Bool(_) => "boolean",
        FrameIrStateDefault::Number(_) => "number",
        FrameIrStateDefault::List => "unknown[]",
        FrameIrStateDefault::Invalid(_) => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use frame_core::{
        ComponentDecl, DataRef, Document, EventBinding, HandlerRef, Identifier, Span, StateDecl,
        StateDefault, StateType, StateValue, UiElement, UiNode, ViewDecl,
    };

    use super::*;

    #[test]
    fn generates_state_and_handler_contracts() {
        let document = document_with_components(vec![component(
            "ChatInput",
            vec![
                state("draft", StateType::Text, StateDefault::Text(String::new())),
                state("sending", StateType::Bool, StateDefault::Bool(false)),
                state(
                    "attempts",
                    StateType::Number,
                    StateDefault::Number("0".to_string()),
                ),
            ],
            vec![element_with_handlers(
                "Send",
                &["sendMessage", "sendMessage"],
            )],
        )]);

        let ts = generate_contracts(&document);

        assert!(ts.contains("export type FrameEventContext<TState, TProps>"));
        assert!(ts.contains("export type ChatInputState = {"));
        assert!(ts.contains("  draft: string;"));
        assert!(ts.contains("  sending: boolean;"));
        assert!(ts.contains("  attempts: number;"));
        assert_eq!(ts.matches("sendMessage(ctx").count(), 1);
    }

    #[test]
    fn generates_empty_handlers_for_components_without_events() {
        let document = document_with_components(vec![component(
            "StaticPanel",
            vec![state(
                "title",
                StateType::Text,
                StateDefault::Text("Hello".to_string()),
            )],
            Vec::new(),
        )]);

        let ts = generate_contracts(&document);

        assert!(ts.contains("export type StaticPanelHandlers = {\n};"));
    }

    #[test]
    fn generates_multiple_components_deterministically() {
        let document = document_with_components(vec![
            component(
                "Composer",
                vec![state(
                    "draft",
                    StateType::Text,
                    StateDefault::Text(String::new()),
                )],
                vec![element_with_handlers("Send", &["sendMessage"])],
            ),
            component(
                "ChannelList",
                Vec::new(),
                vec![element_with_handlers("General", &["selectChannel"])],
            ),
        ]);

        let ts = generate_contracts(&document);

        assert!(
            ts.find("export type ComposerState")
                .expect("composer state")
                < ts.find("export type ChannelListState")
                    .expect("channel state")
        );
        assert!(ts.contains("selectChannel(ctx"));
    }

    fn document_with_components(components: Vec<ComponentDecl>) -> Document {
        Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components,
        }
    }

    fn component(name: &str, state_values: Vec<StateValue>, nodes: Vec<UiNode>) -> ComponentDecl {
        ComponentDecl {
            name: ident(name),
            props: None,
            state: Some(StateDecl {
                values: state_values,
                span: Span::default(),
            }),
            view: Some(ViewDecl {
                nodes,
                span: Span::default(),
            }),
            slots: Vec::new(),
            span: Span::default(),
        }
    }

    fn state(name: &str, value_type: StateType, default: StateDefault) -> StateValue {
        StateValue {
            name: ident(name),
            value_type,
            default,
            span: Span::default(),
        }
    }

    fn element_with_handlers(name: &str, handlers: &[&str]) -> UiNode {
        UiNode::Element(UiElement {
            kind: ident("button"),
            name: ident(name),
            style: None,
            properties: Vec::new(),
            events: handlers
                .iter()
                .map(|handler| EventBinding {
                    event: ident("click"),
                    modifiers: Vec::new(),
                    handler: HandlerRef {
                        name: ident(handler),
                        span: Span::default(),
                    },
                    span: Span::default(),
                })
                .collect(),
            children: Vec::new(),
            span: Span::default(),
        })
    }

    fn ident(name: &str) -> Identifier {
        Identifier::new(name, Span::default())
    }

    #[allow(dead_code)]
    fn data_ref(name: &str) -> DataRef {
        DataRef {
            name: ident(name),
            span: Span::default(),
        }
    }
}
