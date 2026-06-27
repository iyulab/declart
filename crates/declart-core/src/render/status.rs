use crate::model::Status;
use crate::render::svg::SvgBuilder;
use crate::render::theme::Theme;

/// Radius of the status badge dot.
pub(crate) const STATUS_MARKER_R: f32 = 6.0;

/// Inset of the marker center from the item's top-right corner (along both axes).
const INSET: f32 = STATUS_MARKER_R + 3.0;

/// Marker shape per status. Shape is a second, color-independent encoding so status stays
/// distinguishable in monochrome and under color-vision deficiency (the `accessible` theme also
/// uses colorblind-safe colors, but shape removes reliance on color entirely).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Circle,
    Triangle,
    Diamond,
}

/// Resolves `(fill_color, shape)` for a status, or `None` when no marker should be drawn.
///
/// `Status::Normal` (and absent status) is the unmarked baseline — it returns `None`.
fn marker_style(status: &Option<Status>, theme: &Theme) -> Option<(String, Shape)> {
    match status {
        Some(Status::Success) => Some((theme.status.success.to_hex(), Shape::Circle)),
        Some(Status::Warning) => Some((theme.status.warning.to_hex(), Shape::Triangle)),
        Some(Status::Critical) => Some((theme.status.critical.to_hex(), Shape::Diamond)),
        Some(Status::Normal) | None => None,
    }
}

/// Draws a status badge centered at `(cx, cy)`.
///
/// The badge is a filled shape ringed with the background color so it reads as a distinct marker
/// over any item fill, and stays visually independent of `emphasis` (outline + bold). Status is
/// dual-encoded by **both color and shape** (success=circle, warning=triangle, critical=diamond).
/// Items with `status = normal` or no status draw nothing.
pub(crate) fn draw_marker_at(
    builder: &mut SvgBuilder,
    status: &Option<Status>,
    cx: f32,
    cy: f32,
    theme: &Theme,
) {
    let Some((fill, shape)) = marker_style(status, theme) else { return };
    let ring = theme.background.to_hex();
    let r = STATUS_MARKER_R;
    match shape {
        Shape::Circle => builder.circle_stroked(cx, cy, r, &fill, &ring, 1.5),
        // Equilateral triangle pointing up (sin60 ≈ 0.866, cos60 = 0.5).
        Shape::Triangle => builder.polygon_stroked(
            &[(cx, cy - r), (cx - 0.866 * r, cy + 0.5 * r), (cx + 0.866 * r, cy + 0.5 * r)],
            &fill,
            &ring,
            1.5,
        ),
        // Diamond (square rotated 45°) — maximally distinct from circle/triangle at small size.
        Shape::Diamond => builder.polygon_stroked(
            &[(cx, cy - r), (cx + r, cy), (cx, cy + r), (cx - r, cy)],
            &fill,
            &ring,
            1.5,
        ),
    }
}

/// Draws a status badge inset from an item's top-right corner `(corner_x, corner_y)`.
/// Convenience over [`draw_marker_at`] for the common box-shaped item.
pub(crate) fn draw_marker(
    builder: &mut SvgBuilder,
    status: &Option<Status>,
    corner_x: f32,
    corner_y: f32,
    theme: &Theme,
) {
    draw_marker_at(builder, status, corner_x - INSET, corner_y + INSET, theme);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::DEFAULT_THEME;

    #[test]
    fn normal_and_none_draw_nothing() {
        assert!(marker_style(&None, &DEFAULT_THEME).is_none());
        assert!(marker_style(&Some(Status::Normal), &DEFAULT_THEME).is_none());
    }

    #[test]
    fn notable_states_resolve_to_theme_colors_and_shapes() {
        assert_eq!(
            marker_style(&Some(Status::Critical), &DEFAULT_THEME).unwrap(),
            (DEFAULT_THEME.status.critical.to_hex(), Shape::Diamond)
        );
        assert_eq!(marker_style(&Some(Status::Success), &DEFAULT_THEME).unwrap().1, Shape::Circle);
        assert_eq!(marker_style(&Some(Status::Warning), &DEFAULT_THEME).unwrap().1, Shape::Triangle);
    }

    #[test]
    fn success_is_a_circle() {
        let mut b = SvgBuilder::new(100.0, 100.0);
        draw_marker(&mut b, &Some(Status::Success), 90.0, 10.0, &DEFAULT_THEME);
        let svg = b.build("#ffffff");
        assert!(svg.contains("<circle"), "success should emit a circle marker");
    }

    #[test]
    fn warning_and_critical_are_polygons_not_circles() {
        // Shape dual-encoding: warning=triangle, critical=diamond are polygons, never circles.
        for s in [Status::Warning, Status::Critical] {
            let mut b = SvgBuilder::new(100.0, 100.0);
            draw_marker(&mut b, &Some(s.clone()), 90.0, 10.0, &DEFAULT_THEME);
            let svg = b.build("#ffffff");
            assert!(svg.contains("<polygon"), "{:?} should emit a polygon marker", s);
            assert!(!svg.contains("<circle"), "{:?} must not be a circle (shape coding)", s);
        }
    }

    #[test]
    fn draw_marker_emits_nothing_for_normal() {
        let mut b = SvgBuilder::new(100.0, 100.0);
        draw_marker(&mut b, &Some(Status::Normal), 90.0, 10.0, &DEFAULT_THEME);
        let svg = b.build("#ffffff");
        assert!(!svg.contains("<circle") && !svg.contains("<polygon"), "normal should emit no marker");
    }
}
