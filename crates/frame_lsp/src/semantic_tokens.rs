use frame_core::language;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokens};

// Token type indices matching the SemanticTokensLegend in backend.rs:
// 0 = KEYWORD, 1 = CLASS, 2 = PROPERTY, 3 = ENUM_MEMBER,
// 4 = VARIABLE, 5 = NUMBER, 6 = COMMENT
const TOKEN_KEYWORD: u32 = 0;
const TOKEN_CLASS: u32 = 1;
const TOKEN_PROPERTY: u32 = 2;
const TOKEN_ENUM_MEMBER: u32 = 3;
const TOKEN_VARIABLE: u32 = 4;
const TOKEN_NUMBER: u32 = 5;
const TOKEN_COMMENT: u32 = 6;

pub fn semantic_tokens(source: &str) -> SemanticTokens {
    let mut raw = Vec::new();
    for (line_index, line) in source.lines().enumerate() {
        let trimmed_start = line.len() - line.trim_start().len();
        let words = line.split_whitespace().collect::<Vec<_>>();
        if words.is_empty() {
            continue;
        }

        if line.trim_start().starts_with("//") {
            raw.push((
                line_index as u32,
                trimmed_start as u32,
                line.trim().len() as u32,
                TOKEN_COMMENT,
            ));
            continue;
        }

        let first = words[0].trim_end_matches('{');
        if first == "#include" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(target) = words.get(1) {
                push_word(line, line_index, target, TOKEN_VARIABLE, &mut raw);
            }
        } else if first == "component" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(
                    line,
                    line_index,
                    name.trim_end_matches('{'),
                    TOKEN_CLASS,
                    &mut raw,
                );
            }
        } else if first == "props" || first == "state" || first == "view" || first == "slot" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
        } else if language::is_ui_primitive(first)
            && words.get(1).is_some_and(|word| word.ends_with('{'))
        {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(name) = words.get(1) {
                let name = name.trim_end_matches('{');
                if let Some((node, style)) = name.split_once(':') {
                    push_word(line, line_index, node, TOKEN_CLASS, &mut raw);
                    push_word(line, line_index, style, TOKEN_ENUM_MEMBER, &mut raw);
                } else {
                    push_word(line, line_index, name, TOKEN_CLASS, &mut raw);
                }
            }
        } else if words.get(2).copied() == Some("=")
            && matches!(
                words.get(1).copied(),
                Some("text" | "string" | "bool" | "number" | "list")
            )
        {
            push_word(line, line_index, first, TOKEN_CLASS, &mut raw);
            if let Some(state_type) = words.get(1) {
                push_word(line, line_index, state_type, TOKEN_ENUM_MEMBER, &mut raw);
            }
            if let Some(value) = words.get(3) {
                let token_type =
                    if matches!(*value, "true" | "false") || value.parse::<f64>().is_ok() {
                        TOKEN_NUMBER
                    } else {
                        TOKEN_VARIABLE
                    };
                push_word(line, line_index, value, token_type, &mut raw);
            }
        } else if first == "on" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(event) = words.get(1) {
                for part in event.split('.') {
                    push_word(line, line_index, part, TOKEN_ENUM_MEMBER, &mut raw);
                }
            }
            if let Some(handler) = words.get(2) {
                push_word(line, line_index, handler, TOKEN_VARIABLE, &mut raw);
            }
        } else if looks_like_component_invocation(line.trim()) {
            let trimmed = line.trim();
            if let Some(open_paren) = trimmed.find('(') {
                let name = &trimmed[..open_paren];
                push_word(line, line_index, name, TOKEN_CLASS, &mut raw);
                for word in words.iter().skip(1) {
                    let value = word
                        .trim_end_matches(',')
                        .trim_end_matches(')')
                        .trim_end_matches(':');
                    if value == "bind" {
                        push_word(line, line_index, value, TOKEN_KEYWORD, &mut raw);
                    } else if value.starts_with('$') {
                        push_word(line, line_index, value, TOKEN_VARIABLE, &mut raw);
                    } else if !value.is_empty() {
                        push_word(line, line_index, value, TOKEN_ENUM_MEMBER, &mut raw);
                    }
                }
            }
        } else if first == "for" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(item) = words.get(1) {
                push_word(line, line_index, item, TOKEN_CLASS, &mut raw);
            }
            if let Some(in_word) = words.get(2) {
                push_word(line, line_index, in_word, TOKEN_KEYWORD, &mut raw);
            }
            if let Some(collection) = words.get(3) {
                push_word(line, line_index, collection, TOKEN_VARIABLE, &mut raw);
            }
            if let Some(key_word) = words.get(4) {
                push_word(line, line_index, key_word, TOKEN_KEYWORD, &mut raw);
            }
            if let Some(key) = words.get(5) {
                push_word(line, line_index, key, TOKEN_VARIABLE, &mut raw);
            }
        } else if language::is_ui_primitive(first) && words.get(1).is_some() {
            // Shorthand UI element without a block: e.g. "action Send"
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(name) = words.get(1) {
                let name = name.trim_end_matches('{');
                if name.starts_with('"') || name.starts_with('$') {
                    push_word(line, line_index, name, TOKEN_VARIABLE, &mut raw);
                } else if let Some((node, style)) = name.split_once(':') {
                    push_word(line, line_index, node, TOKEN_CLASS, &mut raw);
                    push_word(line, line_index, style, TOKEN_ENUM_MEMBER, &mut raw);
                } else {
                    push_word(line, line_index, name, TOKEN_CLASS, &mut raw);
                }
            }
        } else if matches!(
            first,
            "value"
                | "bind"
                | "draft"
                | "send"
                | "source"
                | "goto"
                | "label"
                | "description"
                | "hint"
                | "options"
                | "placeholder"
                | "disabled"
                | "readonly"
                | "style"
                | "show"
                | "hidden"
                | "checked"
                | "selected"
                | "alt"
                | "decorative"
                | "new-window"
        ) {
            push_word(line, line_index, first, TOKEN_PROPERTY, &mut raw);
            for value in words.iter().skip(1) {
                let value = value.trim_end_matches('{');
                let token_type = if matches!(value, "bind" | "when") {
                    TOKEN_KEYWORD
                } else if value.starts_with('$') || value.starts_with('@') {
                    TOKEN_VARIABLE
                } else if value.parse::<f64>().is_ok() {
                    TOKEN_NUMBER
                } else {
                    TOKEN_ENUM_MEMBER
                };
                push_word(
                    line,
                    line_index,
                    value.trim_start_matches('='),
                    token_type,
                    &mut raw,
                );
            }
        } else if first == "supports" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            for value in words.iter().skip(1) {
                push_word(
                    line,
                    line_index,
                    value.trim_end_matches('{'),
                    TOKEN_ENUM_MEMBER,
                    &mut raw,
                );
            }
        } else if first == "style-group" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(
                    line,
                    line_index,
                    name.trim_end_matches('{'),
                    TOKEN_CLASS,
                    &mut raw,
                );
            }
        } else if first == "style-order" {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            for value in words.iter().skip(1) {
                push_word(
                    line,
                    line_index,
                    value.trim_end_matches(',').trim_end_matches('{'),
                    TOKEN_ENUM_MEMBER,
                    &mut raw,
                );
            }
        } else if language::declaration_keywords().contains(&first) {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(
                    line,
                    line_index,
                    name.trim_end_matches('{'),
                    TOKEN_CLASS,
                    &mut raw,
                );
            }
        } else if matches!(
            first,
            "hover"
                | "focus"
                | "focus-visible"
                | "focus-within"
                | "active"
                | "disabled"
                | "checked"
                | "invalid"
                | "required"
                | "target"
                | "from"
                | "to"
        ) || first.ends_with('%')
        {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
        } else if matches!(first, "below" | "above" | "between" | "container") {
            push_word(line, line_index, first, TOKEN_KEYWORD, &mut raw);
            for value in words.iter().skip(1) {
                push_word(
                    line,
                    line_index,
                    value.trim_end_matches('{'),
                    TOKEN_ENUM_MEMBER,
                    &mut raw,
                );
            }
        } else if first == "animation" && words.get(1).is_some_and(|word| word.ends_with('{')) {
            push_word(line, line_index, first, TOKEN_PROPERTY, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(
                    line,
                    line_index,
                    name.trim_end_matches('{'),
                    TOKEN_CLASS,
                    &mut raw,
                );
            }
        } else if language::property_keywords().contains(&first) {
            push_word(line, line_index, first, TOKEN_PROPERTY, &mut raw);
            for value in words.iter().skip(1) {
                let value = value.trim_end_matches('{');
                let token_type =
                    if value.ends_with('%') || value.chars().all(|c| c.is_ascii_digit()) {
                        TOKEN_NUMBER
                    } else if value.starts_with('#') {
                        TOKEN_VARIABLE
                    } else if is_known_value(value) {
                        TOKEN_ENUM_MEMBER
                    } else {
                        TOKEN_VARIABLE
                    };
                push_word(line, line_index, value, token_type, &mut raw);
            }
        } else if language::effect_keywords().contains(&first) {
            push_word(line, line_index, first, TOKEN_PROPERTY, &mut raw);
            for value in words.iter().skip(1) {
                push_word(line, line_index, value, TOKEN_ENUM_MEMBER, &mut raw);
            }
        }
    }

    raw.sort_by_key(|(line, start, _, _)| (*line, *start));

    let mut data = Vec::new();
    let mut previous_line = 0u32;
    let mut previous_start = 0u32;
    for (line, start, length, token_type) in raw {
        let delta_line = line - previous_line;
        let delta_start = if delta_line == 0 {
            start - previous_start
        } else {
            start
        };
        data.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: 0,
        });
        previous_line = line;
        previous_start = start;
    }

    SemanticTokens {
        result_id: None,
        data,
    }
}

fn looks_like_component_invocation(content: &str) -> bool {
    let Some(open_paren) = content.find('(') else {
        return false;
    };
    content.ends_with(')')
        && content[..open_paren]
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_uppercase())
}

fn is_known_value(value: &str) -> bool {
    language::is_known_value(value)
        || matches!(
            value,
            "direction"
                | "wrap"
                | "grow"
                | "shrink"
                | "basis"
                | "row-reverse"
                | "column-reverse"
                | "style"
                | "offset"
                | "up"
                | "down"
                | "backdrop"
                | "selector"
                | "queries"
                | "oklch"
                | "has"
                | "subgrid"
        )
        || language::BORDER_LINE_STYLES.contains(&value)
        || is_tuned_amount(value)
}

fn is_tuned_amount(value: &str) -> bool {
    let Some((amount, percent)) = value.split_once('%') else {
        return false;
    };
    (language::MOVEMENT_AMOUNTS.contains(&amount) || language::VISUAL_AMOUNTS.contains(&amount))
        && !percent.is_empty()
        && percent.chars().all(|character| character.is_ascii_digit())
}

fn push_word(
    line: &str,
    line_index: usize,
    word: &str,
    token_type: u32,
    raw: &mut Vec<(u32, u32, u32, u32)>,
) {
    if word.is_empty() {
        return;
    }
    let Some(start) = line.find(word) else {
        return;
    };
    raw.push((
        line_index as u32,
        start as u32,
        word.chars().count() as u32,
        token_type,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_tokens_for_declarations_properties_and_values() {
        let tokens = semantic_tokens("grid Dashboard {\n  columns sidebar 25%\n}\n");

        assert!(!tokens.data.is_empty());
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_NUMBER));
    }

    #[test]
    fn emits_tokens_for_initial_ui_syntax() {
        let tokens = semantic_tokens(
            "component ChatInput {\n  state {\n    draft text = \"\"\n  }\n  view {\n    MessageComposer(draft bind $draft)\n    button Send:PrimaryButton {\n      value bind $draft\n      on keydown.ctrl.enter @sendMessage\n    }\n  }\n}\n",
        );

        assert!(!tokens.data.is_empty());
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_CLASS));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_VARIABLE));
    }

    #[test]
    fn emits_tokens_for_display_flex_and_logical_sizing() {
        let tokens = semantic_tokens(
            "card Panel {\n  display flex\n  flex direction column\n  inline-size fill\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER));
    }

    #[test]
    fn emits_tokens_for_expanded_typography_values() {
        let tokens = semantic_tokens(
            "text Body {\n  decoration underline\n  whitespace pre-wrap\n  word-break break-word\n  hyphenate auto\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER));
    }

    #[test]
    fn emits_tokens_for_border_styles_and_outline_offsets() {
        let tokens =
            semantic_tokens("card Panel {\n  border style dashed\n  outline offset small\n}\n");

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER));
    }

    #[test]
    fn emits_tokens_for_expanded_interaction_states() {
        let tokens = semantic_tokens(
            "button Field {\n  focus-visible {\n    ring accent\n  }\n  invalid {\n    ring danger\n  }\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY));
    }

    #[test]
    fn emits_tokens_for_tuned_motion_amounts() {
        let tokens = semantic_tokens(
            "card Floating {\n  lift small%44\n  shift up medium\n  tilt right subtle%23\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER));
    }

    #[test]
    fn emits_tokens_for_supports_predicates() {
        let tokens = semantic_tokens("supports display grid {\n  grid AppShell {\n  }\n}\n");

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER));
    }

    #[test]
    fn emits_tokens_for_style_groups() {
        let tokens = semantic_tokens(
            "style-order reset, base, components\nstyle-group components {\n  button Primary {\n  }\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_CLASS));
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER));
    }

    #[test]
    fn emits_tokens_for_keyed_loops_and_conditional_styles() {
        let tokens = semantic_tokens(
            "component ChatInput {\n  view {\n    list Messages {\n      for message in $messages key $message.id {\n        item Message {\n          text $message.body\n        }\n      }\n      action Send {\n        style LoadingButton when $sending\n      }\n    }\n  }\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD && token.delta_start == 6)); // "for"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_VARIABLE && token.length > 1)); // $messages
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY)); // "style"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER && token.length == 13)); // "LoadingButton"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD && token.length == 4)); // "when"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_VARIABLE && token.length == 8));
        // "$sending"
    }

    #[test]
    fn emits_tokens_for_conditional_style_alternate_form() {
        let tokens = semantic_tokens(
            "component ChatInput {\n  view {\n    action Send {\n      style when $sending = LoadingButton\n    }\n  }\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_PROPERTY && token.length == 5)); // "style"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD && token.length == 4)); // "when"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_VARIABLE && token.length == 8)); // "$sending"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_ENUM_MEMBER && token.length == 13));
        // "LoadingButton"
    }

    #[test]
    fn emits_tokens_for_component_invocations_and_props() {
        let tokens = semantic_tokens(
            "component ChatApp {\n  props {\n    channel text\n  }\n  view {\n    ChatPanel(channel: $activeChannel)\n    MessageComposer(draft bind $draft)\n  }\n}\n",
        );

        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_CLASS && token.length == 9)); // ChatPanel
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_KEYWORD)); // "props"
        assert!(tokens
            .data
            .iter()
            .any(|token| token.token_type == TOKEN_VARIABLE)); // $activeChannel
    }

    #[test]
    fn emits_tokens_for_all_semantic_primitives() {
        let primitives = &[
            "screen", "panel", "section", "stack", "row", "grid", "split", "dock", "overlay",
            "scroll", "action", "link", "menu", "toolbar", "tabs", "field", "input", "editor",
            "toggle", "choice", "select", "composer", "title", "text", "label", "badge", "avatar",
            "icon", "image", "media", "list", "feed", "data", "item", "empty", "card", "dialog",
            "popover",
        ];
        let source = primitives
            .iter()
            .map(|p| format!("{p} Name {{\n}}\n"))
            .collect::<String>();
        let tokens = semantic_tokens(&source);

        let primitive_tokens: Vec<_> = tokens
            .data
            .iter()
            .filter(|t| t.token_type == TOKEN_KEYWORD)
            .collect();
        assert!(
            !primitive_tokens.is_empty(),
            "semantic primitives should produce keyword tokens"
        );
    }
}
