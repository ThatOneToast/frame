use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use frame_codegen::{generate_css, generate_typescript};
use frame_core::{semantic::validate, Diagnostic};
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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Check { file } => {
            let source = fs::read_to_string(&file)
                .with_context(|| format!("failed to read {}", file.display()))?;
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
        Command::Compile { file, out } => {
            let source = fs::read_to_string(&file)
                .with_context(|| format!("failed to read {}", file.display()))?;
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

            fs::create_dir_all(&out)?;
            fs::write(out.join("generated.css"), generate_css(&document))?;
            fs::write(out.join("generated.ts"), generate_typescript(&document))?;
            println!("generated {}", out.display());
            Ok(())
        }
    }
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{:?} [{}..{}]: {}",
            diagnostic.severity, diagnostic.span.start, diagnostic.span.end, diagnostic.message
        );
    }
}
