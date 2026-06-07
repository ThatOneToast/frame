use std::path::Path;

use anyhow::Context;
use frame_core::formatting::format_source;

pub fn format_file(file: &Path, check: bool) -> anyhow::Result<()> {
    let source = std::fs::read_to_string(file)
        .with_context(|| format!("failed to read {}", file.display()))?;
    let formatted = format_source(&source);

    if check {
        if formatted == source {
            println!("formatted");
            Ok(())
        } else {
            anyhow::bail!("Frame format check failed: {}", file.display());
        }
    } else {
        std::fs::write(file, formatted)?;
        println!("formatted {}", file.display());
        Ok(())
    }
}
