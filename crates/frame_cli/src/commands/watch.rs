use std::{
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

use anyhow::Context;

use crate::commands::compile::compile_file;

pub fn watch_file(file: &Path, out: &Path, includes: &[std::path::PathBuf]) -> anyhow::Result<()> {
    println!("watching {}", file.display());
    compile_once_for_watch(file, out, includes);

    let mut last_modified = modified_time(file)?;
    loop {
        thread::sleep(Duration::from_millis(500));
        let modified = match modified_time(file) {
            Ok(modified) => modified,
            Err(error) => {
                eprintln!("watch error: {error:#}");
                continue;
            }
        };

        if modified > last_modified {
            last_modified = modified;
            compile_once_for_watch(file, out, includes);
        }
    }
}

fn compile_once_for_watch(file: &Path, out: &Path, includes: &[std::path::PathBuf]) {
    match compile_file(file, out, includes) {
        Ok(()) => println!("Frame compiled successfully"),
        Err(error) => eprintln!("{error:#}"),
    }
}

fn modified_time(file: &Path) -> anyhow::Result<SystemTime> {
    Ok(std::fs::metadata(file)
        .with_context(|| format!("failed to stat {}", file.display()))?
        .modified()?)
}
