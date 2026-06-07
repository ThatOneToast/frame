use std::path::Path;

use frame_core::semantic::validate;

use crate::commands::compile::compile_file_document;
use crate::diagnostics::{has_error_diagnostics, print_diagnostics};

pub fn check_file(file: &Path, includes: &[std::path::PathBuf]) -> anyhow::Result<()> {
    let document = compile_file_document(file, includes)?;
    let diagnostics = validate(&document);

    if !diagnostics.is_empty() {
        print_diagnostics(&diagnostics);
    }

    if has_error_diagnostics(&diagnostics) {
        anyhow::bail!("Frame check failed");
    }

    println!("ok");
    Ok(())
}
