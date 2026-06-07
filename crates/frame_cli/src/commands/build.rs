use std::path::Path;

pub fn build_project() -> anyhow::Result<()> {
    build_project_at(Path::new("."))
}

pub fn build_project_at(root: &Path) -> anyhow::Result<()> {
    let config_path = root.join("frame.config.json");
    if !config_path.exists() {
        anyhow::bail!(
            "No frame.config.json found in {}.\n\nRun `frame init` first or create a frame.config.json.",
            root.display()
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

    let entry_path = root.join(entry);
    let out_path = root.join(out_dir);

    if !entry_path.exists() {
        anyhow::bail!("Entry file `{}` does not exist.", entry);
    }

    let document = crate::commands::compile::compile_file_document(&entry_path, &[])?;
    std::fs::create_dir_all(&out_path)?;
    std::fs::write(
        out_path.join("generated.css"),
        frame_codegen::generate_css(&document),
    )?;
    std::fs::write(
        out_path.join("generated.ts"),
        frame_codegen::generate_typescript(&document),
    )?;
    std::fs::write(
        out_path.join("app.ir.json"),
        frame_codegen::generate_ir_json(&document)?,
    )?;
    std::fs::write(
        out_path.join("app.ir.ts"),
        frame_codegen::generate_ir_typescript(&document)?,
    )?;
    println!("Frame build complete.");
    Ok(())
}

use anyhow::Context;
