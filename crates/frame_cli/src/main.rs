use std::{
    fs,
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime},
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use frame_codegen::{generate_css, generate_typescript};
use frame_core::{formatting::format_source, semantic::validate, Diagnostic};
use frame_parser::parse;

#[derive(Debug, Parser)]
#[command(name = "frame")]
#[command(about = "Frame CSS DSL compiler")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Check {
        file: PathBuf,
    },
    Compile {
        file: PathBuf,
        #[arg(long)]
        out: PathBuf,
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
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Check { file } => check_file(&file),
        Command::Compile { file, out } => compile_file(&file, &out),
        Command::Format { file, check } => format_file(&file, check),
        Command::Watch { file, out } => watch_file(&file, &out),
    }
}

fn check_file(file: &Path) -> anyhow::Result<()> {
    let source =
        fs::read_to_string(file).with_context(|| format!("failed to read {}", file.display()))?;
    let document = match parse(&source) {
        Ok(document) => document,
        Err(error) => {
            print_diagnostics(&error.diagnostics);
            anyhow::bail!("Frame check failed");
        }
    };
    let diagnostics = validate(&document);

    if diagnostics.is_empty() {
        println!("ok");
        Ok(())
    } else {
        print_diagnostics(&diagnostics);
        anyhow::bail!("Frame check failed");
    }
}

fn compile_file(file: &Path, out: &Path) -> anyhow::Result<()> {
    let source =
        fs::read_to_string(file).with_context(|| format!("failed to read {}", file.display()))?;
    let document = match parse(&source) {
        Ok(document) => document,
        Err(error) => {
            print_diagnostics(&error.diagnostics);
            anyhow::bail!("Frame compile failed");
        }
    };
    let diagnostics = validate(&document);

    if !diagnostics.is_empty() {
        print_diagnostics(&diagnostics);
        anyhow::bail!("Frame compile failed");
    }

    fs::create_dir_all(out)?;
    fs::write(out.join("generated.css"), generate_css(&document))?;
    fs::write(out.join("generated.ts"), generate_typescript(&document))?;
    println!("generated {}", out.display());
    Ok(())
}

fn format_file(file: &Path, check: bool) -> anyhow::Result<()> {
    let source =
        fs::read_to_string(file).with_context(|| format!("failed to read {}", file.display()))?;
    let formatted = format_source(&source);

    if check {
        if formatted == source {
            println!("formatted");
            Ok(())
        } else {
            anyhow::bail!("Frame format check failed: {}", file.display());
        }
    } else {
        fs::write(file, formatted)?;
        println!("formatted {}", file.display());
        Ok(())
    }
}

fn watch_file(file: &Path, out: &Path) -> anyhow::Result<()> {
    println!("watching {}", file.display());
    compile_once_for_watch(file, out);

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
            compile_once_for_watch(file, out);
        }
    }
}

fn compile_once_for_watch(file: &Path, out: &Path) {
    match compile_file(file, out) {
        Ok(()) => println!("Frame compiled successfully"),
        Err(error) => eprintln!("{error:#}"),
    }
}

fn modified_time(file: &Path) -> anyhow::Result<SystemTime> {
    Ok(fs::metadata(file)
        .with_context(|| format!("failed to stat {}", file.display()))?
        .modified()?)
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{:?} [{}..{}]: {}",
            diagnostic.severity, diagnostic.span.start, diagnostic.span.end, diagnostic.message
        );
    }
}
