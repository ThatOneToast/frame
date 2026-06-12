use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "frame")]
#[command(about = "Frame compiler and project helper")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Check {
        file: PathBuf,
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
    },
    Compile {
        file: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
        /// CSS generation strategy: `semantic` (default) or `atomic` (experimental).
        #[arg(long = "css-backend", default_value = "semantic")]
        css_backend: String,
    },
    CompileStdin {
        #[arg(long)]
        css_only: bool,
        #[arg(long)]
        filename: Option<PathBuf>,
    },
    EmitIr {
        file: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
    },
    EmitContracts {
        file: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
    },
    Format {
        file: PathBuf,
        #[arg(long)]
        check: bool,
    },
    Watch {
        file: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
    },
    Init {
        #[command(subcommand)]
        target: InitTarget,
    },
    Build {
        #[arg(long)]
        watch: bool,
        /// CSS generation strategy: `semantic` (default) or `atomic` (experimental).
        #[arg(long = "css-backend", default_value = "semantic")]
        css_backend: String,
    },
    Doctor,
    New {
        name: String,
        #[arg(long, default_value = "web")]
        template: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum InitTarget {
    Svelte {
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        yes: bool,
    },
    Web {
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        yes: bool,
    },
}

/// Attempt to locate a `frame.config.json` or project root from a starting path.
pub fn detect_project_root(start: &Path) -> anyhow::Result<PathBuf> {
    let mut current = start;
    loop {
        if current.join("frame.config.json").exists()
            || current.join("package.json").exists()
            || current.join("svelte.config.js").exists()
            || current.join("vite.config.ts").exists()
            || current.join("vite.config.js").exists()
        {
            return Ok(current.to_path_buf());
        }

        let Some(parent) = current.parent() else {
            anyhow::bail!("could not find a project root from {}", start.display());
        };
        current = parent;
    }
}
