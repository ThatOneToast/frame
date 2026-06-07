pub fn word_at(source: &str, offset: usize) -> Option<&str> {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind(|character: char| !is_word_character(character))
        .map_or(0, |index| index + 1);
    let end = source[safe_offset..]
        .find(|character: char| !is_word_character(character))
        .map_or(source.len(), |index| safe_offset + index);

    if start == end {
        None
    } else {
        Some(&source[start..end])
    }
}

fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || matches!(
            character,
            '-' | '_' | '%' | '#' | '.' | '(' | ')' | '$' | '@' | ':'
        )
}

pub fn line_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let end = source[safe_offset..]
        .find('\n')
        .map_or(source.len(), |index| safe_offset + index);

    source[start..end].trim()
}
