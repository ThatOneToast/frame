//! Parser for the first Frame language slice.
//!
//! The parser is deliberately small and line-oriented for the MVP. It produces
//! a shared AST plus structured diagnostics that can be reused by the CLI and
//! future editor tooling.

use std::{error::Error, fmt};

#[allow(unused_imports)]
use frame_core::{
    Block, ComponentDecl, ConditionalBinding, DataRef, Declaration, DeclarationKind, EventBinding,
    HandlerRef, Identifier, Include, Node, PropType, PropValue, PropsDecl, SlotDecl, StateDecl,
    StateDefault, StateType, StateValue, Statement, StyleBinding, TextValue, UiComponentArgument,
    UiComponentArgumentValue, UiComponentInvocation, UiElement, UiNode, UiProperty,
    UiPropertyValue, UiText, ViewDecl,
};
use frame_core::{Diagnostic, Document, Severity, Span};

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

pub(crate) struct Parser<'a> {
    lines: Vec<Line<'a>>,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Line<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            lines: helpers::source_lines(source),
            index: 0,
        }
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
            .map(|line| helpers::content_without_comment(line.text))
    }

    fn advance(&mut self) {
        self.index += 1;
    }
}

mod declarations;
mod document;
mod helpers;
mod ui;

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
keyframes FloatIn {
}
"#;

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.declarations.len(), 13);
        assert_eq!(document.declarations[0].kind, DeclarationKind::Grid);
        assert_eq!(document.declarations[7].kind, DeclarationKind::Tokens);
        assert_eq!(document.declarations[8].kind, DeclarationKind::Center);
        assert_eq!(document.declarations[11].kind, DeclarationKind::Dock);
        assert_eq!(document.declarations[12].kind, DeclarationKind::Keyframes);
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
    fn parses_component_loops_with_optional_keys() {
        let source = r#"
component MessageList {
  props {
    messages list
    selectedId text
  }

  view {
    for message in $messages {
      text $message
    }
    for selected in $messages key $selectedId {
      MessageItem(text: $selected)
    }
  }
}
"#;

        let document = parse(source).expect("parse should succeed");
        let nodes = &document.components[0].view.as_ref().expect("view").nodes;

        assert!(matches!(
            nodes[0],
            UiNode::Loop(ref loop_node)
                if loop_node.item.text == "message"
                    && loop_node.collection.name.text == "messages"
                    && loop_node.key.is_none()
        ));
        assert!(matches!(
            nodes[1],
            UiNode::Loop(ref loop_node)
                if loop_node.item.text == "selected"
                    && loop_node.key.as_ref().map(|key| key.name.text.as_str()) == Some("selectedId")
        ));
    }

    #[test]
    fn parses_semantic_ui_primitives_and_shorthand() {
        let source = r#"
component Chat {
  state {
    draft text = ""
    messages list = []
  }

  view {
    screen Chat {
      title "Messages"

      list Messages {
        source $messages

        item {
          text $message.text
        }

        empty {
          text "No messages"
        }
      }

      composer ChatBox {
        label "Message"
        draft bind $draft
        send @sendMessage
      }

      action Save
    }
  }
}
"#;

        let document = parse(source).expect("semantic UI should parse");
        let screen = match &document.components[0].view.as_ref().unwrap().nodes[0] {
            UiNode::Element(element) => element,
            _ => panic!("expected screen"),
        };

        assert_eq!(screen.kind.text, "screen");
        assert_eq!(screen.children.len(), 4);
        assert!(matches!(
            screen.children[0],
            UiNode::Element(ref element)
                if element.kind.text == "title"
                    && matches!(element.properties[0].value, UiPropertyValue::Literal(ref value) if value == "Messages")
        ));
        assert!(matches!(
            screen.children[1],
            UiNode::Element(ref element)
                if element.kind.text == "list"
                    && matches!(element.children[0], UiNode::Element(ref child) if child.kind.text == "item")
        ));
        assert!(matches!(
            screen.children[2],
            UiNode::Element(ref element)
                if element.kind.text == "composer"
                    && matches!(element.properties[0].value, UiPropertyValue::Bind(ref reference) if reference.name.text == "draft")
                    && matches!(element.properties[1].value, UiPropertyValue::Handler(ref handler) if handler.name.text == "sendMessage")
        ));
        assert!(matches!(
            screen.children[3],
            UiNode::Element(ref element)
                if element.kind.text == "action" && element.name.text == "Save"
        ));
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

  focus-within {
    ring accent
  }

  invalid {
    ring danger
  }
}
"#;

        let document = parse(source).expect("parse should succeed");
        let declaration = &document.declarations[0];

        assert_eq!(declaration.name.text, "QuickLinkCard");
        assert_eq!(declaration.body.len(), 4);
        assert!(matches!(
            declaration.body[1],
            Node::Block(ref block) if block.name == "hover"
        ));
        assert!(matches!(
            declaration.body[2],
            Node::Block(ref block) if block.name == "focus-within"
        ));
        assert!(matches!(
            declaration.body[3],
            Node::Block(ref block) if block.name == "invalid"
        ));
    }

    #[test]
    fn parses_typed_supports_blocks_with_nested_declarations() {
        let source = r#"
supports display grid {
  grid AppShell {
    columns sidebar content
  }
}
"#;

        let document = parse(source).expect("supports block should parse");
        let declaration = &document.declarations[0];

        assert_eq!(declaration.kind, DeclarationKind::Supports);
        assert_eq!(declaration.name.text, "display grid");
        assert!(matches!(
            declaration.body[0],
            Node::Block(ref block) if block.name == "grid AppShell"
        ));
    }

    #[test]
    fn parses_style_groups_and_style_order() {
        let source = r#"
style-order reset, base, components, utilities

style-group components {
  button PrimaryButton {
    surface accent
  }
}
"#;

        let document = parse(source).expect("style groups should parse");

        assert_eq!(document.declarations[0].kind, DeclarationKind::StyleOrder);
        assert_eq!(
            document.declarations[0].name.text,
            "reset, base, components, utilities"
        );
        assert_eq!(document.declarations[1].kind, DeclarationKind::StyleGroup);
        assert_eq!(document.declarations[1].name.text, "components");
        assert!(matches!(
            document.declarations[1].body[0],
            Node::Block(ref block) if block.name == "button PrimaryButton"
        ));
    }

    #[test]
    fn parses_keyframes_and_responsive_blocks() {
        let source = r#"
keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px)
  }

  50% {
    opacity 0.8
  }

  to {
    opacity 1
    transform translateY(0)
  }
}

grid AppShell {
  columns sidebar content

  below tablet {
    columns content
  }

  container narrow {
    columns content
  }
}

card Panel {
  animation FloatIn {
    duration fast
    ease smooth
    fill both
  }
}
"#;

        let document = parse(source).expect("parse should succeed");

        assert_eq!(document.declarations[0].kind, DeclarationKind::Keyframes);
        assert!(matches!(
            document.declarations[0].body[0],
            Node::Block(ref block) if block.name == "from"
        ));
        assert!(matches!(
            document.declarations[1].body[1],
            Node::Block(ref block) if block.name == "below tablet"
        ));
        assert!(matches!(
            document.declarations[2].body[0],
            Node::Block(ref block) if block.name == "animation FloatIn"
        ));
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

    #[test]
    fn parses_initial_ui_component_syntax() {
        let source = r#"
component ChatInput {
  state {
    draft text = ""
    sending bool = false
    count number = 1
  }

  view {
    input MessageBox {
      value bind $draft
      placeholder "Message"
      on keydown.ctrl.enter @sendMessage
    }

    button Send:PrimaryButton {
      text "Send"
      disabled when $sending
      on click @sendMessage
      style when $sending = LoadingButton
    }
  }
}
"#;

        let document = parse(source).expect("component should parse");

        assert_eq!(document.components.len(), 1);
        let component = &document.components[0];
        assert_eq!(component.name.text, "ChatInput");
        assert_eq!(component.state.as_ref().unwrap().values.len(), 3);
        let view = component.view.as_ref().unwrap();
        assert_eq!(view.nodes.len(), 2);
        assert!(matches!(
            view.nodes[1],
            UiNode::Element(ref element)
                if element.kind.text == "button"
                    && element.name.text == "Send"
                    && element.style.as_ref().unwrap().name.text == "PrimaryButton"
                    && element.events[0].handler.name.text == "sendMessage"
        ));
    }

    #[test]
    fn rejects_invalid_ui_event_syntax() {
        let source = r#"
component ChatInput {
  view {
    button Send {
      on click sendMessage
    }
  }
}
"#;

        let error = parse(source).expect_err("event syntax should fail");

        assert!(error.diagnostics[0]
            .message
            .contains("events use `on event[.modifier...] @handler`"));
    }

    #[test]
    fn parses_component_invocations_in_view() {
        let source = r#"
component ChatApp {
  state {
    activeChannel text = "general"
    draft text = ""
  }

  view {
    ChannelSidebar()
    ChatPanel(channel: $activeChannel)
    MessageComposer(draft bind $draft)
  }
}
"#;

        let document = parse(source).expect("component invocations should parse");
        let view = document.components[0].view.as_ref().unwrap();

        assert!(matches!(
            view.nodes[0],
            UiNode::Component(ref invocation)
                if invocation.name.text == "ChannelSidebar"
                    && invocation.arguments.is_empty()
        ));
        assert!(matches!(
            view.nodes[1],
            UiNode::Component(ref invocation)
                if invocation.name.text == "ChatPanel"
                    && invocation.arguments[0].name.text == "channel"
                    && matches!(
                        invocation.arguments[0].value,
                        UiComponentArgumentValue::Data(ref reference)
                            if reference.name.text == "activeChannel"
                    )
        ));
        assert!(matches!(
            view.nodes[2],
            UiNode::Component(ref invocation)
                if invocation.name.text == "MessageComposer"
                    && invocation.arguments[0].name.text == "draft"
                    && matches!(
                        invocation.arguments[0].value,
                        UiComponentArgumentValue::Bind(ref reference)
                            if reference.name.text == "draft"
                    )
        ));
    }

    #[test]
    fn parses_props_and_slots_in_component() {
        let source = r#"
component ChannelButton {
  props {
    channel text
    active bool
    unreadCount number
  }

  state {
    hovered bool = false
  }

  view {
    action Channel {
      text $channel
      disabled when $active
      on press @selectChannel
    }
  }

  slot Default {
    text "No content"
  }
}
"#;

        let document = parse(source).expect("component should parse");
        assert_eq!(document.components.len(), 1);
        let component = &document.components[0];

        let props = component.props.as_ref().unwrap();
        assert_eq!(props.values.len(), 3);
        assert_eq!(props.values[0].name.text, "channel");
        assert!(matches!(props.values[0].value_type, PropType::Text));
        assert_eq!(props.values[2].name.text, "unreadCount");
        assert!(matches!(props.values[2].value_type, PropType::Number));

        let view = component.view.as_ref().unwrap();
        let button = match &view.nodes[0] {
            UiNode::Element(element) => element,
            _ => panic!("expected action element"),
        };
        // text $channel is parsed as a child text node, not a property
        assert!(matches!(
            button.children[0],
            UiNode::Text(ref text)
                if matches!(text.value, TextValue::Data(ref reference) if reference.name.text == "channel")
        ));
        assert!(matches!(
            button.properties[0],
            UiProperty {
                name: ref property_name,
                value: UiPropertyValue::Conditional(ref binding),
                ..
            }
            if property_name.text == "disabled" && binding.condition.name.text == "active"
        ));

        assert_eq!(component.slots.len(), 1);
        assert_eq!(component.slots[0].name.text, "Default");
        assert!(matches!(
            component.slots[0].nodes[0],
            UiNode::Text(ref text)
                if matches!(text.value, TextValue::Literal(ref value) if value == "No content")
        ));
    }

    #[test]
    fn parses_show_when_conditional() {
        let source = r#"
component ChatApp {
  state {
    loggedIn bool = false
  }

  view {
    panel Main {
      show when $loggedIn
      text "Welcome"
    }
  }
}
"#;

        let document = parse(source).expect("component should parse");
        let view = document.components[0].view.as_ref().unwrap();
        let element = match &view.nodes[0] {
            UiNode::Element(element) => element,
            _ => panic!("expected element"),
        };
        assert!(matches!(
            element.properties[0],
            UiProperty {
                name: ref property_name,
                value: UiPropertyValue::Conditional(ref binding),
                ..
            }
            if property_name.text == "show" && binding.condition.name.text == "loggedIn"
        ));
    }

    #[test]
    fn rejects_unknown_component_block() {
        let source = r#"
component ChatApp {
  unknown {
    text "Hello"
  }
}
"#;

        let error = parse(source).expect_err("parse should fail");
        assert!(error.diagnostics[0]
            .message
            .contains("Components currently support"));
    }
}
