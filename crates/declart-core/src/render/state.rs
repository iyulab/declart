use crate::model::{StateDiagram, StateRole, TransitionKind};
use crate::render::theme::Theme;
use super::{font, svg::SvgBuilder};

const STATE_RADIUS: f32 = 36.0;
const STATE_SPACING: f32 = 160.0;
const TITLE_AREA: f32 = 50.0;
const PADDING: f32 = 30.0;
const INITIAL_DOT_R: f32 = 10.0;
const INITIAL_ARROW_LEN: f32 = 30.0;

pub(crate) fn render(d: &StateDiagram, theme: &Theme) -> String {
    let n = d.states.len();
    let title_area = if d.title.is_some() { TITLE_AREA } else { 0.0 };
    // Y position of state circle centers
    let row_y = title_area + PADDING + STATE_RADIUS + 30.0;

    let canvas_width = (PADDING + n as f32 * STATE_SPACING + PADDING).max(400.0);
    let canvas_height = row_y + STATE_RADIUS + 80.0 + PADDING;

    let mut b = SvgBuilder::new(canvas_width, canvas_height);

    if let Some(title) = &d.title {
        b.text(canvas_width / 2.0, title_area / 2.0, title,
               &theme.title_color.to_hex(), theme.typography.title_size);
    }

    // State center X positions
    let state_cx: Vec<f32> = (0..n)
        .map(|i| PADDING + STATE_SPACING * 0.5 + i as f32 * STATE_SPACING)
        .collect();

    // Draw transitions first (circles render on top)
    for t in &d.transitions {
        let Some(fi) = resolve_idx(d, &t.from) else { continue };
        let Some(ti) = resolve_idx(d, &t.to) else { continue };
        let fx = state_cx[fi];
        let tx = state_cx[ti];
        let is_exception = matches!(t.kind, TransitionKind::Exception);
        let color = if is_exception {
            theme.layers.base.interpolate(&theme.background, 0.3).to_hex()
        } else {
            theme.layers.apex.to_hex()
        };

        if fi == ti {
            draw_self_loop(&mut b, fx, row_y, &color, is_exception);
            if let Some(trigger) = &t.trigger {
                b.text(fx, row_y - STATE_RADIUS - 36.0, trigger,
                       &theme.title_color.to_hex(), theme.typography.label_size - 2.0);
            }
        } else {
            let (sx, sy) = circle_edge(fx, row_y, tx - fx, 0.0, STATE_RADIUS);
            let (ex, ey) = circle_edge(tx, row_y, fx - tx, 0.0, STATE_RADIUS);
            if is_exception {
                b.line_dashed(sx, sy, ex, ey, &color, 1.5);
            } else {
                b.line(sx, sy, ex, ey, &color, 1.5);
            }
            draw_arrowhead(&mut b, ex, ey, sx, sy, &color);

            if let Some(trigger) = &t.trigger {
                let mid_x = (sx + ex) / 2.0;
                let above = ti > fi;
                let label_y = if above { row_y - STATE_RADIUS - 16.0 } else { row_y + STATE_RADIUS + 16.0 };
                b.text(mid_x, label_y, trigger,
                       &theme.title_color.to_hex(), theme.typography.label_size - 2.0);
            }
        }
    }

    // Draw state circles
    for (i, state) in d.states.iter().enumerate() {
        let cx = state_cx[i];
        let is_terminal = matches!(state.role, Some(StateRole::Terminal));
        let t = if n > 1 { i as f32 / (n - 1) as f32 } else { 0.0 };
        let fill = theme.layers.apex.interpolate(&theme.layers.base, t);
        let text_col = if fill.is_dark() { &theme.text.on_dark } else { &theme.text.on_light };

        // Terminal: outer ring
        if is_terminal {
            b.circle_stroked(cx, row_y, STATE_RADIUS + 5.0,
                             "none", &theme.layers.apex.to_hex(), 2.0);
        }
        b.circle_stroked(cx, row_y, STATE_RADIUS,
                         &fill.to_hex(), &theme.layers.apex.to_hex(), 1.5);

        // Label with auto-shrink
        let mut font_size = theme.typography.label_size;
        let avail = STATE_RADIUS * 1.6;
        let tw = font::measure_text(&state.label, font_size);
        if tw > avail {
            font_size = (font_size * avail / tw).max(theme.typography.label_size_min);
        }
        let label = if font::measure_text(&state.label, font_size) > avail {
            font::truncate_text(&state.label, font_size, avail)
        } else {
            state.label.clone()
        };
        b.text(cx, row_y, &label, &text_col.to_hex(), font_size);
    }

    // Initial state marker: filled dot → arrow → first initial state
    if let Some(init_idx) = d.states.iter().position(|s| matches!(s.role, Some(StateRole::Initial))) {
        let cx = state_cx[init_idx];
        let dot_x = (cx - STATE_RADIUS - INITIAL_ARROW_LEN - INITIAL_DOT_R).max(PADDING);
        b.circle_stroked(dot_x, row_y, INITIAL_DOT_R,
                         &theme.layers.apex.to_hex(), "none", 0.0);
        let arrow_end_x = cx - STATE_RADIUS;
        let shaft_end = arrow_end_x - 9.0;
        if dot_x + INITIAL_DOT_R < shaft_end {
            b.line(dot_x + INITIAL_DOT_R, row_y, shaft_end, row_y,
                   &theme.layers.apex.to_hex(), 1.5);
        }
        draw_arrowhead(&mut b, arrow_end_x, row_y, dot_x, row_y, &theme.layers.apex.to_hex());
    }

    b.build(&theme.background.to_hex())
}

fn resolve_idx(d: &StateDiagram, key: &str) -> Option<usize> {
    d.states.iter().position(|s| s.id.as_deref() == Some(key))
        .or_else(|| d.states.iter().position(|s| s.label == key))
}

fn circle_edge(cx: f32, cy: f32, dx: f32, dy: f32, r: f32) -> (f32, f32) {
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    (cx + dx / len * r, cy + dy / len * r)
}

fn draw_arrowhead(b: &mut SvgBuilder, tx: f32, ty: f32, sx: f32, sy: f32, color: &str) {
    let dx = tx - sx;
    let dy = ty - sy;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let head = 9.0_f32;
    let ux = dx / len;
    let uy = dy / len;
    let bx = tx - ux * head;
    let by = ty - uy * head;
    let px = -uy * head * 0.4;
    let py = ux * head * 0.4;
    b.polygon(&[(bx + px, by + py), (tx, ty), (bx - px, by - py)], color, "none");
}

fn draw_self_loop(b: &mut SvgBuilder, cx: f32, cy: f32, color: &str, dashed: bool) {
    let top_y = cy - STATE_RADIUS - 22.0;
    let tip_x = cx + 15.0;
    let tip_y = cy - STATE_RADIUS + 5.0;
    if dashed {
        b.line_dashed(cx, cy - STATE_RADIUS, cx, top_y, color, 1.5);
        b.line_dashed(cx, top_y, tip_x, tip_y, color, 1.5);
    } else {
        b.line(cx, cy - STATE_RADIUS, cx, top_y, color, 1.5);
        b.line(cx, top_y, tip_x, tip_y, color, 1.5);
    }
    draw_arrowhead(b, tip_x, tip_y, cx, top_y, color);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{StateDiagram, StateNode, StateRole, StateTransition, TransitionKind};
    use crate::render::DEFAULT_THEME;

    fn two_state() -> StateDiagram {
        StateDiagram {
            title: Some("Order".to_string()),
            states: vec![
                StateNode { label: "Idle".to_string(), id: Some("idle".to_string()), role: Some(StateRole::Initial) },
                StateNode { label: "Done".to_string(), id: None, role: Some(StateRole::Terminal) },
            ],
            transitions: vec![
                StateTransition { from: "idle".to_string(), to: "Done".to_string(), trigger: Some("finish".to_string()), kind: TransitionKind::Normal },
            ],
        }
    }

    #[test]
    fn render_produces_svg() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_state_labels() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.contains("Idle"));
        assert!(svg.contains("Done"));
    }

    #[test]
    fn render_includes_trigger() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.contains("finish"));
    }

    #[test]
    fn render_includes_title() {
        let svg = render(&two_state(), &DEFAULT_THEME);
        assert!(svg.contains("Order"));
    }
}
