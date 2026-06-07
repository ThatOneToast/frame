use std::collections::HashSet;

use crate::{
    symbols::SymbolIndex, DataRef, Diagnostic, EventBinding, Identifier, StateDefault, StateType,
    TextValue, UiComponentArgumentValue, UiElement, UiNode, UiProperty, UiPropertyValue,
};

use super::constants::*;
use super::helpers::*;

pub(crate) fn validate_ui_node(
    node: &UiNode,
    all_names: &HashSet<String>,
    component_names: &HashSet<String>,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
    prop_names: &HashSet<String>,
) {
    match node {
        UiNode::Text(text) => {
            if let TextValue::Data(reference) = &text.value {
                validate_data_ref(reference, all_names, prop_names, diagnostics);
            }
        }
        UiNode::Component(invocation) => {
            if !component_names.contains(&invocation.name.text) {
                let candidates = component_names
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>();
                let suggestion = closest(&invocation.name.text, &candidates)
                    .map(|name| format!("\n\nDid you mean `{name}()`?"))
                    .unwrap_or_default();
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Unknown component `{}`.{suggestion}\n\nDeclare `component {} {{ ... }}` in this file before invoking it.",
                        invocation.name.text, invocation.name.text
                    ),
                    invocation.name.span,
                ));
            }

            for argument in &invocation.arguments {
                match &argument.value {
                    UiComponentArgumentValue::Data(reference)
                    | UiComponentArgumentValue::Bind(reference) => {
                        validate_data_ref(reference, all_names, prop_names, diagnostics);
                    }
                    UiComponentArgumentValue::Literal(_) => {}
                }
            }
        }
        UiNode::Loop(loop_node) => {
            validate_data_ref(&loop_node.collection, all_names, prop_names, diagnostics);
            if !is_valid_style_identifier(&loop_node.item.text) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Invalid loop item `{}`.\n\nUse a simple identifier such as `message`, `item`, or `user`.",
                        loop_node.item.text
                    ),
                    loop_node.item.span,
                ));
            }
            let mut scoped_names = all_names.clone();
            scoped_names.insert(loop_node.item.text.clone());
            if let Some(key) = &loop_node.key {
                validate_data_ref(key, &scoped_names, prop_names, diagnostics);
            }
            for child in &loop_node.children {
                validate_ui_node(
                    child,
                    &scoped_names,
                    component_names,
                    symbols,
                    diagnostics,
                    prop_names,
                );
            }
        }
        UiNode::Element(element) => {
            if !SEMANTIC_UI_PRIMITIVES.contains(&element.kind.text.as_str()) {
                if BROWSER_UI_WORDS.contains(&element.kind.text.as_str()) {
                    let browser_word = element.kind.text.as_str();
                    let suggestion = semantic_alternative_for(browser_word)
                        .map(|(alt, explain)| {
                            format!(
                                "\n\nhelp: did you mean `{alt}`?\n\n  {explain}\n\n  ```frame\n  {alt} Name {{\n    ...\n  }}\n  ```",
                                alt = alt,
                                explain = explain
                            )
                        })
                        .unwrap_or_else(|| {
                            "\n\nhelp: use a Frame semantic primitive such as `action`, `link`, `editor`, `panel`, `list`, or `data` so Frame can preserve intent before renderer lowering.".to_string()
                        });
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "error: `{browser_word}` is a browser implementation word, not author-facing Frame UI syntax.{suggestion}\n\nnote: Frame separates UI intent from renderer targets. The compiler maps semantic primitives to the correct DOM element during lowering.\n\nFor example:\n  - `action`  -> `<button>`\n  - `link`    -> `<a>`\n  - `editor`  -> `<textarea>`\n  - `list`    -> `<ul>` / `<ol>`\n  - `title`   -> `<h1>` ... `<h6>`\n\nIf you need a DOM escape hatch, use the `advanced` block with `css` rules."
                        ),
                        element.kind.span,
                    ));
                    return;
                }
                diagnostics.push(Diagnostic::error(
                    format!(
                        "Unknown UI primitive `{}`.\n\nSupported semantic primitives are: {}.",
                        element.kind.text,
                        SEMANTIC_UI_PRIMITIVES.join(", ")
                    ),
                    element.kind.span,
                ));
            }

            validate_element_accessibility(element, diagnostics);
            validate_element_security(element, diagnostics);

            if let Some(style) = &element.style {
                validate_style_ref(&style.name, symbols, diagnostics);
            } else if !symbols.declarations.contains_key(&element.name.text) {
                diagnostics.push(Diagnostic::info(
                    format!(
                        "`{}` will use automatic style lookup when a matching style exists.",
                        element.name.text
                    ),
                    element.name.span,
                ));
            }

            for property in &element.properties {
                validate_ui_property(
                    property,
                    element.kind.text.as_str(),
                    all_names,
                    prop_names,
                    symbols,
                    diagnostics,
                );
            }

            for event in &element.events {
                validate_event_binding(event, diagnostics);
            }

            validate_element_children(
                element,
                all_names,
                component_names,
                symbols,
                diagnostics,
                prop_names,
            );
        }
    }
}

pub(crate) fn validate_element_children(
    element: &UiElement,
    all_names: &HashSet<String>,
    component_names: &HashSet<String>,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
    prop_names: &HashSet<String>,
) {
    let mut child_names = all_names.clone();
    if matches!(element.kind.text.as_str(), "list" | "feed" | "data") {
        let item_name = singular_item_name(&element.name.text);
        child_names.insert(item_name);
    }
    for child in &element.children {
        validate_ui_node(
            child,
            &child_names,
            component_names,
            symbols,
            diagnostics,
            prop_names,
        );
    }
}

pub(crate) fn validate_element_accessibility(
    element: &UiElement,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let kind = element.kind.text.as_str();
    match kind {
        "image" | "avatar" => {
            if !has_property(element, &["alt", "description"]) && !has_decorative_true(element) {
                diagnostics.push(Diagnostic::warning(
                    "`image` requires alternate text through `alt` or `description`, or `decorative true` when the image is only visual."
                        .to_string(),
                    element.kind.span,
                ));
            }
        }
        "action" => {
            if !has_accessible_name(element) {
                diagnostics.push(Diagnostic::warning(
                    "`action` requires a user-facing name.\n\nUse the node name, visible text, or `label` to describe the command."
                        .to_string(),
                    element.kind.span,
                ));
            }
        }
        "link" => {
            if !has_accessible_name(element) {
                diagnostics.push(Diagnostic::warning(
                    "`link` requires a user-facing name.\n\nUse the node name, visible text, or `label` to describe the destination."
                        .to_string(),
                    element.kind.span,
                ));
            }
            if !has_property(element, &["goto"]) {
                diagnostics.push(Diagnostic::warning(
                    "`link` should include `goto` so renderers know the navigation destination."
                        .to_string(),
                    element.kind.span,
                ));
            }
        }
        "input" | "editor" | "toggle" | "choice" | "select" | "composer" => {
            if !has_accessible_name(element) {
                diagnostics.push(Diagnostic::warning(
                    format!("`{kind}` requires a Frame label or visible name."),
                    element.kind.span,
                ));
            }
        }
        "data" => {
            if !has_property(element, &["source"]) {
                diagnostics.push(Diagnostic::warning(
                    "`data` should declare `source $items` so renderers know the records being presented."
                        .to_string(),
                    element.kind.span,
                ));
            }
        }
        "list" | "feed" => {
            if !has_property(element, &["source"]) {
                diagnostics.push(Diagnostic::warning(
                    format!("`{kind}` should declare `source $items`."),
                    element.kind.span,
                ));
            }
        }
        "dialog" if !has_accessible_name(element) => {
            diagnostics.push(Diagnostic::warning(
                "`dialog` requires a Frame label, title, or visible text so assistive technology can name it."
                    .to_string(),
                element.kind.span,
            ));
        }
        _ => {}
    }
}

pub(crate) fn validate_element_security(element: &UiElement, diagnostics: &mut Vec<Diagnostic>) {
    if property_literal(element, "new-window")
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    {
        diagnostics.push(Diagnostic::info(
            "`new-window true` records navigation intent. DOM lowering must apply safe external-link behavior."
                .to_string(),
            element.kind.span,
        ));
    }
}

pub(crate) fn has_accessible_name(element: &UiElement) -> bool {
    has_property(element, &["label", "title", "text", "value"])
        || !matches!(
            element.name.text.as_str(),
            "action"
                | "link"
                | "input"
                | "editor"
                | "toggle"
                | "choice"
                | "select"
                | "composer"
                | "dialog"
                | "image"
                | "avatar"
        )
        || element.children.iter().any(|child| {
            matches!(child, UiNode::Text(_))
                || matches!(
                    child,
                    UiNode::Element(child)
                        if matches!(child.kind.text.as_str(), "label" | "title" | "text")
                )
        })
}

pub(crate) fn has_decorative_true(element: &UiElement) -> bool {
    property_literal(element, "decorative").is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

pub(crate) fn has_property(element: &UiElement, names: &[&str]) -> bool {
    element
        .properties
        .iter()
        .any(|property| names.contains(&property.name.text.as_str()))
}

pub(crate) fn property_literal<'a>(element: &'a UiElement, name: &str) -> Option<&'a str> {
    element.properties.iter().find_map(|property| {
        if property.name.text == name {
            if let UiPropertyValue::Literal(value) = &property.value {
                return Some(value.as_str());
            }
        }
        None
    })
}

pub(crate) fn has_unsafe_url_scheme(value: &str) -> bool {
    value.split(',').any(|candidate| {
        candidate
            .split_whitespace()
            .next()
            .is_some_and(|url| url.to_ascii_lowercase().starts_with("javascript:"))
    })
}

pub(crate) fn validate_ui_property(
    property: &UiProperty,
    element_kind: &str,
    all_names: &HashSet<String>,
    prop_names: &HashSet<String>,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match &property.value {
        UiPropertyValue::Data(reference) | UiPropertyValue::Bind(reference) => {
            validate_data_ref(reference, all_names, prop_names, diagnostics)
        }
        UiPropertyValue::Handler(handler) => diagnostics.push(Diagnostic::info(
            format!(
                "@{} references an external handler. Frame does not store script bodies inside UI declarations.",
                handler.name.text
            ),
            handler.span,
        )),
        UiPropertyValue::Conditional(binding) => {
            validate_data_ref(&binding.condition, all_names, prop_names, diagnostics);
            // Accessibility: flag common properties that should have conditions
            if property.name.text == "show" {
                diagnostics.push(Diagnostic::info(
                    "`show when` records conditional rendering intent. The DOM runtime patches this node's visibility based on the condition.".to_string(),
                    property.span,
                ));
            }
        }
        UiPropertyValue::StyleWhen { condition, style } => {
            validate_data_ref(condition, all_names, prop_names, diagnostics);
            validate_style_ref(&style.name, symbols, diagnostics);
        }
        UiPropertyValue::Unknown(_) => diagnostics.push(Diagnostic::error(
            format!(
                "Unknown UI property syntax for `{}`.\n\nSupported forms include `label \"Text\"`, `bind $state`, `disabled when $state`, `send @handler`, and `style when $state = StyleName`.",
                property.name.text
            ),
            property.span,
        )),
        UiPropertyValue::Literal(_) => {}
    }

    if property.name.text.starts_with("on") {
        diagnostics.push(Diagnostic::error(
            format!(
                "`{}` looks like an inline event attribute.\n\nUse `on event @handler` so Frame can type and clean up event bindings.",
                property.name.text
            ),
            property.span,
        ));
    }

    if REMOVED_HTML_ATTRIBUTES.contains(&property.name.text.as_str()) {
        diagnostics.push(Diagnostic::error(
            format!(
                "`{}` is browser-centric author-facing syntax.\n\nUse Frame intent properties such as `goto`, `source`, `label`, `description`, or semantic primitives instead.",
                property.name.text
            ),
            property.span,
        ));
    }

    // URL-bearing intent detection
    if is_url_intent_property(property.name.text.as_str(), element_kind) {
        diagnostics.push(Diagnostic::warning(
            format!(
                "`{}` is a navigation or media destination.\n\nRenderers must validate and classify this value before it reaches a URL sink.",
                property.name.text
            ),
            property.span,
        ));

        if let UiPropertyValue::Literal(value) = &property.value {
            if has_unsafe_url_scheme(value) {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "`{}` uses an unsafe URL scheme.\n\n`javascript:` URLs are rejected by default; use a safe http, https, mailto, tel, relative, or fragment URL.",
                        property.name.text
                    ),
                    property.span,
                ));
            }
        }
    }

    if UNSAFE_HTML_ATTRIBUTES.contains(&property.name.text.as_str()) {
        diagnostics.push(Diagnostic::error(
            format!(
                "`{}` is an unsafe HTML injection sink.\n\nFrame text escapes by default. Raw HTML must use an explicit unsafe capability before any renderer may consume it.",
                property.name.text
            ),
            property.span,
        ));
    }

    validate_primitive_specific_property(property, element_kind, diagnostics);
}

pub(crate) fn validate_primitive_specific_property(
    property: &UiProperty,
    element_kind: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    use super::helpers::{primitive_kind_label, valid_properties_for_primitive};

    let valid = valid_properties_for_primitive(element_kind);
    let name = property.name.text.as_str();

    // Skip if the property is globally valid for this primitive
    if valid.contains(&name) {
        // Still check for misused bind forms
        match name {
            "value" => {
                if !matches!(element_kind, "input" | "editor") && is_bind_value(&property.value) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "`value bind` is only valid on input-like primitives (`input`, `editor`).\n\n`{element_kind}` is a {} and does not own editable state.\nUse `field` when the user should edit a value, or place the binding on an `input` or `editor` child.",
                            primitive_kind_label(element_kind)
                        ),
                        property.span,
                    ));
                }
            }
            "checked" => {
                if !matches!(element_kind, "toggle" | "choice") && is_bind_value(&property.value) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "`checked bind` is only valid on toggle-like primitives (`toggle`, `choice`).\n\n`{element_kind}` is a {} and does not represent a binary setting.",
                            primitive_kind_label(element_kind)
                        ),
                        property.span,
                    ));
                }
            }
            "selected" => {
                if !matches!(element_kind, "select" | "choice") && is_bind_value(&property.value) {
                    diagnostics.push(Diagnostic::error(
                        format!(
                            "`selected bind` is only valid on selection primitives (`select`, `choice`).\n\n`{element_kind}` is a {} and does not represent a choice.",
                            primitive_kind_label(element_kind)
                        ),
                        property.span,
                    ));
                }
            }
            "source"
                if !matches!(element_kind, "image" | "avatar" | "media")
                    && is_bind_value(&property.value) =>
            {
                diagnostics.push(Diagnostic::error(
                    format!(
                        "`source` on `{element_kind}` should reference media content, not a state binding.\n\nUse a literal or data reference for media destinations.",
                    ),
                    property.span,
                ));
            }
            _ => {}
        }
        return;
    }

    // Special-case common misuses with teacher-like guidance
    let suggestion = match (element_kind, name) {
        ("text" | "title" | "label" | "badge", "value") => {
            "`text`-like primitives display content. They do not own editable state.\nUse `field` when the user should edit a value."
        }
        ("panel" | "stack" | "row" | "screen" | "section" | "card" | "dialog" | "popover", "value") => {
            "Container primitives group children. They do not own form state.\nPlace `value bind` on an `input`, `editor`, `toggle`, or `select` child instead."
        }
        ("action" | "link", "value") => {
            "Action-like primitives trigger commands. They do not store editable state.\nUse `text` or `label` for visible content."
        }
        ("list" | "feed" | "data", "value") => {
            "Collection primitives render repeated content. They do not store single-item state.\nUse `source $items` to provide collection data."
        }
        ("media" | "image" | "avatar", "value") => {
            "Media-like primitives display visual content. They do not own form state.\nUse `source` or `sources` for media destinations."
        }
        ("input" | "editor", "text") => {
            "Input-like primitives already contain editable text.\nUse `value bind` for state, `placeholder` for help text, or `label` for a visible name."
        }
        ("action" | "link", "placeholder") => {
            "Action-like primitives do not accept placeholder text.\nUse `text` or `label` for the visible command name."
        }
        ("action" | "link", "source") => {
            "Action-like primitives trigger commands. They do not display media.\nUse `goto` for navigation destinations."
        }
        ("field", "value") => {
            "`field` groups a label and a control. It does not own state directly.\nPlace `value bind` on the `input`, `editor`, or `toggle` child instead."
        }
        ("composer", "value") => {
            "`composer` owns draft state through `draft bind`, not `value bind`.\nUse `draft bind $state` to connect the composer to component state."
        }
        ("icon", "text") => {
            "`icon` represents symbolic visual content.\nUse `label` for accessibility text, or `decorative true` when the icon is purely visual."
        }
        ("panel" | "stack" | "row" | "screen" | "section" | "card" | "dialog" | "popover", "source") => {
            "Container primitives group children. They do not own media destinations.\nPlace `source` on an `image`, `avatar`, or `media` child instead."
        }
        ("text" | "title" | "label" | "badge", "source") => {
            "Text-like primitives display content. They do not own media destinations.\nUse `source` on `image`, `avatar`, or `media` instead."
        }
        ("toggle" | "choice" | "select", "source") => {
            "Selection primitives represent settings. They do not own media destinations.\nUse `source` on `image`, `avatar`, or `media` instead."
        }
        _ => return, // Not a known misuse; skip
    };

    diagnostics.push(Diagnostic::error(
        format!(
            "`{name}` is not a valid property for `{element_kind}`.\n\n{}",
            suggestion
        ),
        property.span,
    ));
}

fn is_bind_value(value: &UiPropertyValue) -> bool {
    matches!(value, UiPropertyValue::Bind(_))
}

pub(crate) fn is_url_intent_property(property: &str, element_kind: &str) -> bool {
    URL_ATTRIBUTES.contains(&property)
        || (property == "source"
            && matches!(
                element_kind,
                "image" | "avatar" | "video" | "audio" | "media"
            ))
}

pub(crate) fn validate_event_binding(event: &EventBinding, diagnostics: &mut Vec<Diagnostic>) {
    if !UI_EVENTS.contains(&event.event.text.as_str()) {
        diagnostics.push(Diagnostic::error(
            format!(
                "Unknown event `{}`.\n\nSupported events include: {}.",
                event.event.text,
                UI_EVENTS.join(", ")
            ),
            event.event.span,
        ));
    }
    for modifier in &event.modifiers {
        if !UI_EVENT_MODIFIERS.contains(&modifier.text.as_str()) {
            diagnostics.push(Diagnostic::error(
                format!(
                    "Unknown event modifier `{}`.\n\nSupported modifiers include: {}.",
                    modifier.text,
                    UI_EVENT_MODIFIERS.join(", ")
                ),
                modifier.span,
            ));
        }
    }
    diagnostics.push(Diagnostic::info(
        format!(
            "@{} references an external handler. Frame does not store script bodies inside UI declarations.",
            event.handler.name.text
        ),
        event.handler.span,
    ));
}

pub(crate) fn validate_data_ref(
    reference: &DataRef,
    all_names: &HashSet<String>,
    prop_names: &HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let root = reference
        .name
        .text
        .split('.')
        .next()
        .unwrap_or(&reference.name.text);
    if all_names.contains(&reference.name.text) || all_names.contains(root) {
        // If it's a prop, add a soft note for clarity
        if prop_names.contains(&reference.name.text) || prop_names.contains(root) {
            diagnostics.push(Diagnostic::info(
                format!(
                    "`${}` references a prop. Props are passed from the parent component.",
                    reference.name.text
                ),
                reference.span,
            ));
        }
        return;
    }
    let candidates = all_names.iter().map(String::as_str).collect::<Vec<_>>();
    let suggestion = closest(&reference.name.text, &candidates)
        .map(|name| format!("\n\nDid you mean `${name}`?"))
        .unwrap_or_default();
    diagnostics.push(Diagnostic::error(
        format!(
            "Unknown reference `${}`.{suggestion}\n\nDeclare it in the component `props`, `state`, or an enclosing `for` item before using it in `view`.",
            reference.name.text
        ),
        reference.span,
    ));
}

pub(crate) fn validate_style_ref(
    style: &Identifier,
    symbols: &SymbolIndex,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if symbols.declarations.contains_key(&style.text) {
        return;
    }
    diagnostics.push(Diagnostic::warning(
        format!(
            "Style `{}` is not declared in this file.\n\nFrame records the style reference now; cross-file resolution can satisfy it later.",
            style.text
        ),
        style.span,
    ));
}

pub(crate) fn state_type_label(value_type: &StateType) -> &'static str {
    match value_type {
        StateType::Text => "text",
        StateType::Bool => "bool",
        StateType::Number => "number",
        StateType::List => "list",
        StateType::Unknown(_) => "unknown",
    }
}

pub(crate) fn state_default_label(default: &StateDefault) -> &'static str {
    match default {
        StateDefault::Text(_) => "a text literal",
        StateDefault::Bool(_) => "a bool literal",
        StateDefault::Number(_) => "a number literal",
        StateDefault::List => "an empty list literal",
        StateDefault::Invalid(_) => "an unsupported literal",
    }
}

pub(crate) fn validate_state_default(value: &crate::StateValue, diagnostics: &mut Vec<Diagnostic>) {
    match (&value.value_type, &value.default) {
        (StateType::Text, StateDefault::Text(_))
        | (StateType::Bool, StateDefault::Bool(_))
        | (StateType::Number, StateDefault::Number(_))
        | (StateType::List, StateDefault::List) => {}
        (StateType::Unknown(kind), _) => diagnostics.push(Diagnostic::error(
            format!(
                "Unknown state type `{kind}`.\n\nSupported state types are `text`, `string`, `bool`, `number`, and `list`."
            ),
            value.span,
        )),
        (expected, actual) => diagnostics.push(Diagnostic::error(
            format!(
                "State default for `{}` does not match declared type `{}`.\n\nFound {}.",
                value.name.text,
                state_type_label(expected),
                state_default_label(actual)
            ),
            value.span,
        )),
    }
}
