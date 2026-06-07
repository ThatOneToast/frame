use std::path::Path;

use crate::commands::compile::compile_file_document;

pub fn check_file(file: &Path, includes: &[std::path::PathBuf]) -> anyhow::Result<()> {
    let theme_path = crate::theme::resolve_theme_file(file);
    if let Some(ref theme) = theme_path {
        println!("theme: {}", theme.display());
    } else {
        println!("theme: none");
    }

    // compile_file_document already runs validate and prints diagnostics
    let _document = compile_file_document(file, includes)?;

    println!("ok");
    Ok(())
}
