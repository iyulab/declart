use std::f32::consts::PI;
use crate::model::OrgChartDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const ROOT_R: f32 = 42.0;
const NODE_W: f32 = 100.0;
const NODE_H: f32 = 34.0;
const RING_GAP: f32 = 120.0;
const PADDING: f32 = 28.0;
const TITLE_AREA: f32 = 50.0;

pub fn render(diagram: &OrgChartDiagram, theme: &Theme) -> String {
    let nodes = &diagram.nodes;
    if nodes.is_empty() {
        return SvgBuilder::new(400.0, 300.0).build(&theme.background.to_hex());
    }

    let mut children: std::collections::HashMap<&str, Vec<usize>> =
        std::collections::HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        if let Some(p) = &node.parent {
            children.entry(p.as_str()).or_default().push(i);
        }
    }

    let root_idx = nodes.iter().position(|n| n.parent.is_none()).unwrap_or(0);
    let max_depth = compute_max_depth(root_idx, &children, nodes);
    let canvas_r = ROOT_R + RING_GAP * max_depth.max(1) as f32 + NODE_W / 2.0 + PADDING;
    let canvas_size = (2.0 * canvas_r).max(400.0);

    let title_h = if diagram.title.is_some() { TITLE_AREA } else { PADDING };
    let cx = canvas_size / 2.0;
    let cy = title_h + canvas_size / 2.0;
    let canvas_w = canvas_size;
    let canvas_h = canvas_size + title_h;

    let mut positions = vec![(0.0f32, 0.0f32); nodes.len()];
    let mut depths = vec![0u32; nodes.len()];
    assign_positions(
        root_idx, 0, -PI / 2.0, 3.0 * PI / 2.0,
        &children, nodes, cx, cy, &mut positions, &mut depths,
    );

    let mut builder = SvgBuilder::new(canvas_w, canvas_h);

    if let Some(title) = &diagram.title {
        builder.text(cx, TITLE_AREA / 2.0, title, &theme.title_color.to_hex(),
                     theme.typography.title_size);
    }

    // Connectors (drawn before nodes)
    for (i, node) in nodes.iter().enumerate() {
        let Some(parent_label) = &node.parent else { continue };
        let Some(pi) = nodes.iter().position(|n| &n.label == parent_label) else { continue };
        let (px, py) = positions[pi];
        let (nx, ny) = positions[i];
        let dx = nx - px;
        let dy = ny - py;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 1.0 { continue; }
        let ux = dx / dist;
        let uy = dy / dist;

        let (sx, sy) = if depths[pi] == 0 {
            (px + ux * ROOT_R, py + uy * ROOT_R)
        } else {
            let (nw, nh) = node_size(depths[pi]);
            let t = rect_clip(ux, uy, nw / 2.0, nh / 2.0);
            (px + ux * t, py + uy * t)
        };
        let (nw2, nh2) = node_size(depths[i]);
        let t2 = rect_clip(-ux, -uy, nw2 / 2.0, nh2 / 2.0);
        let (ex, ey) = (nx - ux * t2, ny - uy * t2);

        builder.line(sx, sy, ex, ey, "#AAAAAA", 1.5);
    }

    // Non-root nodes
    for (i, node) in nodes.iter().enumerate() {
        if depths[i] == 0 { continue; }
        let (nx, ny) = positions[i];
        let depth = depths[i];
        let (nw, nh) = node_size(depth);
        let fill = if depth == 1 {
            theme.layers.base
        } else {
            theme.layers.base.interpolate(&theme.background, 0.35)
        };
        let text_color = if fill.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        let bold = depth == 1;

        builder.rect_rounded(
            nx - nw / 2.0, ny - nh / 2.0, nw, nh, 8,
            &fill.to_hex(), "none", 0.0,
        );

        let available = nw * 0.88;
        let mut fs = if bold {
            theme.typography.label_size
        } else {
            (theme.typography.label_size - 1.0).max(theme.typography.label_size_min)
        };
        if font::measure_text(&node.label, fs) > available {
            fs = (fs * available / font::measure_text(&node.label, fs))
                .max(theme.typography.label_size_min);
        }
        let display = if font::measure_text(&node.label, fs) > available {
            font::truncate_text(&node.label, fs, available)
        } else {
            node.label.clone()
        };
        builder.text_weighted(nx, ny, &display, &text_color.to_hex(), fs, bold);
    }

    // Root node drawn last (on top)
    let root = &nodes[root_idx];
    let (rx, ry) = positions[root_idx];
    let fill = theme.layers.apex;
    let text_color = if fill.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
    builder.circle_stroked(rx, ry, ROOT_R, &fill.to_hex(), &theme.background.to_hex(), 3.0);
    let available = ROOT_R * 1.6;
    let mut fs = theme.typography.label_size;
    if font::measure_text(&root.label, fs) > available {
        fs = (fs * available / font::measure_text(&root.label, fs))
            .max(theme.typography.label_size_min);
    }
    let display = if font::measure_text(&root.label, fs) > available {
        font::truncate_text(&root.label, fs, available)
    } else {
        root.label.clone()
    };
    builder.text_weighted(rx, ry, &display, &text_color.to_hex(), fs, true);

    builder.build(&theme.background.to_hex())
}

fn node_size(depth: u32) -> (f32, f32) {
    if depth == 1 { (NODE_W, NODE_H) } else { (NODE_W * 0.88, NODE_H * 0.88) }
}

fn rect_clip(ux: f32, uy: f32, hw: f32, hh: f32) -> f32 {
    let tx = if ux.abs() > 1e-6 { hw / ux.abs() } else { f32::INFINITY };
    let ty = if uy.abs() > 1e-6 { hh / uy.abs() } else { f32::INFINITY };
    tx.min(ty)
}

fn count_leaves(
    idx: usize,
    children: &std::collections::HashMap<&str, Vec<usize>>,
    nodes: &[crate::model::OrgChartNode],
) -> usize {
    let label = nodes[idx].label.as_str();
    match children.get(label) {
        None => 1,
        Some(v) if v.is_empty() => 1,
        Some(v) => v.iter().map(|&c| count_leaves(c, children, nodes)).sum(),
    }
}

fn compute_max_depth(
    idx: usize,
    children: &std::collections::HashMap<&str, Vec<usize>>,
    nodes: &[crate::model::OrgChartNode],
) -> u32 {
    let label = nodes[idx].label.as_str();
    match children.get(label) {
        None => 0,
        Some(v) if v.is_empty() => 0,
        Some(v) => 1 + v.iter().map(|&c| compute_max_depth(c, children, nodes)).max().unwrap_or(0),
    }
}

#[allow(clippy::too_many_arguments)]
fn assign_positions(
    idx: usize,
    depth: u32,
    angle_start: f32,
    angle_end: f32,
    children: &std::collections::HashMap<&str, Vec<usize>>,
    nodes: &[crate::model::OrgChartNode],
    cx: f32,
    cy: f32,
    positions: &mut Vec<(f32, f32)>,
    depths: &mut Vec<u32>,
) {
    depths[idx] = depth;
    if depth == 0 {
        positions[idx] = (cx, cy);
    } else {
        let angle = (angle_start + angle_end) / 2.0;
        let r = RING_GAP * depth as f32;
        positions[idx] = (cx + r * angle.cos(), cy + r * angle.sin());
    }

    let label = nodes[idx].label.as_str();
    let child_list = match children.get(label) {
        Some(v) if !v.is_empty() => v.clone(),
        _ => return,
    };

    let leaf_counts: Vec<usize> = child_list
        .iter()
        .map(|&c| count_leaves(c, children, nodes))
        .collect();
    let total_leaves: usize = leaf_counts.iter().sum();
    let total_angle = angle_end - angle_start;

    let mut current = angle_start;
    for (i, &child) in child_list.iter().enumerate() {
        let frac = if total_leaves > 0 {
            leaf_counts[i] as f32 / total_leaves as f32
        } else {
            1.0 / child_list.len() as f32
        };
        let span = total_angle * frac;
        assign_positions(
            child, depth + 1, current, current + span,
            children, nodes, cx, cy, positions, depths,
        );
        current += span;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{OrgChartDiagram, OrgChartNode};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(nodes: Vec<(&str, Option<&str>)>, title: Option<&str>) -> OrgChartDiagram {
        OrgChartDiagram {
            title: title.map(String::from),
            nodes: nodes.into_iter().map(|(label, parent)| OrgChartNode {
                label: label.to_string(),
                parent: parent.map(String::from),
            }).collect(),
        }
    }

    #[test]
    fn render_produces_svg() {
        let d = make_diagram(vec![("Root", None), ("A", Some("Root")), ("B", Some("Root"))], Some("Test"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_diagram(vec![
            ("Center", None),
            ("Branch1", Some("Center")),
            ("Branch2", Some("Center")),
            ("Leaf", Some("Branch1")),
        ], None);
        let svg = render(&d, &DEFAULT_THEME);
        for label in &["Center", "Branch1", "Branch2", "Leaf"] {
            assert!(svg.contains(label), "missing label: {}", label);
        }
    }

    #[test]
    fn render_single_node_does_not_panic() {
        let d = make_diagram(vec![("Solo", None)], None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Solo"));
    }

    #[test]
    fn render_has_connector_lines() {
        let d = make_diagram(vec![
            ("Root", None),
            ("A", Some("Root")),
            ("B", Some("Root")),
        ], None);
        let svg = render(&d, &DEFAULT_THEME);
        let line_count = svg.matches("<line").count();
        assert_eq!(line_count, 2, "expected 2 connector lines for 2 children");
    }

    #[test]
    fn render_three_level_tree() {
        let d = make_diagram(vec![
            ("Root", None),
            ("L1a", Some("Root")),
            ("L1b", Some("Root")),
            ("L2a", Some("L1a")),
            ("L2b", Some("L1a")),
        ], Some("Three levels"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        for label in &["Root", "L1a", "L1b", "L2a", "L2b"] {
            assert!(svg.contains(label), "missing label: {}", label);
        }
    }
}
