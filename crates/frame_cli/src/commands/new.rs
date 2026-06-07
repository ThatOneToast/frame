use std::{fs, path::Path};

pub fn new_project(name: &str, template: &str) -> anyhow::Result<()> {
    let root = Path::new(name);
    if root.exists() {
        anyhow::bail!("directory `{}` already exists", root.display());
    }

    fs::create_dir_all(root)?;

    match template {
        "svelte" => init_svelte_template(root)?,
        "web" => init_web_template(root)?,
        other => anyhow::bail!("unknown template `{other}`. Use `web` or `svelte`."),
    }

    println!("Created `{name}` ({template} template).");
    println!("\nNext steps:");
    println!("  cd {name}");
    if template == "svelte" {
        println!("  npm install");
        println!("  npm run dev");
    } else {
        println!("  frame compile src/frame/app.frame --out dist/");
    }
    Ok(())
}

fn init_web_template(root: &Path) -> anyhow::Result<()> {
    let frame_dir = root.join("src/frame");
    fs::create_dir_all(&frame_dir)?;

    fs::write(root.join("frame.config.json"), WEB_CONFIG)?;
    fs::write(
        frame_dir.join("app.frame"),
        crate::project::INITIAL_WEB_FRAME_SOURCE,
    )?;
    fs::write(root.join("index.html"), WEB_INDEX)?;
    fs::write(root.join("README.md"), WEB_README)?;

    // Compile the initial source so generated files exist
    let out = frame_dir.clone();
    crate::commands::compile::compile_file(
        &frame_dir.join("app.frame"),
        &out,
        std::slice::from_ref(&frame_dir),
    )?;

    Ok(())
}

fn init_svelte_template(root: &Path) -> anyhow::Result<()> {
    fs::write(root.join("package.json"), SVELTE_PACKAGE_JSON)?;
    fs::write(root.join("vite.config.js"), SVELTE_VITE_CONFIG)?;
    fs::write(root.join("svelte.config.js"), SVELTE_CONFIG)?;
    fs::write(root.join("index.html"), SVELTE_INDEX)?;

    let src = root.join("src");
    fs::create_dir_all(&src)?;
    fs::write(src.join("App.svelte"), SVELTE_APP)?;
    fs::write(src.join("main.js"), SVELTE_MAIN)?;

    let frame_dir = src.join("lib/frame");
    fs::create_dir_all(&frame_dir)?;
    fs::write(
        frame_dir.join("app.frame"),
        crate::project::INITIAL_FRAME_SOURCE,
    )?;

    fs::write(root.join("README.md"), SVELTE_README)?;

    // Compile the initial source so generated files exist
    crate::commands::compile::compile_file(
        &frame_dir.join("app.frame"),
        &frame_dir,
        std::slice::from_ref(&frame_dir),
    )?;

    Ok(())
}

const WEB_CONFIG: &str = r#"{
  "name": "frame-web-app",
  "version": "0.1.0",
  "source": "src/frame/app.frame",
  "outDir": "dist"
}
"#;

const WEB_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Frame App</title>
  <link rel="stylesheet" href="src/frame/generated.css">
</head>
<body>
  <div id="app"></div>
  <script type="module" src="src/frame/generated.ts"></script>
</body>
</html>
"#;

const WEB_README: &str = r#"# Frame Web Project

This is a minimal Frame project.

## Files

- `src/frame/app.frame` — your Frame source
- `src/frame/generated.css` — compiled CSS output
- `src/frame/generated.ts` — generated TypeScript contracts

## Commands

Compile:
```bash
frame compile src/frame/app.frame --out dist/
```

Watch:
```bash
frame watch src/frame/app.frame --out dist/
```
"#;

const SVELTE_PACKAGE_JSON: &str = r#"{
  "name": "frame-svelte-app",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^3.0.0",
    "svelte": "^4.0.0",
    "vite": "^5.0.0",
    "@frame/svelte": "workspace:*"
  }
}
"#;

const SVELTE_VITE_CONFIG: &str = r#"import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { framePlugin } from '@frame/svelte/vite';

export default defineConfig({
  plugins: [
    svelte(),
    framePlugin({
      input: 'src/lib/frame/app.frame',
      outDir: 'src/lib/frame'
    })
  ]
});
"#;

const SVELTE_CONFIG: &str = r#"import { framePreprocess } from '@frame/svelte';

export default {
  preprocess: [framePreprocess()]
};
"#;

const SVELTE_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Frame + Svelte</title>
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.js"></script>
</body>
</html>
"#;

const SVELTE_APP: &str = r#"<script>
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<main>
  <h1>Frame + Svelte</h1>
  <p>Your Frame source is in <code>src/lib/frame/app.frame</code>.</p>
</main>
"#;

const SVELTE_MAIN: &str = r#"import App from './App.svelte';

const app = new App({
  target: document.getElementById('app')
});

export default app;
"#;

const SVELTE_README: &str = r#"# Frame + Svelte Project

This is a Frame project integrated with Svelte and Vite.

## Files

- `src/lib/frame/app.frame` — your Frame source
- `src/lib/frame/generated.css` — compiled CSS output
- `src/lib/frame/generated.ts` — generated TypeScript contracts

## Commands

Install dependencies:
```bash
npm install
```

Start dev server:
```bash
npm run dev
```

Build:
```bash
npm run build
```
"#;
