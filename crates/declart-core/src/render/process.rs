use crate::model::{Emphasis, ItemsDiagram};
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const BOX_HEIGHT: f32 = 60.0;
const BOX_MIN_WIDTH: f32 = 100.0;
const ARROW_GAP: f32 = 20.0;
const PADDING: f32 = 20.0;
const TITLE_AREA: f32 = 50.0;
const ARROW_HEAD_SIZE: f32 = 12.0;
const ARROW_SHAFT_HEIGHT: f32 = 4.0;

pub fn render(diagram: &ItemsDiagram, theme: &Theme) -> String {
    let n = diagram.items.len() as f32;
    let title_y = if diagram.title.is_some() { TITLE_AREA } else { PADDING };
    let canvas_height = title_y + BOX_HEIGHT + PADDING;

    let canvas_width = f32::max(
        600.0,
        n * (BOX_MIN_WIDTH + ARROW_GAP) - ARROW_GAP + 2.0 * PADDING,
    );
    let box_width = (canvas_width - 2.0 * PADDING - (n - 1.0) * ARROW_GAP) / n;

    let mut builder = SvgBuilder::new(canvas_width, canvas_height);

    if let Some(title) = &diagram.title {
        builder.text(
            canvas_width / 2.0,
            TITLE_AREA / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    let box_top = title_y;
    let box_bottom = box_top + BOX_HEIGHT;
    let arrow_y = box_top + BOX_HEIGHT / 2.0;

    for i in 0..diagram.items.len().saturating_sub(1) {
        let i_f = i as f32;
        let arrow_x1 = PADDING + (i_f + 1.0) * box_width + i_f * ARROW_GAP;
        let arrow_x2 = arrow_x1 + ARROW_GAP;
        draw_right_arrow(&mut builder, arrow_x1, arrow_x2, arrow_y, &theme.layers.apex.to_hex());
    }

    for (i, item) in diagram.items.iter().enumerate() {
        let i_f = i as f32;
        let n_items = diagram.items.len() as f32;
        let t = if n_items > 1.0 { i_f / (n_items - 1.0) } else { 0.0 };
        let base_color = theme.layers.apex.interpolate(&theme.layers.base, t);
        let box_color = match &item.emphasis {
            Some(Emphasis::Secondary) => base_color.interpolate(&theme.layers.base, 0.3),
            _ => base_color,
        };
        let text_color = if box_color.is_dark() {
            &theme.text.on_dark
        } else {
            &theme.text.on_light
        };
        let (stroke_color, stroke_width) = match &item.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };

        let box_left = PADDING + i_f * (box_width + ARROW_GAP);
        let box_right = box_left + box_width;

        builder.polygon_stroked(
            &[
                (box_left, box_top),
                (box_right, box_top),
                (box_right, box_bottom),
                (box_left, box_bottom),
            ],
            &box_color.to_hex(),
            &stroke_color,
            stroke_width,
        );

        let available_width = box_width * 0.85;
        let mut font_size = theme.typography.label_size;
        let text_width = font::measure_text(&item.label, font_size);
        if text_width > available_width && available_width > 0.0 {
            font_size = (font_size * available_width / text_width)
                .max(theme.typography.label_size_min);
        }
        let display_label = if font::measure_text(&item.label, font_size) > available_width {
            font::truncate_text(&item.label, font_size, available_width)
        } else {
            item.label.clone()
        };

        let is_bold = matches!(&item.emphasis, Some(Emphasis::Primary));
        let center_x = box_left + box_width / 2.0;
        let center_y = box_top + BOX_HEIGHT / 2.0;
        builder.text_weighted(center_x, center_y, &display_label, &text_color.to_hex(), font_size, is_bold);
    }

    builder.build(&theme.background.to_hex())
}

fn draw_right_arrow(builder: &mut SvgBuilder, x1: f32, x2: f32, y: f32, fill: &str) {
    let shaft_end = x2 - ARROW_HEAD_SIZE;
    let half_shaft = ARROW_SHAFT_HEIGHT / 2.0;
    let half_head = ARROW_HEAD_SIZE / 2.0;

    if shaft_end > x1 {
        builder.polygon(
            &[
                (x1, y - half_shaft),
                (shaft_end, y - half_shaft),
                (shaft_end, y + half_shaft),
                (x1, y + half_shaft),
            ],
            fill,
            "none",
        );
    }

    builder.polygon(
        &[(shaft_end, y - half_head), (x2, y), (shaft_end, y + half_head)],
        fill,
        "none",
    );
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
        let d = make_diagram(3, Some("Test"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_diagram(3, None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Step 1"));
        assert!(svg.contains("Step 2"));
        assert!(svg.contains("Step 3"));
    }

    #[test]
    fn render_has_n_boxes() {
        let d = make_diagram(4, None);
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<polygon").count();
        assert!(count >= 4, "expected at least 4 polygons, got {}", count);
    }

    #[test]
    fn single_item_renders_without_arrows() {
        let d = make_diagram(1, None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Step 1"));
        assert!(!svg.contains("Step 2"));
    }
}
