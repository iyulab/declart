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

    pub(crate) fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub(crate) fn interpolate(&self, other: &Color, t: f32) -> Color {
        Color {
            r: lerp_u8(self.r, other.r, t),
            g: lerp_u8(self.g, other.g, t),
            b: lerp_u8(self.b, other.b, t),
        }
    }

    pub(crate) fn is_dark(&self) -> bool {
        let lum = 0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32;
        lum < 128.0
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

/// Visual theme for rendering. Pass `&DEFAULT_THEME` or `&MONOCHROME_THEME` to [`render`](crate::render::render),
/// or use [`Theme::by_name`] to resolve a theme by its string name, or [`Theme::from_toml`] to
/// load a custom theme from a TOML string.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub(crate) background: Color,
    pub(crate) layers: LayerGradient,
    pub(crate) text: TextColors,
    pub(crate) typography: Typography,
    pub(crate) title_color: Color,
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
            _ => &DEFAULT_THEME,
        }
    }

    /// Returns the names of all built-in themes.
    pub fn names() -> &'static [&'static str] {
        &["default", "monochrome", "accessible", "warm"]
    }

    /// Parses a TOML string into a custom `Theme`.
    ///
    /// The TOML must contain a `[colors]` section with all six color fields in `#RRGGBB` format.
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
        })
    }
}

#[derive(serde::Deserialize)]
struct ThemeConfig {
    colors: ThemeColors,
    typography: Option<ThemeTypography>,
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
};

/// Dark-to-light gray theme with no color hues.
pub static MONOCHROME_THEME: Theme = Theme {
    name: "monochrome",
    background: Color::new(255, 255, 255),
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
};

/// Colorblind-safe theme based on the Okabe-Ito palette (Blue → Sky blue).
/// Distinguishable under deuteranopia and protanopia.
/// WCAG AA contrast: #0072B2 on #FFFFFF = 5.1:1.
pub static ACCESSIBLE_THEME: Theme = Theme {
    name: "accessible",
    background: Color::new(255, 255, 255),
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
};

/// Warm terracotta-to-peach gradient. Suitable for consulting and marketing decks.
pub static WARM_THEME: Theme = Theme {
    name: "warm",
    background: Color::new(255, 250, 247),
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
    fn from_toml_renders_pyramid() {
        let theme = Theme::from_toml(VALID_TOML).unwrap();
        let diagram = crate::parse::parse(r#"kind="pyramid"
[[items]]
label="Top"
[[items]]
label="Bottom""#).unwrap();
        let svg = crate::render::render(&diagram, &theme).unwrap();
        assert!(svg.starts_with("<svg"));
    }

    #[test]
    fn is_dark_boundary() {
        // lum = 0.299*127 + 0.587*127 + 0.114*127 ≈ 127 < 128 → dark
        let dark = Color::new(127, 127, 127);
        assert!(dark.is_dark());
        // lum = 0.299*128 + 0.587*128 + 0.114*128 ≈ 128 → not dark
        let light = Color::new(128, 128, 128);
        assert!(!light.is_dark());
    }
}
