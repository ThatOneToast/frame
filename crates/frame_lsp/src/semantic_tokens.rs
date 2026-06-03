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
        if knowledge::declaration_keywords().contains(&first) {
            push_word(line, line_index, first, 0, &mut raw);
            if let Some(name) = words.get(1) {
                push_word(line, line_index, name.trim_end_matches('{'), 1, &mut raw);
            }
        } else if matches!(first, "hover" | "focus" | "active" | "disabled") {
            push_word(line, line_index, first, 0, &mut raw);
        } else if knowledge::property_keywords().contains(&first) {
            push_word(line, line_index, first, 2, &mut raw);
            for value in words.iter().skip(1) {
                let value = value.trim_end_matches('{');
                let token_type =
                    if value.ends_with('%') || value.chars().all(|c| c.is_ascii_digit()) {
                        5
                    } else if tokens::COLORS.contains(&value) || tokens::SURFACES.contains(&value) {
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
}
