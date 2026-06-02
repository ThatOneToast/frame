use frame_core::formatting::format_source;

pub fn format_document(source: &str) -> String {
    format_source(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_document_text() {
        assert_eq!(
            format_document("card A {\npadding small\n}\n"),
            "card A {\n  padding small\n}\n"
        );
    }
}
