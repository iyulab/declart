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

#[test]
fn a_label_cut_short_is_reported_with_the_label_as_declared() {
    // A Korean label with a space is cut short in a state circle (measured on the published 0.22.0).
    let (svg, diagnostics) = run(&state("느린 시작"));
    assert!(svg.contains('…'), "the picture shows the ellipsis");
    assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
    assert_eq!(diagnostics[0].code, DiagnosticCode::LabelTruncated);
    assert_eq!(diagnostics[0].code.as_str(), "label_truncated");
    assert_eq!(diagnostics[0].label, "느린 시작");
    assert!(diagnostics[0].message.contains("느린 시작"));
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
fn the_picture_is_the_same_as_render_gives() {
    let input = state("느린 시작");
    let diagram = parse(&input).unwrap();
    let plain = render(&diagram, &DEFAULT_THEME).unwrap();
    let (svg, _) = render_diagnosed(&diagram, &DEFAULT_THEME, None).unwrap();
    assert_eq!(plain, svg);
}

#[test]
fn one_render_does_not_leak_into_the_next() {
    let (_, first) = run(&state("느린 시작"));
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
