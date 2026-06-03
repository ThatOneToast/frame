use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};

pub fn folding_ranges(source: &str) -> Vec<FoldingRange> {
    let mut ranges = Vec::new();
    let mut stack: Vec<(u32, String)> = Vec::new();

    for (line_index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.ends_with('{') {
            stack.push((
                line_index as u32,
                trimmed.trim_end_matches('{').trim().to_string(),
            ));
        } else if trimmed == "}" {
            if let Some((start_line, _header)) = stack.pop() {
                if line_index as u32 > start_line {
                    ranges.push(FoldingRange {
                        start_line,
                        start_character: None,
                        end_line: line_index as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: None,
                    });
                }
            }
        }
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_declarations_and_state_blocks() {
        let ranges = folding_ranges("card A {\n  hover {\n    lift small\n  }\n}\n");

        assert_eq!(ranges.len(), 2);
    }
}
