use crate::model::{Emphasis, ItemsDiagram};
use crate::render::{font, status, svg::SvgBuilder, theme::Theme};

const CANVAS_W: f32 = 600.0;
const STAGE_HEIGHT: f32 = 60.0;
const PADDING_H: f32 = 20.0;
const TITLE_AREA: f32 = 60.0;
const BOTTOM_PAD: f32 = 20.0;
const TOP_WIDTH_FRAC: f32 = 0.92;
const SHRINK_PER_STEP: f32 = 0.12;
const CENTER_X: f32 = CANVAS_W / 2.0;

pub fn render(diagram: &ItemsDiagram, theme: &Theme) -> String {
    let n = diagram.items.len() as f32;
    let title_height = if diagram.title.is_some() { TITLE_AREA } else { PADDING_H };
    let canvas_height = title_height + n * STAGE_HEIGHT + BOTTOM_PAD;

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

    for (i, item) in diagram.items.iter().enumerate() {
        let i_f = i as f32;
        let n_items = n;

        let t = if n_items > 1.0 { i_f / (n_items - 1.0) } else { 0.0 };
        let base_color = theme.layers.apex.interpolate(&theme.layers.base, t);
        let stage_color = match &item.emphasis {
            Some(Emphasis::Secondary) => base_color.interpolate(&theme.layers.base, 0.3),
            _ => base_color,
        };
        let text_color = if stage_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        let (stroke_color, stroke_width) = match &item.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };

        // Stage i: top_width = full_width * (1 - i * shrink)
        //          bottom_width = full_width * (1 - (i+1) * shrink)
        let full_w = (CANVAS_W - 2.0 * PADDING_H) * TOP_WIDTH_FRAC;
        let top_w = (full_w * (1.0 - i_f * SHRINK_PER_STEP)).max(full_w * 0.1);
        let bot_w = (full_w * (1.0 - (i_f + 1.0) * SHRINK_PER_STEP)).max(full_w * 0.1);

        let top_y = title_height + i_f * STAGE_HEIGHT;
        let bot_y = top_y + STAGE_HEIGHT;

        builder.polygon_stroked(
            &[
                (CENTER_X - top_w / 2.0, top_y),
                (CENTER_X + top_w / 2.0, top_y),
                (CENTER_X + bot_w / 2.0, bot_y),
                (CENTER_X - bot_w / 2.0, bot_y),
            ],
            &stage_color.to_hex(),
            &stroke_color,
            stroke_width,
        );

        let available_width = top_w.min(bot_w) * 0.85;
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
        builder.text_weighted(
            CENTER_X,
            top_y + STAGE_HEIGHT / 2.0,
            &display_label,
            &text_color.to_hex(),
            font_size,
            is_bold,
        );

        status::draw_marker(&mut builder, &item.status, CENTER_X + top_w / 2.0, top_y, theme);
    }

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Item, ItemsDiagram};
    use crate::render::DEFAULT_THEME;

    fn make_funnel(n: usize) -> ItemsDiagram {
        ItemsDiagram {
            title: Some("Funnel".to_string()),
            items: (0..n)
                .map(|i| Item { label: format!("Stage {}", i + 1), emphasis: None, status: None })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_funnel(4);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_funnel(3);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Stage 1"));
        assert!(svg.contains("Stage 2"));
        assert!(svg.contains("Stage 3"));
    }

    #[test]
    fn render_has_n_trapezoids() {
        let d = make_funnel(5);
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 5, "expected 5 trapezoid polygons");
    }
}
