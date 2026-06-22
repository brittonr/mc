const HASH_DISCRIMINANT_RESET: u8 = 0;
const HASH_DISCRIMINANT_COLOR: u8 = 1;
const HEX_NIBBLE_SHIFT: u8 = 4;
const HEX_ALPHA_VALUE_OFFSET: u8 = 10;
const HEX_DIGITS: &[u8] = b"0123456789abcdef";

impl super::RgbColor {
    /// Converts the RGB color to the closest [`super::NamedColor`] equivalent
    /// (lossy).
    pub fn to_named_lossy(self) -> super::NamedColor {
        let named_colors = [
            super::NamedColor::Aqua,
            super::NamedColor::Black,
            super::NamedColor::Blue,
            super::NamedColor::DarkAqua,
            super::NamedColor::DarkBlue,
            super::NamedColor::DarkGray,
            super::NamedColor::DarkGreen,
            super::NamedColor::DarkPurple,
            super::NamedColor::DarkRed,
            super::NamedColor::Gold,
            super::NamedColor::Gray,
            super::NamedColor::Green,
            super::NamedColor::LightPurple,
            super::NamedColor::Red,
            super::NamedColor::White,
            super::NamedColor::Yellow,
        ];

        let mut closest = named_colors[0];
        let mut closest_distance = squared_distance(closest.into(), self);

        for named in named_colors.into_iter().skip(1) {
            let distance = squared_distance(named.into(), self);
            if distance < closest_distance {
                closest = named;
                closest_distance = distance;
            }
        }

        closest
    }
}

fn squared_distance(left: super::RgbColor, right: super::RgbColor) -> i32 {
    (i32::from(left.r) - i32::from(right.r)).pow(2)
        + (i32::from(left.g) - i32::from(right.g)).pow(2)
        + (i32::from(left.b) - i32::from(right.b)).pow(2)
}

impl super::NamedColor {
    /// Returns the corresponding hex digit of the color.
    pub const fn hex_digit(self) -> char {
        HEX_DIGITS[self as usize] as char
    }

    /// Returns the identifier of the color.
    pub const fn name(self) -> &'static str {
        [
            "black",
            "dark_blue",
            "dark_green",
            "dark_aqua",
            "dark_red",
            "dark_purple",
            "gold",
            "gray",
            "dark_gray",
            "blue",
            "green",
            "aqua",
            "red",
            "light_purple",
            "yellow",
            "white",
        ][self as usize]
    }
}

impl PartialEq for super::Color {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Reset, Self::Reset) => true,
            (Self::Rgb(rgb1), Self::Rgb(rgb2)) => rgb1 == rgb2,
            (Self::Named(normal1), Self::Named(normal2)) => normal1 == normal2,
            (Self::Rgb(rgb), Self::Named(normal)) | (Self::Named(normal), Self::Rgb(rgb)) => {
                rgb == super::RgbColor::from(normal)
            }
            (Self::Reset, _) | (_, Self::Reset) => false,
        }
    }
}

impl std::hash::Hash for super::Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Reset => state.write_u8(HASH_DISCRIMINANT_RESET),
            Self::Rgb(rgb) => {
                state.write_u8(HASH_DISCRIMINANT_COLOR);
                rgb.hash(state);
            }
            Self::Named(normal) => {
                state.write_u8(HASH_DISCRIMINANT_COLOR);
                super::RgbColor::from(*normal).hash(state);
            }
        }
    }
}

impl From<super::NamedColor> for super::RgbColor {
    fn from(value: super::NamedColor) -> Self {
        match value {
            super::NamedColor::Aqua => Self::new(85, 255, 255),
            super::NamedColor::Black => Self::new(0, 0, 0),
            super::NamedColor::Blue => Self::new(85, 85, 255),
            super::NamedColor::DarkAqua => Self::new(0, 170, 170),
            super::NamedColor::DarkBlue => Self::new(0, 0, 170),
            super::NamedColor::DarkGray => Self::new(85, 85, 85),
            super::NamedColor::DarkGreen => Self::new(0, 170, 0),
            super::NamedColor::DarkPurple => Self::new(170, 0, 170),
            super::NamedColor::DarkRed => Self::new(170, 0, 0),
            super::NamedColor::Gold => Self::new(255, 170, 0),
            super::NamedColor::Gray => Self::new(170, 170, 170),
            super::NamedColor::Green => Self::new(85, 255, 85),
            super::NamedColor::LightPurple => Self::new(255, 85, 255),
            super::NamedColor::Red => Self::new(255, 85, 85),
            super::NamedColor::White => Self::new(255, 255, 255),
            super::NamedColor::Yellow => Self::new(255, 255, 85),
        }
    }
}

impl From<super::RgbColor> for super::Color {
    fn from(value: super::RgbColor) -> Self {
        Self::Rgb(value)
    }
}

impl From<super::NamedColor> for super::Color {
    fn from(value: super::NamedColor) -> Self {
        Self::Named(value)
    }
}

impl TryFrom<&str> for super::Color {
    type Error = super::ColorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('#') {
            return Ok(Self::Rgb(super::RgbColor::try_from(value)?));
        }

        if value == "reset" {
            return Ok(Self::Reset);
        }

        Ok(Self::Named(super::NamedColor::try_from(value)?))
    }
}

impl TryFrom<&str> for super::NamedColor {
    type Error = super::ColorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "black" => Ok(super::NamedColor::Black),
            "dark_blue" => Ok(super::NamedColor::DarkBlue),
            "dark_green" => Ok(super::NamedColor::DarkGreen),
            "dark_aqua" => Ok(super::NamedColor::DarkAqua),
            "dark_red" => Ok(super::NamedColor::DarkRed),
            "dark_purple" => Ok(super::NamedColor::DarkPurple),
            "gold" => Ok(super::NamedColor::Gold),
            "gray" => Ok(super::NamedColor::Gray),
            "dark_gray" => Ok(super::NamedColor::DarkGray),
            "blue" => Ok(super::NamedColor::Blue),
            "green" => Ok(super::NamedColor::Green),
            "aqua" => Ok(super::NamedColor::Aqua),
            "red" => Ok(super::NamedColor::Red),
            "light_purple" => Ok(super::NamedColor::LightPurple),
            "yellow" => Ok(super::NamedColor::Yellow),
            "white" => Ok(super::NamedColor::White),
            _ => Err(super::ColorError),
        }
    }
}

impl TryFrom<&str> for super::RgbColor {
    type Error = super::ColorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let &[b'#', r0, r1, g0, g1, b0, b1] = value.as_bytes() {
            Ok(Self {
                r: parse_hex_byte(HexPair { high: r0, low: r1 })?,
                g: parse_hex_byte(HexPair { high: g0, low: g1 })?,
                b: parse_hex_byte(HexPair { high: b0, low: b1 })?,
            })
        } else {
            Err(super::ColorError)
        }
    }
}

#[derive(Clone, Copy)]
struct HexPair {
    high: u8,
    low: u8,
}

#[derive(Clone, Copy)]
struct HexDigitBase(u8);

fn parse_hex_byte(pair: HexPair) -> Result<u8, super::ColorError> {
    Ok((hex_nibble(pair.high)? << HEX_NIBBLE_SHIFT) | hex_nibble(pair.low)?)
}

fn hex_nibble(value: u8) -> Result<u8, super::ColorError> {
    match value {
        b'0'..=b'9' => checked_hex_offset(value, HexDigitBase(b'0')),
        b'a'..=b'f' => checked_hex_alpha_offset(value, HexDigitBase(b'a')),
        b'A'..=b'F' => checked_hex_alpha_offset(value, HexDigitBase(b'A')),
        _ => Err(super::ColorError),
    }
}

fn checked_hex_offset(value: u8, base: HexDigitBase) -> Result<u8, super::ColorError> {
    value.checked_sub(base.0).ok_or(super::ColorError)
}

fn checked_hex_alpha_offset(value: u8, base: HexDigitBase) -> Result<u8, super::ColorError> {
    checked_hex_offset(value, base)?
        .checked_add(HEX_ALPHA_VALUE_OFFSET)
        .ok_or(super::ColorError)
}

impl std::fmt::Display for super::Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reset => write!(f, "reset"),
            Self::Rgb(rgb) => rgb.fmt(f),
            Self::Named(normal) => normal.fmt(f),
        }
    }
}

impl std::fmt::Display for super::RgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl std::fmt::Display for super::NamedColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
