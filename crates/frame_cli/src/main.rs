use std::{
    collections::HashSet,
    env, fs,
    io::{self, Read},
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
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
    },
    Compile {
        file: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long = "include")]
        includes: Vec<PathBuf>,
    },
    CompileStdin {
        #[arg(long)]
        css_only: bool,
        #[arg(long)]
        filename: Option<PathBuf>,
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
}

#[derive(Debug, Subcommand)]
enum InitTarget {
    Svelte {
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        yes: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Check { file, includes } => check_file(&file, &includes),
        Command::Compile {
            file,
            out,
            includes,
        } => compile_file(&file, &out, &includes),
        Command::CompileStdin { css_only, filename } => {
            compile_stdin(css_only, filename.as_deref())
        }
        Command::Format { file, check } => format_file(&file, check),
        Command::Watch {
            file,
            out,
            includes,
        } => watch_file(&file, &out, &includes),
        Command::Init { target } => match target {
            InitTarget::Svelte {
                dry_run,
                force,
                yes,
            } => init_svelte(dry_run, force, yes),
        },
    }
}

fn init_svelte(dry_run: bool, force: bool, _yes: bool) -> anyhow::Result<()> {
    let start = env::current_dir().context("failed to read current directory")?;
    let root = detect_project_root(&start)?;

    if !is_svelte_project(&root) {
        anyhow::bail!(
            "{} does not look like a Svelte or SvelteKit project",
            root.display()
        );
    }

    let frame_dir = root.join("src/lib/frame");
    let frame_file = frame_dir.join("app.frame");
    let svelte_config = root.join("svelte.config.js");
    let vite_config = if root.join("vite.config.ts").exists() {
        root.join("vite.config.ts")
    } else {
        root.join("vite.config.js")
    };
    let package_json = root.join("package.json");

    if dry_run {
        println!("Frame init dry run for {}", root.display());
        println!("would create {}", frame_dir.display());
        println!("would create or preserve {}", frame_file.display());
        println!("would generate generated.css and generated.ts");
        println!("would update Svelte and Vite config when safe");
        if package_json.exists() {
            println!("would add @frame/svelte to devDependencies");
        }
        print_svelte_next_steps();
        return Ok(());
    }

    fs::create_dir_all(&frame_dir)?;
    if force || !frame_file.exists() {
        fs::write(&frame_file, INITIAL_FRAME_SOURCE)?;
    }

    compile_file(&frame_file, &frame_dir, std::slice::from_ref(&frame_dir))?;

    update_svelte_config(&svelte_config)?;
    update_vite_config(&vite_config)?;
    if package_json.exists() {
        update_package_json(&package_json)?;
    }

    println!("Frame is ready.\n");
    print_svelte_next_steps();
    Ok(())
}

fn detect_project_root(start: &Path) -> anyhow::Result<PathBuf> {
    let mut current = start;
    loop {
        if current.join("package.json").exists()
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

fn is_svelte_project(root: &Path) -> bool {
    if root.join("svelte.config.js").exists() {
        return true;
    }

    root.join("package.json").exists()
        && fs::read_to_string(root.join("package.json")).is_ok_and(|package| {
            package.contains("\"svelte\"") || package.contains("\"@sveltejs/kit\"")
        })
}

fn update_svelte_config(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        fs::write(path, DEFAULT_SVELTE_CONFIG)?;
        return Ok(());
    }

    let source = fs::read_to_string(path)?;
    if source.contains("framePreprocess") {
        return Ok(());
    }

    backup_file(path)?;
    let mut updated = ensure_import(&source, "import { framePreprocess } from '@frame/svelte';");
    updated = append_to_array_property(&updated, "preprocess", "framePreprocess()");
    fs::write(path, updated)?;
    Ok(())
}

fn update_vite_config(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        fs::write(path, DEFAULT_VITE_CONFIG)?;
        return Ok(());
    }

    let source = fs::read_to_string(path)?;
    if source.contains("framePlugin") {
        return Ok(());
    }

    backup_file(path)?;
    let mut updated = ensure_import(&source, "import { framePlugin } from '@frame/svelte/vite';");
    updated = append_to_array_property(
        &updated,
        "plugins",
        "framePlugin({ input: 'src/lib/frame/app.frame', outDir: 'src/lib/frame' })",
    );
    fs::write(path, updated)?;
    Ok(())
}

fn update_package_json(path: &Path) -> anyhow::Result<()> {
    let source = fs::read_to_string(path)?;
    if source.contains("\"@frame/svelte\"") {
        return Ok(());
    }

    let updated = if let Some(dev_index) = source.find("\"devDependencies\"") {
        let Some(open_relative) = source[dev_index..].find('{') else {
            print_manual_package_instruction();
            return Ok(());
        };
        let open = dev_index + open_relative;
        let existing_is_empty = source[open + 1..].trim_start().starts_with('}');
        let insert = if existing_is_empty {
            "\n    \"@frame/svelte\": \"workspace:*\"\n  "
        } else {
            "\n    \"@frame/svelte\": \"workspace:*\",\n    "
        };
        let mut next = source.clone();
        next.insert_str(open + 1, insert);
        next
    } else if let Some(last_brace) = source.rfind('}') {
        let needs_comma = source[..last_brace].trim_end().ends_with('}');
        let addition = format!(
            "{}\n  \"devDependencies\": {{\n    \"@frame/svelte\": \"workspace:*\"\n  }}\n",
            if needs_comma { "," } else { "" }
        );
        let mut next = source.clone();
        next.insert_str(last_brace, &addition);
        next
    } else {
        print_manual_package_instruction();
        return Ok(());
    };

    fs::write(path, updated)?;
    Ok(())
}

fn ensure_import(source: &str, import_line: &str) -> String {
    if source.contains(import_line) {
        source.to_string()
    } else {
        format!("{import_line}\n{source}")
    }
}

fn append_to_array_property(source: &str, property: &str, item: &str) -> String {
    if let Some(property_index) = source.find(&format!("{property}:")) {
        if let Some(open_relative) = source[property_index..].find('[') {
            let open = property_index + open_relative;
            let mut updated = source.to_string();
            updated.insert_str(open + 1, &format!("\n    {item},"));
            return updated;
        }
    }

    if let Some(export_index) = source.find("export default") {
        if let Some(open_relative) = source[export_index..].find('{') {
            let open = export_index + open_relative;
            let mut updated = source.to_string();
            updated.insert_str(open + 1, &format!("\n  {property}: [\n    {item}\n  ],"));
            return updated;
        }
    }

    source.to_string()
}

fn backup_file(path: &Path) -> anyhow::Result<()> {
    fs::copy(
        path,
        path.with_extension(format!(
            "{}.bak",
            path.extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or("config")
        )),
    )?;
    Ok(())
}

fn print_svelte_next_steps() {
    println!(
        "External styles:\n  import {{ ui }} from '$lib/frame/generated';\n  import '$lib/frame/generated.css';\n\nInline styles:\n  <style lang=\"frame\">\n    card DemoCard {{\n      surface panel\n      padding medium\n    }}\n  </style>"
    );
}

fn print_manual_package_instruction() {
    eprintln!(
        "Could not safely update package.json. Add devDependency manually: \"@frame/svelte\": \"workspace:*\""
    );
}

const INITIAL_FRAME_SOURCE: &str = r#"grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

area Content {
  in Dashboard
  place content
  surface main
  padding large
}

area Inspector {
  in Dashboard
  place inspector
  surface panel
  padding medium
}

card DemoCard {
  surface panel
  padding medium
  radius large
  shadow medium
}
"#;

const DEFAULT_SVELTE_CONFIG: &str =
    "import { framePreprocess } from '@frame/svelte';\n\nexport default {\n  preprocess: [\n    framePreprocess()\n  ]\n};\n";

const DEFAULT_VITE_CONFIG: &str =
    "import { framePlugin } from '@frame/svelte/vite';\n\nexport default {\n  plugins: [\n    framePlugin({\n      input: 'src/lib/frame/app.frame',\n      outDir: 'src/lib/frame'\n    })\n  ]\n};\n";

fn check_file(file: &Path, includes: &[PathBuf]) -> anyhow::Result<()> {
    let document = compile_file_document(file, includes)?;
    let diagnostics = validate(&document);

    if diagnostics.is_empty() {
        println!("ok");
        Ok(())
    } else {
        print_diagnostics(&diagnostics);
        anyhow::bail!("Frame check failed");
    }
}

fn compile_file(file: &Path, out: &Path, includes: &[PathBuf]) -> anyhow::Result<()> {
    let document = compile_file_document(file, includes)?;

    fs::create_dir_all(out)?;
    fs::write(out.join("generated.css"), generate_css(&document))?;
    fs::write(out.join("generated.ts"), generate_typescript(&document))?;
    println!("generated {}", out.display());
    Ok(())
}

fn compile_stdin(css_only: bool, filename: Option<&Path>) -> anyhow::Result<()> {
    if !css_only {
        anyhow::bail!("compile-stdin currently requires --css-only");
    }

    let mut source = String::new();
    io::stdin()
        .read_to_string(&mut source)
        .context("failed to read Frame source from stdin")?;

    let document = compile_source(&source).with_context(|| {
        filename
            .map(|path| format!("failed to compile {}", path.display()))
            .unwrap_or_else(|| "failed to compile stdin".to_string())
    })?;

    print!("{}", generate_css(&document));
    Ok(())
}

fn compile_file_document(
    file: &Path,
    includes: &[PathBuf],
) -> anyhow::Result<frame_core::Document> {
    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    let document = load_frame_document(file, includes, &mut stack, &mut seen)?;
    let diagnostics = validate(&document);

    if !diagnostics.is_empty() {
        print_diagnostics(&diagnostics);
        anyhow::bail!("Frame compile failed");
    }

    Ok(document)
}

fn load_frame_document(
    file: &Path,
    include_paths: &[PathBuf],
    stack: &mut Vec<PathBuf>,
    seen: &mut HashSet<PathBuf>,
) -> anyhow::Result<frame_core::Document> {
    let file = fs::canonicalize(file).unwrap_or_else(|_| file.to_path_buf());

    if let Some(index) = stack.iter().position(|path| path == &file) {
        let mut cycle = stack[index..]
            .iter()
            .chain(std::iter::once(&file))
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        cycle.dedup();
        anyhow::bail!(
            "Include cycle detected:\n\n{}\n\nRemove one include to break the cycle.",
            cycle.join(" -> ")
        );
    }

    if !seen.insert(file.clone()) {
        return Ok(frame_core::Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: Vec::new(),
        });
    }

    let source =
        fs::read_to_string(&file).with_context(|| format!("failed to read {}", file.display()))?;
    let document = match parse(&source) {
        Ok(document) => document,
        Err(error) => {
            print_diagnostics(&error.diagnostics);
            anyhow::bail!("Frame compile failed");
        }
    };

    stack.push(file.clone());
    let mut declarations = Vec::new();
    for include in &document.includes {
        let candidates = include_candidates(&file, &include.target, include_paths);
        let Some(target) = candidates.iter().find(|candidate| candidate.exists()) else {
            let searched = candidates
                .iter()
                .map(|path| format!("- {}", path.display()))
                .collect::<Vec<_>>()
                .join("\n");
            anyhow::bail!(
                "Could not resolve include `{}`.\n\nSearched:\n{}",
                include.target,
                searched
            );
        };
        let included = load_frame_document(target, include_paths, stack, seen)?;
        declarations.extend(included.declarations);
    }
    stack.pop();

    declarations.extend(document.declarations);
    Ok(frame_core::Document {
        includes: document.includes,
        declarations,
        components: document.components,
    })
}

fn include_candidates(file: &Path, target: &str, include_paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let current_dir = file.parent().unwrap_or_else(|| Path::new("."));
    let target_path = Path::new(target);
    let with_extension = |path: PathBuf| {
        if path.extension().is_some() {
            path
        } else {
            path.with_extension("frame")
        }
    };

    if target.starts_with("./") || target.starts_with("../") || target_path.is_absolute() {
        candidates.push(with_extension(current_dir.join(target_path)));
    } else {
        candidates.push(with_extension(current_dir.join(target)));
        candidates.push(with_extension(PathBuf::from(target)));
        for include_path in include_paths {
            candidates.push(with_extension(include_path.join(target)));
        }
    }

    candidates
}

fn compile_source(source: &str) -> anyhow::Result<frame_core::Document> {
    let document = match parse(source) {
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

    Ok(document)
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

fn watch_file(file: &Path, out: &Path, includes: &[PathBuf]) -> anyhow::Result<()> {
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

fn compile_once_for_watch(file: &Path, out: &Path, includes: &[PathBuf]) {
    match compile_file(file, out, includes) {
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
