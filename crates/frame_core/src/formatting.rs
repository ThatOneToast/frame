const STATE_BLOCKS: &[&str] = &[
    "hover",
    "focus",
    "focus-visible",
    "focus-within",
    "active",
    "disabled",
    "checked",
    "invalid",
    "required",
    "target",
];

pub fn format_source(source: &str) -> String {
    let mut output = Vec::new();
    let mut indent = 0usize;
    let mut previous_top_level = false;
    let mut previous_statement_in_block = false;

    for raw_line in source.lines() {
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let code = code_part(trimmed);
        let closes_block = code.starts_with('}');
        if closes_block {
            indent = indent.saturating_sub(1);
            previous_statement_in_block = false;
        }

        let top_level_declaration = indent == 0 && code.ends_with('{') && !is_state_block(code);
        if top_level_declaration
            && previous_top_level
            && output.last().is_some_and(|line: &String| !line.is_empty())
        {
            output.push(String::new());
        }

        if is_state_block(code)
            && previous_statement_in_block
            && output.last().is_some_and(|line| !line.is_empty())
        {
            output.push(String::new());
        }

        output.push(format!("{}{}", "  ".repeat(indent), trimmed));

        previous_top_level = indent == 0 && code == "}";
        previous_statement_in_block = indent > 0 && !code.ends_with('{') && code != "}";

        if code.ends_with('{') {
            indent += 1;
            previous_statement_in_block = false;
        }
    }

    if !output.is_empty() {
        output.push(String::new());
    }

    output.join("\n")
}

fn is_state_block(line: &str) -> bool {
    let Some(name) = line.strip_suffix('{').map(str::trim) else {
        return false;
    };

    STATE_BLOCKS.contains(&name)
}

fn code_part(line: &str) -> &str {
    line.split_once("//")
        .map_or(line, |(before_comment, _)| before_comment)
        .trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_declarations_and_state_blocks() {
        let source = "card QuickLinkCard {\n surface gradient dusk\n  padding large\nhover {\n lift small\n}\n}\ngrid AppShell {\ncolumns sidebar content\n}\n";

        assert_eq!(
            format_source(source),
            "card QuickLinkCard {\n  surface gradient dusk\n  padding large\n\n  hover {\n    lift small\n  }\n}\n\ngrid AppShell {\n  columns sidebar content\n}\n"
        );
    }

    #[test]
    fn preserves_comment_text() {
        let source = "card A { // comment\n// keep me\npadding small // trailing\n}\n";

        assert_eq!(
            format_source(source),
            "card A { // comment\n  // keep me\n  padding small // trailing\n}\n"
        );
    }
}
