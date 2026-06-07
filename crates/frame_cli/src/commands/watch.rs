use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

use anyhow::Context;
use notify::{EventKind, RecursiveMode, Watcher};

use crate::commands::compile::compile_file;

/// Watch all `.frame` files in the entry file's directory tree and rebuild when any change.
pub fn watch_file(file: &Path, out: &Path, includes: &[PathBuf]) -> anyhow::Result<()> {
    let theme = crate::theme::resolve_theme_file(file);

    println!("Frame watch started");
    println!("entry: {}", file.display());
    if let Some(ref t) = theme {
        println!("theme: {}", t.display());
    } else {
        println!("theme: none");
    }
    println!("output: {}", out.display());

    compile_once_for_watch(file, out, includes);

    let (tx, rx) = mpsc::channel();
    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                ) {
                    let _ = tx.send(event.paths);
                }
            }
        })?;

    // Watch the directory containing the entry file (and subdirectories) so that
    // includes, theme files, and newly added files are all observed.
    let watch_root = file.parent().unwrap_or(Path::new("."));
    watcher.watch(watch_root, RecursiveMode::Recursive)?;

    let debounce = Duration::from_millis(300);
    let mut last_build = std::time::Instant::now();

    while let Ok(paths) = rx.recv() {
        let changed = paths
            .iter()
            .filter(|p| {
                p.extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("frame"))
            })
            .collect::<Vec<_>>();

        if changed.is_empty() {
            continue;
        }

        for p in &changed {
            println!("changed: {}", p.display());
        }

        if last_build.elapsed() < debounce {
            continue;
        }
        last_build = std::time::Instant::now();
        compile_once_for_watch(file, out, includes);
    }

    Ok(())
}

/// Watch a project via `frame build --watch`.
pub fn watch_project(root: &Path) -> anyhow::Result<()> {
    let config_path = root.join("frame.config.json");
    if !config_path.exists() {
        anyhow::bail!(
            "No frame.config.json found in {}.\n\nRun `frame init` first.",
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
        .unwrap_or("src/generated");

    let entry_path = root.join(entry);
    let out_path = root.join(out_dir);

    watch_file(&entry_path, &out_path, &[])
}

fn compile_once_for_watch(file: &Path, out: &Path, includes: &[PathBuf]) {
    match compile_file(file, out, includes) {
        Ok(()) => println!("Frame compiled successfully"),
        Err(error) => eprintln!("Frame build failed\n{error:#}"),
    }
}
