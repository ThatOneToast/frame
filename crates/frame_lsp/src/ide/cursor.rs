use frame_core::{
    language, symbols::index_document, symbols::SymbolIndex, DeclarationKind, Document, UiNode,
};
use frame_parser::parse;

/// A semantic cursor captures IDE-relevant facts about a source position.
///
/// Built from the parsed AST plus source text, it is the single input to
/// completions, hovers, references, and diagnostics.
#[derive(Debug, Clone)]
pub struct SemanticCursor<'a> {
    pub source: &'a str,
    pub offset: usize,
    /// The word the cursor is on or after.
    /// Reserved for future IDE features (e.g., symbol rename, quick fix).
    #[allow(dead_code)]
    pub word: Option<String>,
    /// The token text at the cursor position.
    pub token_text: Option<String>,
    /// Enclosing component name, if any.
    pub enclosing_component: Option<String>,
    /// Enclosing declaration name and kind, if any.
    pub enclosing_declaration: Option<(String, DeclarationKind)>,
    /// Enclosing view node kind, if inside a view block.
    pub enclosing_view_node: Option<String>,
    /// Innermost block header (e.g. "gradient", "hover", "from", "section").
    pub innermost_block: Option<String>,
    /// The cursor slot: what kind of completion/hover is valid here.
    pub slot: CursorSlot,
    /// The cursor scope: local symbols available at this position.
    pub scope: CursorScope,
    /// Whether the syntax around the cursor is incomplete.
    pub is_incomplete: bool,
    /// The parsed document (may be partial).
    pub document: Option<Document>,
    /// Symbol index for the document.
    pub symbols: SymbolIndex,
    /// Line prefix up to the cursor.
    pub line_prefix: String,
}

/// What kind of IDE slot the cursor is in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorSlot {
    /// Top-level, before any declaration.
    RootDeclaration,
    /// Inside a declaration name position.
    DeclarationName,
    /// Inside a declaration body, not in a property value.
    DeclarationBody,
    /// Inside a view body.
    ViewBody,
    /// Inside a view primitive element.
    ViewPrimitive,
    /// Inside a view node name.
    ViewNodeName,
    /// Inside a view property name.
    ViewPropertyName,
    /// Inside a view property value for a known property.
    ViewPropertyValue { property: String },
    /// Inside a style property name.
    StylePropertyName,
    /// Inside a style property value for a known property.
    StylePropertyValue { property: String },
    /// Inside an event name slot (after `on`).
    EventName,
    /// Inside an event modifier slot (after `.`).
    /// Reserved for future modifier completions.
    #[allow(dead_code)]
    EventModifier { event: String },
    /// Inside a handler reference (after `@`).
    HandlerReference,
    /// Inside a data reference (after `$`).
    DataReference,
    /// Inside a state declaration block.
    StateDeclaration,
    /// Inside a state default value.
    /// Reserved for future default-value diagnostics.
    #[allow(dead_code)]
    StateDefaultValue,
    /// Inside a binding target.
    /// Reserved for future bind-target validation.
    #[allow(dead_code)]
    BindingTarget,
    /// Inside an include target.
    IncludeTarget,
    /// Unknown / fallback.
    Unknown,
}

/// Scope information for the cursor position.
#[derive(Debug, Clone, Default)]
pub struct CursorScope {
    /// State variables in the enclosing component.
    pub local_state: Vec<SymbolCandidate>,
    /// Props in the enclosing component.
    pub local_props: Vec<SymbolCandidate>,
    /// Loop variables in scope (nearest first).
    pub loop_vars: Vec<SymbolCandidate>,
    /// Handler names in the enclosing component.
    pub handlers: Vec<SymbolCandidate>,
    /// Declarations and components in the current file.
    pub local_declarations: Vec<SymbolCandidate>,
}

/// A symbol candidate for completions, hovers, or references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolCandidate {
    pub name: String,
    pub kind: SymbolKind,
    pub detail: String,
}

/// Internal classification of a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    State,
    Prop,
    LoopVar,
    Handler,
    Declaration,
    Component,
    /// Reserved for future theme-aware completions.
    #[allow(dead_code)]
    Color,
    /// Reserved for future theme-aware completions.
    #[allow(dead_code)]
    Gradient,
    /// Reserved for future animation-aware completions.
    #[allow(dead_code)]
    Keyframes,
    /// Reserved for future import-aware navigation.
    #[allow(dead_code)]
    Import,
    /// Reserved for future theme-aware completions.
    #[allow(dead_code)]
    Theme,
}

impl<'a> SemanticCursor<'a> {
    /// Build a semantic cursor at the given offset.
    pub fn at(source: &'a str, offset: usize) -> Self {
        let (document, symbols) = match parse(source) {
            Ok(doc) => {
                let symbols = index_document(source, &doc);
                (Some(doc), symbols)
            }
            Err(_) => (None, SymbolIndex::default()),
        };
        Self::at_with_document(source, offset, document, symbols)
    }

    /// Build a semantic cursor with a pre-parsed document and symbol index.
    pub fn at_with_document(
        source: &'a str,
        offset: usize,
        document: Option<Document>,
        symbols: SymbolIndex,
    ) -> Self {
        let safe_offset = offset.min(source.len());
        let word = word_at(source, safe_offset).map(String::from);
        let token_text = token_at(source, safe_offset).map(String::from);
        let line_prefix = line_prefix_at(source, safe_offset);

        let mut cursor = SemanticCursor {
            source,
            offset: safe_offset,
            word,
            token_text,
            enclosing_component: None,
            enclosing_declaration: None,
            enclosing_view_node: None,
            innermost_block: None,
            slot: CursorSlot::Unknown,
            scope: CursorScope::default(),
            is_incomplete: false,
            document,
            symbols,
            line_prefix,
        };

        cursor.analyze();
        cursor
    }

    fn analyze(&mut self) {
        let doc = self.document.clone();
        if let Some(ref doc) = doc {
            self.find_enclosing_component(doc);
            self.find_enclosing_declaration(doc);
            self.find_enclosing_view_node(doc);
            self.build_scope(doc);
            self.determine_slot(doc);
        } else {
            self.is_incomplete = true;
            self.fallback_slot();
        }

        // Populate innermost block from stack
        let stack = block_stack_before(self.source, self.offset);
        self.innermost_block = stack.last().cloned();

        // If we fell back to ViewBody but have no enclosing_view_node,
        // try to infer it from the block stack (e.g. "text Label" → "text").
        // Both primitives and declarations can appear as view nodes.
        if self.slot == CursorSlot::ViewBody && self.enclosing_view_node.is_none() {
            for block in stack.iter().rev() {
                let first = block.split_whitespace().next().unwrap_or("");
                if language::declaration_keywords().contains(&first) {
                    self.enclosing_view_node = Some(first.to_string());
                    break;
                }
            }
        }

        // Mark as incomplete if braces are unbalanced in the source
        let open_count = self.source.chars().filter(|&c| c == '{').count();
        let close_count = self.source.chars().filter(|&c| c == '}').count();
        if open_count != close_count {
            self.is_incomplete = true;
        }
    }

    fn find_enclosing_component(&mut self, doc: &Document) {
        for component in &doc.components {
            if component.span.start <= self.offset && component.span.end >= self.offset {
                self.enclosing_component = Some(component.name.text.clone());
                break;
            }
        }
    }

    fn find_enclosing_declaration(&mut self, doc: &Document) {
        for declaration in &doc.declarations {
            if declaration.span.start <= self.offset && declaration.span.end >= self.offset {
                self.enclosing_declaration =
                    Some((declaration.name.text.clone(), declaration.kind.clone()));
                break;
            }
        }
    }

    fn find_enclosing_view_node(&mut self, doc: &Document) {
        if let Some(ref component_name) = self.enclosing_component {
            if let Some(component) = doc
                .components
                .iter()
                .find(|c| c.name.text == *component_name)
            {
                if let Some(ref view) = component.view {
                    if let Some(kind) = find_element_kind_at_offset(&view.nodes, self.offset) {
                        self.enclosing_view_node = Some(kind);
                        return;
                    }
                }
                for slot in &component.slots {
                    if let Some(kind) = find_element_kind_at_offset(&slot.nodes, self.offset) {
                        self.enclosing_view_node = Some(kind);
                        return;
                    }
                }
            }
        }
    }

    fn build_scope(&mut self, doc: &Document) {
        // Local declarations
        self.scope.local_declarations = doc
            .declarations
            .iter()
            .map(|d| SymbolCandidate {
                name: d.name.text.clone(),
                kind: SymbolKind::Declaration,
                detail: format!("{:?}", d.kind),
            })
            .collect();

        // Components
        self.scope
            .local_declarations
            .extend(doc.components.iter().map(|c| SymbolCandidate {
                name: c.name.text.clone(),
                kind: SymbolKind::Component,
                detail: "component".to_string(),
            }));

        // If inside a component, extract state, props, handlers, loop vars
        if let Some(ref component_name) = self.enclosing_component {
            if let Some(component) = doc
                .components
                .iter()
                .find(|c| c.name.text == *component_name)
            {
                // State
                if let Some(ref state) = component.state {
                    self.scope.local_state = state
                        .values
                        .iter()
                        .map(|v| SymbolCandidate {
                            name: v.name.text.clone(),
                            kind: SymbolKind::State,
                            detail: format!("state {}", state_type_label(&v.value_type)),
                        })
                        .collect();
                }

                // Props
                if let Some(ref props) = component.props {
                    self.scope.local_props = props
                        .values
                        .iter()
                        .map(|v| SymbolCandidate {
                            name: v.name.text.clone(),
                            kind: SymbolKind::Prop,
                            detail: format!("prop {}", prop_type_label(&v.value_type)),
                        })
                        .collect();
                }

                // Handlers and loop vars from view + slots
                if let Some(ref view) = component.view {
                    self.collect_handlers_and_loops(&view.nodes);
                }
                for slot in &component.slots {
                    self.collect_handlers_and_loops(&slot.nodes);
                }
            }
        }
    }

    fn collect_handlers_and_loops(&mut self, nodes: &[UiNode]) {
        for node in nodes {
            match node {
                UiNode::Element(el) => {
                    for event in &el.events {
                        self.scope.handlers.push(SymbolCandidate {
                            name: event.handler.name.text.clone(),
                            kind: SymbolKind::Handler,
                            detail: format!("on {}", event.event.text),
                        });
                    }
                    self.collect_handlers_and_loops(&el.children);
                }
                UiNode::Loop(loop_node) => {
                    self.scope.loop_vars.push(SymbolCandidate {
                        name: loop_node.item.text.clone(),
                        kind: SymbolKind::LoopVar,
                        detail: format!("for {} in ...", loop_node.item.text),
                    });
                    if let Some(ref key) = loop_node.key {
                        self.scope.loop_vars.push(SymbolCandidate {
                            name: key.name.text.clone(),
                            kind: SymbolKind::LoopVar,
                            detail: format!("key {}", key.name.text),
                        });
                    }
                    self.collect_handlers_and_loops(&loop_node.children);
                }
                _ => {}
            }
        }
    }

    fn determine_slot(&mut self, doc: &Document) {
        let line = self.line_prefix.trim();

        // Check cursor position triggers first (more specific than line start)
        if let Some(token) = &self.token_text {
            if token.starts_with('$') {
                self.slot = CursorSlot::DataReference;
                return;
            }
            if token.starts_with('@') {
                self.slot = CursorSlot::HandlerReference;
                return;
            }
        }

        // Include line
        if line.starts_with("#include") {
            self.slot = CursorSlot::IncludeTarget;
            return;
        }

        // Component declaration
        if line.starts_with("component") {
            self.slot = CursorSlot::DeclarationName;
            return;
        }

        // Root declaration
        if self.enclosing_component.is_none() && self.enclosing_declaration.is_none() {
            self.slot = CursorSlot::RootDeclaration;
            return;
        }

        // View body detection
        if self.is_inside_view_block(doc) {
            let first_word = line.split_whitespace().next().unwrap_or("");
            if first_word == "on" {
                self.slot = CursorSlot::EventName;
                return;
            }
            // Property name/value detection in view
            if let Some(prop) = self.view_property_at_cursor(doc) {
                let words: Vec<&str> = line.split_whitespace().collect();
                if words.len() <= 1 && !self.line_prefix.ends_with(' ') {
                    self.slot = CursorSlot::ViewPropertyName;
                } else {
                    self.slot = CursorSlot::ViewPropertyValue {
                        property: prop.clone(),
                    };
                }
                return;
            }
            // UI primitive or declaration
            let token = self.token_text.as_deref().unwrap_or(first_word);
            if language::is_ui_primitive(token) || language::declaration_keywords().contains(&token)
            {
                let words: Vec<&str> = line.split_whitespace().collect();
                if words.len() <= 1 {
                    self.slot = CursorSlot::ViewPrimitive;
                } else {
                    self.slot = CursorSlot::ViewNodeName;
                }
                return;
            }
            self.slot = CursorSlot::ViewBody;
            return;
        }

        // State block detection
        if self.is_inside_state_block() {
            self.slot = CursorSlot::StateDeclaration;
            return;
        }

        // Declaration body
        if self.enclosing_declaration.is_some() {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.is_empty() {
                // Empty line in declaration body → suggest property names
                self.slot = CursorSlot::StylePropertyName;
            } else if words.len() == 1 && !self.line_prefix.ends_with(' ') {
                self.slot = CursorSlot::StylePropertyName;
            } else if let Some(first) = words.first() {
                self.slot = CursorSlot::StylePropertyValue {
                    property: first.to_string(),
                };
            } else {
                self.slot = CursorSlot::DeclarationBody;
            }
            return;
        }

        self.slot = CursorSlot::Unknown;
    }

    fn fallback_slot(&mut self) {
        let line = self.line_prefix.trim();
        let stack = block_stack_before(self.source, self.offset);
        let first_word = line.split_whitespace().next().unwrap_or("");

        if let Some(token) = &self.token_text {
            if token.starts_with('$') {
                self.slot = CursorSlot::DataReference;
                return;
            }
            if token.starts_with('@') {
                self.slot = CursorSlot::HandlerReference;
                return;
            }
        }

        if line.starts_with("#include") {
            self.slot = CursorSlot::IncludeTarget;
            return;
        }

        if first_word == "on" {
            self.slot = CursorSlot::EventName;
            return;
        }

        // Check block stack for known contexts
        // Priority: view > state > declaration > root
        // "view" takes precedence because primitives inside view blocks
        // share names with declaration keywords (e.g. text, card).
        let mut found_view = false;
        let mut found_state = false;
        let mut found_declaration = false;

        for block in &stack {
            let block_first = block.split_whitespace().next().unwrap_or("");
            if block_first == "view" {
                found_view = true;
            }
            if language::state_keywords().contains(&block_first) {
                found_state = true;
            }
            if language::declaration_keywords().contains(&block_first) {
                found_declaration = true;
            }
        }

        if found_view {
            self.slot = CursorSlot::ViewBody;
        } else if found_state {
            self.slot = CursorSlot::StateDeclaration;
        } else if found_declaration {
            let line = self.line_prefix.trim();
            if line.is_empty() {
                self.slot = CursorSlot::StylePropertyName;
            } else {
                self.slot = CursorSlot::DeclarationBody;
            }
        } else {
            self.slot = CursorSlot::RootDeclaration;
        }
    }

    fn is_inside_view_block(&self, doc: &Document) -> bool {
        if let Some(ref component_name) = self.enclosing_component {
            if let Some(component) = doc
                .components
                .iter()
                .find(|c| c.name.text == *component_name)
            {
                if let Some(ref view) = component.view {
                    if view.span.start <= self.offset && view.span.end >= self.offset {
                        return true;
                    }
                }
                for slot in &component.slots {
                    if slot.span.start <= self.offset && slot.span.end >= self.offset {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_inside_state_block(&self) -> bool {
        // Heuristic: inside a component, inside a block that looks like a state keyword
        if self.enclosing_component.is_none() {
            return false;
        }
        let stack = block_stack_before(self.source, self.offset);
        stack
            .iter()
            .any(|header| language::state_keywords().contains(&header.as_str()))
    }

    fn view_property_at_cursor(&self, doc: &Document) -> Option<String> {
        // Find the view element at cursor and check if we're in a property
        if let Some(ref component_name) = self.enclosing_component {
            if let Some(component) = doc
                .components
                .iter()
                .find(|c| c.name.text == *component_name)
            {
                if let Some(ref view) = component.view {
                    return find_property_at_offset(&view.nodes, self.offset);
                }
            }
        }
        None
    }
}

fn find_property_at_offset(nodes: &[UiNode], offset: usize) -> Option<String> {
    for node in nodes {
        match node {
            UiNode::Element(el) if el.span.start <= offset && el.span.end >= offset => {
                for prop in &el.properties {
                    if prop.span.start <= offset && prop.span.end >= offset {
                        return Some(prop.name.text.clone());
                    }
                }
                if let Some(found) = find_property_at_offset(&el.children, offset) {
                    return Some(found);
                }
            }
            UiNode::Loop(loop_node) => {
                if loop_node.span.start <= offset && loop_node.span.end >= offset {
                    if let Some(found) = find_property_at_offset(&loop_node.children, offset) {
                        return Some(found);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn find_element_kind_at_offset(nodes: &[UiNode], offset: usize) -> Option<String> {
    for node in nodes {
        match node {
            UiNode::Element(el) if el.span.start <= offset && el.span.end >= offset => {
                // Check if cursor is directly inside this element's children
                // but not inside a nested element
                if let Some(found) = find_element_kind_at_offset(&el.children, offset) {
                    return Some(found);
                }
                // Cursor is in this element's body
                return Some(el.kind.text.clone());
            }
            UiNode::Loop(loop_node) => {
                if loop_node.span.start <= offset && loop_node.span.end >= offset {
                    return find_element_kind_at_offset(&loop_node.children, offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn word_at(source: &str, offset: usize) -> Option<&str> {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind(|c: char| !is_word_char(c))
        .map_or(0, |i| i + 1);
    let end = source[safe_offset..]
        .find(|c: char| !is_word_char(c))
        .map_or(source.len(), |i| safe_offset + i);
    if start == end {
        None
    } else {
        Some(&source[start..end])
    }
}

fn token_at(source: &str, offset: usize) -> Option<&str> {
    word_at(source, offset)
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '$' || c == '@'
}

fn line_prefix_at(source: &str, offset: usize) -> String {
    let start = source[..offset.min(source.len())]
        .rfind('\n')
        .map_or(0, |i| i + 1);
    source[start..offset.min(source.len())].to_string()
}

fn block_stack_before(source: &str, offset: usize) -> Vec<String> {
    let mut stack = Vec::new();
    let mut line_start = 0usize;
    for (index, character) in source[..offset.min(source.len())].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }
    stack
}

fn state_type_label(value_type: &frame_core::StateType) -> &'static str {
    match value_type {
        frame_core::StateType::Text => "text",
        frame_core::StateType::Bool => "bool",
        frame_core::StateType::Number => "number",
        frame_core::StateType::List => "list",
        frame_core::StateType::Unknown(_) => "unknown",
    }
}

fn prop_type_label(value_type: &frame_core::PropType) -> &'static str {
    match value_type {
        frame_core::PropType::Text => "text",
        frame_core::PropType::Bool => "bool",
        frame_core::PropType::Number => "number",
        frame_core::PropType::List => "list",
        frame_core::PropType::Unknown(_) => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_root_slot() {
        let cursor = SemanticCursor::at("", 0);
        assert_eq!(cursor.slot, CursorSlot::RootDeclaration);
    }

    #[test]
    fn detects_include_slot() {
        let cursor = SemanticCursor::at("#include ", 9);
        assert_eq!(cursor.slot, CursorSlot::IncludeTarget);
    }

    #[test]
    fn detects_component_declaration_name() {
        let cursor = SemanticCursor::at("component ChatApp {\n}", 11);
        assert_eq!(cursor.slot, CursorSlot::DeclarationName);
    }

    #[test]
    fn detects_view_body() {
        let source = "component ChatApp {\n  view {\n    \n  }\n}\n";
        let offset = source.find("    \n").unwrap() + 4;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::ViewBody);
    }

    #[test]
    fn detects_view_body_at_end_of_line() {
        let source = "component ChatInput {\n  view {\n    ";
        let offset = source.len();
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::ViewBody);
    }

    #[test]
    fn detects_enclosing_view_node_for_text() {
        let source = "component ChatInput {\n  view {\n    text Label {\n      ";
        let offset = source.len();
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::ViewBody);
        assert_eq!(cursor.enclosing_view_node, Some("text".to_string()));
    }

    #[test]
    fn detects_view_primitive() {
        let source = "component ChatApp {\n  view {\n    action Send {\n      text \"Hello\"\n    }\n  }\n}\n";
        // Cursor on the `action` keyword itself
        let offset = source.find("action").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::ViewPrimitive);
    }

    #[test]
    fn detects_event_name() {
        let source = "component ChatApp {\n  view {\n    action Send {\n      on press @send\n    }\n  }\n}\n";
        // Use " on " to avoid matching "on" inside "action"
        let offset = source.find(" on ").unwrap() + 4;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::EventName);
    }

    #[test]
    fn detects_data_reference() {
        let source = "component ChatApp {\n  view {\n    text $draft\n  }\n}\n";
        let offset = source.find("$draft").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::DataReference);
    }

    #[test]
    fn detects_handler_reference() {
        let source = "component ChatApp {\n  view {\n    action Send {\n      on press @send\n    }\n  }\n}\n";
        let offset = source.find("@send").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::HandlerReference);
    }

    #[test]
    fn detects_style_property_name() {
        let source = "card ProjectCard {\n  \n}\n";
        let offset = source.find("  \n").unwrap() + 2;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::StylePropertyName);
    }

    #[test]
    fn detects_style_property_value() {
        // Cursor is after `surface ` in a declaration body → value position
        let source = "card ProjectCard {\n  surface panel\n}\n";
        let offset = source.find("surface ").unwrap() + 8;
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(
            cursor.slot,
            CursorSlot::StylePropertyValue {
                property: "surface".to_string()
            }
        );
    }

    #[test]
    fn detects_card_declaration_body() {
        let source = "card A {\n  ";
        let offset = source.len();
        let cursor = SemanticCursor::at(source, offset);
        assert_eq!(cursor.slot, CursorSlot::StylePropertyName);
    }

    #[test]
    fn extracts_local_state() {
        let source = "component ChatApp {\n  state {\n    draft text = \"\"\n  }\n  view {\n    text $draft\n  }\n}\n";
        let offset = source.find("$draft").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert!(cursor.scope.local_state.iter().any(|s| s.name == "draft"));
    }

    #[test]
    fn extracts_local_props() {
        let source = "component ChatApp {\n  props {\n    channel text\n  }\n  view {\n    text $channel\n  }\n}\n";
        let offset = source.find("$channel").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert!(cursor.scope.local_props.iter().any(|s| s.name == "channel"));
    }

    #[test]
    fn extracts_handlers() {
        let source = "component ChatApp {\n  view {\n    action Send {\n      on press @sendMessage\n    }\n  }\n}\n";
        let offset = source.find("@sendMessage").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert!(cursor
            .scope
            .handlers
            .iter()
            .any(|s| s.name == "sendMessage"));
    }

    #[test]
    fn extracts_loop_vars() {
        let source = "component ChatApp {\n  view {\n    list Messages {\n      for msg in $messages {\n        text $msg\n      }\n    }\n  }\n}\n";
        let offset = source.find("$msg").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        assert!(cursor.scope.loop_vars.iter().any(|s| s.name == "msg"));
    }

    #[test]
    fn fallback_on_broken_syntax() {
        let source = "component ChatApp {\n  view {\n    action Send {\n      on press @send\n";
        let offset = source.find("@send").unwrap() + 1;
        let cursor = SemanticCursor::at(source, offset);
        // Should still detect handler reference even with incomplete syntax
        assert_eq!(cursor.slot, CursorSlot::HandlerReference);
        assert!(cursor.is_incomplete);
    }
}
