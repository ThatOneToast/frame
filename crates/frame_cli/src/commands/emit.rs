use std::io::Read;
use std::path::Path;

use anyhow::Context;
use frame_codegen::{generate_contracts, generate_ir_json};

use crate::commands::compile::compile_file_document;

pub fn emit_ir(
    file: &Path,
    out: Option<&Path>,
    includes: &[std::path::PathBuf],
) -> anyhow::Result<()> {
    let document = compile_file_document(file, includes)?;
    let json = generate_ir_json(&document).context("failed to serialize Frame IR")?;

    if let Some(out) = out {
        std::fs::write(out, json)?;
        println!("generated {}", out.display());
    } else {
        print!("{json}");
    }

    Ok(())
}

pub fn emit_contracts(
    file: &Path,
    out: Option<&Path>,
    includes: &[std::path::PathBuf],
) -> anyhow::Result<()> {
    let document = compile_file_document(file, includes)?;
    let contracts = generate_contracts(&document);

    if let Some(out) = out {
        std::fs::write(out, contracts)?;
        println!("generated {}", out.display());
    } else {
        print!("{contracts}");
    }

    Ok(())
}

pub fn compile_stdin(css_only: bool, filename: Option<&Path>) -> anyhow::Result<()> {
    if !css_only {
        anyhow::bail!("compile-stdin currently requires --css-only");
    }

    let mut source = String::new();
    std::io::stdin()
        .read_to_string(&mut source)
        .context("failed to read Frame source from stdin")?;

    let document = crate::commands::compile::compile_source(&source).with_context(|| {
        filename
            .map(|path| format!("failed to compile {}", path.display()))
            .unwrap_or_else(|| "failed to compile stdin".to_string())
    })?;

    print!("{}", frame_codegen::generate_css(&document));
    Ok(())
}
