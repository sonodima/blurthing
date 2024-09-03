mod application;
mod button;
mod container;
mod scrollable;
mod slider;
mod text;
mod text_input;

use iced::{color, Color};

pub use button::Button;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub foreground: Color,

    pub base_100: Color,
    pub base_200: Color,
    pub base_300: Color,
    pub base_400: Color,
    pub base_500: Color,
    pub base_content: Color,

    pub primary_100: Color,
    pub primary_200: Color,
    pub primary_300: Color,
    pub primary_500: Color,
    pub primary_content: Color,
}

impl Palette {
    pub const DARK: Self = Self {
        background: color!(0x060606),
        foreground: color!(0xd6d6d6),

        base_100: color!(0x121212),
        base_200: color!(0x1e1e1e),
        base_300: color!(0x2b2b2b),
        base_400: color!(0x383838),
        base_500: color!(0x454545),
        base_content: color!(0xd6d6d6),

        primary_100: color!(0x297aa3),
        primary_200: color!(0x2e8ab8),
        primary_300: color!(0x3399cc),
        primary_500: color!(0x5cadd6),
        primary_content: color!(0xffffff),
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            palette: Palette::DARK,
        }
    }
}
