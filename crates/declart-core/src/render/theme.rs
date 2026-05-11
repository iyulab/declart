#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn interpolate(&self, other: &Color, t: f32) -> Color {
        Color {
            r: lerp_u8(self.r, other.r, t),
            g: lerp_u8(self.g, other.g, t),
            b: lerp_u8(self.b, other.b, t),
        }
    }

    pub fn is_dark(&self) -> bool {
        let lum = 0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32;
        lum < 140.0
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

pub struct LayerGradient {
    pub apex: Color,
    pub base: Color,
}

pub struct TextColors {
    pub on_dark: Color,
    pub on_light: Color,
}

pub struct Typography {
    pub title_size: f32,
    pub label_size: f32,
    pub label_size_min: f32,
}

pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub layers: LayerGradient,
    pub text: TextColors,
    pub typography: Typography,
    pub title_color: Color,
}

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
