use std::sync::OnceLock;

const FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/NotoSans-Regular.ttf");

static FONT: OnceLock<fontdue::Font> = OnceLock::new();

fn font() -> &'static fontdue::Font {
    FONT.get_or_init(|| {
        fontdue::Font::from_bytes(FONT_BYTES, fontdue::FontSettings::default())
            .expect("bundled NotoSans-Regular.ttf is valid")
    })
}

/// Returns the advance width of `text` rendered at `font_size` points.
pub fn measure_text(text: &str, font_size: f32) -> f32 {
    let f = font();
    text.chars()
        .map(|c| f.metrics(c, font_size).advance_width)
        .sum()
}

/// Returns `text` truncated to fit within `max_width` at `font_size`, appending "…" if needed.
pub fn truncate_text(text: &str, font_size: f32, max_width: f32) -> String {
    if measure_text(text, font_size) <= max_width {
        return text.to_string();
    }
    const ELLIPSIS: &str = "…";
    let ellipsis_w = measure_text(ELLIPSIS, font_size);
    let available = max_width - ellipsis_w;
    if available <= 0.0 {
        return ELLIPSIS.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let mut end = chars.len();
    while end > 0 {
        end -= 1;
        let s: String = chars[..end].iter().collect();
        if measure_text(&s, font_size) <= available {
            return format!("{}{}", s, ELLIPSIS);
        }
    }
    ELLIPSIS.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wider_text_measures_wider() {
        let short = measure_text("Hi", 14.0);
        let long = measure_text("Hello, World!", 14.0);
        assert!(long > short, "longer text should measure wider");
    }

    #[test]
    fn larger_font_measures_wider() {
        let small = measure_text("Hello", 10.0);
        let large = measure_text("Hello", 20.0);
        assert!(large > small, "larger font size should measure wider");
    }

    #[test]
    fn truncate_short_text_unchanged() {
        let result = truncate_text("Hi", 14.0, 1000.0);
        assert_eq!(result, "Hi");
    }

    #[test]
    fn truncate_long_text_adds_ellipsis() {
        let long = "This is a very long label that should definitely be truncated at small widths";
        let result = truncate_text(long, 14.0, 50.0);
        assert!(result.ends_with('…'), "expected ellipsis, got: {}", result);
        assert!(result.len() < long.len(), "result should be shorter than input");
        assert!(measure_text(&result, 14.0) <= 50.0 + 1.0);
    }

    #[test]
    fn truncate_fits_within_max_width() {
        let text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let max_width = 80.0;
        let result = truncate_text(text, 14.0, max_width);
        assert!(
            measure_text(&result, 14.0) <= max_width + 1.0,
            "truncated text width {} exceeds max_width {}",
            measure_text(&result, 14.0),
            max_width
        );
    }
}
