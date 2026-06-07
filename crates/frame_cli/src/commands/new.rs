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
    fs::write(root.join("package.json"), WEB_PACKAGE_JSON)?;
    fs::write(root.join("tsconfig.json"), WEB_TSCONFIG)?;
    fs::write(
        frame_dir.join("app.frame"),
        crate::project::INITIAL_WEB_FRAME_SOURCE,
    )?;
    fs::write(root.join("index.html"), WEB_INDEX)?;
    fs::write(root.join("src/main.ts"), WEB_MAIN_TS)?;
    fs::write(root.join("src/handlers.ts"), WEB_HANDLERS_TS)?;
    fs::write(root.join("README.md"), WEB_README)?;

    // Build the initial source so generated files exist.
    crate::commands::build::build_project_at(root)?;

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
  "entry": "src/frame/app.frame",
  "outDir": "src/generated"
}
"#;

const WEB_PACKAGE_JSON: &str = r#"{
  "name": "frame-web-app",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "devDependencies": {
    "vite": "^5.0.0",
    "typescript": "^5.0.0"
  },
  "dependencies": {
    "@frame/runtime-dom": "workspace:*"
  }
}
"#;

const WEB_TSCONFIG: &str = r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true
  },
  "include": ["src"]
}
"#;

const WEB_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Frame App</title>
  <link rel="stylesheet" href="src/generated/generated.css">
</head>
<body>
  <div id="app"></div>
  <script type="module" src="src/main.ts"></script>
</body>
</html>
"#;

const WEB_MAIN_TS: &str = r#"import { mount } from '@frame/runtime-dom';
import appIr from './generated/app.ir';
import { handlers } from './handlers';

const app = mount(appIr, {
  component: 'App',
  target: document.getElementById('app')!,
  handlers
});

// Expose for debugging
(window as any).frameApp = app;
"#;

const WEB_HANDLERS_TS: &str = r#"import type { FrameHandler } from '@frame/runtime-dom';

export const handlers: Record<string, FrameHandler> = {
  increment({ state }) {
    const current = state.get('count') as number;
    state.set('count', current + 1);
  }
};
"#;

const WEB_README: &str = r#"# Frame Web Project

A standalone Frame UI project using the DOM runtime.

## Files

- `src/frame/app.frame` — your Frame UI source
- `src/generated/generated.css` — compiled CSS output
- `src/generated/app.ir.json` — stable serialized Frame IR
- `src/generated/app.ir.ts` — typed IR module consumed by TypeScript
- `src/main.ts` — app entry point (mounts the Frame runtime)
- `src/handlers.ts` — your handler implementations

## Commands

Build (CSS + IR):
```bash
frame build
```

Compile CSS only:
```bash
frame compile src/frame/app.frame --out src/generated
```

Emit IR only:
```bash
frame emit-ir src/frame/app.frame --out src/generated/app.ir.json
```

Watch:
```bash
frame watch src/frame/app.frame --out src/generated
```

Dev server:
```bash
npm install
npm run dev
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
