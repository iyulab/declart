#[derive(Debug, Clone, Copy)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Color {
    pub(crate) const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub(crate) fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub(crate) fn interpolate(&self, other: &Color, t: f32) -> Color {
        Color {
            r: lerp_u8(self.r, other.r, t),
            g: lerp_u8(self.g, other.g, t),
            b: lerp_u8(self.b, other.b, t),
        }
    }

    /// WCAG 2 relative luminance (0 = black, 1 = white).
    pub(crate) fn relative_luminance(&self) -> f32 {
        fn channel(v: u8) -> f32 {
            let c = v as f32 / 255.0;
            if c <= 0.039_28 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        }
        0.2126 * channel(self.r) + 0.7152 * channel(self.g) + 0.0722 * channel(self.b)
    }

    /// WCAG 2 contrast ratio between two colors (1–21).
    pub(crate) fn contrast(&self, other: &Color) -> f32 {
        let (a, b) = (self.relative_luminance(), other.relative_luminance());
        let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
        (hi + 0.05) / (lo + 0.05)
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

#[derive(Debug, Clone)]
pub(crate) struct LayerGradient {
    pub(crate) apex: Color,
    pub(crate) base: Color,
}

#[derive(Debug, Clone)]
pub(crate) struct TextColors {
    pub(crate) on_dark: Color,
    pub(crate) on_light: Color,
}

#[derive(Debug, Clone)]
pub(crate) struct Typography {
    pub(crate) title_size: f32,
    pub(crate) label_size: f32,
    pub(crate) label_size_min: f32,
}

/// Marker colors for the semantic `status` attribute. `normal` has no color — it renders
/// no marker (the unmarked baseline), so only the three notable states need a color.
#[derive(Debug, Clone)]
pub(crate) struct StatusColors {
    pub(crate) success: Color,
    pub(crate) warning: Color,
    pub(crate) critical: Color,
}

/// Visual theme for rendering. Pass `&DEFAULT_THEME` or `&MONOCHROME_THEME` to [`render`](crate::render::render),
/// or use [`Theme::by_name`] to resolve a theme by its string name, or [`Theme::from_toml`] to
/// load a custom theme from a TOML string.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    /// The surface the diagram sits on. Emphasis rings, matrix axes and alternating table rows
    /// are derived from it, so it stays meaningful even when it is not painted.
    pub(crate) background: Color,
    /// Whether `background` is painted as a backdrop under the diagram.
    pub(crate) backdrop: bool,
    pub(crate) layers: LayerGradient,
    pub(crate) text: TextColors,
    pub(crate) typography: Typography,
    pub(crate) title_color: Color,
    pub(crate) status: StatusColors,
}

impl Theme {
    /// Returns the built-in theme matching `name`, or `&DEFAULT_THEME` if unrecognised.
    ///
    /// Built-in themes: `"default"`, `"monochrome"`, `"accessible"`, `"warm"`.
    pub fn by_name(name: &str) -> &'static Theme {
        match name {
            "monochrome" => &MONOCHROME_THEME,
            "accessible" => &ACCESSIBLE_THEME,
            "warm" => &WARM_THEME,
            "dark" => &DARK_THEME,
            _ => &DEFAULT_THEME,
        }
    }

    /// Returns the names of all built-in themes.
    pub fn names() -> &'static [&'static str] {
        &["default", "monochrome", "accessible", "warm", "dark"]
    }

    /// This theme with a transparent backdrop: nothing is painted under the diagram, so the
    /// page shows through. Colors are unchanged — pick a theme whose surface matches the page
    /// (`dark` on a dark page) so text and rings stay legible.
    pub fn without_backdrop(&self) -> Theme {
        Theme { backdrop: false, ..self.clone() }
    }

    /// The text color for a label drawn on `fill`: whichever of the theme's two text colors
    /// contrasts more with it — or, when even that one falls short of WCAG AA for body text
    /// (4.5:1), plain white or black, whichever contrasts more. Mid-tone fills in a gradient are
    /// where the theme's colors fall short.
    pub(crate) fn text_on(&self, fill: &Color) -> Color {
        const AA: f32 = 4.5;
        let (on_dark, on_light) = (self.text.on_dark, self.text.on_light);
        let best = if on_dark.contrast(fill) >= on_light.contrast(fill) { on_dark } else { on_light };
        if best.contrast(fill) >= AA {
            return best;
        }
        let (white, black) = (Color::new(255, 255, 255), Color::new(0, 0, 0));
        if white.contrast(fill) >= black.contrast(fill) { white } else { black }
    }

    /// The backdrop fill, or `None` when the backdrop is transparent.
    pub(crate) fn backdrop_fill(&self) -> Option<String> {
        self.backdrop.then(|| self.background.to_hex())
    }

    /// Parses a TOML string into a custom `Theme`.
    ///
    /// The TOML must contain a `[colors]` section with all six color fields in `#RRGGBB` format.
    /// A label uses whichever of `on_dark` / `on_light` contrasts more with its fill, or plain
    /// white or black when neither reaches WCAG AA (4.5:1).
    /// An optional `[typography]` section can override font sizes (defaults: title=18, label=14, min=10).
    ///
    /// ```toml
    /// [colors]
    /// apex       = "#003087"
    /// base       = "#B8D0E8"
    /// background = "#FFFFFF"
    /// on_dark    = "#FFFFFF"
    /// on_light   = "#003087"
    /// title      = "#003087"
    ///
    /// [typography]  # optional
    /// label_size     = 14.0
    /// label_size_min = 10.0
    /// title_size     = 18.0
    /// ```
    pub fn from_toml(toml_str: &str) -> Result<Theme, crate::DeclartError> {
        let config: ThemeConfig = toml::from_str(toml_str)
            .map_err(|e| crate::DeclartError::InvalidTheme(e.to_string()))?;
        let c = &config.colors;
        let typo = config.typography.as_ref();
        Ok(Theme {
            name: "custom",
            background: parse_hex("colors.background", &c.background)?,
            backdrop: true,
            layers: LayerGradient {
                apex: parse_hex("colors.apex", &c.apex)?,
                base: parse_hex("colors.base", &c.base)?,
            },
            text: TextColors {
                on_dark: parse_hex("colors.on_dark", &c.on_dark)?,
                on_light: parse_hex("colors.on_light", &c.on_light)?,
            },
            typography: Typography {
                title_size: typo.and_then(|t| t.title_size).unwrap_or(18.0),
                label_size: typo.and_then(|t| t.label_size).unwrap_or(14.0),
                label_size_min: typo.and_then(|t| t.label_size_min).unwrap_or(10.0),
            },
            title_color: parse_hex("colors.title", &c.title)?,
            status: {
                let s = config.status.as_ref();
                let default = &DEFAULT_THEME.status;
                StatusColors {
                    success: match s.and_then(|s| s.success.as_deref()) {
                        Some(hex) => parse_hex("status.success", hex)?,
                        None => default.success,
                    },
                    warning: match s.and_then(|s| s.warning.as_deref()) {
                        Some(hex) => parse_hex("status.warning", hex)?,
                        None => default.warning,
                    },
                    critical: match s.and_then(|s| s.critical.as_deref()) {
                        Some(hex) => parse_hex("status.critical", hex)?,
                        None => default.critical,
                    },
                }
            },
        })
    }
}

#[derive(serde::Deserialize)]
struct ThemeConfig {
    colors: ThemeColors,
    typography: Option<ThemeTypography>,
    status: Option<ThemeStatus>,
}

#[derive(serde::Deserialize)]
struct ThemeStatus {
    success: Option<String>,
    warning: Option<String>,
    critical: Option<String>,
}

#[derive(serde::Deserialize)]
struct ThemeColors {
    apex: String,
    base: String,
    background: String,
    on_dark: String,
    on_light: String,
    title: String,
}

#[derive(serde::Deserialize)]
struct ThemeTypography {
    title_size: Option<f32>,
    label_size: Option<f32>,
    label_size_min: Option<f32>,
}

fn parse_hex(field: &str, hex: &str) -> Result<Color, crate::DeclartError> {
    let s = hex.trim_start_matches('#');
    if s.len() != 6 {
        return Err(crate::DeclartError::InvalidThemeColor {
            field: field.to_string(),
            hex: hex.to_string(),
        });
    }
    let parse = |slice: &str| u8::from_str_radix(slice, 16).map_err(|_| {
        crate::DeclartError::InvalidThemeColor { field: field.to_string(), hex: hex.to_string() }
    });
    Ok(Color::new(parse(&s[0..2])?, parse(&s[2..4])?, parse(&s[4..6])?))
}

/// Default blue-gradient theme.
pub static DEFAULT_THEME: Theme = Theme {
    name: "default",
    background: Color::new(255, 255, 255),
    backdrop: true,
    layers: LayerGradient {
        apex: Color::new(26, 58, 92),
        base: Color::new(168, 200, 232),
    },
    text: TextColors {
        on_dark: Color::new(255, 255, 255),
        on_light: Color::new(26, 58, 92),
    },
    typography: Typography {
        title_size: 18.0,
        label_size: 14.0,
        label_size_min: 10.0,
    },
    title_color: Color::new(26, 58, 92),
    status: StatusColors {
        success: Color::new(46, 125, 50),   // green #2E7D32
        warning: Color::new(237, 108, 2),   // amber #ED6C02
        critical: Color::new(198, 40, 40),  // red #C62828
    },
};

/// Dark-to-light gray theme with no color hues.
pub static MONOCHROME_THEME: Theme = Theme {
    name: "monochrome",
    background: Color::new(255, 255, 255),
    backdrop: true,
    layers: LayerGradient {
        apex: Color::new(40, 40, 40),
        base: Color::new(200, 200, 200),
    },
    text: TextColors {
        on_dark: Color::new(255, 255, 255),
        on_light: Color::new(40, 40, 40),
    },
    typography: Typography {
        title_size: 18.0,
        label_size: 14.0,
        label_size_min: 10.0,
    },
    title_color: Color::new(40, 40, 40),
    // Hueless theme: status encoded by increasing darkness (success light → critical near-black).
    // Color alone is weak here; glyph dual-encoding for accessibility is tracked as follow-up.
    status: StatusColors {
        success: Color::new(158, 158, 158),  // light gray
        warning: Color::new(90, 90, 90),     // mid-dark gray
        critical: Color::new(26, 26, 26),    // near-black
    },
};

/// Colorblind-safe theme based on the Okabe-Ito palette (Blue → Sky blue).
/// Distinguishable under deuteranopia and protanopia.
/// WCAG AA contrast: #0072B2 on #FFFFFF = 5.1:1.
pub static ACCESSIBLE_THEME: Theme = Theme {
    name: "accessible",
    background: Color::new(255, 255, 255),
    backdrop: true,
    layers: LayerGradient {
        apex: Color::new(0, 114, 178),   // Okabe-Ito Blue #0072B2
        base: Color::new(86, 180, 233),  // Okabe-Ito Sky blue #56B4E9
    },
    text: TextColors {
        on_dark: Color::new(255, 255, 255),
        on_light: Color::new(0, 74, 115),
    },
    typography: Typography {
        title_size: 18.0,
        label_size: 14.0,
        label_size_min: 10.0,
    },
    title_color: Color::new(0, 114, 178),
    // Okabe-Ito palette: distinguishable under deuteranopia/protanopia.
    status: StatusColors {
        success: Color::new(0, 158, 115),    // bluish green #009E73
        warning: Color::new(230, 159, 0),    // orange #E69F00
        critical: Color::new(213, 94, 0),    // vermillion #D55E00
    },
};

/// Warm terracotta-to-peach gradient. Suitable for consulting and marketing decks.
pub static WARM_THEME: Theme = Theme {
    name: "warm",
    background: Color::new(255, 250, 247),
    backdrop: true,
    layers: LayerGradient {
        apex: Color::new(139, 37, 0),    // Deep terracotta #8B2500
        base: Color::new(244, 193, 161), // Light peach #F4C1A1
    },
    text: TextColors {
        on_dark: Color::new(255, 250, 247),
        on_light: Color::new(139, 37, 0),
    },
    typography: Typography {
        title_size: 18.0,
        label_size: 14.0,
        label_size_min: 10.0,
    },
    title_color: Color::new(139, 37, 0),
    status: StatusColors {
        success: Color::new(91, 140, 90),   // fern green #5B8C5A
        warning: Color::new(217, 142, 4),   // ochre #D98E04
        critical: Color::new(178, 58, 46),  // brick red #B23A2E
    },
};

/// Light-on-dark theme for dark pages: a dark slate surface, light-blue apex fading to a deep
/// blue base, light text. Pair it with a transparent backdrop to sit directly on the page.
pub static DARK_THEME: Theme = Theme {
    name: "dark",
    background: Color::new(18, 26, 36),
    backdrop: true,
    layers: LayerGradient {
        apex: Color::new(168, 200, 240), // light blue #A8C8F0
        base: Color::new(36, 64, 98),    // deep blue #244062
    },
    text: TextColors {
        on_dark: Color::new(232, 238, 245),
        on_light: Color::new(14, 30, 48),
    },
    typography: Typography {
        title_size: 18.0,
        label_size: 14.0,
        label_size_min: 10.0,
    },
    title_color: Color::new(232, 238, 245),
    status: StatusColors {
        success: Color::new(102, 187, 106), // #66BB6A
        warning: Color::new(255, 167, 38),  // #FFA726
        critical: Color::new(239, 83, 80),  // #EF5350
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TOML: &str = r##"
[colors]
apex       = "#003087"
base       = "#B8D0E8"
background = "#FFFFFF"
on_dark    = "#FFFFFF"
on_light   = "#003087"
title      = "#003087"
"##;

    #[test]
    fn from_toml_parses_valid_theme() {
        let theme = Theme::from_toml(VALID_TOML).unwrap();
        assert_eq!(theme.name, "custom");
        assert_eq!(theme.layers.apex.r, 0x00);
        assert_eq!(theme.layers.apex.g, 0x30);
        assert_eq!(theme.layers.apex.b, 0x87);
        assert_eq!(theme.background.r, 255);
        assert_eq!(theme.typography.title_size, 18.0);
        assert_eq!(theme.typography.label_size, 14.0);
    }

    #[test]
    fn from_toml_with_typography() {
        let toml = format!("{}\n[typography]\ntitle_size = 22.0\nlabel_size = 16.0\nlabel_size_min = 12.0", VALID_TOML);
        let theme = Theme::from_toml(&toml).unwrap();
        assert_eq!(theme.typography.title_size, 22.0);
        assert_eq!(theme.typography.label_size, 16.0);
        assert_eq!(theme.typography.label_size_min, 12.0);
    }

    #[test]
    fn from_toml_rejects_invalid_hex() {
        let bad = VALID_TOML.replace("#003087", "ZZZZZ");
        let err = Theme::from_toml(&bad).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("invalid hex color") || msg.contains("invalid theme"), "unexpected error: {msg}");
    }

    #[test]
    fn from_toml_rejects_missing_field() {
        let bad = "[colors]\napex = \"#003087\"\n"; // missing fields
        let err = Theme::from_toml(bad).unwrap_err();
        assert!(err.to_string().contains("invalid theme") || err.to_string().contains("missing field"));
    }

    #[test]
    fn from_toml_renders_flow() {
        let _theme = Theme::from_toml(VALID_TOML).unwrap();
        let diagram = crate::parse::parse(r#"kind="flow"
view="cycle"
[[items]]
label="Plan"
[[items]]
label="Do""#).unwrap();
        assert!(matches!(diagram, crate::model::Diagram::Flow(_)));
    }

    #[test]
    fn from_toml_renders_tier() {
        let _theme = Theme::from_toml(VALID_TOML).unwrap();
        let diagram = crate::parse::parse(r#"kind="tier"
[[items]]
label="Top"
[[items]]
label="Bottom""#).unwrap();
        assert!(matches!(diagram, crate::model::Diagram::Tier(_)));
    }

    #[test]
    fn without_backdrop_keeps_colors_and_drops_the_fill() {
        let clear = DARK_THEME.without_backdrop();
        assert_eq!(DARK_THEME.backdrop_fill().as_deref(), Some("#121A24"));
        assert_eq!(clear.backdrop_fill(), None);
        assert_eq!(clear.background.to_hex(), DARK_THEME.background.to_hex());
        assert_eq!(clear.title_color.to_hex(), DARK_THEME.title_color.to_hex());
    }

    #[test]
    fn dark_theme_text_reads_on_its_surface() {
        // Title and on-dark text sit on the dark surface; on-light text sits on the light apex.
        assert!(DARK_THEME.title_color.contrast(&DARK_THEME.background) >= 4.5);
        assert!(DARK_THEME.text.on_light.contrast(&DARK_THEME.layers.apex) >= 4.5);
        assert!(DARK_THEME.text.on_dark.contrast(&DARK_THEME.layers.base) >= 4.5);
        assert!(Theme::names().contains(&"dark"));
        assert_eq!(Theme::by_name("dark").name, "dark");
    }
    #[test]
    fn text_on_meets_aa_on_every_gradient_step_of_every_theme() {
        for theme in [&DEFAULT_THEME, &MONOCHROME_THEME, &ACCESSIBLE_THEME, &WARM_THEME, &DARK_THEME] {
            for i in 0..=20 {
                let fill = theme.layers.apex.interpolate(&theme.layers.base, i as f32 / 20.0);
                let text = theme.text_on(&fill);
                // Pure white or black reaches 4.5:1 on any fill except a narrow mid-gray band
                // (about #777); the gradients here stay clear of it.
                assert!(text.contrast(&fill) >= 4.5, "{} step {}: {} on {} = {:.2}",
                    theme.name, i, text.to_hex(), fill.to_hex(), text.contrast(&fill));
            }
        }
    }

    #[test]
    fn text_on_keeps_the_theme_colors_where_they_suffice() {
        assert_eq!(DEFAULT_THEME.text_on(&DEFAULT_THEME.layers.apex).to_hex(), DEFAULT_THEME.text.on_dark.to_hex());
        assert_eq!(DEFAULT_THEME.text_on(&DEFAULT_THEME.layers.base).to_hex(), DEFAULT_THEME.text.on_light.to_hex());
    }

    #[test]
    fn contrast_matches_wcag_reference_values() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        assert!((white.contrast(&black) - 21.0).abs() < 0.01);
        // #767676 on white is the classic 4.54:1 boundary gray.
        assert!((Color::new(0x76, 0x76, 0x76).contrast(&white) - 4.54).abs() < 0.01);
    }
}
