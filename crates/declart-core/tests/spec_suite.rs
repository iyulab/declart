use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

fn spec_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../spec")
}

fn assert_valid_examples(kind: &str) {
    let dir = spec_dir().join(format!("kinds/{}/valid", kind));
    let entries: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("spec dir not found: {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(OsStr::new("toml")))
        .collect();

    assert!(!entries.is_empty(), "no valid TOML examples found in {}", dir.display());

    for entry in entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let result = declart_core::parse(&content);
        assert!(
            result.is_ok(),
            "Expected {:?} to be valid, got error:\n  {}",
            path.file_name().unwrap(),
            result.unwrap_err()
        );
    }
}

fn assert_invalid_examples(kind: &str) {
    let dir = spec_dir().join(format!("kinds/{}/invalid", kind));
    let entries: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("spec dir not found: {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(OsStr::new("toml")))
        .collect();

    assert!(!entries.is_empty(), "no invalid TOML examples found in {}", dir.display());

    for entry in entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let result = declart_core::parse(&content);
        assert!(
            result.is_err(),
            "Expected {:?} to be invalid, but it parsed successfully",
            path.file_name().unwrap()
        );
    }
}

#[test]
fn pyramid_valid_examples_parse_successfully() {
    assert_valid_examples("pyramid");
}

#[test]
fn pyramid_invalid_examples_fail_to_parse() {
    assert_invalid_examples("pyramid");
}

#[test]
fn process_valid_examples_parse_successfully() {
    assert_valid_examples("process");
}

#[test]
fn process_invalid_examples_fail_to_parse() {
    assert_invalid_examples("process");
}

#[test]
fn cycle_valid_examples_parse_successfully() {
    assert_valid_examples("cycle");
}

#[test]
fn cycle_invalid_examples_fail_to_parse() {
    assert_invalid_examples("cycle");
}

#[test]
fn matrix_valid_examples_parse_successfully() {
    assert_valid_examples("matrix");
}

#[test]
fn matrix_invalid_examples_fail_to_parse() {
    assert_invalid_examples("matrix");
}

#[test]
fn hub_spoke_valid_examples_parse_successfully() {
    assert_valid_examples("hub_spoke");
}

#[test]
fn hub_spoke_invalid_examples_fail_to_parse() {
    assert_invalid_examples("hub_spoke");
}

#[test]
fn venn_valid_examples_parse_successfully() {
    assert_valid_examples("venn");
}

#[test]
fn venn_invalid_examples_fail_to_parse() {
    assert_invalid_examples("venn");
}

#[test]
fn timeline_valid_examples_parse_successfully() {
    assert_valid_examples("timeline");
}

#[test]
fn timeline_invalid_examples_fail_to_parse() {
    assert_invalid_examples("timeline");
}

#[test]
fn fishbone_valid_examples_parse_successfully() {
    assert_valid_examples("fishbone");
}

#[test]
fn fishbone_invalid_examples_fail_to_parse() {
    assert_invalid_examples("fishbone");
}
