use crate::model::TimelineDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_WIDTH_MIN: f32 = 800.0;
const MIN_EVENT_SPACING: f32 = 55.0; // minimum average pixels per event
const TITLE_AREA: f32 = 50.0;
const PADDING_H: f32 = 60.0; // horizontal padding for first/last event
const AXIS_Y_OFFSET: f32 = 140.0; // distance from top (below title) to axis
const TICK_HEIGHT: f32 = 10.0;
const LABEL_OFFSET: f32 = 16.0; // distance from tick to label baseline
const DATE_OFFSET: f32 = 16.0; // distance from tick to date label (below axis)
const DOT_RADIUS: f32 = 5.0;
const AXIS_LINE_W: f32 = 2.0;
const DATE_FONT_SIZE: f32 = 10.0;

pub fn render(diagram: &TimelineDiagram, theme: &Theme) -> String {
    let title_h = if diagram.title.is_some() { TITLE_AREA } else { 20.0 };
    // Canvas height: title + space above axis + axis + space below axis (labels + dates)
    let canvas_h = title_h + AXIS_Y_OFFSET + DATE_OFFSET + 30.0 + 40.0;
    let axis_y = title_h + AXIS_Y_OFFSET;

    // Expand canvas width when there are many events to reduce label collision
    let n = diagram.events.len();
    let canvas_w = f32::max(CANVAS_WIDTH_MIN, n as f32 * MIN_EVENT_SPACING + 2.0 * PADDING_H);

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

    // Events are already sorted by parse
    // Map dates to x positions using string comparison (ISO dates sort lexicographically)
    let first_date = &diagram.events[0].date;
    let last_date = &diagram.events[n - 1].date;
    let date_range = date_to_days(last_date) - date_to_days(first_date);
    let usable_width = canvas_w - 2.0 * PADDING_H;

    let x_for = |date: &str| -> f32 {
        if date_range == 0 {
            PADDING_H + usable_width / 2.0
        } else {
            let delta = date_to_days(date) - date_to_days(first_date);
            PADDING_H + (delta as f32 / date_range as f32) * usable_width
        }
    };

    // Draw axis line
    builder.line(
        PADDING_H / 2.0,
        axis_y,
        canvas_w - PADDING_H / 2.0,
        axis_y,
        &theme.layers.apex.to_hex(),
        AXIS_LINE_W,
    );

    for (i, event) in diagram.events.iter().enumerate() {
        let ex = x_for(&event.date);
        let above = i % 2 == 0; // alternate above/below

        // Tick mark
        builder.line(ex, axis_y - TICK_HEIGHT, ex, axis_y + TICK_HEIGHT, &theme.layers.apex.to_hex(), 1.5);

        // Dot
        builder.circle(ex, axis_y, DOT_RADIUS, &theme.layers.apex.to_hex(), 1.0);

        // Event label (above or below axis)
        let label_y = if above {
            axis_y - TICK_HEIGHT - LABEL_OFFSET
        } else {
            axis_y + TICK_HEIGHT + LABEL_OFFSET + DATE_FONT_SIZE
        };

        let available = 100.0_f32;
        let mut fs = theme.typography.label_size;
        let tw = font::measure_text(&event.label, fs);
        if tw > available && available > 0.0 {
            fs = (fs * available / tw).max(theme.typography.label_size_min);
        }
        let display_label = if font::measure_text(&event.label, fs) > available {
            font::truncate_text(&event.label, fs, available)
        } else {
            event.label.clone()
        };
        builder.text(ex, label_y, &display_label, &theme.title_color.to_hex(), fs);

        // Date label (below axis, smaller)
        let date_y = if above {
            axis_y + TICK_HEIGHT + DATE_FONT_SIZE + 4.0
        } else {
            axis_y + TICK_HEIGHT + 4.0
        };
        builder.text(ex, date_y, &event.date, &theme.layers.apex.to_hex(), DATE_FONT_SIZE);
    }

    builder.build(&theme.background.to_hex())
}

/// Convert YYYY-MM-DD to a Julian Day Number for relative positioning.
fn date_to_days(date: &str) -> i32 {
    let bytes = date.as_bytes();
    if bytes.len() != 10 {
        return 0;
    }
    let parse_digits = |b: &[u8]| -> i32 {
        b.iter().fold(0i32, |acc, &d| acc * 10 + (d - b'0') as i32)
    };
    let y = parse_digits(&bytes[0..4]) as i64;
    let m = parse_digits(&bytes[5..7]) as i64;
    let d = parse_digits(&bytes[8..10]) as i64;
    // Julian Day Number (proleptic Gregorian calendar)
    let a = (14 - m) / 12;
    let yr = y + 4800 - a;
    let mo = m + 12 * a - 3;
    (d + (153 * mo + 2) / 5 + 365 * yr + yr / 4 - yr / 100 + yr / 400 - 32045) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{TimelineDiagram, TimelineEvent};
    use crate::render::DEFAULT_THEME;

    fn make_diagram(title: Option<&str>) -> TimelineDiagram {
        TimelineDiagram {
            title: title.map(String::from),
            events: vec![
                TimelineEvent { date: "2024-01-01".to_string(), label: "Start".to_string() },
                TimelineEvent { date: "2024-06-01".to_string(), label: "Middle".to_string() },
                TimelineEvent { date: "2024-12-31".to_string(), label: "End".to_string() },
            ],
        }
    }

    #[test]
    fn render_produces_svg_element() {
        let d = make_diagram(Some("Test"));
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_includes_all_labels() {
        let d = make_diagram(None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("Start"));
        assert!(svg.contains("Middle"));
        assert!(svg.contains("End"));
    }

    #[test]
    fn render_includes_dates() {
        let d = make_diagram(None);
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.contains("2024-01-01"));
        assert!(svg.contains("2024-12-31"));
    }

    #[test]
    fn date_to_days_orders_correctly() {
        assert!(date_to_days("2024-01-01") < date_to_days("2024-06-01"));
        assert!(date_to_days("2024-06-01") < date_to_days("2024-12-31"));
        assert!(date_to_days("2023-12-31") < date_to_days("2024-01-01"));
    }
}
