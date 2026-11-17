//! `render_diagnosed` reports what the engine changed to make labels fit.

use declart_core::render::DEFAULT_THEME;
use declart_core::{parse, render, render_diagnosed, DiagnosticCode};

fn state(first: &str) -> String {
    format!(
        "kind = \"state\"\n[[states]]\nid = \"s0\"\nlabel = \"{first}\"\n[[states]]\nid = \"s1\"\nlabel = \"B\"\n[[transitions]]\nfrom = \"s0\"\nto = \"s1\"\n"
    )
}

fn run(input: &str) -> (String, Vec<declart_core::Diagnostic>) {
    let diagram = parse(input).expect("valid declaration");
    render_diagnosed(&diagram, &DEFAULT_THEME, None).expect("renders")
}

/// Too long for even the largest state circle: wrapped, shrunk to the smallest size, and still cut.
const TOO_LONG: &str = "Awaiting approval from every regional compliance reviewer before release";

#[test]
fn a_label_cut_short_is_reported_with_the_label_as_declared() {
    let (svg, diagnostics) = run(&state(TOO_LONG));
    assert!(svg.contains('…'), "the picture shows the ellipsis");
    assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
    assert_eq!(diagnostics[0].code, DiagnosticCode::LabelTruncated);
    assert_eq!(diagnostics[0].code.as_str(), "label_truncated");
    assert_eq!(diagnostics[0].label, TOO_LONG);
    assert!(diagnostics[0].message.contains(TOO_LONG));
    assert!(!diagnostics[0].hint.is_empty());
}

#[test]
fn a_label_that_fits_is_not_reported() {
    // NEGATIVE: the same shape with a label that fits must stay silent.
    let (svg, diagnostics) = run(&state("느린시작"));
    assert!(!svg.contains('…'));
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn a_korean_label_with_a_space_grows_its_circle_instead_of_being_cut() {
    // 0.22.0 cut this label to "느린 시…"; the circle now grows to hold it.
    let (svg, diagnostics) = run(&state("느린 시작"));
    assert!(!svg.contains('…'));
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn the_picture_is_the_same_as_render_gives() {
    let input = state(TOO_LONG);
    let diagram = parse(&input).unwrap();
    let plain = render(&diagram, &DEFAULT_THEME).unwrap();
    let (svg, _) = render_diagnosed(&diagram, &DEFAULT_THEME, None).unwrap();
    assert_eq!(plain, svg);
}

#[test]
fn one_render_does_not_leak_into_the_next() {
    let (_, first) = run(&state(TOO_LONG));
    assert_eq!(first.len(), 1);
    let (_, second) = run(&state("느린시작"));
    assert!(second.is_empty(), "{second:?}");
}

#[test]
fn a_label_set_smaller_is_reported_as_shrunk_not_truncated() {
    // A flow step a little too wide for its box is shrunk, and nothing is missing from the picture.
    let mut input = String::from("kind = \"flow\"\n");
    for label in ["Plan", "Prepare the quarterly report", "Ship"] {
        input.push_str(&format!("[[items]]\nlabel = \"{label}\"\n"));
    }
    let (svg, diagnostics) = run(&input);
    for d in &diagnostics {
        assert_ne!(d.code, DiagnosticCode::LabelTruncated, "{d:?}");
    }
    assert!(!svg.contains('…'), "no text is dropped");
}
