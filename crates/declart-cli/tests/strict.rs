//! `--strict` turns a label the engine would cut short with "…" into a failing exit code.

use std::path::PathBuf;
use std::process::{Command, Output};

fn state(first: &str) -> String {
    format!(
        "kind = \"state\"\n[[states]]\nid = \"s0\"\nlabel = \"{first}\"\n[[states]]\nid = \"s1\"\nlabel = \"B\"\n[[transitions]]\nfrom = \"s0\"\nto = \"s1\"\n"
    )
}

/// Too long for even the largest state circle: wrapped, shrunk to the smallest size, and still cut.
const TOO_LONG: &str = "Awaiting approval from every regional compliance reviewer before release";

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("declart-cli-strict-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn declart(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_declart"))
        .args(args)
        .output()
        .expect("runs the declart binary")
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn validate_strict_fails_for_a_label_that_would_be_cut_short() {
    let input = scratch("cut.toml");
    std::fs::write(&input, state(TOO_LONG)).unwrap();
    let out = declart(&["validate", "--strict", input.to_str().unwrap()]);
    assert!(!out.status.success(), "{}", stderr(&out));
    let message = stderr(&out);
    assert!(message.contains(TOO_LONG), "{message}");
    assert!(message.contains("hint"), "{message}");
}

#[test]
fn validate_strict_passes_for_a_label_that_fits() {
    // NEGATIVE: the same shape with a label that fits must stay silent.
    let input = scratch("fits.toml");
    std::fs::write(&input, state("느린시작")).unwrap();
    let out = declart(&["validate", "--strict", input.to_str().unwrap()]);
    assert!(out.status.success(), "{}", stderr(&out));
}

#[test]
fn validate_without_strict_still_accepts_a_label_that_would_be_cut_short() {
    let input = scratch("lenient.toml");
    std::fs::write(&input, state(TOO_LONG)).unwrap();
    let out = declart(&["validate", input.to_str().unwrap()]);
    assert!(out.status.success(), "{}", stderr(&out));
}

#[test]
fn render_strict_fails_and_writes_nothing() {
    let input = scratch("render-cut.toml");
    let output = scratch("render-cut.svg");
    let _ = std::fs::remove_file(&output);
    std::fs::write(&input, state(TOO_LONG)).unwrap();
    let out = declart(&["render", "--strict", input.to_str().unwrap()]);
    assert!(!out.status.success(), "{}", stderr(&out));
    assert!(
        !output.exists(),
        "a failed strict render must not leave a picture behind"
    );
}

#[test]
fn render_strict_writes_the_picture_when_every_label_fits() {
    let input = scratch("render-fits.toml");
    let output = scratch("render-fits.svg");
    let _ = std::fs::remove_file(&output);
    std::fs::write(&input, state("느린시작")).unwrap();
    let out = declart(&["render", "--strict", input.to_str().unwrap()]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(output.exists());
}
