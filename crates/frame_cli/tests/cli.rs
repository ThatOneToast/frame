use std::{
    fs,
    path::PathBuf,
    process::Command,
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
