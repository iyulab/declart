//! The label-capacity tables in `spec/kinds/*.md` are generated from `limits`, never written by hand.
//!
//! The test fails when a table no longer matches what the engine measures — a change to the fitting
//! code that moves a limit shows up here. To rewrite the tables:
//! `DECLART_UPDATE_SPEC=1 cargo test -p declart-core --test spec_limits`.

use declart_core::{limits, views, KINDS, MAX_PROBED};
use std::path::PathBuf;

const START: &str = "<!-- limits:start — generated; do not edit by hand -->";
const END: &str = "<!-- limits:end -->";

fn cell(unchanged: Option<usize>, whole: Option<usize>) -> String {
    let show = |n: Option<usize>| n.map_or("—".to_string(), |n| n.to_string());
    format!("{} / {}", show(unchanged), show(whole))
}

fn block(kind: &str) -> String {
    let mut out = String::new();
    out.push_str(START);
    out.push_str("\n## Label capacity\n\n");
    out.push_str(
        "How long a label can be before the engine sets it smaller, wraps it, or cuts it short with \"…\". \
Each number is measured by rendering a small reference declaration with a growing label and reading the \
diagnostics `render_detailed` returns, so the table always agrees with them. Lengths are in characters \
written without spaces: Hangul syllables are one em wide, Latin lowercase letters about half that. \
**unchanged** is the longest label shown at the usual size on one line; **whole** is the longest shown \
without \"…\". A slot's width depends on its siblings, so read a number as a guide for the reference \
declaration named in the second column and use `declart validate --strict` for the declaration itself.\n\n",
    );
    out.push_str(&format!(
        "A dash means the engine reports nothing for the slot up to {MAX_PROBED} characters. That is not a \
promise that any length fits: a slot the engine does not fit at all draws a long label past the picture \
without a warning.\n\n"
    ));
    let known = views(kind).unwrap();
    let views: Vec<Option<&str>> = if known.is_empty() {
        vec![None]
    } else {
        known.iter().map(|v| Some(*v)).collect()
    };
    for view in views {
        if let Some(view) = view {
            out.push_str(&format!("### `{view}`\n\n"));
        }
        out.push_str(
            "| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |\n",
        );
        out.push_str(
            "|------|-------------|----------------------------|---------------------------|\n",
        );
        for s in limits(kind, view).unwrap() {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} |\n",
                s.path,
                s.reference,
                cell(s.hangul.unchanged, s.hangul.whole),
                cell(s.ascii.unchanged, s.ascii.whole)
            ));
        }
        out.push('\n');
    }
    out.push_str(END);
    out.push('\n');
    out
}

fn spec_path(kind: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../spec/kinds/{kind}.md"))
}

/// The file with its generated block replaced (or appended when the file has none yet).
fn with_block(text: &str, block: &str) -> String {
    match (text.find(START), text.find(END)) {
        (Some(start), Some(end)) => {
            let end = end + END.len();
            let tail = text[end..].strip_prefix('\n').unwrap_or(&text[end..]);
            format!("{}{}{}", &text[..start], block, tail)
        }
        _ => format!("{}\n{}", text.trim_end_matches('\n'), block).replacen(
            &format!("\n{START}"),
            &format!("\n\n{START}"),
            1,
        ),
    }
}

#[test]
fn the_spec_tables_match_what_the_engine_measures() {
    let update = std::env::var_os("DECLART_UPDATE_SPEC").is_some();
    for kind in KINDS {
        let path = spec_path(kind);
        let raw =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let crlf = raw.contains("\r\n");
        let text = raw.replace("\r\n", "\n");
        let expected = with_block(&text, &block(kind));
        if update {
            let out = if crlf {
                expected.replace('\n', "\r\n")
            } else {
                expected
            };
            std::fs::write(&path, out).unwrap();
        } else {
            assert_eq!(
                text, expected,
                "spec/kinds/{kind}.md is out of date — run `DECLART_UPDATE_SPEC=1 cargo test -p declart-core --test spec_limits`"
            );
        }
    }
}
