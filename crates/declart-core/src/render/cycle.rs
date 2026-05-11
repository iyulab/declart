use std::f32::consts::PI;

use crate::model::{Emphasis, ItemsDiagram};
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const NODE_W: f32 = 110.0;
const NODE_H: f32 = 50.0;

fn rect_clip(ux: f32, uy: f32, hw: f32, hh: f32) -> f32 {
    let tx = if ux.abs() > 1e-6 { hw / ux.abs() } else { f32::INFINITY };
    let ty = if uy.abs() > 1e-6 { hh / uy.abs() } else { f32::INFINITY };
    tx.min(ty)
}
const PADDING: f32 = 30.0;
const TITLE_AREA: f32 = 50.0;
const ARROW_SHAFT: f32 = 3.0;
const ARROW_HEAD_SIZE: f32 = 12.0;

pub fn render(diagram: &ItemsDiagram, theme: &Theme) -> String {
    let n = diagram.items.len();
    let n_f = n as f32;

    let node_half_diag = ((NODE_W / 2.0).powi(2) + (NODE_H / 2.0).powi(2)).sqrt();
    let min_radius = if n >= 2 {
        node_half_diag / (PI / n_f).sin()
    } else {
        150.0
    };
    let radius = f32::max(130.0, min_radius * 1.15);

    let diagram_size = 2.0 * (radius + node_half_diag + PADDING);
    let title_h = if diagram.title.is_some() { TITLE_AREA } else { PADDING };
    let canvas_w = diagram_size;
    let canvas_h = diagram_size + title_h;
    let cx = canvas_w / 2.0;
    let cy = title_h + diagram_size / 2.0;

    let mut builder = SvgBuilder::new(canvas_w, canvas_h);

    if let Some(title) = &diagram.title {
        builder.text(
            canvas_w / 2.0,
            TITLE_AREA / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    let centers: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let angle = -PI / 2.0 + 2.0 * PI * i as f32 / n_f;
            (cx + radius * angle.cos(), cy + radius * angle.sin())
        })
        .collect();

    for i in 0..n {
        let next = (i + 1) % n;
        let (x1, y1) = centers[i];
        let (x2, y2) = centers[next];

        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1.0 {
            continue;
        }
        let ux = dx / len;
        let uy = dy / len;

        let t = rect_clip(ux, uy, NODE_W / 2.0, NODE_H / 2.0);
        let start_x = x1 + ux * t;
        let start_y = y1 + uy * t;
        let end_x = x2 - ux * t;
        let end_y = y2 - uy * t;

        let arrow_color = &theme.layers.apex.to_hex();

        builder.line(
            start_x,
            start_y,
            end_x - ux * ARROW_HEAD_SIZE,
            end_y - uy * ARROW_HEAD_SIZE,
            arrow_color,
            ARROW_SHAFT,
        );

        let perp_x = -uy * ARROW_HEAD_SIZE / 2.0;
        let perp_y = ux * ARROW_HEAD_SIZE / 2.0;
        let base_x = end_x - ux * ARROW_HEAD_SIZE;
        let base_y = end_y - uy * ARROW_HEAD_SIZE;
        builder.polygon(
            &[
                (base_x + perp_x, base_y + perp_y),
                (end_x, end_y),
                (base_x - perp_x, base_y - perp_y),
            ],
            arrow_color,
            "none",
        );
    }

    for (i, item) in diagram.items.iter().enumerate() {
        let i_f = i as f32;
        let t = if n_f > 1.0 { i_f / (n_f - 1.0) } else { 0.0 };
        let base_color = theme.layers.apex.interpolate(&theme.layers.base, t);
        let node_color = match &item.emphasis {
            Some(Emphasis::Secondary) => base_color.interpolate(&theme.layers.base, 0.3),
            _ => base_color,
        };
        let text_color = if node_color.is_dark() {
            &theme.text.on_dark
        } else {
            &theme.text.on_light
        };
        let (stroke_color, stroke_width) = match &item.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };

        let (nx, ny) = centers[i];
        let x = nx - NODE_W / 2.0;
        let y = ny - NODE_H / 2.0;

        builder.polygon_stroked(
            &[(x, y), (x + NODE_W, y), (x + NODE_W, y + NODE_H), (x, y + NODE_H)],
            &node_color.to_hex(),
            &stroke_color,
            stroke_width,
        );

        let available_width = NODE_W * 0.85;
        let mut font_size = theme.typography.label_size;
        let text_width = font::measure_text(&item.label, font_size);
        if text_width > available_width && available_width > 0.0 {
            font_size = (font_size * available_width / text_width)
                .max(theme.typography.label_size_min);
        }

        let is_bold = matches!(&item.emphasis, Some(Emphasis::Primary));
        builder.text_weighted(nx, ny, &item.label, &text_color.to_hex(), font_size, is_bold);
    }

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Item, ItemsDiagram};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(n: usize, title: Option<&str>) -> ItemsDiagram {
        ItemsDiagram {
            title: title.map(String::from),
            items: (0..n)
                .map(|i| Item { label: format!("Step {}", i + 1), emphasis: None })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_diagram(4, Some("PDCA"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_diagram(4, None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Step 1"));
        assert!(svg.contains("Step 2"));
        assert!(svg.contains("Step 3"));
        assert!(svg.contains("Step 4"));
    }

    #[test]
    fn render_has_n_node_polygons() {
        let d = make_diagram(4, None);
        let svg = render(&d, &DEFAULT_THEME);
        let polygon_count = svg.matches("<polygon").count();
        assert!(polygon_count >= 4, "expected at least 4 polygons, got {}", polygon_count);
    }

    #[test]
    fn render_has_n_arrows() {
        let d = make_diagram(4, None);
        let svg = render(&d, &DEFAULT_THEME);
        let line_count = svg.matches("<line").count();
        assert_eq!(line_count, 4, "expected 4 arrow lines for 4 items");
    }
}
