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
        lum < 140.0
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

pub(crate) struct LayerGradient {
    pub(crate) apex: Color,
    pub(crate) base: Color,
}

pub(crate) struct TextColors {
    pub(crate) on_dark: Color,
    pub(crate) on_light: Color,
}

pub(crate) struct Typography {
    pub(crate) title_size: f32,
    pub(crate) label_size: f32,
    pub(crate) label_size_min: f32,
}

/// Visual theme for rendering. Pass `&DEFAULT_THEME` or `&MONOCHROME_THEME` to [`render`](crate::render::render),
/// or use [`Theme::by_name`] to resolve a theme by its string name.
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
    pub fn by_name(name: &str) -> &'static Theme {
        match name {
            "monochrome" => &MONOCHROME_THEME,
            _ => &DEFAULT_THEME,
        }
    }
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
        label_size_min: 8.0,
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
        label_size_min: 8.0,
    },
    title_color: Color::new(40, 40, 40),
};
