use serde::Serialize;
use std::convert::TryFrom;

/// Mode enum
/// Capture the different modes that can be used
#[derive(Default, Clone, Copy, Debug, Serialize)]
pub enum Mode {
    Outline,
    #[default]
    Full,
    Scan,
    Snake,
    Black,
}

/// Implement Display for Mode
impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(*self))
    }
}

/// Implement From<Mode> for &str
impl From<Mode> for &str {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Outline => "outline",
            Mode::Full => "full",
            Mode::Scan => "scan",
            Mode::Snake => "snake",
            Mode::Black => "black",
        }
    }
}

/// Implement From<&str> for Mode
impl TryFrom<&str> for Mode {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            _ if value.contains("outline") => Self::Outline,
            _ if value.contains("full") => Self::Full,
            _ if value.contains("scan") => Self::Scan,
            _ if value.contains("snake") => Self::Snake,
            _ if value.contains("black") => Self::Black,
            _ => return Err(()),
        })
    }
}
