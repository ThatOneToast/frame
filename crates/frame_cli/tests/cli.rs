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

fn example_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/svelte/src/lib/frame/app.frame")
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
