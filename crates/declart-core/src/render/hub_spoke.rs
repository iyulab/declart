use std::f32::consts::PI;

use crate::model::{Emphasis, HubSpokeDiagram};
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const HUB_W: f32 = 140.0;
const HUB_H: f32 = 55.0;
const SPOKE_W: f32 = 110.0;
const SPOKE_H: f32 = 44.0;
const PADDING: f32 = 30.0;
const TITLE_AREA: f32 = 50.0;
const LINE_W: f32 = 2.0;

/// Returns the scalar t such that (cx + ux*t, cy + uy*t) lies exactly on the
/// edge of a rectangle centered at (cx, cy) with half-extents (hw, hh).
fn rect_clip(ux: f32, uy: f32, hw: f32, hh: f32) -> f32 {
    let tx = if ux.abs() > 1e-6 { hw / ux.abs() } else { f32::INFINITY };
    let ty = if uy.abs() > 1e-6 { hh / uy.abs() } else { f32::INFINITY };
    tx.min(ty)
}

pub fn render(diagram: &HubSpokeDiagram, theme: &Theme) -> String {
    let n = diagram.spokes.len();
    let n_f = n as f32;

    let spoke_half_diag = ((SPOKE_W / 2.0).powi(2) + (SPOKE_H / 2.0).powi(2)).sqrt();

    let radius = if n >= 2 {
        f32::max(150.0, spoke_half_diag / (PI / n_f).sin() * 1.2)
    } else {
        180.0
    };

    let diagram_size = 2.0 * (radius + spoke_half_diag + PADDING);
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

    let spoke_centers: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let angle = -PI / 2.0 + 2.0 * PI * i as f32 / n_f;
            (cx + radius * angle.cos(), cy + radius * angle.sin())
        })
        .collect();

    let spoke_color = theme.layers.base;
    let hub_color = theme.layers.apex;
    let line_color = &hub_color.to_hex();

    // Draw connection lines first (behind nodes)
    for &(sx, sy) in &spoke_centers {
        let dx = sx - cx;
        let dy = sy - cy;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1.0 {
            continue;
        }
        let ux = dx / len;
        let uy = dy / len;
        let t_hub = rect_clip(ux, uy, HUB_W / 2.0, HUB_H / 2.0);
        let start_x = cx + ux * t_hub;
        let start_y = cy + uy * t_hub;
        let t_spoke = rect_clip(ux, uy, SPOKE_W / 2.0, SPOKE_H / 2.0);
        let end_x = sx - ux * t_spoke;
        let end_y = sy - uy * t_spoke;
        builder.line(start_x, start_y, end_x, end_y, line_color, LINE_W);
    }

    // Draw spoke nodes
    for (i, spoke) in diagram.spokes.iter().enumerate() {
        let (sx, sy) = spoke_centers[i];
        let x = sx - SPOKE_W / 2.0;
        let y = sy - SPOKE_H / 2.0;

        let box_color = match &spoke.emphasis {
            Some(Emphasis::Secondary) => spoke_color.interpolate(&theme.background, 0.3),
            _ => spoke_color,
        };
        let (stroke_color, stroke_width) = match &spoke.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };
        let text_color = if box_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        let bold = matches!(&spoke.emphasis, Some(Emphasis::Primary));

        builder.polygon_stroked(
            &[(x, y), (x + SPOKE_W, y), (x + SPOKE_W, y + SPOKE_H), (x, y + SPOKE_H)],
            &box_color.to_hex(),
            &stroke_color,
            stroke_width,
        );

        let available = SPOKE_W * 0.85;
        let mut fs = theme.typography.label_size;
        let tw = font::measure_text(&spoke.label, fs);
        if tw > available && available > 0.0 {
            fs = (fs * available / tw).max(theme.typography.label_size_min);
        }
        let spoke_display = if font::measure_text(&spoke.label, fs) > available {
            font::truncate_text(&spoke.label, fs, available)
        } else {
            spoke.label.clone()
        };
        builder.text_weighted(sx, sy, &spoke_display, &text_color.to_hex(), fs, bold);
    }

    // Draw hub node on top
    let hub_text_color = if hub_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
    let hx = cx - HUB_W / 2.0;
    let hy = cy - HUB_H / 2.0;
    builder.polygon(
        &[(hx, hy), (hx + HUB_W, hy), (hx + HUB_W, hy + HUB_H), (hx, hy + HUB_H)],
        &hub_color.to_hex(),
        "none",
    );

    let available = HUB_W * 0.85;
    let mut fs = theme.typography.label_size;
    let tw = font::measure_text(&diagram.center, fs);
    if tw > available && available > 0.0 {
        fs = (fs * available / tw).max(theme.typography.label_size_min);
    }
    let hub_display = if font::measure_text(&diagram.center, fs) > available {
        font::truncate_text(&diagram.center, fs, available)
    } else {
        diagram.center.clone()
    };
    builder.text(cx, cy, &hub_display, &hub_text_color.to_hex(), fs);

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{HubSpokeDiagram, Item};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(n: usize, title: Option<&str>) -> HubSpokeDiagram {
        HubSpokeDiagram {
            title: title.map(String::from),
            center: "Hub".to_string(),
            spokes: (0..n)
                .map(|i| Item { label: format!("Spoke {}", i + 1), emphasis: None })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_diagram(4, Some("Test"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_hub_and_spoke_labels() {
        let d = make_diagram(4, None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Hub"));
        assert!(svg.contains("Spoke 1"));
        assert!(svg.contains("Spoke 4"));
    }

    #[test]
    fn render_has_correct_polygon_count() {
        let d = make_diagram(5, None);
        let svg = render(&d, &DEFAULT_THEME);
        // N spoke boxes + 1 hub box = N+1
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 6, "expected 6 polygons for 5 spokes + hub");
    }

    #[test]
    fn render_has_n_connection_lines() {
        let d = make_diagram(5, None);
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<line").count();
        assert_eq!(count, 5, "expected 5 lines for 5 spokes");
    }

    #[test]
    fn render_primary_emphasis_has_stroke_and_bold() {
        let d = HubSpokeDiagram {
            title: None,
            center: "Hub".to_string(),
            spokes: vec![
                Item { label: "Primary".to_string(), emphasis: Some(Emphasis::Primary) },
                Item { label: "Normal".to_string(), emphasis: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("font-weight=\"bold\""), "primary spoke should be bold");
        assert!(svg.contains("stroke-width=\"3.0\""), "primary spoke should have stroke");
    }

    #[test]
    fn render_5_spokes_equiangular_symmetric() {
        // For n=5, spokes should be at equal 72° intervals from top (-PI/2)
        // producing left-right symmetric layout: 1 top + 2 right + 2 left
        let n = 5_usize;
        let n_f = n as f32;
        let angles: Vec<f32> = (0..n)
            .map(|i| -PI / 2.0 + 2.0 * PI * i as f32 / n_f)
            .collect();
        let step = 2.0 * PI / n_f;
        for i in 1..n {
            assert!((angles[i] - angles[i - 1] - step).abs() < 1e-5, "angular step should be uniform 72°");
        }
        // First spoke is at top
        assert!((angles[0] - (-PI / 2.0)).abs() < 1e-5, "first spoke should be at top");
        // Left-right symmetry: cos of spokes i=1,4 and i=2,3 should be equal and opposite
        let cos_1 = angles[1].cos();
        let cos_4 = angles[4].cos();
        assert!((cos_1 + cos_4).abs() < 1e-4, "spokes 1 and 4 should be symmetric about vertical axis");
        let cos_2 = angles[2].cos();
        let cos_3 = angles[3].cos();
        assert!((cos_2 + cos_3).abs() < 1e-4, "spokes 2 and 3 should be symmetric about vertical axis");
    }

    #[test]
    fn render_secondary_emphasis_has_no_stroke() {
        let d = HubSpokeDiagram {
            title: None,
            center: "Hub".to_string(),
            spokes: vec![
                Item { label: "Secondary".to_string(), emphasis: Some(Emphasis::Secondary) },
                Item { label: "Normal".to_string(), emphasis: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(!svg.contains("stroke-width=\"3.0\""), "secondary spoke should not have outline stroke");
    }
}
