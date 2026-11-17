//! The schema tables in `spec/kinds/*.md` are generated from `schema`, never written by hand.
//!
//! The test fails when a table no longer matches what `schema` reports. To rewrite the tables:
//! `DECLART_UPDATE_SPEC=1 cargo test -p declart-core --test spec_schema`.

use declart_core::{schema, views, Collection, Field, Schema, KINDS};
use std::path::PathBuf;

const START: &str = "<!-- schema:start — generated; do not edit by hand -->";
const END: &str = "<!-- schema:end -->";

fn fields_table(out: &mut String, fields: &[Field]) {
    out.push_str("| Field | Type | Required | Values / meaning |\n");
    out.push_str("|-------|------|----------|------------------|\n");
    for f in fields {
        let mut detail = f
            .values
            .iter()
            .map(|v| format!("`{v}`"))
            .collect::<Vec<_>>()
            .join(", ");
        if let Some(note) = f.note {
            if !detail.is_empty() {
                detail.push_str(" — ");
            }
            detail.push_str(note);
        }
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            f.name,
            f.ty.as_str(),
            if f.required { "yes" } else { "no" },
            detail
        ));
    }
    out.push('\n');
}

fn count(c: &Collection) -> String {
    match (c.min, c.max) {
        (min, Some(max)) if min == max => format!("exactly {min} elements"),
        (min, Some(max)) => format!("{min} to {max} elements"),
        (0, None) => "any number of elements".to_string(),
        (min, None) => format!("{min} or more elements"),
    }
}

fn section(out: &mut String, s: &Schema) {
    out.push_str("**Fields**\n\n");
    fields_table(out, &s.fields);
    for c in &s.collections {
        out.push_str(&format!("**`{}`** — {}\n\n", c.name, count(c)));
        fields_table(out, &c.fields);
    }
    if !s.rules.is_empty() {
        out.push_str("**Rules**\n\n");
        for rule in &s.rules {
            out.push_str(&format!("- {rule}\n"));
        }
        out.push('\n');
    }
}

fn block(kind: &str) -> String {
    let mut out = String::new();
    out.push_str(START);
    out.push_str("\n## Schema\n\n");
    out.push_str(
        "What a declaration of this kind may contain, as `declart schema` and `schema()` report it. \
The value lists and counts are the ones the parser checks, and a test holds every field, count and rule \
to the parser in both directions, so a declaration written to this page parses. The prose above is the \
explanation; this is the reference.\n\n",
    );
    let known = views(kind).unwrap();
    if known.is_empty() {
        section(&mut out, &schema(kind, None).unwrap());
    } else {
        let none = schema(kind, None).unwrap();
        if none.default_view.is_none() {
            out.push_str("### Without `view`\n\n");
            section(&mut out, &none);
        }
        for view in known {
            out.push_str(&format!("### `{view}`\n\n"));
            section(&mut out, &schema(kind, Some(view)).unwrap());
        }
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
        _ => format!("{}\n\n{}", text.trim_end_matches('\n'), block),
    }
}

#[test]
fn the_spec_schema_tables_match_what_schema_reports() {
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
                "spec/kinds/{kind}.md is out of date — run `DECLART_UPDATE_SPEC=1 cargo test -p declart-core --test spec_schema`"
            );
        }
    }
}
