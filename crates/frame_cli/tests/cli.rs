use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_out_dir() -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir()
        .join(format!("frame-cli-test-{unique}"))
        .join(TEMP_COUNTER.fetch_add(1, Ordering::Relaxed).to_string())
}

fn example_source() -> &'static str {
    r#"grid AppShell {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in AppShell
  place sidebar
  surface panel
  padding small
}

area Content {
  in AppShell
  place content
  surface main
  padding large
}

area Inspector {
  in AppShell
  place inspector
  surface panel
  padding medium
}

card QuickLinkCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
"#
}

fn example_file() -> PathBuf {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary dir should be creatable");
    let path = root.join("app.frame");
    fs::write(&path, example_source()).expect("example should be writable");
    path
}

#[test]
fn checks_example_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(example_file())
        .output()
        .expect("frame check should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn compiles_example_file() {
    let out = temp_out_dir();

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("compile")
        .arg(example_file())
        .arg("--out")
        .arg(&out)
        .output()
        .expect("frame compile should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(out.join("generated.css").exists());
    assert!(out.join("generated.ts").exists());

    fs::remove_dir_all(out).expect("temporary output should be removable");
}

#[test]
fn compile_resolves_includes() {
    let root = temp_out_dir();
    let out = root.join("out");
    fs::create_dir_all(root.join("styles")).expect("temporary input should be creatable");
    fs::write(
        root.join("styles/tokens.frame"),
        "tokens Brand {\n  color brand #7c3aed\n}\n",
    )
    .expect("tokens include should be writable");
    fs::write(
        root.join("app.frame"),
        "#include tokens\n\ncard BrandCard {\n  background brand\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("compile")
        .arg(root.join("app.frame"))
        .arg("--out")
        .arg(&out)
        .arg("--include")
        .arg(root.join("styles"))
        .output()
        .expect("frame compile should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let css = fs::read_to_string(out.join("generated.css")).expect("css should exist");
    let ts = fs::read_to_string(out.join("generated.ts")).expect("ts should exist");
    assert!(css.contains("--frame-color-brand: #7c3aed;"));
    assert!(css.contains("background: var(--frame-color-brand);"));
    assert!(ts.contains("BrandCard"));
    assert!(!ts.contains("Brand:"));

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn emits_initial_ui_ir_json() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    let file = root.join("chat-input.frame");
    fs::write(
        &file,
        "component ChatInput {\n  state {\n    draft text = \"\"\n  }\n\n  view {\n    input MessageBox {\n      value bind $draft\n      on keydown.enter @sendMessage\n    }\n  }\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("emit-ir")
        .arg(&file)
        .output()
        .expect("frame emit-ir should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"version\": 1"));
    assert!(stdout.contains("\"name\": \"ChatInput\""));
    assert!(stdout.contains("\"handler\": \"sendMessage\""));

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn emits_initial_ui_typescript_contracts() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    let file = root.join("chat-input.frame");
    fs::write(
        &file,
        "component ChatInput {\n  state {\n    draft text = \"\"\n    sending bool = false\n  }\n\n  view {\n    action Send {\n      on press @sendMessage\n      on keydown.enter @sendMessage\n    }\n  }\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("emit-contracts")
        .arg(&file)
        .output()
        .expect("frame emit-contracts should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("export type ChatInputState"));
    assert!(stdout.contains("draft: string"));
    assert!(stdout.contains("sending: boolean"));
    assert_eq!(stdout.matches("sendMessage(ctx").count(), 1);

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn check_reports_missing_include() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    fs::write(root.join("app.frame"), "#include dashbord\n").expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(root.join("app.frame"))
        .output()
        .expect("frame check should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Could not resolve include"));

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn formats_file_in_place() {
    let out = temp_out_dir();
    fs::create_dir_all(&out).expect("temporary output should be creatable");
    let file = out.join("app.frame");
    fs::write(
        &file,
        "card A {\npadding small\nhover {\nlift small\n}\n}\n",
    )
    .expect("sample should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("format")
        .arg(&file)
        .output()
        .expect("frame format should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(&file).expect("sample should be readable"),
        "card A {\n  padding small\n\n  hover {\n    lift small\n  }\n}\n"
    );

    fs::remove_dir_all(out).expect("temporary output should be removable");
}

#[test]
fn format_check_fails_for_unformatted_file() {
    let out = temp_out_dir();
    fs::create_dir_all(&out).expect("temporary output should be creatable");
    let file = out.join("app.frame");
    fs::write(&file, "card A {\npadding small\n}\n").expect("sample should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("format")
        .arg(&file)
        .arg("--check")
        .output()
        .expect("frame format --check should run");

    assert!(!output.status.success());

    fs::remove_dir_all(out).expect("temporary output should be removable");
}

#[test]
fn compile_stdin_css_only_outputs_css() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("compile-stdin")
        .arg("--css-only")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("frame compile-stdin should spawn");

    child
        .stdin
        .as_mut()
        .expect("stdin should be open")
        .write_all(b"card TestCard {\n  surface panel\n  padding medium\n}\n")
        .expect("Frame source should be written");

    let output = child
        .wait_with_output()
        .expect("frame compile-stdin should finish");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(".fr-TestCard"));
    assert!(stdout.contains("background: var(--frame-surface-panel);"));
}

#[test]
fn compile_stdin_css_only_fails_for_invalid_frame() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("compile-stdin")
        .arg("--css-only")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("frame compile-stdin should spawn");

    child
        .stdin
        .as_mut()
        .expect("stdin should be open")
        .write_all(b"unknown Broken {\n  padding medium\n}\n")
        .expect("Frame source should be written");

    let output = child
        .wait_with_output()
        .expect("frame compile-stdin should finish");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown declaration kind `unknown`"));
}

#[test]
fn new_web_generates_typed_runtime_project() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary project parent should be creatable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("new")
        .arg("demo-web")
        .arg("--template")
        .arg("web")
        .current_dir(&root)
        .output()
        .expect("frame new web should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let project = root.join("demo-web");
    assert!(project.join("src/app.frame").exists());
    assert!(project.join("src/generated/generated.css").exists());
    assert!(project.join("src/generated/generated.ts").exists());
    assert!(project.join("src/generated/app.ir.json").exists());
    assert!(project.join("src/generated/app.ir.ts").exists());
    assert!(project.join("src/generated/frame.types.ts").exists());
    assert!(project.join("src/generated/frame.handlers.ts").exists());

    let main_ts = fs::read_to_string(project.join("src/main.ts")).expect("main ts");
    let package_json = fs::read_to_string(project.join("package.json")).expect("package json");
    let frame_source = fs::read_to_string(project.join("src/app.frame")).expect("frame source");
    let ir_ts = fs::read_to_string(project.join("src/generated/app.ir.ts")).expect("typed ir");
    let types_ts = fs::read_to_string(project.join("src/generated/frame.types.ts")).expect("types");
    let handlers_ts =
        fs::read_to_string(project.join("src/generated/frame.handlers.ts")).expect("handlers");

    assert!(main_ts.contains("import appIr from './generated/app.ir';"));
    assert!(!main_ts.contains("as any"));
    assert!(package_json.contains("\"frame:build\": \"frame build\""));
    assert!(package_json.contains("\"dev\": \"npm run frame:build && vite\""));
    assert!(package_json.contains("\"build\": \"npm run frame:build && vite build\""));
    assert!(package_json.contains("\"check\": \"npm run frame:build && tsc --noEmit\""));
    assert!(frame_source.contains("screen Main"));
    assert!(frame_source.contains("action Increment"));
    assert!(ir_ts.contains("Source: src/app.frame"));
    assert!(ir_ts.contains("defineFrameIrDocument"));
    assert!(ir_ts.contains("as const"));
    assert!(types_ts.contains("Generated TypeScript contracts"));
    assert!(types_ts.contains("export type FramePressEvent"));
    assert!(handlers_ts.contains("Generated handler skeletons"));
    assert!(handlers_ts.contains("import type { FrameEventContext"));
    assert!(handlers_ts.contains("TODO: implement increment"));

    fs::remove_dir_all(root).expect("temporary project should be removable");
}

#[test]
fn init_web_scaffolds_empty_directory() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary project should be creatable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("init")
        .arg("web")
        .current_dir(&root)
        .output()
        .expect("frame init web should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(root.join("frame.config.json").exists());
    assert!(root.join("package.json").exists());
    assert!(root.join("src/app.frame").exists());
    assert!(root.join("src/main.ts").exists());
    assert!(root.join("src/handlers.ts").exists());
    assert!(root.join("src/generated/app.ir.ts").exists());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Frame web init is ready"));

    fs::remove_dir_all(root).expect("temporary project should be removable");
}

#[test]
fn build_reports_paths_and_preserves_handler_skeletons() {
    let root = temp_out_dir();
    fs::create_dir_all(root.join("src/generated")).expect("temporary project should be creatable");
    fs::write(
        root.join("frame.config.json"),
        r#"{"entry":"src/app.frame","outDir":"src/generated"}"#,
    )
    .expect("config should be writable");
    fs::write(
        root.join("src/app.frame"),
        r#"component App {
  state {
    draft text = ""
  }

  view {
    screen Main {
      action Save {
        on press @save
      }

      action Clear {
        on press @clear
      }
    }
  }
}
"#,
    )
    .expect("frame source should be writable");
    fs::write(
        root.join("src/generated/frame.handlers.ts"),
        "export function save() {\n  // existing user note\n}\n",
    )
    .expect("existing skeleton should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("build")
        .current_dir(&root)
        .output()
        .expect("frame build should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("source:"));
    assert!(stdout.contains("src/app.frame"));
    assert!(stdout.contains("app.ir.ts"));
    assert!(stdout.contains("frame.types.ts"));
    assert!(stdout.contains("frame.handlers.ts"));
    assert!(stdout.contains("appended missing handler stubs"));
    assert!(stdout.contains("warnings: 0"));

    let ir_ts = fs::read_to_string(root.join("src/generated/app.ir.ts")).expect("typed ir");
    let types_ts = fs::read_to_string(root.join("src/generated/frame.types.ts")).expect("types");
    let handlers_ts =
        fs::read_to_string(root.join("src/generated/frame.handlers.ts")).expect("handlers");
    assert!(ir_ts.contains("Generated typed Frame IR"));
    assert!(types_ts.contains("Generated TypeScript contracts"));
    assert!(handlers_ts.contains("existing user note"));
    assert!(handlers_ts.contains("export function save()"));
    assert!(handlers_ts.contains("export function clear("));
    assert!(handlers_ts.contains("import type { FrameEventContext"));

    let ir_before = fs::read_to_string(root.join("src/generated/app.ir.ts")).expect("typed ir");
    let handlers_before =
        fs::read_to_string(root.join("src/generated/frame.handlers.ts")).expect("handlers");
    let second = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("build")
        .current_dir(&root)
        .output()
        .expect("second frame build should run");
    assert!(
        second.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    let second_stdout = String::from_utf8_lossy(&second.stdout);
    assert!(second_stdout.contains("unchanged"));
    assert_eq!(
        ir_before,
        fs::read_to_string(root.join("src/generated/app.ir.ts")).expect("typed ir")
    );
    assert_eq!(
        handlers_before,
        fs::read_to_string(root.join("src/generated/frame.handlers.ts")).expect("handlers")
    );

    fs::remove_dir_all(root).expect("temporary project should be removable");
}

#[test]
fn init_svelte_dry_run_detects_project_without_writing() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary project should be creatable");
    fs::write(
        root.join("package.json"),
        r#"{"devDependencies":{"svelte":"^5.0.0"}}"#,
    )
    .expect("package should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("init")
        .arg("svelte")
        .arg("--dry-run")
        .current_dir(&root)
        .output()
        .expect("frame init svelte --dry-run should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!root.join("src/lib/frame/app.frame").exists());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Frame init dry run"));

    fs::remove_dir_all(root).expect("temporary project should be removable");
}

#[test]
fn init_svelte_creates_frame_files_and_updates_configs() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary project should be creatable");
    fs::write(
        root.join("package.json"),
        r#"{"devDependencies":{"svelte":"^5.0.0"}}"#,
    )
    .expect("package should be writable");
    fs::write(
        root.join("svelte.config.js"),
        "export default {\n  preprocess: []\n};\n",
    )
    .expect("svelte config should be writable");
    fs::write(
        root.join("vite.config.ts"),
        "export default {\n  plugins: []\n};\n",
    )
    .expect("vite config should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("init")
        .arg("svelte")
        .arg("--yes")
        .current_dir(&root)
        .output()
        .expect("frame init svelte should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(root.join("src/lib/frame/app.frame").exists());
    assert!(root.join("src/lib/frame/generated.css").exists());
    assert!(root.join("src/lib/frame/generated.ts").exists());
    assert!(root.join("svelte.config.js.bak").exists());
    assert!(root.join("vite.config.ts.bak").exists());

    let svelte_config = fs::read_to_string(root.join("svelte.config.js")).unwrap();
    let vite_config = fs::read_to_string(root.join("vite.config.ts")).unwrap();
    let package_json = fs::read_to_string(root.join("package.json")).unwrap();

    assert!(svelte_config.contains("framePreprocess"));
    assert!(vite_config.contains("framePlugin"));
    assert!(package_json.contains("\"@frame/svelte\""));

    fs::remove_dir_all(root).expect("temporary project should be removable");
}

#[test]
fn check_valid_multi_file_project_with_components() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    fs::write(
        root.join("messages.frame"),
        "component MessageItem {\n  props {\n    author text\n    body text\n  }\n\n  view {\n    row MessageRow {\n      text $author\n      text $body\n    }\n  }\n}\n",
    )
    .expect("messages should be writable");
    fs::write(
        root.join("app.frame"),
        "#include messages\n\ncomponent ChatApp {\n  view {\n    screen ChatScreen {\n      MessageItem(author: \"System\", body: \"Hello\")\n    }\n  }\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(root.join("app.frame"))
        .output()
        .expect("frame check should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn check_reports_unresolved_imported_component() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    fs::write(
        root.join("app.frame"),
        "component ChatApp {\n  view {\n    screen ChatScreen {\n      MissingComponent()\n    }\n  }\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(root.join("app.frame"))
        .output()
        .expect("frame check should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown component `MissingComponent`"));

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn check_reports_duplicate_symbol_across_includes() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    fs::write(
        root.join("base.frame"),
        "card Panel {\n  surface panel\n}\n",
    )
    .expect("base should be writable");
    fs::write(
        root.join("theme.frame"),
        "card Panel {\n  surface main\n}\n",
    )
    .expect("theme should be writable");
    fs::write(
        root.join("app.frame"),
        "#include base\n#include theme\n\narea Sidebar {\n  in Panel\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(root.join("app.frame"))
        .output()
        .expect("frame check should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Duplicate declaration `Panel`"));

    fs::remove_dir_all(root).expect("temporary output should be removable");
}

#[test]
fn check_preserves_intentional_url_sink_warning() {
    let root = temp_out_dir();
    fs::create_dir_all(&root).expect("temporary input should be creatable");
    fs::write(
        root.join("app.frame"),
        "component MediaApp {\n  view {\n    media Preview {\n      source \"https://example.com/video.mp4\"\n    }\n  }\n}\n",
    )
    .expect("app should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(root.join("app.frame"))
        .output()
        .expect("frame check should run");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("navigation or media destination"));

    fs::remove_dir_all(root).expect("temporary output should be removable");
}
