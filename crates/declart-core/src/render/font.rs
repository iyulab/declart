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
///
/// The bundled font covers Latin, Greek and Cyrillic only. A character it has no glyph
/// for is drawn by the viewer with a fallback font, so its `.notdef` advance says nothing
/// about the drawn width. East Asian wide and fullwidth characters (Hangul, CJK ideographs,
/// kana, fullwidth forms) are measured as one em — the advance every CJK font gives them.
pub fn measure_text(text: &str, font_size: f32) -> f32 {
    let f = font();
    text.chars()
        .map(|c| {
            if f.lookup_glyph_index(c) == 0 && is_wide(c) {
                font_size
            } else {
                f.metrics(c, font_size).advance_width
            }
        })
        .sum()
}

/// East Asian Width `W` / `F` blocks (UAX #11) — the characters fonts draw one em wide.
fn is_wide(c: char) -> bool {
    matches!(c as u32,
        0x1100..=0x115F       // Hangul Jamo initial consonants
        | 0x2E80..=0x303E     // CJK radicals, Kangxi, ideographic description, CJK symbols
        | 0x3041..=0x33FF     // Hiragana, Katakana, Bopomofo, compatibility Jamo, CJK compatibility
        | 0x3400..=0x4DBF     // CJK Extension A
        | 0x4E00..=0x9FFF     // CJK Unified Ideographs
        | 0xA000..=0xA4CF     // Yi
        | 0xA960..=0xA97F     // Hangul Jamo Extended-A
        | 0xAC00..=0xD7A3     // Hangul syllables
        | 0xF900..=0xFAFF     // CJK compatibility ideographs
        | 0xFE30..=0xFE4F     // CJK compatibility forms
        | 0xFF00..=0xFF60     // Fullwidth forms
        | 0xFFE0..=0xFFE6     // Fullwidth signs
        | 0x20000..=0x3FFFD   // CJK Extensions B and beyond
    )
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
    fn hangul_and_cjk_measure_one_em_each() {
        // The bundled font has no glyph for these; the viewer's fallback font draws them one em wide.
        assert_eq!(measure_text("가", 10.0), 10.0);
        assert_eq!(measure_text("漢字", 10.0), 20.0);
        assert_eq!(measure_text("カナ", 12.0), 24.0);
        assert_eq!(measure_text("Ａ", 10.0), 10.0);
    }

    #[test]
    fn mixed_label_adds_latin_advances_to_wide_ems() {
        let latin = measure_text("AI ", 10.0);
        assert!((measure_text("AI 모델", 10.0) - (latin + 20.0)).abs() < 1e-4);
    }

    #[test]
    fn long_hangul_label_truncates_to_its_drawn_width() {
        let label = "사용자가 업로드한 문서를 정제하고 청크로 나누는 전처리 단계";
        let result = truncate_text(label, 13.2, 258.0);
        assert!(result.ends_with('…'), "expected ellipsis, got: {}", result);
        // Every kept syllable costs one em; the drawn width must stay within the box.
        let drawn: f32 = result
            .chars()
            .map(|c| if c == ' ' || c == '…' { measure_text(&c.to_string(), 13.2) } else { 13.2 })
            .sum();
        assert!(drawn <= 258.0 + 0.5, "drawn width {} exceeds 258", drawn);
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
