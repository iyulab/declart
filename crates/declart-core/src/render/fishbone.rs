use crate::model::FishboneDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_WIDTH: f32 = 860.0;
const TITLE_AREA: f32 = 50.0;
const SPINE_MARGIN_LEFT: f32 = 40.0;
const SPINE_MARGIN_RIGHT: f32 = 180.0; // space for effect box
const BRANCH_HEIGHT: f32 = 80.0; // vertical reach of each branch
const SUB_BRANCH_LEN: f32 = 50.0;
const EFFECT_BOX_W: f32 = 150.0;
const EFFECT_BOX_H: f32 = 50.0;
const CAUSE_BOX_W: f32 = 110.0;
const CAUSE_BOX_H: f32 = 38.0;
const SPINE_LINE_W: f32 = 3.0;
const BRANCH_LINE_W: f32 = 2.0;
const ARROW_HEAD_LEN: f32 = 14.0;
const ARROW_HEAD_SIZE: f32 = 10.0;

pub fn render(diagram: &FishboneDiagram, theme: &Theme) -> String {
    let title_h = if diagram.title.is_some() { TITLE_AREA } else { 20.0 };
    let canvas_h = title_h + BRANCH_HEIGHT * 2.0 + 60.0 + 40.0;
    let spine_y = title_h + BRANCH_HEIGHT + 40.0;

    let spine_left = SPINE_MARGIN_LEFT;
    let spine_right = CANVAS_WIDTH - SPINE_MARGIN_RIGHT;

    let mut builder = SvgBuilder::new(CANVAS_WIDTH, canvas_h);

    if let Some(title) = &diagram.title {
        builder.text(
            CANVAS_WIDTH / 2.0,
            TITLE_AREA / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    // Draw main spine (ends just before the arrowhead)
    let effect_x = CANVAS_WIDTH - EFFECT_BOX_W - 10.0;
    builder.line(
        spine_left,
        spine_y,
        effect_x - ARROW_HEAD_LEN,
        spine_y,
        &theme.layers.apex.to_hex(),
        SPINE_LINE_W,
    );
    // Arrowhead pointing right into the effect box (fish head)
    builder.polygon(
        &[
            (effect_x - ARROW_HEAD_LEN, spine_y - ARROW_HEAD_SIZE),
            (effect_x, spine_y),
            (effect_x - ARROW_HEAD_LEN, spine_y + ARROW_HEAD_SIZE),
        ],
        &theme.layers.apex.to_hex(),
        "none",
    );

    // Effect box at right end
    let effect_y = spine_y - EFFECT_BOX_H / 2.0;
    builder.polygon(
        &[
            (effect_x, effect_y),
            (effect_x + EFFECT_BOX_W, effect_y),
            (effect_x + EFFECT_BOX_W, effect_y + EFFECT_BOX_H),
            (effect_x, effect_y + EFFECT_BOX_H),
        ],
        &theme.layers.apex.to_hex(),
        "none",
    );
    let effect_text_color = if theme.layers.apex.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
    let mut efs = theme.typography.label_size;
    let etw = font::measure_text(&diagram.effect, efs);
    let eavail = EFFECT_BOX_W * 0.85;
    if etw > eavail {
        efs = (efs * eavail / etw).max(theme.typography.label_size_min);
    }
    builder.text(
        effect_x + EFFECT_BOX_W / 2.0,
        spine_y,
        &diagram.effect,
        &effect_text_color.to_hex(),
        efs,
    );

    // Distribute causes along the spine
    let n = diagram.causes.len();
    let spine_len = spine_right - spine_left;
    // Causes placed at evenly spaced x positions
    // Each cause connects to the spine at a "foot" point on the spine
    let cause_color = theme.layers.apex.interpolate(&theme.layers.base, 0.35);
    let sub_color = theme.layers.apex.interpolate(&theme.layers.base, 0.65);
    let text_color = if cause_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };
    let sub_text_color = if sub_color.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };

    for (i, cause) in diagram.causes.iter().enumerate() {
        let above = i % 2 == 0;
        // Foot: x position along spine (from left, skipping leftmost 10% and rightmost 10%)
        let foot_x = spine_left + spine_len * 0.1 + (i as f32 + 0.5) / n as f32 * spine_len * 0.8;
        let foot_y = spine_y;

        // Branch end (where cause box is centered)
        let sign = if above { -1.0_f32 } else { 1.0_f32 };
        let head_y = spine_y + sign * BRANCH_HEIGHT;
        let head_x = foot_x - BRANCH_HEIGHT * 0.6; // angle ~56° from spine

        // Draw branch line
        builder.line(foot_x, foot_y, head_x, head_y, &theme.layers.apex.to_hex(), BRANCH_LINE_W);

        // Cause box centered at head
        let bx = head_x - CAUSE_BOX_W / 2.0;
        let by = head_y - CAUSE_BOX_H / 2.0;
        builder.polygon(
            &[(bx, by), (bx + CAUSE_BOX_W, by), (bx + CAUSE_BOX_W, by + CAUSE_BOX_H), (bx, by + CAUSE_BOX_H)],
            &cause_color.to_hex(),
            "none",
        );

        let available = CAUSE_BOX_W * 0.85;
        let mut fs = theme.typography.label_size;
        let tw = font::measure_text(&cause.label, fs);
        if tw > available {
            fs = (fs * available / tw).max(theme.typography.label_size_min);
        }
        builder.text(head_x, head_y, &cause.label, &text_color.to_hex(), fs);

        // Sub-items: diagonal branches off the cause branch (45° away from spine)
        let diag = SUB_BRANCH_LEN * std::f32::consts::FRAC_1_SQRT_2;
        for (j, item) in cause.items.iter().enumerate() {
            let t = (j as f32 + 1.0) / (cause.items.len() as f32 + 1.0);
            let sx = foot_x + (head_x - foot_x) * t;
            let sy = foot_y + (head_y - foot_y) * t;

            // Diagonal: go left and outward (away from spine) at 45°
            let sub_end_x = sx - diag;
            let sub_end_y = sy + sign * diag; // sign=-1 for above → goes up; +1 for below → goes down
            builder.line(sx, sy, sub_end_x, sub_end_y, &sub_color.to_hex(), 1.5);

            let mut sfs = theme.typography.label_size - 2.0;
            let stw = font::measure_text(&item.label, sfs);
            let savail = SUB_BRANCH_LEN * 1.5;
            if stw > savail {
                sfs = (sfs * savail / stw).max(theme.typography.label_size_min);
            }
            builder.text(sub_end_x, sub_end_y + sign * sfs * 0.7, &item.label, &sub_text_color.to_hex(), sfs);
        }
    }

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FishboneCause, FishboneDiagram, Item};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(n_causes: usize, with_items: bool) -> FishboneDiagram {
        FishboneDiagram {
            title: Some("Test".to_string()),
            effect: "Effect".to_string(),
            causes: (0..n_causes)
                .map(|i| FishboneCause {
                    label: format!("Cause {}", i + 1),
                    items: if with_items && i == 0 {
                        vec![Item { label: "Sub 1".to_string(), emphasis: None }]
                    } else {
                        vec![]
                    },
                })
                .collect(),
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_diagram(4, false);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_effect_and_causes() {
        let d = make_diagram(4, false);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Effect"));
        assert!(svg.contains("Cause 1"));
        assert!(svg.contains("Cause 4"));
    }

    #[test]
    fn render_includes_sub_items() {
        let d = make_diagram(2, true);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Sub 1"));
    }

    #[test]
    fn render_has_effect_box_and_arrowhead() {
        let d = make_diagram(2, false);
        let svg = render(&d, &DEFAULT_THEME);
        // 1 arrowhead (triangle) + 1 effect box + 2 cause boxes = 4 polygons minimum
        let polygon_count = svg.matches("<polygon").count();
        assert!(polygon_count >= 4, "expected at least 4 polygons (arrowhead + effect box + cause boxes)");
    }

    #[test]
    fn render_arrowhead_is_triangle() {
        let d = make_diagram(2, false);
        let svg = render(&d, &DEFAULT_THEME);
        // At least one polygon has 3 points (the arrowhead triangle)
        let has_triangle = svg.split("<polygon").skip(1).any(|chunk| {
            if let Some(start) = chunk.find("points=\"") {
                let rest = &chunk[start + 8..];
                if let Some(end) = rest.find('"') {
                    return rest[..end].split_whitespace().count() == 3;
                }
            }
            false
        });
        assert!(has_triangle, "should have a triangular arrowhead polygon");
    }
}
