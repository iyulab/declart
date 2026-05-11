use crate::model::DiagramModel;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_WIDTH: f32 = 600.0;
const LAYER_HEIGHT: f32 = 60.0;
const PADDING_H: f32 = 20.0;
const TITLE_AREA: f32 = 60.0;
const BOTTOM_PAD: f32 = 20.0;
const PYRAMID_WIDTH: f32 = CANVAS_WIDTH - PADDING_H * 2.0;
const CENTER_X: f32 = CANVAS_WIDTH / 2.0;

pub fn render(model: &DiagramModel, theme: &Theme) -> String {
    let n = model.items.len() as f32;
    let title_height = if model.title.is_some() { TITLE_AREA } else { PADDING_H };
    let canvas_height = title_height + n * LAYER_HEIGHT + BOTTOM_PAD;

    let mut builder = SvgBuilder::new(CANVAS_WIDTH, canvas_height);

    if let Some(title) = &model.title {
        builder.text(
            CENTER_X,
            title_height / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    for (i, item) in model.items.iter().enumerate() {
        let i_f = i as f32;
        let n_layers = model.items.len() as f32;

        let t = if n_layers > 1.0 { i_f / (n_layers - 1.0) } else { 0.0 };
        let layer_color = theme.layers.apex.interpolate(&theme.layers.base, t);
        let text_color = if layer_color.is_dark() {
            &theme.text.on_dark
        } else {
            &theme.text.on_light
        };

        let top_width = PYRAMID_WIDTH * (i_f / n_layers);
        let bottom_width = PYRAMID_WIDTH * ((i_f + 1.0) / n_layers);
        let top_y = title_height + i_f * LAYER_HEIGHT;
        let bottom_y = top_y + LAYER_HEIGHT;

        let top_left = CENTER_X - top_width / 2.0;
        let top_right = CENTER_X + top_width / 2.0;
        let bottom_left = CENTER_X - bottom_width / 2.0;
        let bottom_right = CENTER_X + bottom_width / 2.0;

        builder.polygon(
            &[
                (top_left, top_y),
                (top_right, top_y),
                (bottom_right, bottom_y),
                (bottom_left, bottom_y),
            ],
            &layer_color.to_hex(),
            "none",
        );

        let available_width = bottom_width * 0.8;
        let mut font_size = theme.typography.label_size;
        if available_width > 0.0 {
            let text_width = font::measure_text(&item.label, font_size);
            if text_width > available_width {
                font_size = (font_size * available_width / text_width)
                    .max(theme.typography.label_size_min);
            }
        }

        let center_y = (top_y + bottom_y) / 2.0;
        builder.text(CENTER_X, center_y, &item.label, &text_color.to_hex(), font_size);
    }

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DiagramKind, DiagramModel, Item};
    use crate::render::DEFAULT_THEME;

    fn make_model(n: usize, title: Option<&str>) -> DiagramModel {
        DiagramModel {
            kind: DiagramKind::Pyramid,
            title: title.map(String::from),
            items: (0..n)
                .map(|i| Item { label: format!("Layer {}", i), emphasis: None })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let model = make_model(3, Some("Test"));
        let svg = render(&model, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let model = make_model(3, None);
        let svg = render(&model, &DEFAULT_THEME);
        assert!(svg.contains("Layer 0"));
        assert!(svg.contains("Layer 1"));
        assert!(svg.contains("Layer 2"));
    }

    #[test]
    fn render_includes_title_when_present() {
        let model = make_model(2, Some("My Pyramid"));
        let svg = render(&model, &DEFAULT_THEME);
        assert!(svg.contains("My Pyramid"));
    }

    #[test]
    fn render_omits_title_when_absent() {
        let model = make_model(2, None);
        let svg = render(&model, &DEFAULT_THEME);
        assert!(!svg.contains(">My Pyramid<"));
    }

    #[test]
    fn render_has_n_polygons() {
        let model = make_model(5, None);
        let svg = render(&model, &DEFAULT_THEME);
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 5);
    }
}
