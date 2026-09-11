use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../proto/.proto");
    prost_build::compile_protos(&["../proto/.proto"], &["../proto"]).unwrap();

    for path in [
        "main.py",
        "example",
        "frontend",
        "ir",
        "common",
        "generated",
        "pyproject.toml",
        "uv.lock",
        ".python-version",
    ] {
        println!("cargo:rerun-if-changed=../{path}");
    }

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let project_root = manifest_dir.parent().unwrap();
    let output = Command::new("uv")
        .args(["run", "main.py", "example/"])
        .current_dir(project_root)
        .output()
        .expect("Failed to run uv; make sure it is installed and available on PATH");

    assert!(
        output.status.success(),
        "uv run main.py example/ failed ({}):\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    // Cargo hides ordinary build-script stdout, so surface the generator's result.
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        println!("cargo:warning={line}");
    }
    for line in String::from_utf8_lossy(&output.stderr).lines() {
        println!("cargo:warning={line}");
    }
}
