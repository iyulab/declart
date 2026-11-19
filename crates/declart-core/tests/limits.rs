//! `limits` is measured with `render_diagnosed`, so it must agree with it exactly.

use declart_core::render::DEFAULT_THEME;
use declart_core::{limits, parse, render_diagnosed, views, DiagnosticCode, KINDS, MAX_PROBED};

fn every_kind_and_view() -> Vec<(&'static str, Option<&'static str>)> {
    let mut all = Vec::new();
    for kind in KINDS {
        let known = views(kind).expect("every kind lists its views");
        if known.is_empty() {
            all.push((*kind, None));
        }
        for view in known {
            all.push((*kind, Some(*view)));
        }
    }
    all
}

#[test]
fn every_kind_and_view_has_measurable_slots() {
    for (kind, view) in every_kind_and_view() {
        let slots = limits(kind, view).unwrap_or_else(|e| panic!("{kind}/{view:?}: {e}"));
        assert!(!slots.is_empty(), "{kind}/{view:?} has no slot");
        for slot in &slots {
            for capacity in [slot.hangul, slot.ascii] {
                if let (Some(unchanged), Some(whole)) = (capacity.unchanged, capacity.whole) {
                    assert!(
                        unchanged <= whole,
                        "{kind}/{view:?} {}: unchanged {unchanged} > whole {whole}",
                        slot.path
                    );
                }
            }
        }
    }
}

#[test]
fn a_label_just_over_the_limit_is_reported_and_one_at_the_limit_is_not() {
    // The state circle's label is the case reported with real Korean text.
    let slots = limits("state", None).unwrap();
    let slot = slots.iter().find(|s| s.path == "states[].label").unwrap();
    let whole = slot.hangul.whole.expect("a state circle has a limit");

    let render = |n: usize| {
        let label: String = "가나다라마바사아자차카타파하"
            .chars()
            .cycle()
            .take(n)
            .collect();
        let input = format!(
            "kind = \"state\"\n[[states]]\nid = \"s0\"\nlabel = \"{label}\"\n[[states]]\nid = \"s1\"\nlabel = \"Aa1\"\n[[states]]\nid = \"s2\"\nlabel = \"Aa2\"\n[[transitions]]\nfrom = \"s0\"\nto = \"s1\"\ntrigger = \"Go\"\n[[transitions]]\nfrom = \"s1\"\nto = \"s2\"\ntrigger = \"Go\"\n"
        );
        let diagram = parse(&input).unwrap();
        render_diagnosed(&diagram, &DEFAULT_THEME, None).unwrap().1
    };
    let cut =
        |d: &[declart_core::Diagnostic]| d.iter().any(|d| d.code == DiagnosticCode::LabelTruncated);
    assert!(
        !cut(&render(whole)),
        "{whole} characters should be shown whole"
    );
    assert!(
        cut(&render(whole + 1)),
        "{} characters should be cut",
        whole + 1
    );
}

#[test]
fn an_unknown_kind_or_view_is_an_error_with_a_hint() {
    assert!(limits("nope", None).is_err());
    let err = limits("flow", Some("spiral")).unwrap_err().to_string();
    assert!(err.contains("process"), "{err}");
    let err = limits("state", Some("any")).unwrap_err().to_string();
    assert!(err.contains("no views"), "{err}");
}

#[test]
fn nothing_is_probed_past_the_documented_ceiling() {
    for (kind, view) in every_kind_and_view() {
        for slot in limits(kind, view).unwrap() {
            for capacity in [slot.hangul, slot.ascii] {
                for n in [capacity.unchanged, capacity.whole].into_iter().flatten() {
                    assert!(n < MAX_PROBED, "{kind}/{view:?} {}: {n}", slot.path);
                }
            }
        }
    }
}

#[test]
fn print_the_measured_table() {
    // Run with `--nocapture` to read the table; asserts nothing.
    for (kind, view) in every_kind_and_view() {
        for s in limits(kind, view).unwrap() {
            println!(
                "{kind:11}{:11}{:24} hangul {:>4?}/{:>4?}  ascii {:>4?}/{:>4?}  ({})",
                view.unwrap_or("-"),
                s.path,
                s.hangul.unchanged,
                s.hangul.whole,
                s.ascii.unchanged,
                s.ascii.whole,
                s.reference
            );
        }
    }
}

#[test]
fn a_system_link_label_just_over_the_limit_is_cut_and_one_at_the_limit_is_not() {
    // Written out by hand, not built by `limits`: a chain of three nodes, the first link's label grows.
    let slots = limits("system", None).unwrap();
    let slot = slots.iter().find(|s| s.path == "links[].label").unwrap();
    let whole = slot.hangul.whole.expect("a link label has a limit");

    let render = |n: usize| {
        let label: String = "가나다라마바사아자차카타파하"
            .chars()
            .cycle()
            .take(n)
            .collect();
        let input = format!(
            "kind = \"system\"\n[[nodes]]\nid = \"a\"\nlabel = \"Aa10\"\n[[nodes]]\nid = \"b\"\nlabel = \"Aa11\"\n[[nodes]]\nid = \"c\"\nlabel = \"Aa12\"\n[[links]]\nfrom = \"a\"\nto = \"b\"\nlabel = \"{label}\"\n[[links]]\nfrom = \"b\"\nto = \"c\"\nlabel = \"Aa11\"\n"
        );
        let diagram = parse(&input).unwrap();
        render_diagnosed(&diagram, &DEFAULT_THEME, None).unwrap().1
    };
    let cut =
        |d: &[declart_core::Diagnostic]| d.iter().any(|d| d.code == DiagnosticCode::LabelTruncated);
    assert!(
        !cut(&render(whole)),
        "{whole} characters should be shown whole"
    );
    assert!(
        cut(&render(whole + 1)),
        "{} characters should be cut",
        whole + 1
    );
}
