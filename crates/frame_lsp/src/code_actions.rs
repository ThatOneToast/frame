use std::collections::HashMap;

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
}
