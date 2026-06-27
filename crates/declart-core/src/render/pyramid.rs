use crate::model::{Emphasis, ItemsDiagram};
use crate::render::{font, status, svg::SvgBuilder, theme::Theme};

const CANVAS_BASE_W: f32 = 600.0;
const LAYER_HEIGHT: f32 = 60.0;
const PADDING_H: f32 = 20.0;
const TITLE_AREA: f32 = 60.0;
const BOTTOM_PAD: f32 = 20.0;
const PYRAMID_WIDTH: f32 = CANVAS_BASE_W - PADDING_H * 2.0;
const CENTER_X: f32 = CANVAS_BASE_W / 2.0;
const LEADER_GAP: f32 = 12.0;

pub fn render(diagram: &ItemsDiagram, theme: &Theme) -> String {
    let n = diagram.items.len() as f32;
    let title_height = if diagram.title.is_some() { TITLE_AREA } else { PADDING_H };

    // Pre-pass: compute canvas width, expanding right when layers need leader lines.
    let mut canvas_width = CANVAS_BASE_W;
    for (i, item) in diagram.items.iter().enumerate() {
        let i_f = i as f32;
        let top_width = PYRAMID_WIDTH * (i_f / n);
        let bottom_width = PYRAMID_WIDTH * ((i_f + 1.0) / n);
        let mid_width = (top_width + bottom_width) / 2.0;
        let available_width = mid_width * 0.85;
        if available_width > 0.0
            && font::measure_text(&item.label, theme.typography.label_size_min) > available_width
        {
            let text_w = font::measure_text(&item.label, theme.typography.label_size);
            let right_edge = CENTER_X + mid_width / 2.0;
            let needed = right_edge + LEADER_GAP + text_w + PADDING_H;
            canvas_width = canvas_width.max(needed);
        }
    }

    let canvas_height = title_height + n * LAYER_HEIGHT + BOTTOM_PAD;
    let mut builder = SvgBuilder::new(canvas_width, canvas_height);

    if let Some(title) = &diagram.title {
        builder.text(
            CENTER_X,
            title_height / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    for (i, item) in diagram.items.iter().enumerate() {
        let i_f = i as f32;
        let n_layers = diagram.items.len() as f32;

        let t = if n_layers > 1.0 { i_f / (n_layers - 1.0) } else { 0.0 };
        let base_color = theme.layers.apex.interpolate(&theme.layers.base, t);
        let layer_color = match &item.emphasis {
            Some(Emphasis::Secondary) => base_color.interpolate(&theme.layers.base, 0.3),
            _ => base_color,
        };
        let text_color = if layer_color.is_dark() {
            &theme.text.on_dark
        } else {
            &theme.text.on_light
        };
        let (stroke_color, stroke_width) = match &item.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };

        let top_width = PYRAMID_WIDTH * (i_f / n_layers);
        let bottom_width = PYRAMID_WIDTH * ((i_f + 1.0) / n_layers);
        let top_y = title_height + i_f * LAYER_HEIGHT;
        let bottom_y = top_y + LAYER_HEIGHT;
        let center_y = (top_y + bottom_y) / 2.0;

        let top_left = CENTER_X - top_width / 2.0;
        let top_right = CENTER_X + top_width / 2.0;
        let bottom_left = CENTER_X - bottom_width / 2.0;
        let bottom_right = CENTER_X + bottom_width / 2.0;

        builder.polygon_stroked(
            &[
                (top_left, top_y),
                (top_right, top_y),
                (bottom_right, bottom_y),
                (bottom_left, bottom_y),
            ],
            &layer_color.to_hex(),
            &stroke_color,
            stroke_width,
        );

        let mid_width = (top_width + bottom_width) / 2.0;
        let available_width = mid_width * 0.85;

        let needs_leader = available_width > 0.0
            && font::measure_text(&item.label, theme.typography.label_size_min) > available_width;

        if needs_leader {
            // Draw label outside the layer with a leader line on the right.
            let right_edge = CENTER_X + mid_width / 2.0;
            let text_w = font::measure_text(&item.label, theme.typography.label_size);
            let text_x = right_edge + LEADER_GAP + text_w / 2.0;
            let leader_color = &theme.title_color.to_hex();
            builder.line(right_edge, center_y, right_edge + LEADER_GAP, center_y, leader_color, 1.0);
            let is_bold = matches!(&item.emphasis, Some(Emphasis::Primary));
            builder.text_weighted(
                text_x,
                center_y,
                &item.label,
                leader_color,
                theme.typography.label_size,
                is_bold,
            );
        } else {
            let mut font_size = theme.typography.label_size;
            if available_width > 0.0 {
                let text_width = font::measure_text(&item.label, font_size);
                if text_width > available_width {
                    font_size = (font_size * available_width / text_width)
                        .max(theme.typography.label_size_min);
                }
            }
            let label = if available_width > 0.0 {
                let tw = font::measure_text(&item.label, font_size);
                if tw > available_width {
                    truncate_label(&item.label, available_width, font_size)
                } else {
                    item.label.clone()
                }
            } else {
                item.label.clone()
            };

            let is_bold = matches!(&item.emphasis, Some(Emphasis::Primary));
            builder.text_weighted(CENTER_X, center_y, &label, &text_color.to_hex(), font_size, is_bold);
        }

        // Marker on the layer's right edge at its vertical center (trapezoid apex is too narrow
        // for a corner marker; the right-edge midpoint stays inside every layer).
        let marker_cx = CENTER_X + mid_width / 2.0 - status::STATUS_MARKER_R - 2.0;
        status::draw_marker_at(&mut builder, &item.status, marker_cx, center_y, theme);
    }

    builder.build(&theme.background.to_hex())
}

fn truncate_label(label: &str, available_width: f32, font_size: f32) -> String {
    let ellipsis = "…";
    let ellipsis_w = font::measure_text(ellipsis, font_size);
    let max_w = (available_width - ellipsis_w).max(0.0);
    let mut result = String::new();
    let mut acc = 0.0f32;
    for c in label.chars() {
        let cw = font::measure_text(&c.to_string(), font_size);
        if acc + cw > max_w {
            break;
        }
        result.push(c);
        acc += cw;
    }
    format!("{}{}", result, ellipsis)
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
                .map(|i| Item { label: format!("Layer {}", i), emphasis: None, status: None })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_diagram(3, Some("Test"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_diagram(3, None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Layer 0"));
        assert!(svg.contains("Layer 1"));
        assert!(svg.contains("Layer 2"));
    }

    #[test]
    fn render_includes_title_when_present() {
        let d = make_diagram(2, Some("My Pyramid"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("My Pyramid"));
    }

    #[test]
    fn render_omits_title_when_absent() {
        let d = make_diagram(2, None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(!svg.contains(">My Pyramid<"));
    }

    #[test]
    fn render_has_n_polygons() {
        let d = make_diagram(5, None);
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 5);
    }

    #[test]
    fn render_layer_status_emits_marker() {
        use crate::model::Status;
        let d = ItemsDiagram {
            title: None,
            items: vec![
                Item { label: "Apex".to_string(), emphasis: None, status: Some(Status::Warning) },
                Item { label: "Base".to_string(), emphasis: None, status: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains(&DEFAULT_THEME.status.warning.to_hex()), "a tier layer with status should render a marker");
    }

    #[test]
    fn render_long_apex_label_uses_leader_line() {
        let d = ItemsDiagram {
            title: None,
            items: (0..7)
                .map(|i| Item {
                    label: if i == 0 { "Self-Actualization".to_string() } else { format!("Layer {}", i) },
                    emphasis: None,
                    status: None,
                })
                .collect(),
        };
        let svg = render(&d, &DEFAULT_THEME);
        // With leader line: full label appears untruncated (no ellipsis)
        assert!(svg.contains("Self-Actualization"), "long apex label should appear in full via leader line");
        assert!(!svg.contains("…"), "leader line should avoid truncation");
        // A <line> element exists for the leader line (pyramids have no lines otherwise)
        assert!(svg.contains("<line"), "leader line element should be present");
    }

    #[test]
    fn render_very_long_apex_label_expands_canvas() {
        // 12-layer pyramid: apex mid_width≈23, right_edge≈312.
        // needed = 312 + 12 + text_w + 20. For expansion: text_w > 256.
        // "An Extremely Long Self-Actualization Goal" at 14px ≈ 273px > 256 → needed > 600.
        let long_label = "An Extremely Long Self-Actualization Goal";
        let d = ItemsDiagram {
            title: None,
            items: (0..12)
                .map(|i| Item {
                    label: if i == 0 { long_label.to_string() } else { format!("L{}", i) },
                    emphasis: None,
                    status: None,
                })
                .collect(),
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains(long_label), "full label should appear via leader line");
        let width_start = svg.find("width=\"").unwrap() + 7;
        let width_end = svg[width_start..].find('"').unwrap() + width_start;
        let width: f32 = svg[width_start..width_end].parse().unwrap();
        assert!(width > CANVAS_BASE_W, "canvas should expand for very long apex label, got {}", width);
    }
}
