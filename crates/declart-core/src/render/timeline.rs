use crate::date::PartialDate;
use crate::model::{TimelineDiagram, TimelineWhen};
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

// Period band: bars stacked in lanes below the milestone labels.
const BAND_TOP_OFFSET: f32 = 64.0; // distance from axis to the first lane
const BAR_HEIGHT: f32 = 22.0;
const LANE_HEIGHT: f32 = 44.0; // bar + its date range + gap
const BAND_BOTTOM_PAD: f32 = 16.0;
const MIN_BAR_WIDTH: f32 = 6.0;
const BAR_TEXT_PAD: f32 = 6.0; // horizontal inset of a label inside its bar
const MIN_INSIDE_LABEL: f32 = 24.0; // narrower bars put their label to the right
const LANE_GAP: f32 = 8.0; // minimum horizontal clearance between items sharing a lane

/// A period laid out on the canvas.
struct Bar<'a> {
    x0: f32,
    x1: f32,
    label: &'a str,
    dates: String,
    lane: usize,
}

pub fn render(diagram: &TimelineDiagram, theme: &Theme) -> String {
    let title_h = if diagram.title.is_some() { TITLE_AREA } else { 20.0 };
    let axis_y = title_h + AXIS_Y_OFFSET;

    // Expand canvas width when there are many events to reduce label collision
    let n = diagram.events.len();
    let canvas_w = f32::max(CANVAS_WIDTH_MIN, n as f32 * MIN_EVENT_SPACING + 2.0 * PADDING_H);
    let usable_width = canvas_w - 2.0 * PADDING_H;

    // Events are sorted by start in parse, and every date was validated there.
    let first_day = start_day(diagram.events[0].when.start());
    let last_day = diagram
        .events
        .iter()
        .map(|e| match &e.when {
            TimelineWhen::Point(date) => start_day(date),
            TimelineWhen::Span { end, .. } => parse_date(end).end_day_exclusive(),
        })
        .max()
        .unwrap_or(first_day);
    let day_range = last_day - first_day;
    let x_for = |d: i64| -> f32 {
        if day_range == 0 {
            PADDING_H + usable_width / 2.0
        } else {
            PADDING_H + ((d - first_day) as f32 / day_range as f32) * usable_width
        }
    };

    let bars = layout_bars(diagram, theme, &x_for);
    let lanes = bars.iter().map(|b| b.lane + 1).max().unwrap_or(0);
    let band_top = axis_y + BAND_TOP_OFFSET;
    // Canvas height: title + space above axis + axis + space below axis (labels + dates),
    // extended by the period lanes when there are any.
    let canvas_h = if lanes == 0 {
        title_h + AXIS_Y_OFFSET + DATE_OFFSET + 30.0 + 40.0
    } else {
        band_top + lanes as f32 * LANE_HEIGHT + BAND_BOTTOM_PAD
    };

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

    // Draw axis line
    builder.line(
        PADDING_H / 2.0,
        axis_y,
        canvas_w - PADDING_H / 2.0,
        axis_y,
        &theme.layers.apex.to_hex(),
        AXIS_LINE_W,
    );

    // Period guides go first so milestones, labels, and bars all sit on top of them.
    draw_guides(&mut builder, &bars, theme, axis_y, band_top);

    let points: Vec<(&str, &str)> = diagram
        .events
        .iter()
        .filter_map(|e| match &e.when {
            TimelineWhen::Point(date) => Some((date.as_str(), e.label.as_str())),
            TimelineWhen::Span { .. } => None,
        })
        .collect();
    let n_points = points.len();

    for (i, (date, label)) in points.iter().enumerate() {
        let ex = x_for(start_day(date));
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

        // available width scales with milestone spacing to reduce horizontal label collision.
        // Half the average inter-milestone gap, capped to [40, 100] px.
        let avg_spacing = if n_points > 1 { usable_width / (n_points - 1) as f32 } else { usable_width };
        let available = (avg_spacing * 0.45).clamp(40.0, 100.0);
        let (display_label, fs) = fit_label(label, theme, available);
        builder.text(ex, label_y, &display_label, &theme.title_color.to_hex(), fs);

        // Date label (below axis, smaller)
        let date_y = if above {
            axis_y + TICK_HEIGHT + DATE_FONT_SIZE + 4.0
        } else {
            axis_y + TICK_HEIGHT + 4.0
        };
        builder.text(ex, date_y, date, &theme.layers.apex.to_hex(), DATE_FONT_SIZE);
    }

    draw_bars(&mut builder, &bars, theme, band_top);

    builder.build(&theme.background.to_hex())
}

fn parse_date(date: &str) -> PartialDate {
    PartialDate::parse(date).expect("timeline dates are validated by parse")
}

fn start_day(date: &str) -> i64 {
    parse_date(date).start_day()
}

/// Shrinks a label toward the theme minimum, then truncates it, to fit `available` px.
fn fit_label(label: &str, theme: &Theme, available: f32) -> (String, f32) {
    let mut fs = theme.typography.label_size;
    let tw = font::measure_text(label, fs);
    if tw > available && available > 0.0 {
        fs = (fs * available / tw).max(theme.typography.label_size_min);
    }
    let display = if font::measure_text(label, fs) > available {
        font::truncate_text(label, fs, available)
    } else {
        label.to_string()
    };
    (display, fs)
}

/// Whether a bar is wide enough to hold its label; narrower bars put it just to the right.
fn label_inside(bar_w: f32) -> bool {
    bar_w - 2.0 * BAR_TEXT_PAD >= MIN_INSIDE_LABEL
}

/// Positions every period and packs overlapping ones into lanes (first lane that fits).
fn layout_bars<'a>(diagram: &'a TimelineDiagram, theme: &Theme, x_for: &dyn Fn(i64) -> f32) -> Vec<Bar<'a>> {
    let mut lane_ends: Vec<f32> = Vec::new();
    let mut bars = Vec::new();
    for event in &diagram.events {
        let TimelineWhen::Span { start, end } = &event.when else { continue };
        let x0 = x_for(start_day(start));
        let x1 = f32::max(x_for(parse_date(end).end_day_exclusive()), x0 + MIN_BAR_WIDTH);
        let dates = format!("{start} – {end}");

        // Horizontal extent the bar claims in its lane: the bar, its centered date range,
        // and an outside label when the bar is too narrow to hold one.
        let center = (x0 + x1) / 2.0;
        let half_dates = font::measure_text(&dates, DATE_FONT_SIZE) / 2.0;
        let left = f32::min(x0, center - half_dates) - LANE_GAP / 2.0;
        let mut right = f32::max(x1, center + half_dates);
        if !label_inside(x1 - x0) {
            let outside = x1 + BAR_TEXT_PAD + font::measure_text(&event.label, theme.typography.label_size_min);
            right = f32::max(right, outside);
        }
        right += LANE_GAP / 2.0;

        let lane = match lane_ends.iter().position(|&lane_end| lane_end <= left) {
            Some(lane) => lane,
            None => {
                lane_ends.push(f32::NEG_INFINITY);
                lane_ends.len() - 1
            }
        };
        lane_ends[lane] = right;
        bars.push(Bar { x0, x1, label: &event.label, dates, lane });
    }
    bars
}

/// Faint dashed lines from each period's ends up to the axis, tying the bar to the time scale.
fn draw_guides(builder: &mut SvgBuilder, bars: &[Bar], theme: &Theme, axis_y: f32, band_top: f32) {
    let guide = theme.layers.base.to_hex();
    for bar in bars {
        let top = band_top + bar.lane as f32 * LANE_HEIGHT;
        builder.line_dashed(bar.x0, axis_y, bar.x0, top, &guide, 1.0);
        builder.line_dashed(bar.x1, axis_y, bar.x1, top, &guide, 1.0);
    }
}

fn draw_bars(builder: &mut SvgBuilder, bars: &[Bar], theme: &Theme, band_top: f32) {
    let fill = theme.layers.apex.interpolate(&theme.layers.base, 0.35);
    let on_fill = if fill.is_dark() { theme.text.on_dark } else { theme.text.on_light };
    for bar in bars {
        let top = band_top + bar.lane as f32 * LANE_HEIGHT;
        let w = bar.x1 - bar.x0;
        let mid_y = top + BAR_HEIGHT / 2.0;

        builder.rect_rounded(bar.x0, top, w, BAR_HEIGHT, 4, &fill.to_hex(), "none", 0.0);
        if label_inside(w) {
            let (label, fs) = fit_label(bar.label, theme, w - 2.0 * BAR_TEXT_PAD);
            builder.text((bar.x0 + bar.x1) / 2.0, mid_y, &label, &on_fill.to_hex(), fs);
        } else {
            let fs = theme.typography.label_size_min;
            let lw = font::measure_text(bar.label, fs);
            builder.text(bar.x1 + BAR_TEXT_PAD + lw / 2.0, mid_y, bar.label, &theme.title_color.to_hex(), fs);
        }
        builder.text(
            (bar.x0 + bar.x1) / 2.0,
            top + BAR_HEIGHT + DATE_FONT_SIZE,
            &bar.dates,
            &theme.layers.apex.to_hex(),
            DATE_FONT_SIZE,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{TimelineDiagram, TimelineEvent, TimelineWhen};
    use crate::render::DEFAULT_THEME;

    fn point(date: &str, label: &str) -> TimelineEvent {
        TimelineEvent { when: TimelineWhen::Point(date.to_string()), label: label.to_string() }
    }

    fn span(start: &str, end: &str, label: &str) -> TimelineEvent {
        TimelineEvent {
            when: TimelineWhen::Span { start: start.to_string(), end: end.to_string() },
            label: label.to_string(),
        }
    }

    fn make_diagram(title: Option<&str>) -> TimelineDiagram {
        TimelineDiagram {
            title: title.map(String::from),
            events: vec![
                point("2024-01-01", "Start"),
                point("2024-06-01", "Middle"),
                point("2024-12-31", "End"),
            ],
        }
    }

    /// Canvas height from the root `<svg … height="…">` attribute.
    fn svg_height(svg: &str) -> f32 {
        let rest = &svg[svg.find("height=\"").unwrap() + 8..];
        rest[..rest.find('"').unwrap()].parse().unwrap()
    }

    fn rect_count(svg: &str) -> usize {
        svg.matches("<rect x=").count()
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
    fn dense_timeline_truncates_long_labels() {
        // 15 events: canvas_w = max(800, 15*55+120) = 945. avg_spacing = (945-120)/14 ≈ 58.9.
        // available = (58.9 * 0.45).clamp(40, 100) ≈ 26.5 → clamped to 40.0.
        // A 40-char label must be truncated.
        let events: Vec<TimelineEvent> = (0..15)
            .map(|i| point(&format!("2024-{:02}-01", (i % 12) + 1), "A Very Long Event Label That Should Be Truncated"))
            .collect();
        let d = TimelineDiagram { title: None, events };
        let svg = render(&d, &DEFAULT_THEME);
        assert!(svg.starts_with("<svg"));
        // Full label should not appear; truncated version (with ellipsis) should
        assert!(!svg.contains("A Very Long Event Label That Should Be Truncated"),
            "long label should be truncated in dense timeline");
    }

    #[test]
    fn period_renders_as_bar_with_date_range() {
        let d = TimelineDiagram {
            title: None,
            events: vec![point("2024-01-15", "Alpha"), span("2024-02", "2024-03", "Beta")],
        };
        let svg = render(&d, &DEFAULT_THEME);
        assert_eq!(rect_count(&svg), 1, "one bar");
        assert!(svg.contains("Beta"));
        assert!(svg.contains("2024-02 – 2024-03"));
    }

    #[test]
    fn point_only_timeline_has_no_period_band() {
        let svg = render(&make_diagram(None), &DEFAULT_THEME);
        assert_eq!(rect_count(&svg), 0, "no bars");
    }

    #[test]
    fn overlapping_periods_stack_into_lanes() {
        let separate = TimelineDiagram {
            title: None,
            events: vec![span("2024-01", "2024-03", "A"), span("2024-07", "2024-09", "B")],
        };
        let overlapping = TimelineDiagram {
            title: None,
            events: vec![span("2024-01", "2024-06", "A"), span("2024-03", "2024-09", "B")],
        };
        let one_lane = svg_height(&render(&separate, &DEFAULT_THEME));
        let two_lanes = svg_height(&render(&overlapping, &DEFAULT_THEME));
        assert_eq!(two_lanes - one_lane, LANE_HEIGHT);
    }

    #[test]
    fn axis_range_extends_to_inclusive_period_end() {
        // The only milestone is at the start; the period must still end inside the canvas.
        let d = TimelineDiagram {
            title: None,
            events: vec![point("2024-01", "Kickoff"), span("2024-01", "2024-12", "Build")],
        };
        let svg = render(&d, &DEFAULT_THEME);
        let bar = &svg[svg.rfind("<rect x=").unwrap()..];
        let attr = |name: &str| -> f32 {
            let rest = &bar[bar.find(&format!("{name}=\"")).unwrap() + name.len() + 2..];
            rest[..rest.find('"').unwrap()].parse().unwrap()
        };
        let right = attr("x") + attr("width");
        assert!((right - (CANVAS_WIDTH_MIN - PADDING_H)).abs() < 0.5, "bar should end at the right edge, got {right}");
    }
}
