#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    Red,
    Blue,
}

impl Team {
    pub fn label(self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
        }
    }

    pub fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Red,
        }
    }
}
