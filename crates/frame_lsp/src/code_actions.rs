use std::collections::HashMap;

use frame_core::symbols::index_document;

use frame_parser::parse;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, Position, Range, TextEdit, Url, WorkspaceEdit,
};

use crate::diagnostics::position_for_offset;
use crate::document_symbols::collect_document_symbols;

pub fn code_actions_for_source(source: &str, uri: &Url) -> Vec<CodeActionOrCommand> {
    let mut actions = Vec::new();

    typo_action(
        source,
        uri,
        "pannel",
        "panel",
        "Replace with `surface panel`",
        &mut actions,
    );
    typo_action(
        source,
        uri,
        "padd",
        "padding",
        "Replace with `padding`",
        &mut actions,
    );

    for replacement in browser_primitive_replacements(source) {
        actions.push(edit_action(
            &format!("Convert `{}` to `{}`", replacement.from, replacement.to),
            uri,
            replacement.range,
            replacement.to.to_string(),
        ));
    }

    for replacement in browser_event_replacements(source) {
        actions.push(edit_action(
            &format!("Replace browser event with `on {}`", replacement.event),
            uri,
            replacement.range,
            replacement.new_text,
        ));
    }

    for style in missing_style_references(source) {
        actions.push(edit_action(
            &format!("Create style `{style}`"),
            uri,
            insertion_at_end(source),
            format!("\ncard {style} {{\n  padding medium\n}}\n"),
        ));
    }

    for handler in missing_handler_references(source) {
        actions.push(create_handler_action(uri, &handler));
    }

    for (name, inferred_type, component_range) in missing_state_references(source) {
        actions.push(edit_action(
            &format!("Create state `{name}`"),
            uri,
            component_range,
            format!("  {name} {inferred_type} = \"\"\n"),
        ));
    }

    for (name, inferred_type, component_range) in missing_prop_references(source) {
        actions.push(edit_action(
            &format!("Create prop `{name}`"),
            uri,
            component_range,
            format!("  {name} {inferred_type}\n"),
        ));
    }

    for missing_grid in missing_grid_references(source) {
        actions.push(edit_action(
            "Create grid",
            uri,
            insertion_at_start(),
            format!("grid {missing_grid} {{\n  columns sidebar content\n  gap medium\n}}\n\n"),
        ));
    }

    for (area_name, insert_offset) in areas_without_place(source) {
        actions.push(edit_action(
            "Add `place` for area",
            uri,
            Range {
                start: position_for_offset(source, insert_offset),
                end: position_for_offset(source, insert_offset),
            },
            format!("  place {}\n", area_name.to_ascii_lowercase()),
        ));
    }

    for (grid_name, columns, insert_offset) in grids_with_named_columns(source) {
        if !columns.is_empty() {
            actions.push(edit_action(
                "Create matching area blocks",
                uri,
                Range {
                    start: position_for_offset(source, insert_offset),
                    end: position_for_offset(source, insert_offset),
                },
                matching_areas(&grid_name, &columns),
            ));
        }

        if columns.len() == 3 {
            if let Some(range) = columns_line_range(source, &grid_name) {
                actions.push(edit_action(
                    "Convert to `columns 25% 50% 25%`",
                    uri,
                    range,
                    "  columns 25% 50% 25%".to_string(),
                ));
            }
        }
    }

    for (card_name, insert_offset) in cards_without_hover(source) {
        actions.push(edit_action(
            &format!("Add hover lift/glow effects to `{card_name}`"),
            uri,
            Range {
                start: position_for_offset(source, insert_offset),
                end: position_for_offset(source, insert_offset),
            },
            "  hover {\n    lift small\n    glow accent\n  }\n".to_string(),
        ));
    }

    for replacement in advanced_css_replacements(source) {
        actions.push(edit_action(
            &format!("Replace advanced CSS with `{}`", replacement.frame.trim()),
            uri,
            replacement.range,
            replacement.frame,
        ));
    }

    actions
}

struct AdvancedReplacement {
    range: Range,
    frame: String,
}

struct WordReplacement {
    range: Range,
    from: String,
    to: &'static str,
}

struct EventReplacement {
    range: Range,
    event: &'static str,
    new_text: String,
}

fn browser_primitive_replacements(source: &str) -> Vec<WordReplacement> {
    let mut replacements = Vec::new();
    let mut offset = 0usize;

    for line in source.lines() {
        let leading = line.len() - line.trim_start().len();
        let trimmed = line.trim_start();
        let Some(first) = trimmed.split_whitespace().next() else {
            offset += line.len() + 1;
            continue;
        };
        let Some(to) = browser_primitive_replacement(first) else {
            offset += line.len() + 1;
            continue;
        };
        replacements.push(WordReplacement {
            range: Range {
                start: position_for_offset(source, offset + leading),
                end: position_for_offset(source, offset + leading + first.len()),
            },
            from: first.to_string(),
            to,
        });
        offset += line.len() + 1;
    }

    replacements
}

fn browser_primitive_replacement(word: &str) -> Option<&'static str> {
    match word {
        "button" => Some("action"),
        "a" => Some("link"),
        "div" => Some("panel"),
        "main" => Some("screen"),
        "aside" => Some("dock"),
        "textarea" => Some("editor"),
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Some("title"),
        "ul" | "ol" => Some("list"),
        _ => None,
    }
}

fn browser_event_replacements(source: &str) -> Vec<EventReplacement> {
    let mut replacements = Vec::new();
    let mut offset = 0usize;

    for line in source.lines() {
        let leading = line.len() - line.trim_start().len();
        let trimmed = line.trim_start();
        if let Some((event, handler)) = browser_event_replacement(trimmed) {
            replacements.push(EventReplacement {
                range: Range {
                    start: position_for_offset(source, offset + leading),
                    end: position_for_offset(source, offset + line.len()),
                },
                event,
                new_text: format!("{}on {} {}", " ".repeat(leading), event, handler.trim()),
            });
        }
        offset += line.len() + 1;
    }

    replacements
}

fn browser_event_replacement(line: &str) -> Option<(&'static str, &str)> {
    let (browser_event, handler) = line.split_once(' ')?;
    let event = match browser_event {
        "onclick" => "press",
        "onchange" => "change",
        "oninput" => "input",
        "onkeydown" => "keydown",
        "onkeyup" => "keyup",
        _ => return None,
    };
    Some((event, handler))
}

fn missing_style_references(source: &str) -> Vec<String> {
    let Ok(document) = parse(source) else {
        return Vec::new();
    };
    let symbols = index_document(source, &document);
    let mut refs = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(header) = trimmed.strip_suffix('{') {
            if let Some(style) = header.split_whitespace().nth(1).and_then(|name| {
                name.split_once(':')
                    .map(|(_, style)| style.trim().to_string())
            }) {
                refs.push(style);
            }
        }
        let words = trimmed.split_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["style", "when", _, "=", style] => refs.push((*style).to_string()),
            ["style", style, "when", _] => refs.push((*style).to_string()),
            _ => {}
        }
    }
    refs.sort();
    refs.dedup();
    refs.into_iter()
        .filter(|style| !symbols.declarations.contains_key(style))
        .collect()
}

fn advanced_css_replacements(source: &str) -> Vec<AdvancedReplacement> {
    let mut replacements = Vec::new();
    let mut offset = 0usize;
    let mut in_advanced = false;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "advanced {" {
            in_advanced = true;
        } else if in_advanced && trimmed == "}" {
            in_advanced = false;
        } else if in_advanced {
            if let Some(frame) = advanced_css_replacement(trimmed) {
                let leading = line.find("css").unwrap_or(0);
                replacements.push(AdvancedReplacement {
                    range: Range {
                        start: position_for_offset(source, offset + leading),
                        end: position_for_offset(source, offset + line.len()),
                    },
                    frame,
                });
            }
        }
        offset += line.len() + 1;
    }

    replacements
}

fn advanced_css_replacement(line: &str) -> Option<String> {
    let words = line.split_whitespace().collect::<Vec<_>>();
    let property = words.get(1)?.trim_matches('"');
    let value = words.get(2).copied().unwrap_or_default();

    let replacement = match (property, value) {
        ("appearance", "none") => "control reset".to_string(),
        ("cursor", "pointer") => "interactive".to_string(),
        ("box-sizing", "border-box") => "box border".to_string(),
        ("overflow", "hidden") => "overflow hidden".to_string(),
        ("overflow-y", "auto") => "scroll y".to_string(),
        ("overflow-x", "auto") => "scroll x".to_string(),
        ("width", "100%") => "width fill".to_string(),
        ("margin", "0") => "margin none".to_string(),
        ("overflow-wrap", "anywhere") => "wrap anywhere".to_string(),
        ("text-transform", "uppercase") => "case uppercase".to_string(),
        ("line-height", "1.45") => "line relaxed".to_string(),
        ("letter-spacing", "0") => "letter normal".to_string(),
        ("white-space", "nowrap") => "truncate".to_string(),
        ("text-align", value) => format!("align-text {value}"),
        ("min-width", "0") => "min-width zero".to_string(),
        ("justify-self", "center") | ("align-self", "center") => "self center".to_string(),
        ("border-top", _) => edge_border_replacement("top", &words)?,
        ("border-right", _) => edge_border_replacement("right", &words)?,
        ("border-bottom", _) => edge_border_replacement("bottom", &words)?,
        ("border-left", _) => edge_border_replacement("left", &words)?,
        _ => return None,
    };

    Some(replacement)
}

fn edge_border_replacement(edge: &str, words: &[&str]) -> Option<String> {
    let value = words.join(" ");
    let color = value
        .split("var(--frame-color-")
        .nth(1)?
        .split(')')
        .next()?;
    Some(format!("border {edge} {color}"))
}

fn typo_action(
    source: &str,
    uri: &Url,
    typo: &str,
    replacement: &str,
    title: &str,
    actions: &mut Vec<CodeActionOrCommand>,
) {
    let Some(start) = source.find(typo) else {
        return;
    };
    actions.push(edit_action(
        title,
        uri,
        Range {
            start: position_for_offset(source, start),
            end: position_for_offset(source, start + typo.len()),
        },
        replacement.to_string(),
    ));
}

fn missing_grid_references(source: &str) -> Vec<String> {
    let symbols = collect_document_symbols(source);
    source
        .lines()
        .filter_map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            (words.first() == Some(&"in"))
                .then(|| words.get(1).copied())
                .flatten()
        })
        .filter(|name| !symbols.grids.contains_key(*name))
        .map(ToOwned::to_owned)
        .collect()
}

fn areas_without_place(source: &str) -> Vec<(String, usize)> {
    let mut areas = Vec::new();
    let mut offset = 0usize;
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index].trim();
        if line.starts_with("area ") && line.ends_with('{') {
            let area_name = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("Area")
                .trim_end_matches('{')
                .to_string();
            let mut has_in = false;
            let mut has_place = false;
            let mut insert_offset = offset + lines[index].len() + 1;
            index += 1;
            while index < lines.len() && lines[index].trim() != "}" {
                let trimmed = lines[index].trim();
                has_in |= trimmed.starts_with("in ");
                has_place |= trimmed.starts_with("place ");
                insert_offset += lines[index].len() + 1;
                index += 1;
            }
            if has_in && !has_place {
                areas.push((area_name, insert_offset));
            }
        }
        offset += lines.get(index).map_or(0, |line| line.len() + 1);
        index += 1;
    }

    areas
}

fn grids_with_named_columns(source: &str) -> Vec<(String, Vec<String>, usize)> {
    let mut grids = Vec::new();
    let mut offset = 0usize;
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index].trim();
        if line.starts_with("grid ") && line.ends_with('{') {
            let grid_name = line.split_whitespace().nth(1).unwrap_or("Grid").to_string();
            let mut columns = Vec::new();
            let mut insert_offset = offset + lines[index].len() + 1;
            index += 1;
            while index < lines.len() && lines[index].trim() != "}" {
                let trimmed = lines[index].trim();
                if trimmed.starts_with("columns ") {
                    columns = trimmed
                        .split_whitespace()
                        .skip(1)
                        .filter(|value| {
                            value
                                .chars()
                                .next()
                                .is_some_and(|character| character.is_ascii_alphabetic())
                                && !matches!(*value, "responsive" | "cards" | "subgrid")
                        })
                        .map(ToOwned::to_owned)
                        .collect();
                }
                insert_offset += lines[index].len() + 1;
                index += 1;
            }
            grids.push((grid_name, columns, insert_offset + 1));
        }
        offset += lines.get(index).map_or(0, |line| line.len() + 1);
        index += 1;
    }

    grids
}

fn cards_without_hover(source: &str) -> Vec<(String, usize)> {
    let mut cards = Vec::new();
    let mut offset = 0usize;
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index].trim();
        if line.starts_with("card ") && line.ends_with('{') {
            let card_name = line.split_whitespace().nth(1).unwrap_or("Card").to_string();
            let mut has_hover = false;
            let mut insert_offset = offset + lines[index].len() + 1;
            index += 1;
            while index < lines.len() && lines[index].trim() != "}" {
                has_hover |= lines[index].trim().starts_with("hover ");
                insert_offset += lines[index].len() + 1;
                index += 1;
            }
            if !has_hover {
                cards.push((card_name, insert_offset));
            }
        }
        offset += lines.get(index).map_or(0, |line| line.len() + 1);
        index += 1;
    }

    cards
}

fn columns_line_range(source: &str, grid_name: &str) -> Option<Range> {
    let mut in_grid = false;
    let mut offset = 0usize;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == format!("grid {grid_name} {{") {
            in_grid = true;
        } else if in_grid && trimmed == "}" {
            return None;
        } else if in_grid && trimmed.starts_with("columns ") {
            let leading = line.find("columns").unwrap_or(0);
            return Some(Range {
                start: position_for_offset(source, offset + leading),
                end: position_for_offset(source, offset + line.len()),
            });
        }
        offset += line.len() + 1;
    }
    None
}

fn matching_areas(grid_name: &str, columns: &[String]) -> String {
    let mut output = String::from("\n");
    for column in columns {
        let name = title_case(column);
        let surface = if matches!(column.as_str(), "content" | "main") {
            "main"
        } else {
            "panel"
        };
        let padding = if matches!(column.as_str(), "content" | "main") {
            "large"
        } else {
            "medium"
        };
        output.push_str(&format!(
            "\narea {name} {{\n  in {grid_name}\n  place {column}\n  surface {surface}\n  padding {padding}\n}}\n"
        ));
    }
    output
}

fn title_case(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return "Area".to_string();
    };
    first.to_ascii_uppercase().to_string() + chars.as_str()
}

fn insertion_at_start() -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    }
}

fn insertion_at_end(source: &str) -> Range {
    let line = source.lines().count() as u32;
    Range {
        start: Position { line, character: 0 },
        end: Position { line, character: 0 },
    }
}

fn edit_action(title: &str, uri: &Url, range: Range, new_text: String) -> CodeActionOrCommand {
    let mut changes = HashMap::new();
    changes.insert(uri.clone(), vec![TextEdit { range, new_text }]);

    CodeActionOrCommand::CodeAction(CodeAction {
        title: title.to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..WorkspaceEdit::default()
        }),
        ..CodeAction::default()
    })
}

fn missing_handler_references(source: &str) -> Vec<String> {
    let Ok(document) = parse(source) else {
        return Vec::new();
    };
    let mut handlers = std::collections::HashSet::new();
    for component in &document.components {
        if let Some(view) = &component.view {
            collect_handlers_from_nodes(&view.nodes, &mut handlers);
        }
        for slot in &component.slots {
            collect_handlers_from_nodes(&slot.nodes, &mut handlers);
        }
    }
    handlers.into_iter().collect()
}

fn collect_handlers_from_nodes(
    nodes: &[frame_core::UiNode],
    handlers: &mut std::collections::HashSet<String>,
) {
    for node in nodes {
        match node {
            frame_core::UiNode::Element(element) => {
                for event in &element.events {
                    handlers.insert(event.handler.name.text.clone());
                }
                collect_handlers_from_nodes(&element.children, handlers);
            }
            frame_core::UiNode::Loop(loop_node) => {
                collect_handlers_from_nodes(&loop_node.children, handlers);
            }
            _ => {}
        }
    }
}

fn create_handler_action(uri: &Url, handler: &str) -> CodeActionOrCommand {
    let ts_uri = handler_ts_uri(uri);
    let document_changes = tower_lsp::lsp_types::DocumentChanges::Operations(vec![
        tower_lsp::lsp_types::DocumentChangeOperation::Op(
            tower_lsp::lsp_types::ResourceOp::Create(tower_lsp::lsp_types::CreateFile {
                uri: ts_uri.clone(),
                options: Some(tower_lsp::lsp_types::CreateFileOptions {
                    overwrite: Some(false),
                    ignore_if_exists: Some(true),
                }),
                annotation_id: None,
            }),
        ),
    ]);

    CodeActionOrCommand::CodeAction(CodeAction {
        title: format!("Create handler `{handler}`"),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            document_changes: Some(document_changes),
            ..WorkspaceEdit::default()
        }),
        ..CodeAction::default()
    })
}

fn handler_ts_uri(frame_uri: &Url) -> Url {
    let path = frame_uri.to_file_path().unwrap_or_default();
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let ts_path = parent.join("handlers.ts");
    Url::from_file_path(ts_path).unwrap_or_else(|_| frame_uri.clone())
}

fn missing_state_references(source: &str) -> Vec<(String, String, Range)> {
    let Ok(document) = parse(source) else {
        return Vec::new();
    };
    let mut missing = Vec::new();
    for component in &document.components {
        let mut known_names = std::collections::HashSet::new();
        if let Some(state) = &component.state {
            for value in &state.values {
                known_names.insert(value.name.text.clone());
            }
        }
        if let Some(props) = &component.props {
            for value in &props.values {
                known_names.insert(value.name.text.clone());
            }
        }
        if let Some(view) = &component.view {
            collect_missing_refs(&view.nodes, &known_names, &mut missing, source);
        }
        for slot in &component.slots {
            collect_missing_refs(&slot.nodes, &known_names, &mut missing, source);
        }
    }
    missing
}

fn collect_missing_refs(
    nodes: &[frame_core::UiNode],
    known_names: &std::collections::HashSet<String>,
    missing: &mut Vec<(String, String, Range)>,
    source: &str,
) {
    for node in nodes {
        match node {
            frame_core::UiNode::Text(text) => {
                if let frame_core::TextValue::Data(data_ref) = &text.value {
                    if !known_names.contains(&data_ref.name.text)
                        && !data_ref.name.text.contains('.')
                    {
                        missing.push((
                            data_ref.name.text.clone(),
                            infer_state_type_from_usage(data_ref.name.text.as_str()),
                            crate::diagnostics::range_for_span(source, data_ref.name.span),
                        ));
                    }
                }
            }
            frame_core::UiNode::Element(element) => {
                for property in &element.properties {
                    match &property.value {
                        frame_core::UiPropertyValue::Data(data_ref)
                        | frame_core::UiPropertyValue::Bind(data_ref) => {
                            if !known_names.contains(&data_ref.name.text) {
                                missing.push((
                                    data_ref.name.text.clone(),
                                    infer_state_type_from_usage(&data_ref.name.text),
                                    crate::diagnostics::range_for_span(source, data_ref.name.span),
                                ));
                            }
                        }
                        frame_core::UiPropertyValue::Conditional(binding) => {
                            if !known_names.contains(&binding.condition.name.text) {
                                missing.push((
                                    binding.condition.name.text.clone(),
                                    "bool".to_string(),
                                    crate::diagnostics::range_for_span(
                                        source,
                                        binding.condition.name.span,
                                    ),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
                collect_missing_refs(&element.children, known_names, missing, source);
            }
            frame_core::UiNode::Loop(loop_node) => {
                let mut loop_names = known_names.clone();
                loop_names.insert(loop_node.item.text.clone());
                collect_missing_refs(&loop_node.children, &loop_names, missing, source);
            }
            _ => {}
        }
    }
}

fn infer_state_type_from_usage(name: &str) -> String {
    match name {
        "sending" | "active" | "selected" | "invalid" | "disabled" | "checked" | "loggedIn"
        | "compact" | "collapsed" | "hidden" | "open" => "bool".to_string(),
        "count" | "attempts" | "unreadCount" => "number".to_string(),
        "messages" | "items" | "channels" | "events" | "invoices" | "groups" => "list".to_string(),
        _ => "text".to_string(),
    }
}

fn missing_prop_references(source: &str) -> Vec<(String, String, Range)> {
    let Ok(document) = parse(source) else {
        return Vec::new();
    };
    let mut missing = Vec::new();
    for component in &document.components {
        let mut known_props = std::collections::HashSet::new();
        if let Some(props) = &component.props {
            for value in &props.values {
                known_props.insert(value.name.text.clone());
            }
        }
        if let Some(view) = &component.view {
            collect_missing_props(&view.nodes, &known_props, &mut missing, source);
        }
    }
    missing
}

fn collect_missing_props(
    nodes: &[frame_core::UiNode],
    known_props: &std::collections::HashSet<String>,
    missing: &mut Vec<(String, String, Range)>,
    source: &str,
) {
    for node in nodes {
        match node {
            frame_core::UiNode::Component(invocation) => {
                for arg in &invocation.arguments {
                    match &arg.value {
                        frame_core::UiComponentArgumentValue::Data(data_ref)
                        | frame_core::UiComponentArgumentValue::Bind(data_ref) => {
                            if !known_props.contains(&data_ref.name.text) {
                                missing.push((
                                    data_ref.name.text.clone(),
                                    infer_state_type_from_usage(&data_ref.name.text),
                                    crate::diagnostics::range_for_span(source, data_ref.name.span),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
            frame_core::UiNode::Element(element) => {
                collect_missing_props(&element.children, known_props, missing, source);
            }
            frame_core::UiNode::Loop(loop_node) => {
                collect_missing_props(&loop_node.children, known_props, missing, source);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::Url;

    use super::*;

    #[test]
    fn offers_typo_fix_and_grid_creation() {
        let source = "area Sidebar {\n  in Dashboard\n  surface pannel\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);
        let titles = actions
            .iter()
            .filter_map(|action| match action {
                CodeActionOrCommand::CodeAction(action) => Some(action.title.as_str()),
                CodeActionOrCommand::Command(_) => None,
            })
            .collect::<Vec<_>>();

        assert!(titles.contains(&"Replace with `surface panel`"));
        assert!(titles.contains(&"Create grid"));
        assert!(titles.contains(&"Add `place` for area"));
    }

    #[test]
    fn offers_create_matching_area_blocks() {
        let source = "grid Dashboard {\n  columns sidebar content inspector\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);

        assert!(actions.iter().any(|action| matches!(
            action,
            CodeActionOrCommand::CodeAction(action)
                if action.title == "Create matching area blocks"
        )));
    }

    #[test]
    fn offers_native_replacements_for_common_advanced_css() {
        let source = "button ChannelButton {\n  advanced {\n    css \"appearance\" none\n    css \"cursor\" pointer\n    css \"overflow-y\" auto\n    css \"border-right\" 1px solid var(--frame-color-terminal-border)\n  }\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);
        let titles = actions
            .iter()
            .filter_map(|action| match action {
                CodeActionOrCommand::CodeAction(action) => Some(action.title.as_str()),
                CodeActionOrCommand::Command(_) => None,
            })
            .collect::<Vec<_>>();

        assert!(titles.contains(&"Replace advanced CSS with `control reset`"));
        assert!(titles.contains(&"Replace advanced CSS with `interactive`"));
        assert!(titles.contains(&"Replace advanced CSS with `scroll y`"));
        assert!(titles.contains(&"Replace advanced CSS with `border right terminal-border`"));
    }

    #[test]
    fn offers_browser_to_semantic_ui_migrations() {
        let source = "component Demo {\n  view {\n    button Send:PrimaryAction {\n      onclick @sendMessage\n      onchange @changeMessage\n    }\n    div Sidebar {\n    }\n  }\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);
        let titles = actions
            .iter()
            .filter_map(|action| match action {
                CodeActionOrCommand::CodeAction(action) => Some(action.title.as_str()),
                CodeActionOrCommand::Command(_) => None,
            })
            .collect::<Vec<_>>();

        assert!(titles.contains(&"Convert `button` to `action`"));
        assert!(titles.contains(&"Convert `div` to `panel`"));
        assert!(titles.contains(&"Replace browser event with `on press`"));
        assert!(titles.contains(&"Replace browser event with `on change`"));
        assert!(titles.contains(&"Create style `PrimaryAction`"));
    }

    #[test]
    fn offers_create_missing_handler() {
        let source = "component ChatInput {\n  view {\n    action Send {\n      on press @sendMessage\n    }\n  }\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);
        let titles = actions
            .iter()
            .filter_map(|action| match action {
                CodeActionOrCommand::CodeAction(action) => Some(action.title.as_str()),
                CodeActionOrCommand::Command(_) => None,
            })
            .collect::<Vec<_>>();

        assert!(titles.contains(&"Create handler `sendMessage`"));
    }

    #[test]
    fn offers_create_missing_state() {
        let source = "component ChatInput {\n  view {\n    text $draft\n  }\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);
        let titles = actions
            .iter()
            .filter_map(|action| match action {
                CodeActionOrCommand::CodeAction(action) => Some(action.title.as_str()),
                CodeActionOrCommand::Command(_) => None,
            })
            .collect::<Vec<_>>();

        assert!(titles.contains(&"Create state `draft`"));
    }

    #[test]
    fn offers_create_missing_style_for_conditional_alias() {
        let source = "component ChatInput {\n  view {\n    action Send {\n      style LoadingButton when $sending\n    }\n  }\n}\n";
        let uri = Url::parse("file:///demo.frame").unwrap();
        let actions = code_actions_for_source(source, &uri);
        let titles = actions
            .iter()
            .filter_map(|action| match action {
                CodeActionOrCommand::CodeAction(action) => Some(action.title.as_str()),
                CodeActionOrCommand::Command(_) => None,
            })
            .collect::<Vec<_>>();

        assert!(titles.contains(&"Create style `LoadingButton`"));
    }
}
