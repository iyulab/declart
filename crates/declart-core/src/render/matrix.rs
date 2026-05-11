use crate::model::{Emphasis, MatrixDiagram};
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_SIZE: f32 = 500.0;
const TITLE_AREA: f32 = 50.0;
const PADDING: f32 = 50.0;
const AXIS_LABEL_SIZE: f32 = 13.0;
const AXIS_DIRECTION_SIZE: f32 = 9.0;
const AXIS_LINE_W: f32 = 2.0;

pub fn render(diagram: &MatrixDiagram, theme: &Theme) -> String {
    let title_h = if diagram.title.is_some() { TITLE_AREA } else { 20.0 };
    let canvas_w = CANVAS_SIZE;
    let canvas_h = CANVAS_SIZE + title_h;

    // Grid area: inset by PADDING for axis labels
    let grid_left = PADDING;
    let grid_top = title_h + PADDING / 2.0;
    let grid_right = canvas_w - PADDING / 2.0;
    let grid_bottom = canvas_h - PADDING;
    let grid_w = grid_right - grid_left;
    let grid_h = grid_bottom - grid_top;
    let mid_x = grid_left + grid_w / 2.0;
    let mid_y = grid_top + grid_h / 2.0;

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

    // Quadrant backgrounds (4 cells)
    let colors = [
        theme.layers.apex.interpolate(&theme.layers.base, 0.1),  // top-left
        theme.layers.apex.interpolate(&theme.layers.base, 0.3),  // top-right
        theme.layers.apex.interpolate(&theme.layers.base, 0.6),  // bottom-left
        theme.layers.apex.interpolate(&theme.layers.base, 0.9),  // bottom-right
    ];

    let cells = [
        (grid_left, grid_top, mid_x, mid_y),       // top-left
        (mid_x, grid_top, grid_right, mid_y),       // top-right
        (grid_left, mid_y, mid_x, grid_bottom),     // bottom-left
        (mid_x, mid_y, grid_right, grid_bottom),    // bottom-right
    ];

    for (idx, (x1, y1, x2, y2)) in cells.iter().enumerate() {
        let quadrant = &diagram.quadrants[idx];
        let base_color = &colors[idx];
        let box_color = match &quadrant.emphasis {
            Some(Emphasis::Secondary) => base_color.interpolate(&theme.background, 0.3),
            _ => *base_color,
        };
        let (stroke_color, stroke_width) = match &quadrant.emphasis {
            Some(Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => ("none".to_string(), 0.0_f32),
        };
        let text_color = if box_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        let bold = matches!(&quadrant.emphasis, Some(Emphasis::Primary));

        builder.polygon_stroked(
            &[(*x1, *y1), (*x2, *y1), (*x2, *y2), (*x1, *y2)],
            &box_color.to_hex(),
            &stroke_color,
            stroke_width,
        );

        let cx = (x1 + x2) / 2.0;
        let cy = (y1 + y2) / 2.0;
        let cell_w = (x2 - x1) * 0.85;
        let mut font_size = theme.typography.label_size;
        let text_w = font::measure_text(&quadrant.label, font_size);
        if text_w > cell_w && cell_w > 0.0 {
            font_size = (font_size * cell_w / text_w).max(theme.typography.label_size_min);
        }
        builder.text_weighted(cx, cy, &quadrant.label, &text_color.to_hex(), font_size, bold);
    }

    // Axis divider lines
    let axis_color = &theme.background.to_hex();
    builder.line(grid_left, mid_y, grid_right, mid_y, axis_color, AXIS_LINE_W);
    builder.line(mid_x, grid_top, mid_x, grid_bottom, axis_color, AXIS_LINE_W);

    let label_color = &theme.title_color.to_hex();
    let dir_color = &theme.title_color.to_hex();
    let x_label_y = canvas_h - PADDING / 3.0;
    let y_label_x = PADDING / 3.0;
    let y_label_y = (grid_top + grid_bottom) / 2.0;

    // X-axis: centered label + Low/High direction indicators at grid edges
    builder.text((grid_left + grid_right) / 2.0, x_label_y, &diagram.x_axis, label_color, AXIS_LABEL_SIZE);
    builder.text(grid_left, x_label_y, "Low", dir_color, AXIS_DIRECTION_SIZE);
    builder.text(grid_right, x_label_y, "High", dir_color, AXIS_DIRECTION_SIZE);

    // Y-axis: rotated label + Low/High at grid bottom/top (horizontal, no rotation needed for short words)
    builder.text_rotated(y_label_x, y_label_y, &diagram.y_axis, label_color, AXIS_LABEL_SIZE, -90.0);
    builder.text(y_label_x, grid_bottom, "Low", dir_color, AXIS_DIRECTION_SIZE);
    builder.text(y_label_x, grid_top, "High", dir_color, AXIS_DIRECTION_SIZE);

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Item, MatrixDiagram};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(title: Option<&str>) -> MatrixDiagram {
        MatrixDiagram {
            title: title.map(String::from),
            x_axis: "Importance".to_string(),
            y_axis: "Urgency".to_string(),
            quadrants: vec![
                Item { label: "Do First".to_string(), emphasis: None },
                Item { label: "Schedule".to_string(), emphasis: None },
                Item { label: "Delegate".to_string(), emphasis: None },
                Item { label: "Eliminate".to_string(), emphasis: None },
            ],
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_diagram(Some("Eisenhower"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_quadrant_labels() {
        let d = make_diagram(None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Do First"));
        assert!(svg.contains("Schedule"));
        assert!(svg.contains("Delegate"));
        assert!(svg.contains("Eliminate"));
    }

    #[test]
    fn render_includes_axis_labels() {
        let d = make_diagram(None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Importance"));
        assert!(svg.contains("Urgency"));
        // Direction indicators
        assert!(svg.matches(">Low<").count() >= 2, "should have Low indicator for both axes");
        assert!(svg.matches(">High<").count() >= 2, "should have High indicator for both axes");
    }

    #[test]
    fn render_has_four_quadrant_polygons() {
        let d = make_diagram(None);
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 4, "expected 4 quadrant polygons, got {}", count);
    }

    #[test]
    fn render_primary_quadrant_has_stroke_and_bold() {
        let d = MatrixDiagram {
            title: None,
            x_axis: "X".to_string(),
            y_axis: "Y".to_string(),
            quadrants: vec![
                Item { label: "Primary".to_string(), emphasis: Some(Emphasis::Primary) },
                Item { label: "Q2".to_string(), emphasis: None },
                Item { label: "Q3".to_string(), emphasis: None },
                Item { label: "Q4".to_string(), emphasis: None },
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("font-weight=\"bold\""), "primary quadrant should be bold");
        assert!(svg.contains("stroke-width=\"3.0\""), "primary quadrant should have stroke");
    }
}
