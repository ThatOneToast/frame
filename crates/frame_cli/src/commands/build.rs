use std::path::Path;

pub fn build_project() -> anyhow::Result<()> {
    let config_path = std::path::PathBuf::from("frame.config.json");
    if !config_path.exists() {
        anyhow::bail!(
            "No frame.config.json found in the current directory.\n\nRun `frame init` first or create a frame.config.json."
        );
    }

    let config = std::fs::read_to_string(&config_path)?;
    let config: serde_json::Value =
        serde_json::from_str(&config).with_context(|| "failed to parse frame.config.json")?;

    let entry = config
        .get("entry")
        .and_then(|v| v.as_str())
        .unwrap_or("src/app.frame");
    let out_dir = config
        .get("outDir")
        .and_then(|v| v.as_str())
        .unwrap_or("src/frame");

    let entry_path = Path::new(entry);
    let out_path = Path::new(out_dir);

    if !entry_path.exists() {
        anyhow::bail!("Entry file `{}` does not exist.", entry);
    }

    crate::commands::compile::compile_file(entry_path, out_path, &[])?;
    println!("Frame build complete.");
    Ok(())
}

use anyhow::Context;
