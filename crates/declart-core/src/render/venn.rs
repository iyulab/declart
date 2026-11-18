use std::f32::consts::PI;

use crate::model::VennDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_SIZE: f32 = 520.0;
const TITLE_AREA: f32 = 50.0;
const SET_LABEL_SIZE: f32 = 15.0;
const INTERSECTION_LABEL_SIZE: f32 = 12.0;
const CIRCLE_OPACITY: f32 = 0.45;

/// Center spacing of a three-set diagram, in radii.
const TRIPLE_SPACING: f32 = 1.0;
/// How far from the diagram center the label of a set's own region sits, in radii.
const SET_LABEL_REACH: f32 = 1.08;
/// How far from the diagram center the label of a two-set overlap sits, in radii, on the side
/// away from the third circle.
const PAIR_LABEL_REACH: f32 = 0.78;

pub fn render(diagram: &VennDiagram, theme: &Theme) -> String {
    let n = diagram.sets.len();
    let title_h = if diagram.title.is_some() {
        TITLE_AREA
    } else {
        20.0
    };
    let canvas_w = CANVAS_SIZE;
    let canvas_h = CANVAS_SIZE + title_h;
    let cx = canvas_w / 2.0;
    let mut cy = title_h + CANVAS_SIZE / 2.0;

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
    // Distance between circle centers. Two sets overlap broadly; three sets sit on a triangle
    // whose sides equal the radius, so each circle passes through the other two's centers and
    // every one of the seven regions is big enough to hold a label.
    let overlap = if n == 2 { r * 0.55 } else { r * TRIPLE_SPACING };

    if n == 3 {
        // The triangle is taller above its center than below; center the whole figure.
        let tri_r = r * TRIPLE_SPACING / 3.0_f32.sqrt();
        cy += ((tri_r + r) - (tri_r / 2.0 + r)) / 2.0;
    }

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

    // Unit vector from the diagram center toward each circle's center.
    let outward: Vec<(f32, f32)> = centers
        .iter()
        .map(|(ccx, ccy)| {
            let (dx, dy) = (ccx - cx, ccy - cy);
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            (dx / len, dy / len)
        })
        .collect();

    // Set labels — in the part of a circle that no other circle covers.
    for (i, (ccx, ccy)) in centers.iter().enumerate() {
        let (label_x, label_y) = if n == 3 {
            (
                cx + outward[i].0 * r * SET_LABEL_REACH,
                cy + outward[i].1 * r * SET_LABEL_REACH,
            )
        } else {
            (
                ccx + outward[i].0 * (r * 0.65),
                ccy + outward[i].1 * (r * 0.65),
            )
        };

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

        let (lx, ly) = if n == 3 && relevant.len() == 2 {
            // The region lying in both circles and outside the third: on the axis from the
            // diagram center away from the third circle.
            let third = (0..3).find(|i| !relevant.contains(i)).unwrap_or(0);
            (
                cx - outward[third].0 * r * PAIR_LABEL_REACH,
                cy - outward[third].1 * r * PAIR_LABEL_REACH,
            )
        } else if n == 3 {
            // Covered by every circle: the diagram center.
            (cx, cy)
        } else {
            // Two sets: the middle of the lens.
            let lx = relevant.iter().map(|&i| centers[i].0).sum::<f32>() / relevant.len() as f32;
            let ly = relevant.iter().map(|&i| centers[i].1).sum::<f32>() / relevant.len() as f32;
            (lx, ly)
        };

        let available = r * 0.7;
        let mut fs = INTERSECTION_LABEL_SIZE;
        let tw = font::measure_text(&intersection.label, fs);
        if tw > available && available > 0.0 {
            fs = (fs * available / tw).max(theme.typography.label_size_min);
        }
        builder.text(lx, ly, &intersection.label, &theme.title_color.to_hex(), fs);
    }

    builder.build(theme.backdrop_fill().as_deref())
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
                VennSet {
                    label: "A".to_string(),
                },
                VennSet {
                    label: "B".to_string(),
                },
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
                VennSet {
                    label: "X".to_string(),
                },
                VennSet {
                    label: "Y".to_string(),
                },
                VennSet {
                    label: "Z".to_string(),
                },
            ],
            intersections: vec![],
        };
        let svg = render(&d, &DEFAULT_THEME);
        let count = svg.matches("<circle").count();
        assert_eq!(count, 3);
    }

    /// Numeric value of `name="…"` in a tag.
    fn attr(tag: &str, name: &str) -> f32 {
        let key = format!("{name}=\"");
        let start = tag.find(&key).expect("attribute") + key.len();
        let end = tag[start..].find('"').unwrap();
        tag[start..start + end].parse().unwrap()
    }

    /// Which circles (by index) cover the point where the text `label` is drawn.
    fn circles_covering(svg: &str, label: &str) -> Vec<usize> {
        let circles: Vec<(f32, f32, f32)> = svg
            .split("<circle")
            .skip(1)
            .map(|c| (attr(c, "cx"), attr(c, "cy"), attr(c, "r")))
            .collect();
        let tag = svg
            .split("<text")
            .find(|t| t.contains(&format!(">{label}</text>")))
            .expect("label drawn");
        let (x, y) = (attr(tag, "x"), attr(tag, "y"));
        circles
            .iter()
            .enumerate()
            .filter(|(_, (cx, cy, r))| (x - cx).hypot(y - cy) < *r)
            .map(|(i, _)| i)
            .collect()
    }

    #[test]
    fn three_set_labels_sit_in_their_own_regions() {
        let set = |l: &str| VennSet { label: l.into() };
        let hit = |names: &[&str], label: &str| VennIntersection {
            sets: names.iter().map(|n| n.to_string()).collect(),
            label: label.into(),
        };
        let d = VennDiagram {
            title: None,
            sets: vec![set("Frontend"), set("Backend"), set("DevOps")],
            intersections: vec![
                hit(&["Frontend", "Backend"], "APIs"),
                hit(&["Backend", "DevOps"], "Infra"),
                hit(&["Frontend", "DevOps"], "CI/CD"),
                hit(&["Frontend", "Backend", "DevOps"], "Full-Stack"),
            ],
        };
        let svg = render(&d, &DEFAULT_THEME);
        // A set's own label is covered by that circle alone; an overlap by exactly its circles.
        assert_eq!(circles_covering(&svg, "Frontend"), vec![0]);
        assert_eq!(circles_covering(&svg, "Backend"), vec![1]);
        assert_eq!(circles_covering(&svg, "DevOps"), vec![2]);
        assert_eq!(circles_covering(&svg, "APIs"), vec![0, 1]);
        assert_eq!(circles_covering(&svg, "Infra"), vec![1, 2]);
        assert_eq!(circles_covering(&svg, "CI/CD"), vec![0, 2]);
        assert_eq!(circles_covering(&svg, "Full-Stack"), vec![0, 1, 2]);
    }

    #[test]
    fn three_set_figure_stays_inside_the_canvas() {
        let d = VennDiagram {
            title: Some("T".into()),
            sets: ["X", "Y", "Z"]
                .iter()
                .map(|l| VennSet {
                    label: l.to_string(),
                })
                .collect(),
            intersections: vec![],
        };
        let svg = render(&d, &DEFAULT_THEME);
        let height = attr(svg.split("<svg").nth(1).unwrap(), "height");
        for c in svg.split("<circle").skip(1) {
            let (cx, cy, r) = (attr(c, "cx"), attr(c, "cy"), attr(c, "r"));
            assert!(cx - r >= 0.0 && cx + r <= CANVAS_SIZE, "x out of canvas");
            assert!(
                cy - r >= TITLE_AREA - 10.0 && cy + r <= height,
                "y out of canvas"
            );
        }
    }
}
