use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    pub includes: Vec<Include>,
    pub declarations: Vec<Declaration>,
    pub components: Vec<ComponentDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Include {
    pub target: String,
    pub span: Span,
    pub target_span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Declaration {
    pub kind: DeclarationKind,
    pub name: Identifier,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentDecl {
    pub name: Identifier,
    pub state: Option<StateDecl>,
    pub view: Option<ViewDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDecl {
    pub values: Vec<StateValue>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateValue {
    pub name: Identifier,
    pub value_type: StateType,
    pub default: StateDefault,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateType {
    Text,
    Bool,
    Number,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateDefault {
    Text(String),
    Bool(bool),
    Number(String),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewDecl {
    pub nodes: Vec<UiNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiNode {
    Element(UiElement),
    Text(UiText),
    Component(UiComponentInvocation),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiElement {
    pub kind: Identifier,
    pub name: Identifier,
    pub style: Option<StyleBinding>,
    pub properties: Vec<UiProperty>,
    pub events: Vec<EventBinding>,
    pub children: Vec<UiNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiText {
    pub value: TextValue,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiComponentInvocation {
    pub name: Identifier,
    pub arguments: Vec<UiComponentArgument>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiComponentArgument {
    pub name: Identifier,
    pub value: UiComponentArgumentValue,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiComponentArgumentValue {
    Data(DataRef),
    Bind(DataRef),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextValue {
    Literal(String),
    Data(DataRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiProperty {
    pub name: Identifier,
    pub value: UiPropertyValue,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiPropertyValue {
    Literal(String),
    Data(DataRef),
    Bind(DataRef),
    Conditional(ConditionalBinding),
    StyleWhen {
        condition: DataRef,
        style: StyleBinding,
    },
    Unknown(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventBinding {
    pub event: Identifier,
    pub modifiers: Vec<Identifier>,
    pub handler: HandlerRef,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataRef {
    pub name: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlerRef {
    pub name: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleBinding {
    pub name: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionalBinding {
    pub condition: DataRef,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclarationKind {
    Grid,
    Area,
    Card,
    Stack,
    Row,
    Button,
    Text,
    Tokens,
    Center,
    Split,
    Overlay,
    Dock,
    Keyframes,
    Supports,
    StyleGroup,
    StyleOrder,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    Statement(Statement),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub words: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub name: String,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub text: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(text: impl Into<String>, span: Span) -> Self {
        Self {
            text: text.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
