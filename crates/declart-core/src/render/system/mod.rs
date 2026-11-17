//! System diagram: components in nested groups, joined by labelled links.
//!
//! The engine places everything. Each group is laid out first, as a layered graph of its own
//! nodes and sub-groups (see [`layout`]), then stands as one box in the level above; so a
//! group's box can never overlap another box. Links are routed afterwards on a grid that
//! nodes may not be entered from (see [`route`]), and their labels are set on a free stretch of
//! the path. The diagram is laid out left to right and top to bottom, and the one with the
//! fewer crossings and bends and the better proportions is drawn.

mod layout;
mod route;

use super::{font, status, svg::SvgBuilder};
use crate::model::{Emphasis, LinkDirection, LinkRelation, SystemDiagram, SystemRole};
use crate::render::theme::Theme;
use layout::{layout_level, snap, snap_up, LevelEdge, G};
use route::{End, Obstacles, Router};
use std::collections::HashMap;

const MARGIN: f32 = 40.0;
const TITLE_AREA: f32 = 50.0;
const GROUP_PAD: f32 = 20.0;
const GROUP_HEADER: f32 = 30.0;
const NODE_PAD_X: f32 = 14.0;
const NODE_MIN_W: f32 = 90.0;
const NODE_MAX_W: f32 = 230.0;
const NODE_MIN_H: f32 = 40.0;
const NODE_MAX_H: f32 = 400.0;
const LINK_LABEL_MAX_W: f32 = 180.0;
/// Widest a group's label is set; a longer one is shrunk or cut, so one label cannot stretch
/// the whole diagram.
const GROUP_LABEL_MAX_W: f32 = 320.0;
const ARROW_LEN: f32 = 9.0;
const ARROW_HALF: f32 = 4.5;
const TARGET_ASPECT: f32 = 1.6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    fn x1(&self) -> f32 {
        self.x + self.w
    }

    fn y1(&self) -> f32 {
        self.y + self.h
    }

    fn grow(&self, m: f32) -> Rect {
        Rect::new(self.x - m, self.y - m, self.w + 2.0 * m, self.h + 2.0 * m)
    }

    /// Whether the two boxes share area (touching edges do not count).
    pub(crate) fn overlaps(&self, o: &Rect) -> bool {
        self.x < o.x1() && o.x < self.x1() && self.y < o.y1() && o.y < self.y1()
    }

    pub(crate) fn contains(&self, o: &Rect) -> bool {
        o.x >= self.x && o.y >= self.y && o.x1() <= self.x1() && o.y1() <= self.y1()
    }

    /// Whether the segment `a`–`b` passes through the inside of the box.
    pub(crate) fn hit_by(&self, a: (f32, f32), b: (f32, f32)) -> bool {
        let (lx, hx) = (a.0.min(b.0), a.0.max(b.0));
        let (ly, hy) = (a.1.min(b.1), a.1.max(b.1));
        lx < self.x1() && hx > self.x && ly < self.y1() && hy > self.y
    }
}

pub(crate) struct NodeBox {
    pub rect: Rect,
    pub lines: Vec<String>,
    pub size: f32,
}

pub(crate) struct GroupBox {
    pub text: String,
    pub size: f32,
    pub rect: Rect,
    pub label_rect: Rect,
    pub depth: usize,
}

pub(crate) struct LinkLabel {
    pub text: String,
    pub rect: Rect,
    pub size: f32,
    /// False when no free place for the label was found.
    pub placed: bool,
    /// How many other links' segments the label's plate covers.
    pub covers: usize,
}

pub(crate) struct LinkPath {
    pub points: Vec<(f32, f32)>,
    /// False when no route avoiding the nodes was found and a straight line stands in.
    pub routed: bool,
    pub label: Option<LinkLabel>,
}

/// Everything the drawing needs, in pixels.
pub(crate) struct Scene {
    pub width: f32,
    pub height: f32,
    pub title_area: f32,
    pub nodes: Vec<NodeBox>,
    pub groups: Vec<GroupBox>,
    pub links: Vec<LinkPath>,
}

/// Label sizes, measured once and shared by both orientations.
struct Measured {
    nodes: Vec<(f32, f32, Vec<String>, f32)>,
    /// (text as set, width) per group label.
    group_labels: Vec<(String, f32, f32)>,
    /// (text as set, size, width, height) per link label.
    links: Vec<Option<(String, f32, f32, f32)>>,
}

/// Most characters of a label that are looked at. A box holds a few dozen characters at most,
/// so anything past this is cut, and fitting a longer text only costs time.
const LABEL_MAX_CHARS: usize = 300;

/// `text` cut to [`LABEL_MAX_CHARS`] characters (with an ellipsis), and whether it was cut.
fn bounded(text: &str) -> (String, bool) {
    if text.chars().count() <= LABEL_MAX_CHARS {
        return (text.to_string(), false);
    }
    let cut: String = text.chars().take(LABEL_MAX_CHARS).collect();
    (format!("{cut}…"), true)
}

/// [`font::fit_lines`] for a label of any length, reporting the label as declared.
fn fit_label(
    text: &str,
    font_size: f32,
    min_size: f32,
    max_width: f32,
    max_height: f32,
) -> (Vec<String>, f32) {
    let (cut, was_cut) = bounded(text);
    let (lines, size, outcome) = font::fit_probe(&cut, font_size, min_size, max_width, max_height);
    let outcome = if was_cut {
        font::FitOutcome::Truncated
    } else {
        outcome
    };
    match outcome {
        font::FitOutcome::AsIs => {}
        font::FitOutcome::Shrunk => {
            crate::diagnostic::record(crate::DiagnosticCode::LabelShrunk, text)
        }
        font::FitOutcome::Truncated => {
            crate::diagnostic::record(crate::DiagnosticCode::LabelTruncated, text)
        }
    }
    (lines, size)
}

fn longest_word(text: &str, size: f32) -> f32 {
    text.split_whitespace()
        .map(|w| font::measure_text(w, size))
        .fold(0.0, f32::max)
}

fn measure(d: &SystemDiagram, theme: &Theme) -> Measured {
    let base = theme.typography.label_size;
    let min = theme.typography.label_size_min;
    let max_h = base * font::LINE_HEIGHT * 2.0 + 1.0;
    let nodes = d
        .nodes
        .iter()
        .map(|n| {
            let (label, _) = bounded(&n.label);
            let mut w = NODE_MIN_W;
            let fits = loop {
                let inner = w - 2.0 * NODE_PAD_X;
                let (_, size, outcome) = font::fit_probe(&label, base, min, inner, max_h);
                if outcome == font::FitOutcome::AsIs && longest_word(&label, size) <= inner {
                    break true;
                }
                if w >= NODE_MAX_W {
                    break false;
                }
                w += G;
            };
            if !fits {
                w = NODE_MAX_W;
            }
            let (lines, size) = fit_label(&n.label, base, min, w - 2.0 * NODE_PAD_X, max_h);
            // A node many links meet is as tall as it needs to give each one a slot.
            let degree = d
                .links
                .iter()
                .filter(|l| l.from == n.id || l.to == n.id)
                .count();
            let slots_h = (20.0 + 20.0 * (degree as f32 / 2.0).ceil()).min(NODE_MAX_H);
            let h = snap_up(
                (lines.len() as f32 * size * font::LINE_HEIGHT + 20.0)
                    .max(NODE_MIN_H)
                    .max(slots_h),
            );
            (w, h, lines, size)
        })
        .collect();
    let group_labels = d
        .groups
        .iter()
        .map(|g| {
            let (lines, size) = fit_label(
                &g.label,
                base,
                min,
                GROUP_LABEL_MAX_W,
                base * font::LINE_HEIGHT + 1.0,
            );
            let line = lines.into_iter().next().unwrap_or_default();
            let w = font::measure_text(&line, size) + 8.0;
            (line, w, size)
        })
        .collect();
    let link_size = (base * 0.85).max(min);
    let links = d
        .links
        .iter()
        .map(|l| {
            l.label.as_ref().map(|text| {
                let (lines, size) = fit_label(
                    text,
                    link_size,
                    min * 0.85,
                    LINK_LABEL_MAX_W,
                    link_size * font::LINE_HEIGHT + 1.0,
                );
                let line = lines.into_iter().next().unwrap_or_default();
                let w = font::measure_text(&line, size) + 12.0;
                (line, size, w, size * font::LINE_HEIGHT + 6.0)
            })
        })
        .collect();
    Measured {
        nodes,
        group_labels,
        links,
    }
}

/// A container's laid-out content: where each child sits, and how big the whole is.
struct Placed {
    offsets: Vec<(f32, f32)>,
    w: f32,
    h: f32,
}

struct Builder<'a> {
    d: &'a SystemDiagram,
    m: &'a Measured,
    /// `Some(lr)` lays every level out in that direction; `None` picks per level.
    mode: Option<bool>,
    n: usize,
    /// Children of each container, groups then nodes; the root is the last entry.
    children: Vec<Vec<usize>>,
    /// (container, item a, item b) per link.
    link_level: Vec<(usize, usize, usize)>,
    end_item: Vec<(usize, usize)>,
    placed: Vec<Option<Placed>>,
}

impl<'a> Builder<'a> {
    fn new(d: &'a SystemDiagram, m: &'a Measured, mode: Option<bool>) -> Self {
        let n = d.nodes.len();
        let gc = d.groups.len();
        let mut item_of: HashMap<&str, usize> = HashMap::new();
        for (i, node) in d.nodes.iter().enumerate() {
            item_of.insert(node.id.as_str(), i);
        }
        for (g, group) in d.groups.iter().enumerate() {
            item_of.insert(group.id.as_str(), n + g);
        }
        let root = gc;
        let mut parent: Vec<Option<usize>> = vec![None; n + gc];
        let mut children: Vec<Vec<usize>> = vec![vec![]; gc + 1];
        for (g, group) in d.groups.iter().enumerate() {
            let p = group.parent.as_deref().map(|id| item_of[id] - n);
            parent[n + g] = p.map(|p| n + p);
            children[p.unwrap_or(root)].push(n + g);
        }
        for (i, node) in d.nodes.iter().enumerate() {
            let p = node.group.as_deref().map(|id| item_of[id] - n);
            parent[i] = p.map(|p| n + p);
            children[p.unwrap_or(root)].push(i);
        }
        let chain = |mut item: usize| -> Vec<usize> {
            let mut c = vec![item];
            while let Some(p) = parent[item] {
                c.push(p);
                item = p;
            }
            c.reverse();
            c
        };
        let mut link_level = Vec::new();
        let mut end_item = Vec::new();
        for l in &d.links {
            let (a, b) = (item_of[l.from.as_str()], item_of[l.to.as_str()]);
            end_item.push((a, b));
            let (ca, cb) = (chain(a), chain(b));
            let mut i = 0;
            while i < ca.len() && i < cb.len() && ca[i] == cb[i] {
                i += 1;
            }
            let container = if i == 0 { root } else { ca[i - 1] - n };
            link_level.push((container, ca[i], cb[i]));
        }
        Builder {
            d,
            m,
            mode,
            n,
            children,
            link_level,
            end_item,
            placed: (0..=gc).map(|_| None).collect(),
        }
    }

    fn root(&self) -> usize {
        self.d.groups.len()
    }

    /// A group's box: its content with padding, a header and room for its label.
    fn group_size(&mut self, g: usize) -> (f32, f32) {
        self.place(g);
        let p = self.placed[g].as_ref().expect("placed");
        let w = snap_up((p.w + 2.0 * GROUP_PAD).max(self.m.group_labels[g].1 + 2.0 * GROUP_PAD));
        let h = snap_up(p.h + GROUP_HEADER + GROUP_PAD);
        (w, h)
    }

    fn item_size(&mut self, item: usize) -> (f32, f32) {
        if item < self.n {
            let (w, h, _, _) = &self.m.nodes[item];
            (*w, *h)
        } else {
            self.group_size(item - self.n)
        }
    }

    fn place(&mut self, c: usize) {
        if self.placed[c].is_some() {
            return;
        }
        let kids = self.children[c].clone();
        let sizes: Vec<(f32, f32)> = kids.iter().map(|&k| self.item_size(k)).collect();
        let mut edges = Vec::new();
        for (l, &(container, a, b)) in self.link_level.iter().enumerate() {
            if container != c {
                continue;
            }
            let at = |x: usize| {
                kids.iter()
                    .position(|&k| k == x)
                    .expect("child of container")
            };
            edges.push(LevelEdge {
                a: at(a),
                b: at(b),
                label_w: self.m.links[l].as_ref().map_or(0.0, |x| x.2),
            });
        }
        let lay = match self.mode {
            Some(lr) => layout_level(&sizes, &edges, lr),
            // A level of nodes only reads left to right, so every chain runs the same way. A
            // level that holds groups takes the direction that gives it the better proportions:
            // groups may stack top to bottom while the chain inside each runs left to right.
            None => {
                let a = layout_level(&sizes, &edges, true);
                if kids.iter().all(|&k| k < self.n) {
                    a
                } else {
                    let b = layout_level(&sizes, &edges, false);
                    if shape_cost(&b) < shape_cost(&a) {
                        b
                    } else {
                        a
                    }
                }
            }
        };
        self.placed[c] = Some(Placed {
            offsets: lay.pos,
            w: lay.w,
            h: lay.h,
        });
    }

    /// Fills in absolute boxes below container `c`, whose content starts at `origin`.
    fn assign(
        &mut self,
        c: usize,
        origin: (f32, f32),
        rects: &mut [Rect],
        depth: usize,
        depths: &mut [usize],
    ) {
        let kids = self.children[c].clone();
        for (k, &item) in kids.iter().enumerate() {
            let off = self.placed[c].as_ref().expect("placed").offsets[k];
            let (w, h) = self.item_size(item);
            let (x, y) = (origin.0 + off.0, origin.1 + off.1);
            rects[item] = Rect::new(x, y, w, h);
            if item >= self.n {
                let g = item - self.n;
                depths[g] = depth;
                let inner = self.placed[g].as_ref().expect("placed");
                let px = snap((w - inner.w) / 2.0);
                self.assign(g, (x + px, y + GROUP_HEADER), rects, depth + 1, depths);
            }
        }
    }
}

pub(crate) fn scene(d: &SystemDiagram, theme: &Theme) -> Scene {
    let m = measure(d, theme);
    let left_to_right = arrange(d, &m, Some(true));
    let mixed = arrange(d, &m, None);
    let first = build(d, &m, &left_to_right);
    // A flat diagram lays out the same either way; route it once.
    if mixed == left_to_right {
        return first;
    }
    let second = build(d, &m, &mixed);
    if score(&second) < score(&first) {
        second
    } else {
        first
    }
}

/// How badly a laid-out level is proportioned and how much room it takes — lower is better.
fn shape_cost(l: &layout::LevelLayout) -> f32 {
    if l.w <= 0.0 || l.h <= 0.0 {
        return 0.0;
    }
    (l.w / l.h / TARGET_ASPECT).ln().abs() + 0.4 * (l.w * l.h).ln()
}

/// Where every node and group sits, before any link is routed.
#[derive(PartialEq)]
struct Arranged {
    width: f32,
    height: f32,
    title_area: f32,
    rects: Vec<Rect>,
    depths: Vec<usize>,
    end_item: Vec<(usize, usize)>,
}

fn arrange(d: &SystemDiagram, m: &Measured, mode: Option<bool>) -> Arranged {
    let mut b = Builder::new(d, m, mode);
    let root = b.root();
    b.place(root);
    let title_area = if d.title.is_some() { TITLE_AREA } else { 0.0 };
    let (rw, rh) = {
        let p = b.placed[root].as_ref().expect("placed");
        (p.w, p.h)
    };
    let width = snap_up(rw + 2.0 * MARGIN);
    let height = snap_up(rh + 2.0 * MARGIN + title_area);
    let total = b.n + d.groups.len();
    let mut rects = vec![Rect::new(0.0, 0.0, 0.0, 0.0); total];
    let mut depths = vec![0usize; d.groups.len()];
    b.assign(
        root,
        (MARGIN, MARGIN + title_area),
        &mut rects,
        0,
        &mut depths,
    );
    Arranged {
        width,
        height,
        title_area,
        rects,
        depths,
        end_item: b.end_item,
    }
}

/// Routes the links of an arrangement and places their labels.
fn build(d: &SystemDiagram, m: &Measured, a: &Arranged) -> Scene {
    let Arranged {
        width,
        height,
        title_area,
        rects,
        depths,
        end_item,
    } = a;
    let (width, height, title_area) = (*width, *height, *title_area);
    let n = d.nodes.len();
    let total = n + d.groups.len();

    let nodes: Vec<NodeBox> = (0..n)
        .map(|i| NodeBox {
            rect: rects[i],
            lines: m.nodes[i].2.clone(),
            size: m.nodes[i].3,
        })
        .collect();
    let groups: Vec<GroupBox> = (0..d.groups.len())
        .map(|g| {
            let r = rects[n + g];
            let lw = m.group_labels[g].1;
            GroupBox {
                text: m.group_labels[g].0.clone(),
                size: m.group_labels[g].2,
                rect: r,
                label_rect: Rect::new(r.x + (r.w - lw) / 2.0, r.y + 6.0, lw, 18.0),
                depth: depths[g],
            }
        })
        .collect();

    // Route the links, shortest first.
    let node_rects: Vec<Rect> = nodes.iter().map(|n| n.rect).collect();
    let group_rects: Vec<Rect> = groups.iter().map(|g| g.rect).collect();
    let label_rects: Vec<Rect> = groups.iter().map(|g| g.label_rect).collect();
    let mut router = Router::new(
        width,
        height,
        &Obstacles {
            nodes: &node_rects,
            groups: &group_rects,
            group_labels: &label_rects,
        },
    );
    let centre = |r: &Rect| (r.x + r.w / 2.0, r.y + r.h / 2.0);
    let mut order: Vec<usize> = (0..d.links.len()).collect();
    order.sort_by(|&x, &y| {
        let dist = |l: usize| {
            let (a, b) = end_item[l];
            let (ca, cb) = (centre(&rects[a]), centre(&rects[b]));
            (ca.0 - cb.0).abs() + (ca.1 - cb.1).abs()
        };
        dist(x).total_cmp(&dist(y)).then(x.cmp(&y))
    });
    let mut taken: Vec<Vec<u8>> = vec![vec![]; total];
    let mut paths: Vec<Option<Vec<(f32, f32)>>> = vec![None; d.links.len()];
    for &l in &order {
        let (a, bi) = end_item[l];
        let (mut ta, mut tb) = (
            std::mem::take(&mut taken[a]),
            std::mem::take(&mut taken[bi]),
        );
        paths[l] = router.as_mut().and_then(|r| {
            r.route(
                &End { rect: rects[a] },
                &mut ta,
                &End { rect: rects[bi] },
                &mut tb,
            )
        });
        taken[a] = ta;
        taken[bi] = tb;
    }
    let mut links: Vec<LinkPath> = (0..d.links.len())
        .map(|l| match paths[l].take() {
            Some(points) => LinkPath {
                points,
                routed: true,
                label: None,
            },
            None => {
                let (a, bi) = end_item[l];
                LinkPath {
                    points: vec![centre(&rects[a]), centre(&rects[bi])],
                    routed: false,
                    label: None,
                }
            }
        })
        .collect();

    // Labels, once every path is known.
    let canvas = Rect::new(0.0, 0.0, width, height);
    let mut placed_labels: Vec<Rect> = Vec::new();
    for l in 0..links.len() {
        let Some((text, size, w, h)) = m.links[l].clone() else {
            continue;
        };
        let (rect, ok, covers) = place_label(
            l,
            &links,
            w,
            h,
            &node_rects,
            &label_rects,
            &placed_labels,
            &canvas,
        );
        placed_labels.push(rect);
        links[l].label = Some(LinkLabel {
            text,
            rect,
            size,
            placed: ok,
            covers,
        });
    }

    Scene {
        width,
        height,
        title_area,
        nodes,
        groups,
        links,
    }
}

/// Finds a place for link `l`'s label on its path: free of nodes, group labels and other labels,
/// away from other links, and on the longest stretch where it can.
#[allow(clippy::too_many_arguments)]
fn place_label(
    l: usize,
    links: &[LinkPath],
    w: f32,
    h: f32,
    nodes: &[Rect],
    group_labels: &[Rect],
    placed: &[Rect],
    canvas: &Rect,
) -> (Rect, bool, usize) {
    let pts = &links[l].points;
    let mut best: Option<(f32, Rect, bool, usize)> = None;
    for s in pts.windows(2) {
        let (a, b) = (s[0], s[1]);
        let len = (b.0 - a.0).abs() + (b.1 - a.1).abs();
        if len < 20.0 {
            continue;
        }
        for t in [0.5f32, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8] {
            let (cx, cy) = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
            // Keep clear of the arrowheads at both ends of the path.
            let from_end = |p: (f32, f32)| (cx - p.0).abs() + (cy - p.1).abs();
            if from_end(pts[0]) < 14.0 || from_end(pts[pts.len() - 1]) < 14.0 {
                continue;
            }
            let rect = Rect::new(cx - w / 2.0, cy - h / 2.0, w, h);
            let mut hard = 0.0f32;
            if !canvas.contains(&rect) {
                hard += 1.0;
            }
            let padded = rect.grow(2.0);
            hard += nodes.iter().filter(|r| r.overlaps(&padded)).count() as f32;
            hard += group_labels.iter().filter(|r| r.overlaps(&padded)).count() as f32;
            hard += placed.iter().filter(|r| r.overlaps(&padded)).count() as f32;
            let mut soft = (1.0 - (len.min(200.0) / 200.0)) * 20.0 + (t - 0.5).abs() * 10.0;
            let mut covers = 0;
            for (k, other) in links.iter().enumerate() {
                for seg in other.points.windows(2) {
                    if rect.hit_by(seg[0], seg[1]) && k != l {
                        soft += 40.0;
                        covers += 1;
                    }
                }
            }
            // A label right against an arrowhead reads as part of it.
            let clear = rect.grow(ARROW_LEN + 4.0);
            for end in [pts[0], pts[pts.len() - 1]] {
                if end.0 > clear.x && end.0 < clear.x1() && end.1 > clear.y && end.1 < clear.y1() {
                    soft += 60.0;
                }
            }
            for p in &pts[1..pts.len() - 1] {
                if p.0 > rect.x && p.0 < rect.x1() && p.1 > rect.y && p.1 < rect.y1() {
                    soft += 30.0;
                }
            }
            let score = hard * 1000.0 + soft;
            if best.as_ref().is_none_or(|(s, _, _, _)| score < *s) {
                best = Some((score, rect, hard == 0.0, covers));
            }
        }
    }
    match best {
        Some((_, rect, ok, covers)) => (rect, ok, covers),
        None => {
            let a = pts[0];
            let b = pts[pts.len() - 1];
            let (cx, cy) = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            (Rect::new(cx - w / 2.0, cy - h / 2.0, w, h), false, 0)
        }
    }
}

/// How good a laid-out scene is — lower is better.
fn score(s: &Scene) -> f32 {
    let unrouted = s.links.iter().filter(|l| !l.routed).count() as f32;
    let unplaced = s
        .links
        .iter()
        .filter(|l| l.label.as_ref().is_some_and(|x| !x.placed))
        .count() as f32;
    let covered: usize = s
        .links
        .iter()
        .filter_map(|l| l.label.as_ref().map(|x| x.covers))
        .sum();
    let bends: usize = s
        .links
        .iter()
        .map(|l| l.points.len().saturating_sub(2))
        .sum();
    let length: f32 = s
        .links
        .iter()
        .map(|l| {
            l.points
                .windows(2)
                .map(|w| (w[1].0 - w[0].0).abs() + (w[1].1 - w[0].1).abs())
                .sum::<f32>()
        })
        .sum();
    let aspect = (s.width / s.height / TARGET_ASPECT).ln().abs();
    unrouted * 1e5
        + unplaced * 1e4
        + covered as f32 * 2e3
        + crossings(s) as f32 * 400.0
        + bends as f32 * 5.0
        + length * 0.1
        + aspect * 500.0
}

/// Pairs of link segments that cross each other.
pub(crate) fn crossings(s: &Scene) -> usize {
    let mut count = 0;
    for (i, a) in s.links.iter().enumerate() {
        for b in &s.links[i + 1..] {
            for sa in a.points.windows(2) {
                for sb in b.points.windows(2) {
                    if segments_cross(sa[0], sa[1], sb[0], sb[1]) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

/// Whether two axis-aligned segments cross properly (an end touching does not count).
fn segments_cross(a0: (f32, f32), a1: (f32, f32), b0: (f32, f32), b1: (f32, f32)) -> bool {
    let a_h = a0.1 == a1.1;
    let b_h = b0.1 == b1.1;
    if a_h == b_h {
        return false;
    }
    let (h0, h1, v0, v1) = if a_h {
        (a0, a1, b0, b1)
    } else {
        (b0, b1, a0, a1)
    };
    let x = v0.0;
    let y = h0.1;
    x > h0.0.min(h1.0) && x < h0.0.max(h1.0) && y > v0.1.min(v1.1) && y < v0.1.max(v1.1)
}

pub(crate) fn render(d: &SystemDiagram, theme: &Theme) -> String {
    let scene = scene(d, theme);
    let mut b = SvgBuilder::new(scene.width, scene.height);
    let line = theme.layers.apex.to_hex();

    if let Some(title) = &d.title {
        b.text(
            scene.width / 2.0,
            scene.title_area / 2.0 + 10.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    // Groups, outermost first, so nested groups sit on top of their parents.
    let mut order: Vec<usize> = (0..scene.groups.len()).collect();
    order.sort_by_key(|&g| (scene.groups[g].depth, g));
    for g in order {
        let gb = &scene.groups[g];
        let mix = 0.9 - 0.07 * gb.depth as f32;
        let fill = theme.layers.base.interpolate(&theme.background, mix);
        let stroke = theme.layers.base.interpolate(&theme.background, 0.45);
        b.rect_styled(
            gb.rect.x,
            gb.rect.y,
            gb.rect.w,
            gb.rect.h,
            12,
            &fill.to_hex(),
            &stroke.to_hex(),
            1.5,
            false,
        );
    }
    for gb in &scene.groups {
        b.text_weighted(
            gb.label_rect.x + gb.label_rect.w / 2.0,
            gb.label_rect.y + gb.label_rect.h / 2.0,
            &gb.text,
            &theme.title_color.to_hex(),
            gb.size,
            true,
        );
    }

    // Links.
    for (l, lp) in scene.links.iter().enumerate() {
        let link = &d.links[l];
        let dashed = link.relation == LinkRelation::Indirect;
        // The line is broken where the label sits, so the label needs no plate behind it and
        // reads on any page colour.
        match lp
            .label
            .as_ref()
            .and_then(|x| open_gap(&lp.points, &x.rect))
        {
            Some((before, after)) => {
                b.polyline(&before, &line, 1.5, dashed);
                b.polyline(&after, &line, 1.5, dashed);
            }
            None => b.polyline(&lp.points, &line, 1.5, dashed),
        }
        let n = lp.points.len();
        if n >= 2 {
            if link.direction != LinkDirection::None {
                arrowhead(&mut b, lp.points[n - 1], lp.points[n - 2], &line);
            }
            if link.direction == LinkDirection::Both {
                arrowhead(&mut b, lp.points[0], lp.points[1], &line);
            }
        }
    }

    // Nodes.
    for (i, nb) in scene.nodes.iter().enumerate() {
        let node = &d.nodes[i];
        let r = nb.rect;
        let mut fill = theme.layers.apex.interpolate(&theme.layers.base, 0.3);
        if matches!(node.emphasis, Some(Emphasis::Secondary)) {
            fill = fill.interpolate(&theme.layers.base, 0.3);
        }
        let mut stroke = theme.layers.apex.to_hex();
        let mut stroke_w = 1.5;
        if matches!(node.emphasis, Some(Emphasis::Primary)) {
            stroke = theme.background.to_hex();
            stroke_w = 3.0;
        }
        match node.role {
            Some(SystemRole::Store) => {
                fill = theme.layers.base.interpolate(&theme.background, 0.5);
                let e = 8.0;
                let (x, y, w, h) = (r.x, r.y, r.w, r.h);
                let body = format!(
                    "M{:.1},{:.1} a{:.1},{e:.1} 0 0 1 {:.1},0 v{:.1} a{:.1},{e:.1} 0 0 1 -{:.1},0 z",
                    x,
                    y + e,
                    w / 2.0,
                    w,
                    h - 2.0 * e,
                    w / 2.0,
                    w,
                );
                let lid = format!(
                    "M{:.1},{:.1} a{:.1},{e:.1} 0 0 0 {:.1},0",
                    x,
                    y + e,
                    w / 2.0,
                    w
                );
                b.path(&body, &fill.to_hex(), &stroke, stroke_w, false);
                b.path(&lid, "none", &stroke, stroke_w, false);
            }
            Some(SystemRole::External) => {
                fill = theme.background.interpolate(&theme.layers.base, 0.12);
                b.rect_styled(
                    r.x,
                    r.y,
                    r.w,
                    r.h,
                    10,
                    &fill.to_hex(),
                    &stroke,
                    stroke_w,
                    true,
                );
            }
            Some(SystemRole::Actor) => {
                b.rect_styled(
                    r.x,
                    r.y,
                    r.w,
                    r.h,
                    (r.h / 2.0) as u32,
                    &fill.to_hex(),
                    &stroke,
                    stroke_w,
                    false,
                );
            }
            None => {
                b.rect_styled(
                    r.x,
                    r.y,
                    r.w,
                    r.h,
                    10,
                    &fill.to_hex(),
                    &stroke,
                    stroke_w,
                    false,
                );
            }
        }
        let text_color = theme.text_on(&fill);
        let bold = matches!(node.emphasis, Some(Emphasis::Primary));
        let cy = if matches!(node.role, Some(SystemRole::Store)) {
            r.y + r.h / 2.0 + 4.0
        } else {
            r.y + r.h / 2.0
        };
        b.text_lines(
            r.x + r.w / 2.0,
            cy,
            &nb.lines,
            &text_color.to_hex(),
            nb.size,
            Some(bold),
        );
        status::draw_marker(&mut b, &node.status, r.x + r.w, r.y, theme);
    }

    // Link labels sit in the gap left in their link.
    for lp in &scene.links {
        if let Some(label) = &lp.label {
            let r = label.rect;
            b.text(
                r.x + r.w / 2.0,
                r.y + r.h / 2.0,
                &label.text,
                &theme.title_color.to_hex(),
                label.size,
            );
        }
    }
    b.build(theme.backdrop_fill().as_deref())
}

/// Splits `points` into the part before and the part after `rect`, which the path crosses on one
/// straight segment. `None` when no segment can be cut (a diagonal stand-in line).
type Polyline = Vec<(f32, f32)>;

fn open_gap(points: &[(f32, f32)], rect: &Rect) -> Option<(Polyline, Polyline)> {
    const PAD: f32 = 3.0;
    for (i, s) in points.windows(2).enumerate() {
        let (a, b) = (s[0], s[1]);
        if !rect.hit_by(a, b) {
            continue;
        }
        let horizontal = a.1 == b.1;
        let vertical = a.0 == b.0;
        if !horizontal && !vertical {
            return None;
        }
        let (lo, hi, from, to) = if horizontal {
            (rect.x - PAD, rect.x1() + PAD, a.0, b.0)
        } else {
            (rect.y - PAD, rect.y1() + PAD, a.1, b.1)
        };
        // Walking from `a` to `b`, the line ends at the near side of the gap and resumes at the far.
        let (near, far) = if from <= to { (lo, hi) } else { (hi, lo) };
        let at = |v: f32| if horizontal { (v, a.1) } else { (a.0, v) };
        let mut before: Vec<(f32, f32)> = points[..=i].to_vec();
        before.push(at(near.clamp(from.min(to), from.max(to))));
        let mut after: Vec<(f32, f32)> = vec![at(far.clamp(from.min(to), from.max(to)))];
        after.extend_from_slice(&points[i + 1..]);
        return Some((before, after));
    }
    None
}

/// A filled arrowhead whose tip is `tip`, pointing away from `from`.
fn arrowhead(b: &mut SvgBuilder, tip: (f32, f32), from: (f32, f32), color: &str) {
    let (dx, dy) = (tip.0 - from.0, tip.1 - from.1);
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return;
    }
    let (ux, uy) = (dx / len, dy / len);
    let base = (tip.0 - ux * ARROW_LEN, tip.1 - uy * ARROW_LEN);
    let (px, py) = (-uy * ARROW_HALF, ux * ARROW_HALF);
    b.polygon(
        &[tip, (base.0 + px, base.1 + py), (base.0 - px, base.1 - py)],
        color,
        "none",
    );
}

#[cfg(test)]
mod tests;
