use crate::model::OrgChartDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const NODE_W: f32 = 120.0;
const NODE_H: f32 = 40.0;
const H_GAP: f32 = 20.0;
const V_GAP: f32 = 60.0;
const PADDING: f32 = 20.0;
const TITLE_AREA: f32 = 50.0;
const CONNECTOR_COLOR: &str = "#888888";

pub fn render(diagram: &OrgChartDiagram, theme: &Theme) -> String {
    let nodes = &diagram.nodes;
    if nodes.is_empty() {
        return SvgBuilder::new(400.0, 200.0).build(&theme.background.to_hex());
    }

    // Build parent→children map.
    let children: std::collections::HashMap<&str, Vec<usize>> = {
        let mut map: std::collections::HashMap<&str, Vec<usize>> = std::collections::HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            if let Some(parent_id) = &node.parent {
                map.entry(parent_id.as_str()).or_default().push(i);
            }
        }
        map
    };
    let root_idx = nodes.iter().position(|n| n.parent.is_none()).unwrap_or(0);

    // Assign X positions using a recursive subtree width calculation.
    let mut x_positions = vec![0.0_f32; nodes.len()];
    let mut y_positions = vec![0.0_f32; nodes.len()];
    let mut next_x = PADDING;

    assign_positions(
        root_idx,
        0,
        &children,
        nodes,
        &mut x_positions,
        &mut y_positions,
        &mut next_x,
    );

    let max_x = x_positions.iter().cloned().fold(0.0_f32, f32::max) + NODE_W + PADDING;
    let max_y = y_positions.iter().cloned().fold(0.0_f32, f32::max);
    let title_height = if diagram.title.is_some() { TITLE_AREA } else { PADDING };
    let canvas_w = max_x.max(400.0);
    let canvas_h = title_height + max_y + NODE_H + PADDING;

    let mut builder = SvgBuilder::new(canvas_w, canvas_h);

    if let Some(title) = &diagram.title {
        builder.text(
            canvas_w / 2.0,
            title_height / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    // Draw connectors first (below nodes).
    for (i, node) in nodes.iter().enumerate() {
        if let Some(parent_id) = &node.parent {
            if let Some(parent_idx) = nodes.iter().position(|n| &n.id == parent_id) {
                let px = x_positions[parent_idx] + NODE_W / 2.0;
                let py = title_height + y_positions[parent_idx] + NODE_H;
                let cx = x_positions[i] + NODE_W / 2.0;
                let cy = title_height + y_positions[i];
                let mid_y = py + (cy - py) / 2.0;
                // Elbow connector: down → horizontal → down
                builder.line(px, py, px, mid_y, CONNECTOR_COLOR, 1.5);
                builder.line(px, mid_y, cx, mid_y, CONNECTOR_COLOR, 1.5);
                builder.line(cx, mid_y, cx, cy, CONNECTOR_COLOR, 1.5);
            }
        }
    }

    // Draw nodes.
    for (i, node) in nodes.iter().enumerate() {
        let x = x_positions[i];
        let y = title_height + y_positions[i];
        let is_root = node.parent.is_none();
        let fill = if is_root {
            theme.layers.apex.to_hex()
        } else {
            theme.layers.base.to_hex()
        };
        let node_color = if is_root { &theme.layers.apex } else { &theme.layers.base };
        let text_color = if node_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };

        builder.polygon_stroked(
            &[(x, y), (x + NODE_W, y), (x + NODE_W, y + NODE_H), (x, y + NODE_H)],
            &fill,
            "none",
            0.0,
        );

        let available = NODE_W * 0.85;
        let mut font_size = theme.typography.label_size;
        let text_w = font::measure_text(&node.label, font_size);
        if text_w > available {
            font_size = (font_size * available / text_w).max(theme.typography.label_size_min);
        }
        let display = if font::measure_text(&node.label, font_size) > available {
            font::truncate_text(&node.label, font_size, available)
        } else {
            node.label.clone()
        };

        builder.text(
            x + NODE_W / 2.0,
            y + NODE_H / 2.0,
            &display,
            &text_color.to_hex(),
            font_size,
        );
    }

    builder.build(&theme.background.to_hex())
}

/// Recursively assigns X/Y positions for a subtree rooted at `idx`.
/// Returns the total width consumed by this subtree.
fn assign_positions(
    idx: usize,
    depth: u32,
    children: &std::collections::HashMap<&str, Vec<usize>>,
    nodes: &[crate::model::OrgChartNode],
    x_positions: &mut Vec<f32>,
    y_positions: &mut Vec<f32>,
    next_x: &mut f32,
) -> f32 {
    let node_id = nodes[idx].id.as_str();
    let y = depth as f32 * (NODE_H + V_GAP);
    y_positions[idx] = y;

    let child_indices = children.get(node_id).cloned().unwrap_or_default();
    if child_indices.is_empty() {
        // Leaf node: place at next_x.
        x_positions[idx] = *next_x;
        let width = NODE_W + H_GAP;
        *next_x += width;
        return width;
    }

    // Internal node: center over its children.
    let subtree_start = *next_x;
    let mut total_width = 0.0_f32;
    for &child_idx in &child_indices {
        total_width += assign_positions(child_idx, depth + 1, children, nodes, x_positions, y_positions, next_x);
    }
    let subtree_end = *next_x;
    // Center the parent over its children.
    x_positions[idx] = (subtree_start + subtree_end - H_GAP) / 2.0 - NODE_W / 2.0;
    total_width
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{OrgChartDiagram, OrgChartNode};
    use crate::render::DEFAULT_THEME;

    fn make_simple_org() -> OrgChartDiagram {
        OrgChartDiagram {
            title: Some("Company".to_string()),
            nodes: vec![
                OrgChartNode { id: "ceo".to_string(), label: "CEO".to_string(), parent: None },
                OrgChartNode { id: "cto".to_string(), label: "CTO".to_string(), parent: Some("ceo".to_string()) },
                OrgChartNode { id: "cfo".to_string(), label: "CFO".to_string(), parent: Some("ceo".to_string()) },
            ],
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_simple_org();
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_simple_org();
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("CEO"));
        assert!(svg.contains("CTO"));
        assert!(svg.contains("CFO"));
    }

    #[test]
    fn render_includes_connectors() {
        let d = make_simple_org();
        let svg = render(&d, &DEFAULT_THEME);
        // Lines are used for connectors
        assert!(svg.contains("<line"));
    }
}
