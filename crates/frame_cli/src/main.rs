use std::path::Path;

use clap::Parser;

mod args;
mod commands;
mod diagnostics;
mod include;
mod project;
mod theme;

fn parse_backend(value: &str) -> anyhow::Result<frame_codegen::CssBackend> {
    frame_codegen::CssBackend::parse(value).ok_or_else(|| {
        anyhow::anyhow!("unknown css backend `{value}`; use `semantic` or `atomic` (experimental)")
    })
}

fn main() -> anyhow::Result<()> {
    let cli = args::Cli::parse();

    match cli.command {
        args::Command::Check { file, includes } => commands::check::check_file(&file, &includes),
        args::Command::Compile {
            file,
            out,
            includes,
            css_backend,
        } => commands::compile::compile_file(&file, &out, &includes, parse_backend(&css_backend)?),
        args::Command::CompileStdin { css_only, filename } => {
            commands::emit::compile_stdin(css_only, filename.as_deref())
        }
        args::Command::EmitIr {
            file,
            out,
            includes,
        } => commands::emit::emit_ir(&file, out.as_deref(), &includes),
        args::Command::EmitContracts {
            file,
            out,
            includes,
        } => commands::emit::emit_contracts(&file, out.as_deref(), &includes),
        args::Command::Format { file, check } => commands::format::format_file(&file, check),
        args::Command::Watch {
            file,
            out,
            includes,
        } => commands::watch::watch_file(&file, &out, &includes),
        args::Command::Init { target } => match target {
            args::InitTarget::Svelte {
                dry_run,
                force,
                yes,
            } => commands::init::init_svelte(dry_run, force, yes),
            args::InitTarget::Web {
                dry_run,
                force,
                yes,
            } => commands::init::init_web(dry_run, force, yes),
        },
        args::Command::Build { watch, css_backend } => {
            let backend = parse_backend(&css_backend)?;
            if watch {
                commands::watch::watch_project(Path::new("."))
            } else {
                commands::build::build_project(backend)
            }
        }
        args::Command::Doctor => commands::doctor::doctor(),
        args::Command::New { name, template } => commands::new::new_project(&name, &template),
    }
}
