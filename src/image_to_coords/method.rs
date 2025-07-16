use serde::Serialize;
use std::convert::TryFrom;

/// Method enum
/// Capture the different methods that can be used
#[derive(Default, Clone, Copy, Debug, Serialize)]
pub enum Method {
    Outline,
    #[default]
    Full,
    Scan,
    Snake,
    Black,
    Dynamic,
    Zigzag,
}

/// Implement Display for Method
impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(*self))
    }
}

/// Implement From<Method> for &str
impl From<Method> for &str {
    fn from(method: Method) -> Self {
        match method {
            Method::Outline => "outline",
            Method::Full => "full",
            Method::Scan => "scan",
            Method::Snake => "snake",
            Method::Black => "black",
            Method::Dynamic => "dynamic",
            Method::Zigzag => "zigzag",
        }
    }
}

/// Implement From<&str> for Method
impl TryFrom<&str> for Method {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            _ if value.contains("outline") => Self::Outline,
            _ if value.contains("full") => Self::Full,
            _ if value.contains("scan") => Self::Scan,
            _ if value.contains("snake") => Self::Snake,
            _ if value.contains("black") => Self::Black,
            _ if value.contains("dynamic") => Self::Dynamic,
            _ if value.contains("zigzag") => Self::Zigzag,
            _ => return Err(()),
        })
    }
}
