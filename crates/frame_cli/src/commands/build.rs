use std::path::{Path, PathBuf};

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

    let theme_path = crate::theme::resolve_theme_file(&entry_path);
    if let Some(ref theme) = theme_path {
        println!("theme: {}", theme.display());
    } else {
        println!("theme: none");
    }

    let document = crate::commands::compile::compile_file_document(&entry_path, &[])?;
    std::fs::create_dir_all(&out_path)?;
    let mut outputs = vec![
        write_generated_file(
            out_path.join("generated.css"),
            frame_codegen::generate_css(&document),
        )?,
        write_generated_file(
            out_path.join("generated.ts"),
            frame_codegen::generate_typescript(&document),
        )?,
        write_generated_file(
            out_path.join("app.ir.json"),
            frame_codegen::generate_ir_json(&document)?,
        )?,
        write_generated_file(
            out_path.join("app.ir.ts"),
            generated_header(
                entry,
                "Generated typed Frame IR. Do not edit; regenerate with `frame build`.",
                true,
            ) + &frame_codegen::generate_ir_typescript(&document)?,
        )?,
        write_generated_file(
            out_path.join("frame.types.ts"),
            generated_header(
                entry,
                "Generated TypeScript contracts. Do not edit; regenerate with `frame build`.",
                true,
            ) + &frame_codegen::generate_contracts(&document),
        )?,
    ];
    let skeleton_path = out_path.join("frame.handlers.ts");
    outputs.push(write_handler_skeletons(
        skeleton_path,
        generated_header(
            entry,
            "Generated handler skeletons. Safe to copy into src/handlers.ts; `frame build` only appends missing stubs.",
            false,
        ) + &frame_codegen::generate_skeletons(&document),
    )?);
    println!("Frame build complete.");
    println!("  source: {}", entry_path.display());
    println!("  output: {}", out_path.display());
    for output in outputs {
        println!("  {} {}", output.action.label(), output.path.display());
        if output.action == FileAction::Appended {
            println!("    appended missing handler stubs without rewriting existing content");
        }
    }
    println!("  warnings: 0");
    Ok(())
}

use anyhow::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileAction {
    Created,
    Updated,
    Unchanged,
    Appended,
}

impl FileAction {
    fn label(self) -> &'static str {
        match self {
            FileAction::Created => "created",
            FileAction::Updated => "updated",
            FileAction::Unchanged => "unchanged",
            FileAction::Appended => "appended",
        }
    }
}

#[derive(Debug)]
struct GeneratedFileStatus {
    path: PathBuf,
    action: FileAction,
}

fn write_generated_file(path: PathBuf, contents: String) -> anyhow::Result<GeneratedFileStatus> {
    let action = if !path.exists() {
        std::fs::write(&path, contents)?;
        FileAction::Created
    } else {
        let existing = std::fs::read_to_string(&path)?;
        if existing == contents {
            FileAction::Unchanged
        } else {
            std::fs::write(&path, contents)?;
            FileAction::Updated
        }
    };

    Ok(GeneratedFileStatus { path, action })
}

fn write_handler_skeletons(path: PathBuf, contents: String) -> anyhow::Result<GeneratedFileStatus> {
    if !path.exists() {
        std::fs::write(&path, contents)?;
        return Ok(GeneratedFileStatus {
            path,
            action: FileAction::Created,
        });
    }

    let existing = std::fs::read_to_string(&path)?;
    let mut updated = existing.clone();
    if !updated.starts_with("// Generated handler skeletons.") {
        updated = format!("{}{}", generated_file_header(&contents), updated);
    }
    if !updated.contains("from './frame.types'") {
        updated = insert_after_generated_header(&updated, generated_type_import(&contents));
    }

    let mut appended = false;
    for (name, block) in handler_skeleton_blocks(&contents) {
        if !existing.contains(&format!("export function {name}(")) {
            if !updated.ends_with('\n') {
                updated.push('\n');
            }
            updated.push('\n');
            updated.push_str(&block);
            if !updated.ends_with('\n') {
                updated.push('\n');
            }
            appended = true;
        }
    }

    if updated == existing {
        Ok(GeneratedFileStatus {
            path,
            action: FileAction::Unchanged,
        })
    } else {
        std::fs::write(&path, updated)?;
        Ok(GeneratedFileStatus {
            path,
            action: if appended {
                FileAction::Appended
            } else {
                FileAction::Updated
            },
        })
    }
}

fn handler_skeleton_blocks(contents: &str) -> Vec<(String, String)> {
    contents
        .split("\nexport function ")
        .skip(1)
        .filter_map(|block| {
            let name = block.split_once('(')?.0.to_string();
            Some((name, format!("export function {block}")))
        })
        .collect()
}

fn generated_file_header(contents: &str) -> &str {
    contents.split_once("\n\n").map_or("", |(header, _)| {
        contents
            .get(..header.len() + 2)
            .expect("header split should be valid utf-8 boundary")
    })
}

fn generated_type_import(contents: &str) -> &str {
    contents
        .lines()
        .find(|line| line.starts_with("import type "))
        .unwrap_or("")
}

fn insert_after_generated_header(contents: &str, insert: &str) -> String {
    if insert.is_empty() {
        return contents.to_string();
    }
    if let Some((header, rest)) = contents.split_once("\n\n") {
        format!("{header}\n\n{insert}\n\n{rest}")
    } else {
        format!("{insert}\n\n{contents}")
    }
}

fn generated_header(entry: &str, summary: &str, generated_only: bool) -> String {
    format!(
        "// {summary}\n// Source: {entry}\n// Ownership: {}\n\n",
        if generated_only {
            "generated-only"
        } else {
            "generated reference; user code belongs in src/handlers.ts"
        }
    )
}
