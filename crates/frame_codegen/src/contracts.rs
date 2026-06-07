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

    // Event-specific context aliases for semantic primitives
    output.push_str(
        "export type FramePressEvent<TState, TProps> = FrameEventContext<TState, TProps>;\n",
    );
    output.push_str(
        "export type FrameInputEvent<TState, TProps> = FrameEventContext<TState, TProps>;\n",
    );
    output.push_str(
        "export type FrameToggleEvent<TState, TProps> = FrameEventContext<TState, TProps>;\n",
    );
    output.push_str(
        "export type FrameKeyboardEvent<TState, TProps> = FrameEventContext<TState, TProps>;\n",
    );
    output.push_str(
        "export type FrameFormEvent<TState, TProps> = FrameEventContext<TState, TProps>;\n\n",
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
        for handler in &handlers {
            let event_type = handler_event_type(component, &handler.name);
            let context_type = build_context_type(component, &event_type);
            output.push_str(&format!(
                "  {}(ctx: {}): void | Promise<void>;\n",
                property_name(&handler.name),
                context_type
            ));
        }
        output.push_str("};\n\n");
    }

    output
}

fn build_context_type(component: &FrameIrComponent, event_type: &str) -> String {
    let state_type = format!("{}State", component.name);
    let props_type = if component.props.is_empty() {
        "{}".to_string()
    } else {
        format!("{}Props", component.name)
    };
    format!("{}<{state_type}, {props_type}>", event_type)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandlerInfo {
    name: String,
    event_type: String,
}

fn component_handlers(component: &FrameIrComponent) -> Vec<HandlerInfo> {
    let mut handlers: BTreeSet<String> = BTreeSet::new();
    let mut event_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for node in &component.nodes {
        collect_handlers(node, &mut handlers, &mut event_map);
    }
    for slot in &component.slots {
        for node in &slot.fallback {
            collect_handlers(node, &mut handlers, &mut event_map);
        }
    }

    handlers
        .into_iter()
        .map(|name| HandlerInfo {
            event_type: event_map
                .get(&name)
                .cloned()
                .unwrap_or_else(|| "FrameEventContext".to_string()),
            name,
        })
        .collect()
}

fn collect_handlers(
    node: &FrameIrNode,
    handlers: &mut BTreeSet<String>,
    event_map: &mut std::collections::HashMap<String, String>,
) {
    match node {
        FrameIrNode::Element(element) => {
            for event in &element.events {
                handlers.insert(event.handler.clone());
                let typ = event_type_alias(&event.event);
                // Prefer the most specific event type if a handler is reused
                event_map
                    .entry(event.handler.clone())
                    .and_modify(|existing| {
                        if typ == "FrameEventContext" {
                            return;
                        }
                        if *existing == "FrameEventContext" || existing == &typ {
                            *existing = typ.clone();
                        }
                    })
                    .or_insert(typ);
            }
            for child in &element.children {
                collect_handlers(child, handlers, event_map);
            }
        }
        FrameIrNode::List(list) => {
            for child in &list.children {
                collect_handlers(child, handlers, event_map);
            }
        }
        FrameIrNode::Text(_) | FrameIrNode::Component(_) => {}
    }
}

fn event_type_alias(event: &str) -> String {
    match event {
        "press" | "click" => "FramePressEvent".to_string(),
        "input" => "FrameInputEvent".to_string(),
        "change" => "FrameInputEvent".to_string(),
        "keydown" | "keyup" | "keypress" => "FrameKeyboardEvent".to_string(),
        "submit" | "reset" => "FrameFormEvent".to_string(),
        _ => "FrameEventContext".to_string(),
    }
}

fn handler_event_type(component: &FrameIrComponent, handler_name: &str) -> String {
    for node in &component.nodes {
        if let Some(typ) = find_handler_event_type(node, handler_name) {
            return typ;
        }
    }
    for slot in &component.slots {
        for node in &slot.fallback {
            if let Some(typ) = find_handler_event_type(node, handler_name) {
                return typ;
            }
        }
    }
    "FrameEventContext".to_string()
}

fn find_handler_event_type(node: &FrameIrNode, handler_name: &str) -> Option<String> {
    match node {
        FrameIrNode::Element(element) => {
            for event in &element.events {
                if event.handler == handler_name {
                    return Some(event_type_alias(&event.event));
                }
            }
            for child in &element.children {
                if let Some(typ) = find_handler_event_type(child, handler_name) {
                    return Some(typ);
                }
            }
            None
        }
        FrameIrNode::List(list) => {
            for child in &list.children {
                if let Some(typ) = find_handler_event_type(child, handler_name) {
                    return Some(typ);
                }
            }
            None
        }
        FrameIrNode::Text(_) | FrameIrNode::Component(_) => None,
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

pub fn generate_skeletons(document: &Document) -> String {
    let ir = lower_document_to_ir(document);
    generate_skeletons_from_components(&ir.components)
}

fn generate_skeletons_from_components(components: &[FrameIrComponent]) -> String {
    let mut output = String::from(
        "// Frame generated handler skeletons.\n// This file is generated non-destructively.\n// Copy functions into your own handlers.ts and implement them.\n\n",
    );

    #[derive(Debug, Clone)]
    struct ComponentHandler {
        component_name: String,
        has_props: bool,
        handler: HandlerInfo,
    }

    let mut all_handlers: Vec<ComponentHandler> = Vec::new();
    for component in components {
        for handler in component_handlers(component) {
            all_handlers.push(ComponentHandler {
                component_name: component.name.clone(),
                has_props: !component.props.is_empty(),
                handler,
            });
        }
    }

    if all_handlers.is_empty() {
        output.push_str("// No handlers found in this project.\n");
        return output;
    }

    // Group by component for stable output
    let mut by_component: std::collections::BTreeMap<String, (bool, Vec<HandlerInfo>)> =
        std::collections::BTreeMap::new();
    for entry in all_handlers {
        let (name, has_props, handler) = (entry.component_name, entry.has_props, entry.handler);
        by_component
            .entry(name)
            .or_insert((has_props, Vec::new()))
            .1
            .push(handler);
    }

    for (component_name, (has_props, handlers)) in by_component {
        output.push_str(&format!("// Component: {component_name}\n"));
        for handler in handlers {
            let context_type =
                build_context_type_from_name(&component_name, &handler.event_type, has_props);
            output.push_str(&format!(
                "export function {name}(ctx: {context_type}): void {{\n  // TODO: implement {name}\n}}\n\n",
                name = property_name(&handler.name),
            ));
        }
    }

    output
}

fn build_context_type_from_name(component_name: &str, event_type: &str, has_props: bool) -> String {
    let props_type = if has_props {
        format!("{component_name}Props")
    } else {
        "{}".to_string()
    };
    format!("{event_type}<{component_name}State, {props_type}>")
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

    #[test]
    fn generates_handler_skeletons_with_todo_comments() {
        let document = document_with_components(vec![component(
            "ChatInput",
            vec![state(
                "draft",
                StateType::Text,
                StateDefault::Text(String::new()),
            )],
            vec![element_with_handlers("Send", &["sendMessage"])],
        )]);

        let skeletons = generate_skeletons(&document);
        assert!(skeletons.contains("export function sendMessage"));
        assert!(skeletons.contains("TODO: implement sendMessage"));
        assert!(skeletons.contains("ChatInputState"));
    }

    #[test]
    fn generates_empty_skeleton_when_no_handlers() {
        let document = document_with_components(vec![component(
            "StaticPanel",
            vec![state(
                "title",
                StateType::Text,
                StateDefault::Text("Hello".to_string()),
            )],
            Vec::new(),
        )]);

        let skeletons = generate_skeletons(&document);
        assert!(skeletons.contains("No handlers found"));
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
