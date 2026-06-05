//! End-to-end tests for the `apm-forensic` binary.
//!
//! Spawns the compiled binary (via `CARGO_BIN_EXE_*`) so the CLI's `main` is
//! exercised end-to-end, including argument parsing and exit codes.

use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_apm-forensic"))
}

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/apm_map.bin");

#[test]
fn analyses_real_apm_image() {
    let out = bin().arg(FIXTURE).output().unwrap();
    assert!(out.status.success(), "clean image should exit 0");
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("APM Forensic Analysis"), "{s}");
    assert!(s.contains("Apple_HFS"), "{s}");
}

#[test]
fn no_args_prints_usage() {
    let out = bin().output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("usage"));
}

#[test]
fn help_flag_prints_usage() {
    let out = bin().arg("--help").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn missing_file_errors() {
    let out = bin()
        .arg("/nonexistent/does-not-exist.bin")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("cannot open"));
}

#[test]
fn non_apm_input_exits_failure() {
    let path = std::env::temp_dir().join(format!("apm_e2e_{}.bin", std::process::id()));
    std::fs::write(&path, vec![0u8; 4096]).unwrap();
    let out = bin().arg(&path).output().unwrap();
    assert_eq!(out.status.code(), Some(1), "non-APM input is a failure");
    let _ = std::fs::remove_file(&path);
}

#[cfg(feature = "serde")]
#[test]
fn json_output_emits_block_size() {
    let out = bin().args(["--json", FIXTURE]).output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("\"block_size\""));
}
