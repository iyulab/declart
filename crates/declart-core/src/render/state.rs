//! State diagram: the states sit on one row; every transition is drawn as an arc above (forward)
//! or below (backward) the row, so opposite and non-adjacent transitions never share a line.
//! A node grows to hold its whole label, and arcs whose labels would collide take separate
//! lanes.
use super::{font, svg::SvgBuilder};
use crate::diagnostic::{self, DiagnosticCode};
use crate::model::{StateDiagram, StateRole, TransitionKind};
use crate::render::theme::Theme;

use font::FitOutcome;

const R_MIN: f32 = 36.0;
const R_MAX: f32 = 68.0;
const R_STEP: f32 = 4.0;
const NODE_GAP_MIN: f32 = 64.0;
const NODE_GAP_MAX: f32 = 130.0;
const TITLE_AREA: f32 = 50.0;
const PADDING: f32 = 30.0;
const INITIAL_DOT_R: f32 = 10.0;
const INITIAL_ARROW_LEN: f32 = 30.0;
/// Height of the first lane's arc apex above the tallest node.
const LANE_BASE: f32 = 14.0;
/// Vertical distance between neighbouring lanes.
const LANE_STEP: f32 = 40.0;
/// Distance from an arc apex to the centre of its label.
const LABEL_RISE: f32 = 12.0;
/// Clearance kept between two labels in the same lane.
const LABEL_CLEARANCE: f32 = 10.0;
/// Angle, from the horizontal, at which an arc leaves and enters a node.
const ARC_ANGLE_DEG: f32 = 55.0;

#[derive(Debug, Clone)]
struct Node {
    cx: f32,
    r: f32,
    lines: Vec<String>,
    size: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Above,
    Below,
}

#[derive(Debug, Clone)]
enum Shape {
    /// A direct line between two adjacent nodes — the only transition between them.
    Straight,
    Arc,
    Loop,
}

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    side: Side,
    lane: usize,
    shape: Shape,
    exception: bool,
    trigger: Option<String>,
    /// Label centre and width, when the transition has a trigger.
    label: Option<(f32, f32, f32)>,
}

struct Layout {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    row_y: f32,
    width: f32,
    height: f32,
    title_area: f32,
    label_size: f32,
}

struct Transition {
    from: usize,
    to: usize,
    trigger: Option<String>,
    exception: bool,
}

pub(crate) fn render(d: &StateDiagram, theme: &Theme) -> String {
    let layout = layout(d, theme);
    let mut b = SvgBuilder::new(layout.width, layout.height);

    if let Some(title) = &d.title {
        b.text(
            layout.width / 2.0,
            layout.title_area / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    let n = layout.nodes.len();
    let row_y = layout.row_y;

    // Transitions first (circles render on top of the arc ends).
    for e in &layout.edges {
        let color = if e.exception {
            theme
                .layers
                .base
                .interpolate(&theme.background, 0.3)
                .to_hex()
        } else {
            theme.layers.apex.to_hex()
        };
        draw_edge(&mut b, &layout, e, &color);
    }

    // State circles.
    for (i, (state, node)) in d.states.iter().zip(&layout.nodes).enumerate() {
        let is_terminal = matches!(state.role, Some(StateRole::Terminal));
        let t = if n > 1 {
            i as f32 / (n - 1) as f32
        } else {
            0.0
        };
        let fill = theme.layers.apex.interpolate(&theme.layers.base, t);
        let text_col = &theme.text_on(&fill);

        if is_terminal {
            b.circle_stroked(
                node.cx,
                row_y,
                node.r + 5.0,
                "none",
                &theme.layers.apex.to_hex(),
                2.0,
            );
        }
        b.circle_stroked(
            node.cx,
            row_y,
            node.r,
            &fill.to_hex(),
            &theme.layers.apex.to_hex(),
            1.5,
        );
        b.text_lines(
            node.cx,
            row_y,
            &node.lines,
            &text_col.to_hex(),
            node.size,
            None,
        );
    }

    // Trigger labels sit above everything else.
    for e in &layout.edges {
        if let (Some(trigger), Some((x, y, _))) = (&e.trigger, e.label) {
            b.text(
                x,
                y,
                trigger,
                &theme.title_color.to_hex(),
                layout.label_size,
            );
        }
    }

    // Initial state marker: filled dot → arrow → first initial state.
    if let Some(init_idx) = d
        .states
        .iter()
        .position(|s| matches!(s.role, Some(StateRole::Initial)))
    {
        let node = &layout.nodes[init_idx];
        let dot_x = node.cx - node.r - INITIAL_ARROW_LEN - INITIAL_DOT_R;
        b.circle_stroked(
            dot_x,
            row_y,
            INITIAL_DOT_R,
            &theme.layers.apex.to_hex(),
            "none",
            0.0,
        );
        let arrow_end_x = node.cx - node.r;
        let shaft_end = arrow_end_x - 9.0;
        if dot_x + INITIAL_DOT_R < shaft_end {
            b.line(
                dot_x + INITIAL_DOT_R,
                row_y,
                shaft_end,
                row_y,
                &theme.layers.apex.to_hex(),
                1.5,
            );
        }
        draw_arrowhead(
            &mut b,
            arrow_end_x,
            row_y,
            dot_x,
            row_y,
            &theme.layers.apex.to_hex(),
        );
    }

    b.build(theme.backdrop_fill().as_deref())
}

fn layout(d: &StateDiagram, theme: &Theme) -> Layout {
    let n = d.states.len();
    let label_size = theme.typography.label_size - 2.0;

    let transitions: Vec<Transition> = d
        .transitions
        .iter()
        .filter_map(|t| {
            Some(Transition {
                from: resolve_idx(d, &t.from)?,
                to: resolve_idx(d, &t.to)?,
                trigger: t.trigger.clone(),
                exception: matches!(t.kind, TransitionKind::Exception),
            })
        })
        .collect();

    // Each node grows until its whole label fits.
    let mut nodes: Vec<Node> = d.states.iter().map(|s| fit_node(&s.label, theme)).collect();
    let r_row = nodes.iter().map(|n| n.r).fold(R_MIN, f32::max);

    // Horizontal placement: the gap after a node holds the widest label of a transition
    // between the pair (capped), and room for the initial marker before the initial state.
    let label_w = |t: &Transition| {
        t.trigger
            .as_deref()
            .map_or(0.0, |s| font::measure_text(s, label_size))
    };
    let init_idx = d
        .states
        .iter()
        .position(|s| matches!(s.role, Some(StateRole::Initial)));
    let mut gaps = vec![NODE_GAP_MIN; n.saturating_sub(1)];
    for t in &transitions {
        if t.from != t.to && t.from.abs_diff(t.to) == 1 {
            let k = t.from.min(t.to);
            gaps[k] = gaps[k].max(label_w(t).min(NODE_GAP_MAX));
        }
    }
    if let Some(i) = init_idx {
        if i > 0 {
            gaps[i - 1] += 2.0 * INITIAL_DOT_R + INITIAL_ARROW_LEN;
        }
    }
    let mut x = 0.0;
    for i in 0..n {
        if i > 0 {
            x += nodes[i - 1].r + gaps[i - 1] + nodes[i].r;
        }
        nodes[i].cx = x;
    }

    // Edges: shape and side first, then lanes so that colliding labels and arcs are stacked.
    let mut pair_count = std::collections::HashMap::<(usize, usize), usize>::new();
    for t in &transitions {
        let key = (t.from.min(t.to), t.from.max(t.to));
        *pair_count.entry(key).or_default() += 1;
    }
    let mut edges: Vec<Edge> = transitions
        .iter()
        .map(|t| {
            let key = (t.from.min(t.to), t.from.max(t.to));
            let shape = if t.from == t.to {
                Shape::Loop
            } else if t.from.abs_diff(t.to) == 1 && pair_count[&key] == 1 {
                Shape::Straight
            } else {
                Shape::Arc
            };
            let side = if t.from >= t.to && !matches!(shape, Shape::Loop) {
                Side::Below
            } else {
                Side::Above
            };
            Edge {
                from: t.from,
                to: t.to,
                side,
                lane: 0,
                shape,
                exception: t.exception,
                trigger: t.trigger.clone(),
                label: None,
            }
        })
        .collect();

    // Extent along x that an edge occupies in its lane: the arc and its label.
    let extent = |e: &Edge| -> (f32, f32) {
        let (a, b) = match e.shape {
            Shape::Loop => (
                nodes[e.from].cx - 0.8 * nodes[e.from].r,
                nodes[e.from].cx + 0.8 * nodes[e.from].r,
            ),
            Shape::Straight => {
                let mid = (nodes[e.from].cx + nodes[e.to].cx) / 2.0;
                (mid, mid)
            }
            Shape::Arc => {
                let (l, r) = (
                    nodes[e.from].cx.min(nodes[e.to].cx),
                    nodes[e.from].cx.max(nodes[e.to].cx),
                );
                (l, r)
            }
        };
        let mid = (nodes[e.from].cx + nodes[e.to].cx) / 2.0;
        let w = e
            .trigger
            .as_deref()
            .map_or(0.0, |s| font::measure_text(s, label_size));
        if w > 0.0 {
            (a.min(mid - w / 2.0), b.max(mid + w / 2.0))
        } else {
            (a, b)
        }
    };
    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| {
        let e = &edges[i];
        (
            matches!(e.side, Side::Below),
            e.from.abs_diff(e.to),
            e.from.min(e.to),
            i,
        )
    });
    let mut placed: Vec<usize> = Vec::new();
    for &i in &order {
        let (lo, hi) = extent(&edges[i]);
        let mut lane = 0;
        loop {
            let clash = placed.iter().any(|&j| {
                let o = &edges[j];
                if o.side != edges[i].side || o.lane != lane {
                    return false;
                }
                let (olo, ohi) = extent(o);
                lo < ohi + LABEL_CLEARANCE && olo < hi + LABEL_CLEARANCE
            });
            if !clash {
                break;
            }
            lane += 1;
        }
        edges[i].lane = lane;
        placed.push(i);
    }

    // Vertical extent of the lanes on each side.
    let lane_height = |lane: usize| r_row + LANE_BASE + lane as f32 * LANE_STEP;
    let top_lanes = edges
        .iter()
        .filter(|e| e.side == Side::Above)
        .map(|e| e.lane + 1)
        .max();
    let bottom_lanes = edges
        .iter()
        .filter(|e| e.side == Side::Below)
        .map(|e| e.lane + 1)
        .max();
    let reach = |lanes: Option<usize>| match lanes {
        Some(l) => lane_height(l - 1) + LABEL_RISE + label_size,
        None => 0.0,
    };
    let title_area = if d.title.is_some() { TITLE_AREA } else { 0.0 };
    let top = (r_row + 8.0).max(reach(top_lanes));
    let bottom = (r_row + 8.0).max(reach(bottom_lanes));
    let row_y = title_area + PADDING + top;

    // Label positions.
    for e in &mut edges {
        let Some(trigger) = &e.trigger else { continue };
        let w = font::measure_text(trigger, label_size);
        let mid = (nodes[e.from].cx + nodes[e.to].cx) / 2.0;
        let apex = lane_height(e.lane) + LABEL_RISE;
        let y = match e.side {
            Side::Above => row_y - apex,
            Side::Below => row_y + apex,
        };
        e.label = Some((mid, y, w));
    }

    // Shift so the leftmost thing (a label, a node, the initial marker) sits at PADDING.
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    for nd in &nodes {
        min_x = min_x.min(nd.cx - nd.r - 5.0);
        max_x = max_x.max(nd.cx + nd.r + 5.0);
    }
    if let Some(i) = init_idx {
        min_x = min_x.min(nodes[i].cx - nodes[i].r - INITIAL_ARROW_LEN - 2.0 * INITIAL_DOT_R);
    }
    for e in &edges {
        if let Some((x, _, w)) = e.label {
            min_x = min_x.min(x - w / 2.0);
            max_x = max_x.max(x + w / 2.0);
        }
    }
    let shift = PADDING - min_x;
    for nd in &mut nodes {
        nd.cx += shift;
    }
    for e in &mut edges {
        if let Some((x, _, _)) = &mut e.label {
            *x += shift;
        }
    }
    let width = (max_x - min_x + 2.0 * PADDING).max(400.0);
    // Centre the diagram when the minimum width applies.
    let extra = (width - (max_x - min_x + 2.0 * PADDING)) / 2.0;
    if extra > 0.0 {
        for nd in &mut nodes {
            nd.cx += extra;
        }
        for e in &mut edges {
            if let Some((x, _, _)) = &mut e.label {
                *x += extra;
            }
        }
    }
    let height = row_y + bottom + PADDING;

    Layout {
        nodes,
        edges,
        row_y,
        width,
        height,
        title_area,
        label_size,
    }
}

/// The smallest circle that holds the whole label, wrapped if it helps; the label is shrunk or
/// cut short only when even the largest circle cannot hold it.
fn fit_node(label: &str, theme: &Theme) -> Node {
    let size = theme.typography.label_size;
    let min = theme.typography.label_size_min;
    // The first circle that holds the label without breaking a word in the middle; failing
    // that, the first that holds it at all; failing that, the largest.
    let longest_word = label
        .split_whitespace()
        .map(|w| font::measure_text(w, size))
        .fold(0.0_f32, f32::max);
    let mut fallback: Option<(f32, Vec<String>, f32, FitOutcome)> = None;
    let mut chosen: Option<(f32, Vec<String>, f32, FitOutcome)> = None;
    let mut r = R_MIN;
    while r <= R_MAX {
        let (lines, s, outcome) = font::fit_probe(label, size, min, r * 1.5, r * 1.2);
        if outcome != FitOutcome::Truncated {
            if outcome == FitOutcome::AsIs && longest_word <= r * 1.5 {
                chosen = Some((r, lines, s, outcome));
                break;
            }
            fallback.get_or_insert((r, lines, s, outcome));
        }
        r += R_STEP;
    }
    let (r, lines, s, outcome) = chosen.or(fallback).unwrap_or_else(|| {
        let (lines, s, outcome) = font::fit_probe(label, size, min, R_MAX * 1.5, R_MAX * 1.2);
        (R_MAX, lines, s, outcome)
    });
    match outcome {
        FitOutcome::AsIs => {}
        FitOutcome::Shrunk => diagnostic::record(DiagnosticCode::LabelShrunk, label),
        FitOutcome::Truncated => diagnostic::record(DiagnosticCode::LabelTruncated, label),
    }
    Node {
        cx: 0.0,
        r,
        lines,
        size: s,
    }
}

fn draw_edge(b: &mut SvgBuilder, l: &Layout, e: &Edge, color: &str) {
    let (from, to) = (&l.nodes[e.from], &l.nodes[e.to]);
    let row_y = l.row_y;
    match e.shape {
        Shape::Straight => {
            let (sx, sy) = circle_edge(from.cx, row_y, to.cx - from.cx, 0.0, from.r);
            let (ex, ey) = circle_edge(to.cx, row_y, from.cx - to.cx, 0.0, to.r);
            if e.exception {
                b.line_dashed(sx, sy, ex, ey, color, 1.5);
            } else {
                b.line(sx, sy, ex, ey, color, 1.5);
            }
            draw_arrowhead(b, ex, ey, sx, sy, color);
        }
        Shape::Arc => {
            let dir = if to.cx >= from.cx { 1.0 } else { -1.0 };
            let up = if e.side == Side::Above { -1.0 } else { 1.0 };
            let a = (ARC_ANGLE_DEG + 10.0 * e.lane.min(3) as f32).to_radians();
            let p0 = (
                from.cx + dir * from.r * a.cos(),
                row_y + up * from.r * a.sin(),
            );
            let p1 = (to.cx - dir * to.r * a.cos(), row_y + up * to.r * a.sin());
            let apex = row_y
                + up * (l.nodes.iter().map(|n| n.r).fold(R_MIN, f32::max)
                    + LANE_BASE
                    + e.lane as f32 * LANE_STEP);
            let cy = (apex - 0.25 * (p0.1 + p1.1) / 2.0) / 0.75;
            let k = (0.04 * (p1.0 - p0.0).abs()).max(8.0);
            let c1 = (p0.0 + dir * k, cy);
            let c2 = (p1.0 - dir * k, cy);
            b.path_cubic([p0, c1, c2, p1], color, 1.5, e.exception);
            draw_arrowhead(b, p1.0, p1.1, c2.0, c2.1, color);
        }
        Shape::Loop => {
            let r = from.r;
            let p0 = (from.cx - 0.28 * r, row_y - 0.96 * r);
            let p1 = (from.cx + 0.28 * r, row_y - 0.96 * r);
            let apex = row_y
                - (l.nodes.iter().map(|n| n.r).fold(R_MIN, f32::max)
                    + LANE_BASE
                    + e.lane as f32 * LANE_STEP);
            let cy = (apex - 0.25 * p0.1) / 0.75;
            let c1 = (from.cx - 0.6 * r, cy);
            let c2 = (from.cx + 0.6 * r, cy);
            b.path_cubic([p0, c1, c2, p1], color, 1.5, e.exception);
            draw_arrowhead(b, p1.0, p1.1, c2.0, c2.1, color);
        }
    }
}

fn resolve_idx(d: &StateDiagram, key: &str) -> Option<usize> {
    d.states
        .iter()
        .position(|s| s.id.as_deref() == Some(key))
        .or_else(|| d.states.iter().position(|s| s.label == key))
}

fn circle_edge(cx: f32, cy: f32, dx: f32, dy: f32, r: f32) -> (f32, f32) {
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    (cx + dx / len * r, cy + dy / len * r)
}

fn draw_arrowhead(b: &mut SvgBuilder, tx: f32, ty: f32, sx: f32, sy: f32, color: &str) {
    let dx = tx - sx;
    let dy = ty - sy;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let head = 9.0_f32;
    let ux = dx / len;
    let uy = dy / len;
    let bx = tx - ux * head;
    let by = ty - uy * head;
    let px = -uy * head * 0.4;
    let py = ux * head * 0.4;
    b.polygon(
        &[(bx + px, by + py), (tx, ty), (bx - px, by - py)],
        color,
        "none",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{StateDiagram, StateNode, StateRole, StateTransition, TransitionKind};
    use crate::render::DEFAULT_THEME;

    fn two_state() -> StateDiagram {
        StateDiagram {
            title: Some("Order".to_string()),
            states: vec![
                StateNode {
                    label: "Idle".to_string(),
                    id: Some("idle".to_string()),
                    role: Some(StateRole::Initial),
                },
                StateNode {
                    label: "Done".to_string(),
                    id: None,
                    role: Some(StateRole::Terminal),
                },
            ],
            transitions: vec![StateTransition {
                from: "idle".to_string(),
                to: "Done".to_string(),
                trigger: Some("finish".to_string()),
                kind: TransitionKind::Normal,
            }],
        }
    }

    /// The TCP congestion-control lifecycle: three states, six transitions, opposite and
    /// non-adjacent pairs among them.
    fn tcp() -> StateDiagram {
        let state = |label: &str, role: Option<StateRole>| StateNode {
            label: label.to_string(),
            id: None,
            role,
        };
        let tr = |from: &str, to: &str, trigger: &str| StateTransition {
            from: from.to_string(),
            to: to.to_string(),
            trigger: Some(trigger.to_string()),
            kind: TransitionKind::Normal,
        };
        StateDiagram {
            title: Some("혼잡제어".to_string()),
            states: vec![
                state("느린 시작", Some(StateRole::Initial)),
                state("혼잡 회피", None),
                state("빠른 회복", None),
            ],
            transitions: vec![
                tr("느린 시작", "혼잡 회피", "cwnd ≥ ssthresh"),
                tr("느린 시작", "빠른 회복", "중복 ACK 3개"),
                tr("혼잡 회피", "빠른 회복", "중복 ACK 3개"),
                tr("빠른 회복", "혼잡 회피", "새 ACK 수신"),
                tr("혼잡 회피", "느린 시작", "타임아웃"),
                tr("빠른 회복", "느린 시작", "타임아웃"),
                tr("느린 시작", "느린 시작", "타임아웃"),
            ],
        }
    }

    #[test]
    fn render_produces_svg() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_state_labels() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.contains("Idle"));
        assert!(svg.contains("Done"));
    }

    #[test]
    fn render_includes_trigger() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.contains("finish"));
    }

    #[test]
    fn render_includes_title() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.contains("Order"));
    }

    #[test]
    fn a_lone_adjacent_transition_stays_a_straight_line() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(!svg.contains("<path"), "{svg}");
    }

    #[test]
    fn long_labels_are_set_whole() {
        let (svg, diags) = diagnostic::collect(|| render(&tcp(), &DEFAULT_THEME));
        assert!(diags.is_empty(), "{diags:?}");
        assert!(!svg.contains('…'));
        assert!(svg.contains("느린 시작") || svg.contains("느린"));
    }

    #[test]
    fn a_node_grows_to_hold_a_long_english_label() {
        let mut d = two_state();
        d.states[1].label = "Awaiting approval".to_string();
        let (_, diags) = diagnostic::collect(|| render(&d, &DEFAULT_THEME));
        assert!(
            !diags
                .iter()
                .any(|g| g.code == DiagnosticCode::LabelTruncated),
            "{diags:?}"
        );
    }

    #[test]
    fn opposite_transitions_take_opposite_sides() {
        let l = layout(&tcp(), &DEFAULT_THEME);
        let find = |from: usize, to: usize| {
            l.edges
                .iter()
                .find(|e| e.from == from && e.to == to)
                .unwrap()
        };
        assert_eq!(find(1, 2).side, Side::Above);
        assert_eq!(find(2, 1).side, Side::Below);
        assert_eq!(find(0, 1).side, Side::Above);
        assert_eq!(find(1, 0).side, Side::Below);
    }

    #[test]
    fn labels_never_overlap() {
        let l = layout(&tcp(), &DEFAULT_THEME);
        let rects: Vec<(f32, f32, f32)> = l.edges.iter().filter_map(|e| e.label).collect();
        let h = l.label_size * 1.2;
        for (i, a) in rects.iter().enumerate() {
            for b in &rects[i + 1..] {
                let apart_x = (a.0 - b.0).abs() >= (a.2 + b.2) / 2.0;
                let apart_y = (a.1 - b.1).abs() >= h;
                assert!(apart_x || apart_y, "labels collide: {a:?} {b:?}");
            }
        }
    }

    #[test]
    fn nodes_never_overlap() {
        let l = layout(&tcp(), &DEFAULT_THEME);
        for w in l.nodes.windows(2) {
            assert!(w[1].cx - w[0].cx > w[0].r + w[1].r, "{w:?}");
        }
    }

    #[test]
    fn everything_stays_inside_the_canvas() {
        let l = layout(&tcp(), &DEFAULT_THEME);
        for nd in &l.nodes {
            assert!(nd.cx - nd.r >= 0.0 && nd.cx + nd.r <= l.width, "{nd:?}");
        }
        for e in &l.edges {
            if let Some((x, y, w)) = e.label {
                assert!(x - w / 2.0 >= 0.0 && x + w / 2.0 <= l.width);
                assert!(y >= 0.0 && y <= l.height);
            }
        }
    }

    #[test]
    fn parallel_transitions_between_a_pair_take_separate_lanes() {
        let mut d = two_state();
        d.transitions.push(StateTransition {
            from: "idle".to_string(),
            to: "Done".to_string(),
            trigger: Some("auto".to_string()),
            kind: TransitionKind::Normal,
        });
        let l = layout(&d, &DEFAULT_THEME);
        assert_ne!(l.edges[0].lane, l.edges[1].lane);
    }
}
