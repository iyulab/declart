use declart_core::render::Theme;
use wasm_bindgen::prelude::*;

/// Renders a TOML or JSON diagram declaration to SVG.
///
/// - `input`: TOML or JSON declaration string (auto-detected by leading `{`)
/// - `theme`: `"default"`, `"monochrome"`, `"accessible"`, or `"warm"` (unknown values fall back to `"default"`)
/// - `width`: optional canvas width in pixels (height scales proportionally)
///
/// Returns the SVG string on success, or a `JsError` with a descriptive message on failure.
#[wasm_bindgen]
pub fn render(input: &str, theme: &str, width: Option<u32>) -> Result<String, JsError> {
    let diagram = declart_core::parse_auto(input)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let t = Theme::by_name(theme);
    declart_core::render_opts(&diagram, t, width)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Renders a JSON diagram declaration to SVG with a named theme.
///
/// - `input`: JSON declaration string
/// - `theme`: `"default"`, `"monochrome"`, `"accessible"`, or `"warm"`
/// - `width`: optional canvas width in pixels
///
/// Returns the SVG string on success, or a `JsError` on failure.
#[wasm_bindgen]
pub fn render_json(input: &str, theme: &str, width: Option<u32>) -> Result<String, JsError> {
    let diagram = declart_core::parse_json(input)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let t = Theme::by_name(theme);
    declart_core::render_opts(&diagram, t, width)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Renders a TOML or JSON diagram declaration with a custom theme defined as a TOML string.
///
/// - `input`: TOML or JSON declaration string (auto-detected)
/// - `theme_toml`: TOML theme string (see [`Theme::from_toml`])
/// - `width`: optional canvas width in pixels
///
/// Returns the SVG string on success, or a `JsError` on parse/validation failure.
#[wasm_bindgen]
pub fn render_with_theme_toml(input: &str, theme_toml: &str, width: Option<u32>) -> Result<String, JsError> {
    let diagram = declart_core::parse_auto(input)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let theme = Theme::from_toml(theme_toml)
        .map_err(|e| JsError::new(&e.to_string()))?;
    declart_core::render_opts(&diagram, &theme, width)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Validates a TOML or JSON diagram declaration without rendering.
///
/// Returns `Ok(())` if valid, or a `JsError` with the parse/validation error message.
#[wasm_bindgen]
pub fn validate(input: &str) -> Result<(), JsError> {
    declart_core::parse_auto(input)
        .map(|_| ())
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Returns a comma-separated list of supported theme names.
#[wasm_bindgen]
pub fn themes() -> String {
    Theme::names().join(",")
}

/// Returns a comma-separated list of supported diagram kind names.
#[wasm_bindgen]
pub fn kinds() -> String {
    "flow,tier,hierarchy,timeline,matrix,hub_spoke,venn,comparison".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_valid_pyramid() {
        let input = "kind = \"tier\"\n[[items]]\nlabel = \"Top\"\n[[items]]\nlabel = \"Bottom\"\n";
        let svg = render(input, "default", None).unwrap();
        assert!(svg.contains("<svg"), "expected SVG output");
        assert!(svg.contains("Top"), "SVG should contain label text");
    }

    #[test]
    fn render_with_width() {
        let input = "kind = \"tier\"\n[[items]]\nlabel = \"Top\"\n[[items]]\nlabel = \"Bottom\"\n";
        let svg = render(input, "default", Some(400)).unwrap();
        assert!(svg.contains("width=\"400\""), "expected width=400 in SVG");
    }

    #[test]
    fn render_unknown_theme_falls_back() {
        let input = "kind = \"tier\"\n[[items]]\nlabel = \"Top\"\n[[items]]\nlabel = \"Bottom\"\n";
        let svg = render(input, "nonexistent", None).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn validate_valid_input() {
        let input = "kind = \"flow\"\n[[items]]\nlabel = \"Item\"\n";
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
    fn render_with_theme_toml_valid() {
        let diagram = "kind = \"tier\"\n[[items]]\nlabel = \"Top\"\n[[items]]\nlabel = \"Bottom\"\n";
        let theme = r##"
[colors]
apex       = "#003087"
base       = "#B8D0E8"
background = "#FFFFFF"
on_dark    = "#FFFFFF"
on_light   = "#003087"
title      = "#003087"
"##;
        let svg = render_with_theme_toml(diagram, theme, None).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("#003087"));
    }

    #[test]
    fn render_with_theme_toml_invalid_theme() {
        let result = declart_core::render::Theme::from_toml("not valid toml ???");
        assert!(result.is_err(), "invalid theme TOML should be rejected");
    }

    #[test]
    fn kinds_returns_all_supported() {
        let k = kinds();
        assert!(k.contains("flow"));
        assert!(k.contains("tier"));
        assert!(k.contains("hierarchy"));
        assert!(k.contains("timeline"));
        assert!(k.contains("comparison"));
    }

    #[test]
    fn render_json_valid_pyramid() {
        let input = r#"{"kind":"tier","items":[{"label":"Top"},{"label":"Bottom"}]}"#;
        let svg = render_json(input, "default", None).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Top"));
    }

    #[test]
    fn render_json_rejects_invalid() {
        let result = declart_core::parse_json(r#"{"kind":"unknown_kind"}"#);
        assert!(result.is_err());
    }

    #[test]
    fn render_auto_detects_json() {
        let json = r#"{"kind":"flow","items":[{"label":"A"},{"label":"B"}]}"#;
        let svg = render(json, "default", None).unwrap();
        assert!(svg.contains("<svg"));
    }
}
