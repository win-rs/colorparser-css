use std::fmt;

use crate::{Error, ErrorKind, Result as ColorResult, Solid, gradient::Gradient, parse};

#[cfg_attr(feature = "schema", derive(schema_jsonrs::JsonSchema))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColorValue {
    Solid(Solid),
    Gradient(Gradient),
}

#[cfg_attr(feature = "schema", derive(schema_jsonrs::JsonSchema))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Color(pub ColorValue);

impl Color {
    /// Create color from CSS color string.
    pub fn from_html<S: AsRef<str>>(s: S) -> ColorResult<Self> {
        Self::try_from(s.as_ref())
    }

    /// Create color from CSS color string.
    #[cfg(feature = "theme")]
    pub fn from_html_with_theme<S: AsRef<str>, P: AsRef<str>>(s: S, path: P) -> ColorResult<Self> {
        Self::try_from_theme(s.as_ref(), path.as_ref())
    }

    /// Convert a `Color` to a `Gradient`.
    pub fn to_gradient(&self) -> ColorResult<Gradient> {
        match self.0.clone() {
            ColorValue::Gradient(grad) => Ok(grad),
            _ => Err(Error::new(ErrorKind::InvalidFunction, self.to_string())),
        }
    }

    /// Convert a `Color` to a `Solid`.
    pub fn to_solid(&self) -> ColorResult<Solid> {
        match self.0.clone() {
            ColorValue::Solid(solid) => Ok(solid),
            _ => Err(Error::new(ErrorKind::InvalidFunction, self.to_string())),
        }
    }

    #[cfg(feature = "theme")]
    fn try_from_theme(s: &str, path: &str) -> ColorResult<Self> {
        parse(s, Some(path))
    }
}

impl TryFrom<Gradient> for Color {
    type Error = Error;

    fn try_from(gradient: Gradient) -> Result<Self, Self::Error> {
        Ok(Color(ColorValue::Gradient(gradient)))
    }
}

impl TryFrom<Solid> for Color {
    type Error = Error;

    fn try_from(solid: Solid) -> Result<Self, Self::Error> {
        Ok(Color(ColorValue::Solid(solid)))
    }
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse(s, None)
    }
}

impl fmt::Display for ColorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorValue::Solid(solid) => write!(f, "{}", solid),
            ColorValue::Gradient(gradient) => write!(f, "{}", gradient),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({})", self.0)
    }
}
