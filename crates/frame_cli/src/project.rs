use std::{env, fs, path::Path};

use anyhow::Context;

pub fn init_svelte(dry_run: bool, force: bool, _yes: bool) -> anyhow::Result<()> {
    let start = env::current_dir().context("failed to read current directory")?;
    let root = crate::args::detect_project_root(&start)?;

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

    crate::commands::compile::compile_file(
        &frame_file,
        &frame_dir,
        std::slice::from_ref(&frame_dir),
    )?;

    update_svelte_config(&svelte_config)?;
    update_vite_config(&vite_config)?;
    if package_json.exists() {
        update_package_json(&package_json)?;
    }

    println!("Frame is ready.\n");
    print_svelte_next_steps();
    Ok(())
}

pub fn init_web(dry_run: bool, force: bool, _yes: bool) -> anyhow::Result<()> {
    let start = env::current_dir().context("failed to read current directory")?;
    let root = crate::args::detect_project_root(&start).unwrap_or(start);

    let src_dir = root.join("src");
    let frame_file = src_dir.join("app.frame");
    let generated_dir = src_dir.join("generated");

    if dry_run {
        println!("Frame init web dry run for {}", root.display());
        println!("would create {}", src_dir.display());
        println!("would create or preserve {}", frame_file.display());
        println!("would create frame.config.json when missing");
        println!(
            "would create package.json, index.html, src/main.ts, and src/handlers.ts when missing"
        );
        println!("would generate CSS, typed IR, contracts, and handler skeletons");
        return Ok(());
    }

    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&generated_dir)?;
    if force || !frame_file.exists() {
        fs::write(&frame_file, INITIAL_WEB_FRAME_SOURCE)?;
    }

    write_if_missing_or_forced(
        &root.join("frame.config.json"),
        crate::commands::new::WEB_CONFIG,
        force,
    )?;
    write_if_missing_or_forced(
        &root.join("package.json"),
        &crate::commands::new::web_package_json(),
        force,
    )?;
    write_if_missing_or_forced(
        &root.join("tsconfig.json"),
        crate::commands::new::WEB_TSCONFIG,
        force,
    )?;
    write_if_missing_or_forced(
        &root.join("index.html"),
        crate::commands::new::WEB_INDEX,
        force,
    )?;
    write_if_missing_or_forced(
        &root.join("src/main.ts"),
        crate::commands::new::WEB_MAIN_TS,
        force,
    )?;
    write_if_missing_or_forced(
        &root.join("src/handlers.ts"),
        crate::commands::new::WEB_HANDLERS_TS,
        force,
    )?;

    crate::commands::build::build_project_at(&root)?;
    println!("Frame web init is ready.\n");
    Ok(())
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

fn write_if_missing_or_forced(path: &Path, contents: &str, force: bool) -> anyhow::Result<()> {
    if force || !path.exists() {
        fs::write(path, contents)?;
    }
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

pub const INITIAL_FRAME_SOURCE: &str = r#"grid Dashboard {
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

pub const INITIAL_WEB_FRAME_SOURCE: &str = r#"component App {
  state {
    count number = 0
  }

  view {
    screen Main {
      title "Frame App"

      card CounterCard {
        text $count

        action Increment {
          text "Increment"
          on press @increment
        }
      }
    }
  }
}
"#;

const DEFAULT_SVELTE_CONFIG: &str =
    "import { framePreprocess } from '@frame/svelte';\n\nexport default {\n  preprocess: [\n    framePreprocess()\n  ]\n};\n";

const DEFAULT_VITE_CONFIG: &str =
    "import { framePlugin } from '@frame/svelte/vite';\n\nexport default {\n  plugins: [\n    framePlugin({\n      input: 'src/lib/frame/app.frame',\n      outDir: 'src/lib/frame'\n    })\n  ]\n};\n";
