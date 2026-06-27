use crate::model::FlowDiagram;
use crate::render::theme::Theme;
use super::{font, status, svg::SvgBuilder};

const LANE_WIDTH: f32 = 180.0;
const STEP_HEIGHT: f32 = 56.0;
const STEP_V_GAP: f32 = 36.0;
const HEADER_HEIGHT: f32 = 40.0;
const TITLE_AREA: f32 = 50.0;
const PADDING: f32 = 20.0;
const STEP_H_MARGIN: f32 = 14.0;

pub(crate) fn render(d: &FlowDiagram, theme: &Theme) -> String {
    // Collect unique actors in first-appearance order
    let mut actors: Vec<&str> = Vec::new();
    for item in &d.items {
        let a = item.actor.as_deref().unwrap_or("");
        if !actors.contains(&a) {
            actors.push(a);
        }
    }
    let num_actors = actors.len();
    let num_steps = d.items.len();

    let title_area = if d.title.is_some() { TITLE_AREA } else { 0.0 };
    let canvas_width = (PADDING + num_actors as f32 * LANE_WIDTH + PADDING).max(400.0);
    let canvas_height = title_area
        + HEADER_HEIGHT
        + num_steps as f32 * (STEP_HEIGHT + STEP_V_GAP)
        + PADDING;

    let mut b = SvgBuilder::new(canvas_width, canvas_height);

    // Title
    if let Some(title) = &d.title {
        b.text(canvas_width / 2.0, title_area / 2.0, title,
               &theme.title_color.to_hex(), theme.typography.title_size);
    }

    let header_top = title_area;
    let content_top = header_top + HEADER_HEIGHT;
    let content_bot = canvas_height - PADDING;

    // Vertical lane dividers (full height, skip first)
    for col in 1..num_actors {
        let x = PADDING + col as f32 * LANE_WIDTH;
        b.line(x, header_top, x, content_bot,
               &theme.layers.base.to_hex(), 1.0);
    }

    // Actor header labels with background
    for (col, &actor) in actors.iter().enumerate() {
        let lane_cx = PADDING + col as f32 * LANE_WIDTH + LANE_WIDTH / 2.0;
        let t = if num_actors > 1 { col as f32 / (num_actors - 1) as f32 } else { 0.0 };
        let bg = theme.layers.apex.interpolate(&theme.layers.base, t);
        let text_col = if bg.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        b.rect_rounded(PADDING + col as f32 * LANE_WIDTH, header_top,
                       LANE_WIDTH, HEADER_HEIGHT, 0,
                       &bg.to_hex(), "none", 0.0);
        b.text(lane_cx, header_top + HEADER_HEIGHT / 2.0, actor,
               &text_col.to_hex(), theme.typography.label_size);
    }

    // Steps and arrows
    let step_width = LANE_WIDTH - STEP_H_MARGIN * 2.0;
    for (i, item) in d.items.iter().enumerate() {
        let actor = item.actor.as_deref().unwrap_or("");
        let col = actors.iter().position(|&a| a == actor).unwrap_or(0);
        let lane_cx = PADDING + col as f32 * LANE_WIDTH + LANE_WIDTH / 2.0;
        let box_x = PADDING + col as f32 * LANE_WIDTH + STEP_H_MARGIN;
        let box_y = content_top + i as f32 * (STEP_HEIGHT + STEP_V_GAP) + STEP_V_GAP / 2.0;

        let t = if num_actors > 1 { col as f32 / (num_actors - 1) as f32 } else { 0.0 };
        let base_color = theme.layers.apex.interpolate(&theme.layers.base, t)
            .interpolate(&theme.background, 0.25);
        let box_color = match &item.emphasis {
            Some(crate::model::Emphasis::Secondary) => base_color.interpolate(&theme.background, 0.35),
            _ => base_color,
        };
        let text_col = if box_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
        let (stroke_color, stroke_width) = match &item.emphasis {
            Some(crate::model::Emphasis::Primary) => (theme.background.to_hex(), 3.0_f32),
            _ => (theme.layers.apex.to_hex(), 1.0_f32),
        };

        b.rect_rounded(box_x, box_y, step_width, STEP_HEIGHT, 6,
                       &box_color.to_hex(), &stroke_color, stroke_width);

        let is_bold = matches!(&item.emphasis, Some(crate::model::Emphasis::Primary));
        let mut font_size = theme.typography.label_size;
        let avail = step_width * 0.85;
        let tw = font::measure_text(&item.label, font_size);
        if tw > avail {
            font_size = (font_size * avail / tw).max(theme.typography.label_size_min);
        }
        let label = if font::measure_text(&item.label, font_size) > avail {
            font::truncate_text(&item.label, font_size, avail)
        } else {
            item.label.clone()
        };
        b.text_weighted(lane_cx, box_y + STEP_HEIGHT / 2.0, &label, &text_col.to_hex(), font_size, is_bold);

        status::draw_marker(&mut b, &item.status, box_x + step_width, box_y, theme);

        // Arrow to next step
        if let Some(next) = d.items.get(i + 1) {
            let next_actor = next.actor.as_deref().unwrap_or("");
            let next_col = actors.iter().position(|&a| a == next_actor).unwrap_or(0);
            let next_cx = PADDING + next_col as f32 * LANE_WIDTH + LANE_WIDTH / 2.0;
            let next_box_y = content_top + (i + 1) as f32 * (STEP_HEIGHT + STEP_V_GAP)
                + STEP_V_GAP / 2.0;
            draw_arrow(&mut b, lane_cx, box_y + STEP_HEIGHT,
                       next_cx, next_box_y, &theme.layers.apex.to_hex());
        }
    }

    b.build(&theme.background.to_hex())
}

fn draw_arrow(b: &mut SvgBuilder, x1: f32, y1: f32, x2: f32, y2: f32, color: &str) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1.0 { return; }
    let head = 10.0_f32;
    let ux = dx / len;
    let uy = dy / len;
    let sx = x2 - ux * head;
    let sy = y2 - uy * head;
    b.line(x1, y1, sx, sy, color, 1.5);
    let px = -uy * head * 0.4;
    let py = ux * head * 0.4;
    b.polygon(&[(sx + px, sy + py), (x2, y2), (sx - px, sy - py)], color, "none");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FlowDiagram, FlowItem, FlowView};
    use crate::render::DEFAULT_THEME;

    fn make_swimlane(actors_and_labels: &[(&str, &str)], title: Option<&str>) -> FlowDiagram {
        FlowDiagram {
            title: title.map(String::from),
            view: FlowView::Swimlane,
            items: actors_and_labels.iter().map(|(actor, label)| FlowItem {
                actor: Some(actor.to_string()),
                label: label.to_string(),
                emphasis: None,
                status: None,
            }).collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_swimlane(&[("A", "Step 1"), ("B", "Step 2")], None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_actor_names() {
        let d = make_swimlane(&[("고객", "주문"), ("시스템", "처리")], Some("Flow"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("고객"));
        assert!(svg.contains("시스템"));
    }

    #[test]
    fn render_includes_step_labels() {
        let d = make_swimlane(&[("A", "Alpha"), ("B", "Beta"), ("A", "Gamma")], None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Alpha"));
        assert!(svg.contains("Beta"));
        assert!(svg.contains("Gamma"));
    }

    #[test]
    fn render_includes_title() {
        let d = make_swimlane(&[("A", "X"), ("B", "Y")], Some("My Title"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("My Title"));
    }
}
