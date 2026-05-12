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
        .filter(|e| {
            let p = e.path();
            let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
            ext == "toml" || ext == "json"
        })
        .collect();

    assert!(!entries.is_empty(), "no valid examples found in {}", dir.display());

    for entry in entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let result = declart_core::parse_auto(&content);
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
fn flow_valid_examples_parse_successfully() {
    assert_valid_examples("flow");
}

#[test]
fn flow_invalid_examples_fail_to_parse() {
    assert_invalid_examples("flow");
}

#[test]
fn tier_valid_examples_parse_successfully() {
    assert_valid_examples("tier");
}

#[test]
fn tier_invalid_examples_fail_to_parse() {
    assert_invalid_examples("tier");
}

#[test]
fn hierarchy_valid_examples_parse_successfully() {
    assert_valid_examples("hierarchy");
}

#[test]
fn hierarchy_invalid_examples_fail_to_parse() {
    assert_invalid_examples("hierarchy");
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
fn comparison_valid_examples_parse_successfully() {
    assert_valid_examples("comparison");
}

#[test]
fn comparison_invalid_examples_fail_to_parse() {
    assert_invalid_examples("comparison");
}

#[test]
fn state_valid_examples_parse_successfully() {
    assert_valid_examples("state");
}

#[test]
fn state_invalid_examples_fail_to_parse() {
    assert_invalid_examples("state");
}

fn assert_valid_themes() {
    let dir = spec_dir().join("themes/valid");
    let entries: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("spec dir not found: {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(OsStr::new("toml")))
        .collect();

    assert!(!entries.is_empty(), "no valid TOML theme examples found in {}", dir.display());

    for entry in entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let result = declart_core::render::Theme::from_toml(&content);
        assert!(
            result.is_ok(),
            "Expected {:?} to be a valid theme, got error:\n  {}",
            path.file_name().unwrap(),
            result.unwrap_err()
        );
    }
}

fn assert_invalid_themes() {
    let dir = spec_dir().join("themes/invalid");
    let entries: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("spec dir not found: {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(OsStr::new("toml")))
        .collect();

    assert!(!entries.is_empty(), "no invalid TOML theme examples found in {}", dir.display());

    for entry in entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let result = declart_core::render::Theme::from_toml(&content);
        assert!(
            result.is_err(),
            "Expected {:?} to be an invalid theme, but it parsed successfully",
            path.file_name().unwrap()
        );
    }
}

#[test]
fn theme_valid_examples_parse_successfully() {
    assert_valid_themes();
}

#[test]
fn theme_invalid_examples_fail_to_parse() {
    assert_invalid_themes();
}
