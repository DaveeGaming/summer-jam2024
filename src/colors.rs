use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub FG_PRIMARY: Color, 
    pub FG_SECONDARY: Color,
    pub BG_PRIMARY: Color,
    pub BG_SECONDARY: Color,
}

impl ColorPalette {
    pub fn default() -> Self {
        ColorPalette {
            FG_PRIMARY: BLUE,
            FG_SECONDARY: RED,
            BG_PRIMARY: Color{ r: 0.0, g: 0.0, b: 0.1, a: 1.0},
            BG_SECONDARY: Color{ r: 0.1, g: 0.0, b: 0.0, a: 1.0},
        }
    }

    pub fn create_from(primary: Color, secondary: Color) -> Self {
        ColorPalette {
            FG_PRIMARY: primary,
            FG_SECONDARY: secondary,
            BG_PRIMARY: Color{ r: primary.r * 0.08, g: primary.g * 0.08, b: primary.b * 0.08, a: 1.0},
            BG_SECONDARY: Color{ r: secondary.r * 0.08, g: secondary.g * 0.08, b: secondary.b * 0.08, a: 1.0},
        }
    }
}