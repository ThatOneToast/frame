pub fn doctor() -> anyhow::Result<()> {
    let mut issues = Vec::new();

    if !std::path::Path::new("frame.config.json").exists() {
        issues.push("No frame.config.json found. Run `frame init` to create one.");
    }

    let has_cargo = std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !has_cargo {
        issues.push("Cargo (Rust) not found on PATH. Install Rust to build Frame tools.");
    }

    let lsp_on_path = std::process::Command::new("frame_lsp")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !lsp_on_path {
        issues.push(
            "frame_lsp not found on PATH. Install with `cargo install --path crates/frame_lsp`.",
        );
    }

    let has_vite = std::path::Path::new("vite.config.ts").exists()
        || std::path::Path::new("vite.config.js").exists();
    let has_svelte = std::path::Path::new("svelte.config.js").exists();

    if has_vite {
        println!("Vite config detected.");
    }
    if has_svelte {
        println!("Svelte config detected.");
    }

    if issues.is_empty() {
        println!("Frame doctor: everything looks good.");
    } else {
        eprintln!("Frame doctor found issues:");
        for issue in issues {
            eprintln!("  - {issue}");
        }
    }

    Ok(())
}
