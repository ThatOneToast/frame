use std::path::Path;

use frame_codegen::{generate_css, generate_typescript};
use frame_core::semantic::validate;

use crate::diagnostics::{has_error_diagnostics, print_diagnostics};
use crate::include::load_frame_document;

pub fn compile_file(
    file: &Path,
    out: &Path,
    includes: &[std::path::PathBuf],
) -> anyhow::Result<()> {
    let document = compile_file_document(file, includes)?;

    std::fs::create_dir_all(out)?;
    std::fs::write(out.join("generated.css"), generate_css(&document))?;
    std::fs::write(out.join("generated.ts"), generate_typescript(&document))?;
    println!("generated {}", out.display());
    Ok(())
}

pub fn compile_file_document(
    file: &Path,
    includes: &[std::path::PathBuf],
) -> anyhow::Result<frame_core::Document> {
    let mut stack = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let document = load_frame_document(file, includes, &mut stack, &mut seen)?;
    let diagnostics = validate(&document);

    if !diagnostics.is_empty() {
        print_diagnostics(&diagnostics);
    }

    if has_error_diagnostics(&diagnostics) {
        anyhow::bail!("Frame compile failed");
    }

    Ok(document)
}

pub fn compile_source(source: &str) -> anyhow::Result<frame_core::Document> {
    let document = match frame_parser::parse(source) {
        Ok(document) => document,
        Err(error) => {
            print_diagnostics(&error.diagnostics);
            anyhow::bail!("Frame compile failed");
        }
    };
    let diagnostics = validate(&document);

    if !diagnostics.is_empty() {
        print_diagnostics(&diagnostics);
    }

    if has_error_diagnostics(&diagnostics) {
        anyhow::bail!("Frame compile failed");
    }

    Ok(document)
}
