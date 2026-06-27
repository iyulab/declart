use crate::model::{Emphasis, ItemsDiagram};
use crate::render::{font, status, svg::SvgBuilder, theme::Theme};

const CANVAS_W: f32 = 600.0;
const MAX_R: f32 = 240.0;
const CENTER_X: f32 = CANVAS_W / 2.0;
const TITLE_AREA: f32 = 60.0;
const PADDING: f32 = 20.0;
const BOTTOM_PAD: f32 = 20.0;

/// Renders an ordered layer list as nested concentric rings (onion model): the first item is the
/// innermost core, each subsequent item a ring enclosing it. Order = inner → outer (containment).
pub fn render(diagram: &ItemsDiagram, theme: &Theme) -> String {
    let n = diagram.items.len();
    let n_f = n as f32;
    let title_height = if diagram.title.is_some() { TITLE_AREA } else { PADDING };
    let center_y = title_height + MAX_R;
    let canvas_height = title_height + 2.0 * MAX_R + BOTTOM_PAD;

    let mut builder = SvgBuilder::new(CANVAS_W, canvas_height);

    if let Some(title) = &diagram.title {
        builder.text(
            CENTER_X,
            title_height / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    // Per-layer color (core = apex, periphery = base), reused for fill and label contrast.
    let layer_color = |i: usize, emphasis: &Option<Emphasis>| {
        let t = if n > 1 { i as f32 / (n_f - 1.0) } else { 0.0 };
        let base = theme.layers.apex.interpolate(&theme.layers.base, t);
        match emphasis {
            Some(Emphasis::Secondary) => base.interpolate(&theme.layers.base, 0.3),
            _ => base,
        }
    };

    // Rings: paint outermost first so inner disks land on top, revealing each band.
    for i in (0..n).rev() {
        let item = &diagram.items[i];
        let r_out = MAX_R * (i as f32 + 1.0) / n_f;
        let color = layer_color(i, &item.emphasis);
        let (stroke, stroke_w) = match &item.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };
        builder.circle_stroked(CENTER_X, center_y, r_out, &color.to_hex(), &stroke, stroke_w);
    }

    // Labels + status markers, stacked in the upper hemisphere (one per band).
    for (i, item) in diagram.items.iter().enumerate() {
        let r_out = MAX_R * (i as f32 + 1.0) / n_f;
        let r_mid = MAX_R * (i as f32 + 0.5) / n_f;

        // Core (i == 0) sits at dead center; outer bands float above center at their mid-radius.
        // `half` is the horizontal half-width available to the label at that height.
        let (label_y, half) = if i == 0 {
            (center_y, r_out * 0.7)
        } else {
            (center_y - r_mid, (r_out * r_out - r_mid * r_mid).max(0.0).sqrt() * 0.9)
        };
        let available = 2.0 * half;

        // Shrink to fit, then truncate as a last resort (no leader lines — they would cross rings).
        let mut font_size = theme.typography.label_size;
        let measured = font::measure_text(&item.label, font_size);
        if available > 0.0 && measured > available {
            font_size = (font_size * available / measured).max(theme.typography.label_size_min);
        }
        let label = font::truncate_text(&item.label, font_size, available);

        let color = layer_color(i, &item.emphasis);
        let text_color = if color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        let is_bold = matches!(&item.emphasis, Some(Emphasis::Primary));
        builder.text_weighted(CENTER_X, label_y, &label, &text_color.to_hex(), font_size, is_bold);

        // Marker at the band's right edge at label height, inset to stay inside the ring.
        let marker_cx = CENTER_X + half - status::STATUS_MARKER_R - 2.0;
        status::draw_marker_at(&mut builder, &item.status, marker_cx, label_y, theme);
    }

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Emphasis, Item, ItemsDiagram, Status};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(n: usize, title: Option<&str>) -> ItemsDiagram {
        ItemsDiagram {
            title: title.map(String::from),
            items: (0..n)
                .map(|i| Item { label: format!("Ring {}", i), emphasis: None, status: None })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let svg = render(&make_diagram(3, Some("Onion")), &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let svg = render(&make_diagram(3, None), &DEFAULT_THEME);
        assert!(svg.contains("Ring 0"));
        assert!(svg.contains("Ring 1"));
        assert!(svg.contains("Ring 2"));
    }

    #[test]
    fn render_includes_title_when_present() {
        let svg = render(&make_diagram(2, Some("My Onion")), &DEFAULT_THEME);
        assert!(svg.contains("My Onion"));
    }

    #[test]
    fn render_omits_title_when_absent() {
        let svg = render(&make_diagram(2, None), &DEFAULT_THEME);
        assert!(!svg.contains(">My Onion<"));
    }

    #[test]
    fn render_draws_one_circle_per_layer() {
        // Painter's algorithm: each layer is a filled circle (largest first). No status → no
        // status-marker circles polluting the count.
        let svg = render(&make_diagram(4, None), &DEFAULT_THEME);
        let count = svg.matches("<circle").count();
        assert_eq!(count, 4, "expected one circle per layer");
    }

    #[test]
    fn render_layer_status_emits_marker() {
        let d = ItemsDiagram {
            title: None,
            items: vec![
                Item { label: "Core".to_string(), emphasis: None, status: Some(Status::Warning) },
                Item { label: "Outer".to_string(), emphasis: None, status: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(
            svg.contains(&DEFAULT_THEME.status.warning.to_hex()),
            "a layer with status should render a marker"
        );
    }

    #[test]
    fn render_primary_emphasis_has_stroke() {
        let d = ItemsDiagram {
            title: None,
            items: vec![
                Item { label: "Core".to_string(), emphasis: Some(Emphasis::Primary), status: None },
                Item { label: "Outer".to_string(), emphasis: None, status: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("stroke-width=\"3.0\""), "primary layer should be stroked");
    }

    #[test]
    fn render_long_label_truncates() {
        let d = ItemsDiagram {
            title: None,
            // A long label in the small core ring cannot fit and should be truncated with an ellipsis.
            items: vec![
                Item {
                    label: "An Extremely Long Core Layer Label That Cannot Possibly Fit".to_string(),
                    emphasis: None,
                    status: None,
                },
                Item { label: "B".to_string(), emphasis: None, status: None },
                Item { label: "C".to_string(), emphasis: None, status: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains('…'), "an oversized label should be truncated with an ellipsis");
    }
}
