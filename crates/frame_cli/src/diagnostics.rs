use frame_core::{Diagnostic, Severity};

pub fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{:?} [{}..{}]: {}",
            diagnostic.severity, diagnostic.span.start, diagnostic.span.end, diagnostic.message
        );
    }
}

pub fn has_error_diagnostics(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
}
