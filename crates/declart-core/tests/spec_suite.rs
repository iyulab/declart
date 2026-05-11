use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn spec_dir() -> &'static Path {
    // Integration tests run from the crate root (crates/declart-core/),
    // so ../../spec reaches the repo root spec/ directory.
    Path::new("../../spec")
}

#[test]
fn pyramid_valid_examples_parse_successfully() {
    let dir = spec_dir().join("kinds/pyramid/valid");
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

#[test]
fn pyramid_invalid_examples_fail_to_parse() {
    let dir = spec_dir().join("kinds/pyramid/invalid");
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
