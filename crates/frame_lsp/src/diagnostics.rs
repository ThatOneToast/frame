use frame_core::{semantic::validate, Diagnostic as FrameDiagnostic, Severity, Span};
use frame_parser::parse;
use tower_lsp::lsp_types::{Diagnostic as LspDiagnostic, DiagnosticSeverity, Position, Range};

pub fn diagnostics_for_source(source: &str) -> Vec<FrameDiagnostic> {
    match parse(source) {
        Ok(document) => validate(&document),
        Err(error) => error.diagnostics,
    }
}

pub fn to_lsp_diagnostic(source: &str, diagnostic: FrameDiagnostic) -> LspDiagnostic {
    LspDiagnostic {
        range: range_for_span(source, diagnostic.span),
        severity: Some(match diagnostic.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
        }),
        source: Some("frame".to_string()),
        message: diagnostic.message,
        ..LspDiagnostic::default()
    }
}

pub fn range_for_span(source: &str, span: Span) -> Range {
    let start = position_for_offset(source, span.start);
    let mut end = position_for_offset(source, span.end);

    if start == end {
        end.character += 1;
    }

    Range { start, end }
}

pub fn position_for_offset(source: &str, offset: usize) -> Position {
    let mut line = 0;
    let mut character = 0;

    for (index, value) in source.char_indices() {
        if index >= offset {
            break;
        }

        if value == '\n' {
            line += 1;
            character = 0;
        } else {
            character += value.len_utf16() as u32;
        }
    }

    Position { line, character }
}

pub fn offset_for_position(source: &str, position: Position) -> usize {
    let mut line = 0;
    let mut character = 0;

    for (index, value) in source.char_indices() {
        if line == position.line && character == position.character {
            return index;
        }

        if value == '\n' {
            line += 1;
            character = 0;
        } else {
            character += value.len_utf16() as u32;
        }
    }

    source.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_byte_offsets_to_utf16_positions() {
        let source = "grid A {\n  gap médium\n}\n";
        let offset = source.find("médium").expect("sample contains value");

        assert_eq!(
            position_for_offset(source, offset),
            Position {
                line: 1,
                character: 6,
            }
        );
    }

    #[test]
    fn returns_parser_diagnostics() {
        let diagnostics = diagnostics_for_source("card Broken {\n  magic {\n  }\n}\n");

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("unknown nested block"));
    }

    #[test]
    fn returns_semantic_diagnostics() {
        let diagnostics = diagnostics_for_source(
            "grid AppShell {\n  columns sidebar\n}\narea Sidebar {\n  in Missing\n}\n",
        );

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("unknown grid"));
    }
}
