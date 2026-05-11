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
}
