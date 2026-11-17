//! Label capacity — how long a label can be before the engine changes it.
//!
//! The engine fits a label into its slot by setting it smaller, wrapping it, or cutting it short with
//! "…". [`limits`] reports where those happen for a kind, so a declaration can be written to fit
//! before it is rendered. The numbers are **measured, not tabulated**: each one is found by growing a
//! label in a small reference declaration until [`render_diagnosed`] reports the change, so they
//! always agree with the diagnostics.
//!
//! A limit holds for the reference declaration it was measured in (its size is in `reference`). A
//! slot's width depends on its siblings — a longer sibling or more items can narrow it — so read a
//! limit as a guide to the order of magnitude, and use [`render_diagnosed`] for the declaration
//! itself.

use crate::error::DeclartError;
use crate::parse::checked_view;
use crate::render::{render_diagnosed, DEFAULT_THEME};
use crate::{parse, DiagnosticCode};
use serde::Serialize;

/// The longest label measured. A slot that still shows a label this long unchanged reports no limit.
pub const MAX_PROBED: usize = 120;

/// How many characters of one script a slot takes before the engine steps in.
///
/// `None` means the engine reports nothing up to [`MAX_PROBED`] characters. That is **not** a promise
/// that any length fits: a slot the engine does not fit at all (a label of a `matrix` quadrant, a
/// `venn` set, a `state` transition trigger) draws a long label past the picture and reports
/// nothing, while a slot whose picture grows with the label (`tier` `pyramid`) really has no limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Capacity {
    /// The longest label shown at its usual size on one line; one more character is set smaller or
    /// wrapped.
    pub unchanged: Option<usize>,
    /// The longest label shown whole; one more character is cut short with "…".
    pub whole: Option<usize>,
}

/// The capacity of one place in a declaration where a label goes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SlotLimit {
    /// Where the label goes in the declaration, such as `items[].label` or `states[].label`.
    pub path: &'static str,
    /// The reference declaration the numbers were measured in, such as `3 items`.
    pub reference: String,
    /// Hangul syllables (each drawn one em wide), written without spaces.
    pub hangul: Capacity,
    /// Lowercase Latin letters, written without spaces.
    pub ascii: Capacity,
}

type Build = fn(view: &str, labels: &[String]) -> String;

struct Slot {
    path: &'static str,
    /// How many labels of this path the reference declaration holds.
    count: usize,
    reference: &'static str,
    build: Build,
}

/// Reports how long a label can be in each place it goes in `kind`, for `view` (omitted: the kind's
/// first view).
///
/// # Errors
/// [`DeclartError::UnknownKind`] for an unknown kind, [`DeclartError::InvalidValue`] for a view the
/// kind does not have.
pub fn limits(kind: &str, view: Option<&str>) -> Result<Vec<SlotLimit>, DeclartError> {
    let (known, view) = checked_view(kind, view)?;
    let view = view.or_else(|| known.first().copied()).unwrap_or("");
    Ok(slots(kind, view)
        .iter()
        .map(|slot| SlotLimit {
            path: slot.path,
            reference: slot.reference.to_string(),
            hangul: measure(kind, view, slot, HANGUL),
            ascii: measure(kind, view, slot, ASCII),
        })
        .collect())
}

const HANGUL: &str = "가나다라마바사아자차카타파하";
const ASCII: &str = "abcdefghijklmnopqrstuvwxyz";

fn probe(alphabet: &str, n: usize) -> String {
    alphabet.chars().cycle().take(n).collect()
}

fn filler(i: usize) -> String {
    format!("Aa{i}")
}

/// Renders the reference declaration with an `n`-character label at position `at` and reports whether
/// `accept` holds for its diagnostics.
fn holds(
    kind: &str,
    view: &str,
    slot: &Slot,
    alphabet: &str,
    n: usize,
    accept: fn(&[crate::Diagnostic]) -> bool,
) -> bool {
    (0..slot.count).all(|at| {
        let labels: Vec<String> = (0..slot.count)
            .map(|i| {
                if i == at {
                    probe(alphabet, n)
                } else {
                    filler(i)
                }
            })
            .collect();
        let source = (slot.build)(view, &labels);
        let diagram = parse(&source)
            .unwrap_or_else(|e| panic!("reference for {kind}/{view} {}: {e}", slot.path));
        let (_, diagnostics) = render_diagnosed(&diagram, &DEFAULT_THEME, None)
            .unwrap_or_else(|e| panic!("reference for {kind}/{view} {}: {e}", slot.path));
        accept(&diagnostics)
    })
}

fn nothing_changed(d: &[crate::Diagnostic]) -> bool {
    d.is_empty()
}

fn nothing_cut(d: &[crate::Diagnostic]) -> bool {
    d.iter().all(|d| d.code != DiagnosticCode::LabelTruncated)
}

fn measure(kind: &str, view: &str, slot: &Slot, alphabet: &str) -> Capacity {
    Capacity {
        unchanged: largest(kind, view, slot, alphabet, nothing_changed),
        whole: largest(kind, view, slot, alphabet, nothing_cut),
    }
}

/// The largest label length for which `accept` holds in every position, found by bisection (a longer
/// label never fits better than a shorter one). `None` when it holds up to [`MAX_PROBED`].
fn largest(
    kind: &str,
    view: &str,
    slot: &Slot,
    alphabet: &str,
    accept: fn(&[crate::Diagnostic]) -> bool,
) -> Option<usize> {
    if holds(kind, view, slot, alphabet, MAX_PROBED, accept) {
        return None;
    }
    // `lo` holds (a one-character label always does), `hi` does not.
    let (mut lo, mut hi) = (1, MAX_PROBED);
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if holds(kind, view, slot, alphabet, mid, accept) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}

// ---- reference declarations -------------------------------------------------------------------

fn header(kind: &str, view: &str) -> String {
    if view.is_empty() {
        format!("kind = \"{kind}\"\n")
    } else {
        format!("kind = \"{kind}\"\nview = \"{view}\"\n")
    }
}

fn items(kind: &'static str) -> Build {
    match kind {
        "flow" => |view, labels| {
            let mut s = header("flow", view);
            for (i, label) in labels.iter().enumerate() {
                s.push_str(&format!("[[items]]\nlabel = \"{label}\"\n"));
                if view == "swimlane" {
                    s.push_str(&format!("actor = \"Team {}\"\n", i % 2));
                }
            }
            s
        },
        _ => |view, labels| {
            let mut s = header("tier", view);
            for label in labels {
                s.push_str(&format!("[[items]]\nlabel = \"{label}\"\n"));
            }
            s
        },
    }
}

/// A hierarchy with `roots` (no parent) and `children` (under the first root). `org_chart` and
/// `mind_map` take exactly one root; `fishbone` takes two or more (its cause categories).
fn hierarchy(view: &str, roots: &[String], children: &[String], effect: Option<&str>) -> String {
    let mut s = header("hierarchy", view);
    if let Some(effect) = effect {
        s.push_str(&format!("effect = \"{effect}\"\n"));
    }
    for (i, label) in roots.iter().enumerate() {
        s.push_str(&format!("[[nodes]]\nid = \"r{i}\"\nlabel = \"{label}\"\n"));
    }
    for (i, label) in children.iter().enumerate() {
        s.push_str(&format!(
            "[[nodes]]\nid = \"c{i}\"\nlabel = \"{label}\"\nparent = \"r0\"\n"
        ));
    }
    s
}

fn default_roots(view: &str) -> Vec<String> {
    (0..if view == "fishbone" { 2 } else { 1 })
        .map(|i| filler(20 + i))
        .collect()
}

fn events(view: &str, labels: &[String]) -> String {
    let mut s = header("timeline", view);
    for (i, label) in labels.iter().enumerate() {
        s.push_str(&format!(
            "[[events]]\ndate = \"2024-{:02}-01\"\nlabel = \"{label}\"\n",
            i + 1
        ));
    }
    s
}

fn quadrants(view: &str, labels: &[String]) -> String {
    let mut s = header("matrix", view);
    s.push_str("x_axis = \"X\"\ny_axis = \"Y\"\n");
    for label in labels {
        s.push_str(&format!("[[quadrants]]\nlabel = \"{label}\"\n"));
    }
    s
}

fn center(view: &str, labels: &[String]) -> String {
    let mut s = header("hub_spoke", view);
    s.push_str(&format!("center = \"{}\"\n", labels[0]));
    for i in 0..3 {
        s.push_str(&format!("[[spokes]]\nlabel = \"{}\"\n", filler(i + 1)));
    }
    s
}

fn spokes(view: &str, labels: &[String]) -> String {
    let mut s = header("hub_spoke", view);
    s.push_str("center = \"Hub\"\n");
    for label in labels {
        s.push_str(&format!("[[spokes]]\nlabel = \"{label}\"\n"));
    }
    s
}

fn sets(view: &str, labels: &[String]) -> String {
    let mut s = header("venn", view);
    for label in labels {
        s.push_str(&format!("[[sets]]\nlabel = \"{label}\"\n"));
    }
    s.push_str(&format!(
        "[[intersections]]\nsets = [\"{}\", \"{}\"]\nlabel = \"Both\"\n",
        labels[0], labels[1]
    ));
    s
}

fn intersection(view: &str, labels: &[String]) -> String {
    let mut s = header("venn", view);
    s.push_str("[[sets]]\nlabel = \"One\"\n[[sets]]\nlabel = \"Two\"\n");
    s.push_str(&format!(
        "[[intersections]]\nsets = [\"One\", \"Two\"]\nlabel = \"{}\"\n",
        labels[0]
    ));
    s
}

fn comparison(view: &str, columns: &[String], rows: &[String], cells: &[String]) -> String {
    let mut s = header("comparison", view);
    for column in columns {
        s.push_str(&format!("[[columns]]\nlabel = \"{column}\"\n"));
    }
    for (r, row) in rows.iter().enumerate() {
        s.push_str(&format!("[[rows]]\nlabel = \"{row}\"\n"));
        for (c, column) in columns.iter().enumerate() {
            s.push_str(&format!(
                "\"{column}\" = \"{}\"\n",
                cells[r * columns.len() + c]
            ));
        }
    }
    s
}

fn states(view: &str, labels: &[String]) -> String {
    let mut s = header("state", view);
    for (i, label) in labels.iter().enumerate() {
        s.push_str(&format!("[[states]]\nid = \"s{i}\"\nlabel = \"{label}\"\n"));
    }
    for i in 0..labels.len() - 1 {
        s.push_str(&format!(
            "[[transitions]]\nfrom = \"s{i}\"\nto = \"s{}\"\ntrigger = \"Go\"\n",
            i + 1
        ));
    }
    s
}

fn triggers(view: &str, labels: &[String]) -> String {
    let mut s = header("state", view);
    for i in 0..=labels.len() {
        s.push_str(&format!(
            "[[states]]\nid = \"s{i}\"\nlabel = \"{}\"\n",
            filler(i)
        ));
    }
    for (i, label) in labels.iter().enumerate() {
        s.push_str(&format!(
            "[[transitions]]\nfrom = \"s{i}\"\nto = \"s{}\"\ntrigger = \"{label}\"\n",
            i + 1
        ));
    }
    s
}

fn slots(kind: &str, view: &str) -> Vec<Slot> {
    let slot = |path, count, reference, build| Slot {
        path,
        count,
        reference,
        build,
    };
    match kind {
        "flow" => vec![slot("items[].label", 3, "3 items", items("flow"))],
        "tier" => vec![slot("items[].label", 3, "3 items", items("tier"))],
        "hierarchy" => {
            let fishbone = view == "fishbone";
            let mut slots = vec![
                slot(
                    "nodes[].label (no parent)",
                    if fishbone { 2 } else { 1 },
                    if fishbone {
                        "2 categories, 2 causes"
                    } else {
                        "a root, 2 children"
                    },
                    |v, l| hierarchy(v, l, &[filler(10), filler(11)], None),
                ),
                slot(
                    "nodes[].label (parent set)",
                    2,
                    if fishbone {
                        "2 categories, 2 causes"
                    } else {
                        "a root, 2 children"
                    },
                    |v, l| hierarchy(v, &default_roots(v), l, None),
                ),
            ];
            if fishbone {
                slots.push(slot("effect", 1, "2 categories, 2 causes", |v, l| {
                    hierarchy(v, &default_roots(v), &[filler(10), filler(11)], Some(&l[0]))
                }));
            }
            slots
        }
        "timeline" => vec![slot("events[].label", 4, "4 milestones", events)],
        "matrix" => vec![slot("quadrants[].label", 4, "4 quadrants", quadrants)],
        "hub_spoke" => vec![
            slot("center", 1, "3 spokes", center),
            slot("spokes[].label", 3, "3 spokes", spokes),
        ],
        "venn" => vec![
            slot("sets[].label", 2, "2 sets", sets),
            slot("intersections[].label", 1, "2 sets", intersection),
        ],
        "comparison" => vec![
            slot("columns[].label", 2, "2 columns, 2 rows", |v, l| {
                comparison(v, l, &[filler(0), filler(1)], &vec![filler(2); 4])
            }),
            slot("rows[].label", 2, "2 columns, 2 rows", |v, l| {
                comparison(v, &[filler(0), filler(1)], l, &vec![filler(2); 4])
            }),
            slot("rows[].<column>", 4, "2 columns, 2 rows", |v, l| {
                comparison(v, &[filler(0), filler(1)], &[filler(2), filler(3)], l)
            }),
        ],
        "state" => vec![
            slot("states[].label", 3, "3 states", states),
            slot("transitions[].trigger", 2, "3 states", triggers),
        ],
        _ => Vec::new(),
    }
}
