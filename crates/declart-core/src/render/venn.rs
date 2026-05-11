use std::f32::consts::PI;

use crate::model::VennDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_SIZE: f32 = 520.0;
const TITLE_AREA: f32 = 50.0;
const SET_LABEL_SIZE: f32 = 15.0;
const INTERSECTION_LABEL_SIZE: f32 = 12.0;
const CIRCLE_OPACITY: f32 = 0.45;

pub fn render(diagram: &VennDiagram, theme: &Theme) -> String {
    let n = diagram.sets.len();
    let title_h = if diagram.title.is_some() { TITLE_AREA } else { 20.0 };
    let canvas_w = CANVAS_SIZE;
    let canvas_h = CANVAS_SIZE + title_h;
    let cx = canvas_w / 2.0;
    let cy = title_h + CANVAS_SIZE / 2.0;

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

    let r = if n == 2 { 160.0_f32 } else { 150.0_f32 };
    let overlap = r * 0.55; // distance between circle centers

    // Circle centers
    let centers: Vec<(f32, f32)> = match n {
        2 => vec![(cx - overlap / 2.0, cy), (cx + overlap / 2.0, cy)],
        3 => {
            // Equilateral triangle arrangement
            let tri_r = overlap / (3.0_f32.sqrt()); // circumradius of equilateral triangle with side = overlap
            (0..3)
                .map(|i| {
                    let angle = -PI / 2.0 + 2.0 * PI * i as f32 / 3.0;
                    (cx + tri_r * angle.cos(), cy + tri_r * angle.sin())
                })
                .collect()
        }
        _ => vec![],
    };

    // Color palette for sets
    let colors = [
        theme.layers.apex,
        theme.layers.apex.interpolate(&theme.layers.base, 0.5),
        theme.layers.base,
    ];

    // Draw circles with opacity
    for (i, (ccx, ccy)) in centers.iter().enumerate() {
        let color = colors[i % colors.len()];
        builder.circle(*ccx, *ccy, r, &color.to_hex(), CIRCLE_OPACITY);
    }

    // Set labels — positioned toward the outer edge (away from center)
    for (i, (ccx, ccy)) in centers.iter().enumerate() {
        let dx = ccx - cx;
        let dy = ccy - cy;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let label_x = ccx + (dx / len) * (r * 0.65);
        let label_y = ccy + (dy / len) * (r * 0.65);

        builder.text(
            label_x,
            label_y,
            &diagram.sets[i].label,
            &theme.title_color.to_hex(),
            SET_LABEL_SIZE,
        );
    }

    // Intersection labels
    for intersection in &diagram.intersections {
        let relevant: Vec<usize> = intersection
            .sets
            .iter()
            .filter_map(|s| diagram.sets.iter().position(|set| set.label == *s))
            .collect();

        if relevant.is_empty() {
            continue;
        }

        // Position: centroid of the relevant circle centers, pulled toward diagram center
        let lx = relevant.iter().map(|&i| centers[i].0).sum::<f32>() / relevant.len() as f32;
        let ly = relevant.iter().map(|&i| centers[i].1).sum::<f32>() / relevant.len() as f32;
        // For 2-set intersection, pull toward diagram center a bit
        let pull = if relevant.len() == 2 { 0.3 } else { 0.0 };
        let lx = lx + (cx - lx) * pull;
        let ly = ly + (cy - ly) * pull;

        let available = r * 0.7;
        let mut fs = INTERSECTION_LABEL_SIZE;
        let tw = font::measure_text(&intersection.label, fs);
        if tw > available && available > 0.0 {
            fs = (fs * available / tw).max(theme.typography.label_size_min);
        }
        builder.text(lx, ly, &intersection.label, &theme.title_color.to_hex(), fs);
    }

    builder.build(&theme.background.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{VennDiagram, VennIntersection, VennSet};
    use crate::render::DEFAULT_THEME;

    fn make_two_set(title: Option<&str>) -> VennDiagram {
        VennDiagram {
            title: title.map(String::from),
            sets: vec![
                VennSet { label: "A".to_string() },
                VennSet { label: "B".to_string() },
            ],
            intersections: vec![VennIntersection {
                sets: vec!["A".to_string(), "B".to_string()],
                label: "A ∩ B".to_string(),
            }],
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_two_set(Some("Test"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_set_labels() {
        let d = make_two_set(None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains(">A<") || svg.contains("\"A\"") || svg.contains(">A "));
        assert!(svg.contains("A ∩ B"));
    }

    #[test]
    fn render_has_two_circles() {
        let d = make_two_set(None);
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<circle").count();
        assert_eq!(count, 2, "expected 2 circles for 2-set Venn");
    }

    #[test]
    fn render_three_set_has_three_circles() {
        let d = VennDiagram {
            title: None,
            sets: vec![
                VennSet { label: "X".to_string() },
                VennSet { label: "Y".to_string() },
                VennSet { label: "Z".to_string() },
            ],
            intersections: vec![],
        };
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<circle").count();
        assert_eq!(count, 3);
    }
}
