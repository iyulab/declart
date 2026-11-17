//! Mechanical checks of the system layout: the corpus of real diagrams in `spec/kinds/system/valid`
//! must lay out with no overlapping boxes, no link through a node, no label on a node or on
//! another label, and the same picture on every run.

use super::*;
use crate::model::Diagram;
use crate::parse::parse;
use crate::render::DEFAULT_THEME;
use std::path::PathBuf;
use std::time::Instant;

fn corpus() -> Vec<(String, SystemDiagram)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../spec/kinds/system/valid");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("corpus directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("toml"))
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|p| {
            let text = std::fs::read_to_string(&p).expect("read corpus file");
            let Diagram::System(d) = parse(&text).expect("corpus file parses") else {
                panic!("{} is not a system diagram", p.display());
            };
            (p.file_stem().unwrap().to_string_lossy().into_owned(), d)
        })
        .collect()
}

/// Every way `scene` breaks the layout's promises, as readable lines. Empty means it holds.
fn problems(d: &SystemDiagram, s: &Scene) -> Vec<String> {
    let mut out = Vec::new();
    let canvas = Rect::new(0.0, 0.0, s.width, s.height);

    let group_of_item = |item: usize| -> Option<usize> {
        if item < d.nodes.len() {
            d.nodes[item]
                .group
                .as_deref()
                .and_then(|id| d.groups.iter().position(|g| g.id == id))
        } else {
            d.groups[item - d.nodes.len()]
                .parent
                .as_deref()
                .and_then(|id| d.groups.iter().position(|g| g.id == id))
        }
    };
    let ancestors = |item: usize| -> Vec<usize> {
        let mut v = Vec::new();
        let mut at = group_of_item(item);
        while let Some(g) = at {
            v.push(g);
            at = group_of_item(d.nodes.len() + g);
        }
        v
    };

    for (i, a) in s.nodes.iter().enumerate() {
        if !canvas.contains(&a.rect) {
            out.push(format!("node {} outside the canvas", d.nodes[i].id));
        }
        for (j, b) in s.nodes.iter().enumerate().skip(i + 1) {
            if a.rect.overlaps(&b.rect) {
                out.push(format!(
                    "nodes {} and {} overlap",
                    d.nodes[i].id, d.nodes[j].id
                ));
            }
        }
        let anc = ancestors(i);
        for (g, gb) in s.groups.iter().enumerate() {
            let inside = anc.contains(&g);
            if inside && !gb.rect.contains(&a.rect) {
                out.push(format!(
                    "node {} sticks out of group {}",
                    d.nodes[i].id, d.groups[g].id
                ));
            }
            if !inside && gb.rect.overlaps(&a.rect) {
                out.push(format!(
                    "node {} overlaps group {}",
                    d.nodes[i].id, d.groups[g].id
                ));
            }
        }
    }
    for (g, a) in s.groups.iter().enumerate() {
        if !canvas.contains(&a.rect) {
            out.push(format!("group {} outside the canvas", d.groups[g].id));
        }
        if !a.rect.contains(&a.label_rect) {
            out.push(format!("group {} label sticks out", d.groups[g].id));
        }
        for (i, n) in s.nodes.iter().enumerate() {
            if a.label_rect.overlaps(&n.rect) {
                out.push(format!(
                    "group {} label on node {}",
                    d.groups[g].id, d.nodes[i].id
                ));
            }
        }
        let item = d.nodes.len() + g;
        let anc = ancestors(item);
        for (h, b) in s.groups.iter().enumerate().skip(g + 1) {
            let related = anc.contains(&h) || ancestors(d.nodes.len() + h).contains(&g);
            if !related && a.rect.overlaps(&b.rect) {
                out.push(format!(
                    "groups {} and {} overlap",
                    d.groups[g].id, d.groups[h].id
                ));
            }
            if related {
                let (outer, inner) = if anc.contains(&h) { (b, a) } else { (a, b) };
                if !outer.rect.contains(&inner.rect) {
                    out.push(format!(
                        "nested group sticks out: {} / {}",
                        d.groups[g].id, d.groups[h].id
                    ));
                }
            }
        }
    }

    for (l, lp) in s.links.iter().enumerate() {
        let name = format!("{} → {}", d.links[l].from, d.links[l].to);
        if !lp.routed {
            out.push(format!("link {name} has no route"));
        }
        for p in &lp.points {
            if p.0 < 0.0 || p.1 < 0.0 || p.0 > s.width || p.1 > s.height {
                out.push(format!("link {name} leaves the canvas"));
            }
        }
        for seg in lp.points.windows(2) {
            for (i, n) in s.nodes.iter().enumerate() {
                if n.rect.hit_by(seg[0], seg[1]) {
                    out.push(format!("link {name} runs through node {}", d.nodes[i].id));
                }
            }
            for (g, gb) in s.groups.iter().enumerate() {
                if gb.label_rect.hit_by(seg[0], seg[1]) {
                    out.push(format!(
                        "link {name} crosses the label of group {}",
                        d.groups[g].id
                    ));
                }
            }
        }
        if let Some(label) = &lp.label {
            if !label.placed {
                out.push(format!("label of link {name} has no free place"));
            }
            if !canvas.contains(&label.rect) {
                out.push(format!("label of link {name} outside the canvas"));
            }
            for (i, n) in s.nodes.iter().enumerate() {
                if label.rect.overlaps(&n.rect) {
                    out.push(format!("label of link {name} on node {}", d.nodes[i].id));
                }
            }
            for (k, other) in s.links.iter().enumerate().skip(l + 1) {
                if let Some(ol) = &other.label {
                    if label.rect.overlaps(&ol.rect) {
                        out.push(format!("labels of links {l} and {k} overlap"));
                    }
                }
            }
            for (k, other) in s.links.iter().enumerate() {
                if k == l {
                    continue;
                }
                for seg in other.points.windows(2) {
                    if label.rect.hit_by(seg[0], seg[1]) {
                        out.push(format!("label of link {name} covers link {k}"));
                    }
                }
            }
        }
    }

    // Two links must not run on top of each other.
    for (l, a) in s.links.iter().enumerate() {
        for (k, b) in s.links.iter().enumerate().skip(l + 1) {
            for sa in a.points.windows(2) {
                for sb in b.points.windows(2) {
                    if collinear_overlap(sa[0], sa[1], sb[0], sb[1]) {
                        out.push(format!("links {l} and {k} run on top of each other"));
                    }
                }
            }
        }
    }
    out
}

fn collinear_overlap(a0: (f32, f32), a1: (f32, f32), b0: (f32, f32), b1: (f32, f32)) -> bool {
    if a0.1 == a1.1 && b0.1 == b1.1 && a0.1 == b0.1 {
        return a0.0.min(a1.0).max(b0.0.min(b1.0)) < a0.0.max(a1.0).min(b0.0.max(b1.0));
    }
    if a0.0 == a1.0 && b0.0 == b1.0 && a0.0 == b0.0 {
        return a0.1.min(a1.1).max(b0.1.min(b1.1)) < a0.1.max(a1.1).min(b0.1.max(b1.1));
    }
    false
}

#[test]
fn the_corpus_lays_out_without_problems() {
    let mut failed = Vec::new();
    for (name, d) in corpus() {
        let start = Instant::now();
        let s = scene(&d, &DEFAULT_THEME);
        let ms = start.elapsed().as_millis();
        let bends: usize = s
            .links
            .iter()
            .map(|l| l.points.len().saturating_sub(2))
            .sum();
        println!(
            "{name:<20} {:>4}x{:<4} crossings {:>2} bends {:>3} {ms:>4} ms",
            s.width,
            s.height,
            crossings(&s),
            bends
        );
        let p = problems(&d, &s);
        for line in &p {
            println!("    {line}");
        }
        if !p.is_empty() {
            failed.push(format!("{name}: {} problem(s)", p.len()));
        }
    }
    assert!(failed.is_empty(), "{failed:#?}");
}

#[test]
fn the_corpus_renders_the_same_picture_every_time() {
    for (name, d) in corpus() {
        let a = render(&d, &DEFAULT_THEME);
        let b = render(&d, &DEFAULT_THEME);
        assert_eq!(a, b, "{name} differs between runs");
        assert!(a.starts_with("<svg"), "{name}");
    }
}

/// A diagram at the declared limits — 16 nodes, 6 groups (two nested), 24 links — from a fixed
/// pseudo-random sequence.
fn at_the_limits(seed: u64) -> String {
    let mut state = seed;
    let mut next = move |m: usize| -> usize {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((state >> 33) as usize) % m
    };
    let mut text = String::from("kind = \"system\"\n");
    let groups = ["g0", "g1", "g2", "g3", "g4", "g5"];
    for (i, g) in groups.iter().enumerate() {
        text += &format!("[[groups]]\nid = \"{g}\"\nlabel = \"Group {i} boundary\"\n");
        if i >= 4 {
            text += &format!("parent = \"g{}\"\n", i - 4);
        }
    }
    for i in 0..16 {
        text += &format!("[[nodes]]\nid = \"n{i}\"\nlabel = \"Component number {i}\"\n");
        let g = next(8);
        if g < 6 {
            text += &format!("group = \"{}\"\n", groups[g]);
        }
    }
    let mut made = std::collections::BTreeSet::new();
    while made.len() < 24 {
        let (a, b) = (next(16), next(16));
        if a != b {
            made.insert((a, b));
        }
    }
    for (k, (a, b)) in made.into_iter().enumerate() {
        text += &format!("[[links]]\nfrom = \"n{a}\"\nto = \"n{b}\"\n");
        if k % 2 == 0 {
            text += "label = \"calls\"\n";
        }
        if k % 5 == 0 {
            text += "relation = \"indirect\"\n";
        }
    }
    text
}

#[test]
fn a_diagram_at_the_limits_lays_out_cleanly_and_quickly() {
    let mut slowest = 0;
    for seed in 1..=6u64 {
        let text = at_the_limits(seed);
        let Diagram::System(d) = parse(&text).expect("limit diagram parses") else {
            panic!("system");
        };
        let start = Instant::now();
        let s = scene(&d, &DEFAULT_THEME);
        let ms = start.elapsed().as_millis();
        slowest = slowest.max(ms);
        let p = problems(&d, &s);
        println!(
            "seed {seed}: {}x{} crossings {} {ms} ms, {} problem(s)",
            s.width,
            s.height,
            crossings(&s),
            p.len()
        );
        for line in p.iter().take(8) {
            println!("    {line}");
        }
        // A label that sits on a neighbouring link is soft in a graph this dense; anything
        // else — an overlap, a link through a node, an unrouted link — is not.
        let hard: Vec<&String> = p.iter().filter(|l| !l.contains("covers link")).collect();
        assert!(hard.is_empty(), "seed {seed}: {hard:?}");
    }
    println!("slowest: {slowest} ms");
    // The budget is 100 ms in a release build; an unoptimised test build is far slower.
    assert!(slowest < 3000, "slowest {slowest} ms");
}

#[test]
fn the_corpus_shows_every_label_whole() {
    for (name, d) in corpus() {
        let (_, diagnostics) = crate::diagnostic::collect(|| render(&d, &DEFAULT_THEME));
        let cut: Vec<_> = diagnostics
            .iter()
            .filter(|x| x.code == crate::DiagnosticCode::LabelTruncated)
            .collect();
        assert!(cut.is_empty(), "{name}: {cut:?}");
    }
}

#[test]
#[ignore = "diagnostic: prints both orientations"]
fn both_orientations() {
    for (name, d) in corpus() {
        let m = measure(&d, &DEFAULT_THEME);
        for mode in [Some(true), Some(false), None] {
            let s = build(&d, &m, &arrange(&d, &m, mode));
            let bends: usize = s
                .links
                .iter()
                .map(|l| l.points.len().saturating_sub(2))
                .sum();
            println!(
                "{name:<20} {} {:>4}x{:<4} score {:>8.0} crossings {} bends {} unrouted {} problems {}",
                match mode { Some(true) => "LR", Some(false) => "TB", None => "AUTO" },
                s.width,
                s.height,
                score(&s),
                crossings(&s),
                bends,
                s.links.iter().filter(|l| !l.routed).count(),
                problems(&d, &s).len()
            );
            for line in problems(&d, &s) {
                println!("      - {line}");
            }
        }
    }
}

fn laid_out(text: &str) -> (SystemDiagram, Scene) {
    let Diagram::System(d) = parse(text).expect("parses") else {
        panic!("system");
    };
    let s = scene(&d, &DEFAULT_THEME);
    (d, s)
}

/// Inputs inside the limits that stress the router: a hub with every link, the same pair linked
/// both ways and repeatedly, links to whole groups, an empty group and very long labels.
fn awkward_inputs() -> Vec<(&'static str, String)> {
    let mut hub = String::from("kind = \"system\"\n[[nodes]]\nid = \"hub\"\nlabel = \"Hub\"\n");
    for i in 0..15 {
        hub += &format!("[[nodes]]\nid = \"n{i}\"\nlabel = \"Leaf {i}\"\n");
    }
    for i in 0..15 {
        hub += &format!("[[links]]\nfrom = \"hub\"\nto = \"n{i}\"\nlabel = \"link {i}\"\n");
    }
    for i in 0..9 {
        hub += &format!("[[links]]\nfrom = \"n{i}\"\nto = \"hub\"\n");
    }

    let mut repeated = String::from(
        "kind = \"system\"\n[[nodes]]\nid = \"a\"\nlabel = \"A\"\n[[nodes]]\nid = \"b\"\nlabel = \"B\"\n",
    );
    for i in 0..24 {
        let (from, to) = if i % 2 == 0 { ("a", "b") } else { ("b", "a") };
        repeated += &format!("[[links]]\nfrom = \"{from}\"\nto = \"{to}\"\nlabel = \"call {i}\"\n");
    }

    let groups = r#"kind = "system"
[[groups]]
id = "g1"
label = "Zone one"
[[groups]]
id = "g2"
label = "Zone two"
[[groups]]
id = "empty"
label = "A group nothing sits in"
[[nodes]]
id = "a"
label = "A"
group = "g1"
[[nodes]]
id = "b"
label = "B"
group = "g2"
[[nodes]]
id = "c"
label = "C"
[[links]]
from = "g1"
to = "g2"
label = "between zones"
[[links]]
from = "c"
to = "g1"
[[links]]
from = "g2"
to = "c"
[[links]]
from = "empty"
to = "c"
"#
    .to_string();

    let long = format!(
        "kind = \"system\"\n[[groups]]\nid = \"g\"\nlabel = \"{}\"\n[[nodes]]\nid = \"a\"\nlabel = \"{}\"\ngroup = \"g\"\n[[nodes]]\nid = \"b\"\nlabel = \"Short\"\n[[links]]\nfrom = \"a\"\nto = \"b\"\nlabel = \"{}\"\n",
        "A group label that is far longer than any sensible boundary name would be ".repeat(3),
        "Awaiting approval from every regional compliance reviewer before release ".repeat(3),
        "a link label that runs on and on ".repeat(6),
    );
    vec![
        ("hub", hub),
        ("repeated", repeated),
        ("group ends", groups),
        ("long labels", long),
    ]
}

#[test]
fn awkward_inputs_route_every_link_and_never_panic() {
    for (name, text) in awkward_inputs() {
        let (d, s) = laid_out(&text);
        let p = problems(&d, &s);
        for line in p.iter().take(6) {
            println!("{name}: {line}");
        }
        let unrouted = s.links.iter().filter(|l| !l.routed).count();
        assert_eq!(unrouted, 0, "{name}: {unrouted} link(s) have no route");
        // Two nodes joined by two dozen labelled links leave no room for every label: the
        // labels may then crowd each other, but every link must still be routed around the nodes.
        let hard: Vec<&String> = p
            .iter()
            .filter(|l| !l.contains("covers link"))
            .filter(|l| name != "repeated" || !l.contains("label"))
            .collect();
        assert!(hard.is_empty(), "{name}: {hard:?}");
    }
}

#[test]
fn every_path_starts_and_ends_on_the_box_it_joins() {
    for (name, d) in corpus() {
        let s = scene(&d, &DEFAULT_THEME);
        let n = d.nodes.len();
        for (l, lp) in s.links.iter().enumerate() {
            let rect_of = |id: &str| -> Rect {
                match d.nodes.iter().position(|x| x.id == id) {
                    Some(i) => s.nodes[i].rect,
                    None => s.groups[d.groups.iter().position(|g| g.id == id).unwrap()].rect,
                }
            };
            let on_border = |p: (f32, f32), r: Rect| {
                let inside_x = p.0 >= r.x && p.0 <= r.x1();
                let inside_y = p.1 >= r.y && p.1 <= r.y1();
                inside_x && inside_y && (p.0 == r.x || p.0 == r.x1() || p.1 == r.y || p.1 == r.y1())
            };
            let _ = n;
            assert!(
                on_border(lp.points[0], rect_of(&d.links[l].from)),
                "{name} link {l} does not start on its source"
            );
            assert!(
                on_border(*lp.points.last().unwrap(), rect_of(&d.links[l].to)),
                "{name} link {l} does not end on its target"
            );
        }
    }
}

#[test]
fn a_system_diagram_describes_its_groups_and_links() {
    let text = "kind = \"system\"\ntitle = \"Shop\"\n[[groups]]\nid = \"back\"\nlabel = \"Back end\"\n[[nodes]]\nid = \"api\"\nlabel = \"API\"\ngroup = \"back\"\n[[nodes]]\nid = \"user\"\nlabel = \"User\"\nrole = \"actor\"\n[[links]]\nfrom = \"user\"\nto = \"back\"\nlabel = \"orders\"\n[[links]]\nfrom = \"api\"\nto = \"user\"\nrelation = \"indirect\"\ndirection = \"both\"\n";
    let diagram = parse(text).unwrap();
    assert_eq!(crate::describe::accessible_name(&diagram), "Shop");
    assert_eq!(
        crate::describe::describe(&diagram),
        "System: Back end (API), User. Links: User → Back end (orders); API ↔ User [indirect]."
    );
    let svg = crate::render::render(&diagram, &DEFAULT_THEME).unwrap();
    assert!(svg.contains("role=\"img\"") && svg.contains("<desc>"));
}

#[test]
fn enormous_labels_cannot_stretch_the_diagram() {
    let huge = "wide ".repeat(4000);
    let text = format!(
        "kind = \"system\"\n[[groups]]\nid = \"g\"\nlabel = \"{huge}\"\n[[nodes]]\nid = \"a\"\nlabel = \"{huge}\"\ngroup = \"g\"\n[[nodes]]\nid = \"b\"\nlabel = \"B\"\n[[links]]\nfrom = \"a\"\nto = \"b\"\nlabel = \"{huge}\"\n"
    );
    let start = Instant::now();
    let (d, s) = laid_out(&text);
    assert!(
        s.width < 1500.0 && s.height < 1500.0,
        "{}x{}",
        s.width,
        s.height
    );
    assert!(s.links.iter().all(|l| l.routed));
    assert!(problems(&d, &s).is_empty());
    assert!(start.elapsed().as_secs() < 5);
}
