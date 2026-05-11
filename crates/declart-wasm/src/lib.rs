use declart_core::render::Theme;
use wasm_bindgen::prelude::*;

/// Renders a TOML diagram declaration to SVG.
///
/// - `input`: TOML declaration string
/// - `theme`: `"default"` or `"monochrome"` (unknown values fall back to `"default"`)
/// - `width`: optional canvas width in pixels (height scales proportionally)
///
/// Returns the SVG string on success, or a `JsError` with a descriptive message on failure.
#[wasm_bindgen]
pub fn render(input: &str, theme: &str, width: Option<u32>) -> Result<String, JsError> {
    let diagram = declart_core::parse(input)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let t = Theme::by_name(theme);
    declart_core::render_opts(&diagram, t, width)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Validates a TOML diagram declaration without rendering.
///
/// Returns `Ok(())` if valid, or a `JsError` with the parse/validation error message.
#[wasm_bindgen]
pub fn validate(input: &str) -> Result<(), JsError> {
    declart_core::parse(input)
        .map(|_| ())
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Returns a comma-separated list of supported theme names.
#[wasm_bindgen]
pub fn themes() -> String {
    "default,monochrome".to_string()
}

/// Returns a comma-separated list of supported diagram kind names.
#[wasm_bindgen]
pub fn kinds() -> String {
    "pyramid,process,cycle,matrix,hub_spoke,venn,timeline,fishbone".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_valid_pyramid() {
        let input = r#"kind = "pyramid"
[[items]]
label = "Top"
[[items]]
label = "Bottom"
"#;
        let svg = render(input, "default", None).unwrap();
        assert!(svg.contains("<svg"), "expected SVG output");
        assert!(svg.contains("Top"), "SVG should contain label text");
    }

    #[test]
    fn render_with_width() {
        let input = r#"kind = "pyramid"
[[items]]
label = "Top"
[[items]]
label = "Bottom"
"#;
        let svg = render(input, "default", Some(400)).unwrap();
        assert!(svg.contains("width=\"400\""), "expected width=400 in SVG");
    }

    #[test]
    fn render_unknown_theme_falls_back() {
        let input = r#"kind = "pyramid"
[[items]]
label = "Top"
[[items]]
label = "Bottom"
"#;
        let svg = render(input, "nonexistent", None).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn validate_valid_input() {
        let input = r#"kind = "pyramid"
[[items]]
label = "Item"
"#;
        assert!(validate(input).is_ok());
    }

    #[test]
    fn validate_rejects_unknown_kind() {
        // Test core parse logic; JsError wrapping is tested only on wasm32
        let input = r#"kind = "unknown_kind"
"#;
        assert!(declart_core::parse(input).is_err());
    }

    #[test]
    fn themes_returns_nonempty() {
        assert!(!themes().is_empty());
    }

    #[test]
    fn kinds_returns_all_eight() {
        let k = kinds();
        assert!(k.contains("pyramid"));
        assert!(k.contains("fishbone"));
    }
}
