use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_out_dir() -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("frame-cli-test-{unique}"))
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
