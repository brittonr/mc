//! [`Color`] and related data structures.

mod codec;
mod convert;

#[cfg(test)]
mod tests;

/// Text color
#[derive(Default, Debug, PartialOrd, Eq, Ord, Clone, Copy)]
// API: keep the established public color type name.
#[allow(path_segment_repetition)]
pub enum Color {
    /// The default color for the text will be used, which varies by context
    /// (in some cases, it's white; in others, it's black; in still others, it
    /// is a shade of gray that isn't normally used on text).
    #[default]
    Reset,
    /// RGB Color
    Rgb(RgbColor),
    /// One of the 16 named Minecraft colors
    Named(NamedColor),
}

/// RGB Color
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash, Debug)]
// API: keep the established public RGB color type name.
#[allow(path_segment_repetition)]
pub struct RgbColor {
    /// Red channel
    pub r: u8,
    /// Green channel
    pub g: u8,
    /// Blue channel
    pub b: u8,
}

/// Named Minecraft color
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash, Debug)]
// API: keep the established public named color type name.
#[allow(path_segment_repetition)]
pub enum NamedColor {
    /// Hex digit: `0`, name: `black`
    Black = 0,
    /// Hex digit: `1`, name: `dark_blue`
    DarkBlue,
    /// Hex digit: `2`, name: `dark_green`
    DarkGreen,
    /// Hex digit: `3`, name: `dark_aqua`
    DarkAqua,
    /// Hex digit: `4`, name: `dark_red`
    DarkRed,
    /// Hex digit: `5`, name: `dark_purple`
    DarkPurple,
    /// Hex digit: `6`, name: `gold`
    Gold,
    /// Hex digit: `7`, name: `gray`
    Gray,
    /// Hex digit: `8`, name: `dark_gray`
    DarkGray,
    /// Hex digit: `9`, name: `blue`
    Blue,
    /// Hex digit: `a`, name: `green`
    Green,
    /// Hex digit: `b`, name: `aqua`
    Aqua,
    /// Hex digit: `c`, name: `red`
    Red,
    /// Hex digit: `d`, name: `light_purple`
    LightPurple,
    /// Hex digit: `e`, name: `yellow`
    Yellow,
    /// Hex digit: `f`, name: `white`
    White,
}

/// Color parsing error
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd, Clone, Copy, std::hash::Hash, Eq, Ord)]
#[error("invalid color name or hex code")]
// API: keep the established public color error type name.
#[allow(path_segment_repetition)]
pub struct ColorError;

impl Color {
    pub const RESET: Self = Self::Reset;
    pub const AQUA: Self = Self::Named(NamedColor::Aqua);
    pub const BLACK: Self = Self::Named(NamedColor::Black);
    pub const BLUE: Self = Self::Named(NamedColor::Blue);
    pub const DARK_AQUA: Self = Self::Named(NamedColor::DarkAqua);
    pub const DARK_BLUE: Self = Self::Named(NamedColor::DarkBlue);
    pub const DARK_GRAY: Self = Self::Named(NamedColor::DarkGray);
    pub const DARK_GREEN: Self = Self::Named(NamedColor::DarkGreen);
    pub const DARK_PURPLE: Self = Self::Named(NamedColor::DarkPurple);
    pub const DARK_RED: Self = Self::Named(NamedColor::DarkRed);
    pub const GOLD: Self = Self::Named(NamedColor::Gold);
    pub const GRAY: Self = Self::Named(NamedColor::Gray);
    pub const GREEN: Self = Self::Named(NamedColor::Green);
    pub const LIGHT_PURPLE: Self = Self::Named(NamedColor::LightPurple);
    pub const RED: Self = Self::Named(NamedColor::Red);
    pub const WHITE: Self = Self::Named(NamedColor::White);
    pub const YELLOW: Self = Self::Named(NamedColor::Yellow);

    /// Constructs a new RGB color
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(RgbColor::new(r, g, b))
    }
}

impl RgbColor {
    /// Constructs a new color from red, green, and blue components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
