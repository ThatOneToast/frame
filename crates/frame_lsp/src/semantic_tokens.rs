use frame_core::{knowledge, tokens};
use tower_lsp::lsp_types::{SemanticToken, SemanticTokens};

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
                6,
            ));
            continue;
        }

        let first = words[0].trim_end_matches('{');
        if first == "#include" {
            push_word(line, line_index, first, 0, &mut raw);
            if let Some(target) = words.get(1) {
                push_word(line, line_index, target, 4, &mut raw);
            }
        } else if first == "supports" {
            push_word(line, line_index, first, 0, &mut raw);
            for value in words.iter().skip(1) {
                push_word(line, line_index, value.trim_end_matches('{'), 3, &mut raw);
            }
        } else if first == "style-group" {
            push_word(line, line_index, first, 0, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(line, line_index, name.trim_end_matches('{'), 1, &mut raw);
            }
        } else if first == "style-order" {
            push_word(line, line_index, first, 0, &mut raw);
            for value in words.iter().skip(1) {
                push_word(
                    line,
                    line_index,
                    value.trim_end_matches(',').trim_end_matches('{'),
                    3,
                    &mut raw,
                );
            }
        } else if knowledge::declaration_keywords().contains(&first) {
            push_word(line, line_index, first, 0, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(line, line_index, name.trim_end_matches('{'), 1, &mut raw);
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
            push_word(line, line_index, first, 0, &mut raw);
        } else if matches!(first, "below" | "above" | "between" | "container") {
            push_word(line, line_index, first, 0, &mut raw);
            for value in words.iter().skip(1) {
                push_word(line, line_index, value.trim_end_matches('{'), 3, &mut raw);
            }
        } else if first == "animation" && words.get(1).is_some_and(|word| word.ends_with('{')) {
            push_word(line, line_index, first, 2, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(line, line_index, name.trim_end_matches('{'), 1, &mut raw);
            }
        } else if knowledge::property_keywords().contains(&first) {
            push_word(line, line_index, first, 2, &mut raw);
            for value in words.iter().skip(1) {
                let value = value.trim_end_matches('{');
                let token_type =
                    if value.ends_with('%') || value.chars().all(|c| c.is_ascii_digit()) {
                        5
                    } else if value.starts_with('#') {
                        4
                    } else if is_known_value(value) {
                        3
                    } else {
                        4
                    };
                push_word(line, line_index, value, token_type, &mut raw);
            }
        } else if tokens::EFFECTS.contains(&first) {
            push_word(line, line_index, first, 2, &mut raw);
            for value in words.iter().skip(1) {
                push_word(line, line_index, value, 3, &mut raw);
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

fn is_known_value(value: &str) -> bool {
    tokens::COLORS.contains(&value)
        || tokens::SURFACES.contains(&value)
        || tokens::SPACING.contains(&value)
        || tokens::MOVEMENT_AMOUNTS.contains(&value)
        || tokens::VISUAL_AMOUNTS.contains(&value)
        || tokens::RADII.contains(&value)
        || tokens::SHADOWS.contains(&value)
        || tokens::ALIGN.contains(&value)
        || tokens::JUSTIFY.contains(&value)
        || tokens::POSITIONS.contains(&value)
        || tokens::ANCHORS.contains(&value)
        || tokens::EDGES.contains(&value)
        || tokens::GRADIENT_TYPES.contains(&value)
        || tokens::GRADIENT_CORNERS.contains(&value)
        || tokens::TRANSITIONS.contains(&value)
        || tokens::DURATIONS.contains(&value)
        || tokens::EASES.contains(&value)
        || tokens::ANIMATIONS.contains(&value)
        || tokens::BREAKPOINTS.contains(&value)
        || tokens::CONTAINERS.contains(&value)
        || tokens::ANIMATION_FILLS.contains(&value)
        || tokens::ANIMATION_DIRECTIONS.contains(&value)
        || tokens::ANIMATION_PLAY_STATES.contains(&value)
        || tokens::BORDER_STYLES.contains(&value)
        || tokens::Z_LAYERS.contains(&value)
        || tokens::DISPLAY.contains(&value)
        || tokens::VISIBILITY.contains(&value)
        || tokens::FLEX_DIRECTIONS.contains(&value)
        || tokens::FLEX_WRAPS.contains(&value)
        || tokens::TEXT_CASES.contains(&value)
        || tokens::TEXT_ALIGN.contains(&value)
        || tokens::TEXT_DECORATIONS.contains(&value)
        || tokens::WHITE_SPACE.contains(&value)
        || tokens::WORD_BREAKS.contains(&value)
        || tokens::HYPHENS.contains(&value)
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
        || tokens::BORDER_LINE_STYLES.contains(&value)
        || is_tuned_amount(value)
}

fn is_tuned_amount(value: &str) -> bool {
    let Some((amount, percent)) = value.split_once('%') else {
        return false;
    };
    (tokens::MOVEMENT_AMOUNTS.contains(&amount) || tokens::VISUAL_AMOUNTS.contains(&amount))
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
        assert!(tokens.data.iter().any(|token| token.token_type == 5));
    }

    #[test]
    fn emits_tokens_for_display_flex_and_logical_sizing() {
        let tokens = semantic_tokens(
            "card Panel {\n  display flex\n  flex direction column\n  inline-size fill\n}\n",
        );

        assert!(tokens.data.iter().any(|token| token.token_type == 2));
        assert!(tokens.data.iter().any(|token| token.token_type == 3));
    }

    #[test]
    fn emits_tokens_for_expanded_typography_values() {
        let tokens = semantic_tokens(
            "text Body {\n  decoration underline\n  whitespace pre-wrap\n  word-break break-word\n  hyphenate auto\n}\n",
        );

        assert!(tokens.data.iter().any(|token| token.token_type == 2));
        assert!(tokens.data.iter().any(|token| token.token_type == 3));
    }

    #[test]
    fn emits_tokens_for_border_styles_and_outline_offsets() {
        let tokens =
            semantic_tokens("card Panel {\n  border style dashed\n  outline offset small\n}\n");

        assert!(tokens.data.iter().any(|token| token.token_type == 2));
        assert!(tokens.data.iter().any(|token| token.token_type == 3));
    }

    #[test]
    fn emits_tokens_for_expanded_interaction_states() {
        let tokens = semantic_tokens(
            "button Field {\n  focus-visible {\n    ring accent\n  }\n  invalid {\n    ring danger\n  }\n}\n",
        );

        assert!(tokens.data.iter().any(|token| token.token_type == 0));
        assert!(tokens.data.iter().any(|token| token.token_type == 2));
    }

    #[test]
    fn emits_tokens_for_tuned_motion_amounts() {
        let tokens = semantic_tokens(
            "card Floating {\n  lift small%44\n  shift up medium\n  tilt right subtle%23\n}\n",
        );

        assert!(tokens.data.iter().any(|token| token.token_type == 2));
        assert!(tokens.data.iter().any(|token| token.token_type == 3));
    }

    #[test]
    fn emits_tokens_for_supports_predicates() {
        let tokens = semantic_tokens("supports display grid {\n  grid AppShell {\n  }\n}\n");

        assert!(tokens.data.iter().any(|token| token.token_type == 0));
        assert!(tokens.data.iter().any(|token| token.token_type == 3));
    }

    #[test]
    fn emits_tokens_for_style_groups() {
        let tokens = semantic_tokens(
            "style-order reset, base, components\nstyle-group components {\n  button Primary {\n  }\n}\n",
        );

        assert!(tokens.data.iter().any(|token| token.token_type == 0));
        assert!(tokens.data.iter().any(|token| token.token_type == 1));
        assert!(tokens.data.iter().any(|token| token.token_type == 3));
    }
}
