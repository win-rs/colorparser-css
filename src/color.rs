use std::fmt;

use crate::{Error, ErrorKind, Result as ColorResult, Solid, gradient::Gradient, parse};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColorValue {
    Solid(Solid),
    Gradient(Gradient),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Color(pub ColorValue);

impl Color {
    /// Create color from CSS color string.
    ///
    /// # Examples
    /// ```
    /// use csscolorparser::Solid;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let c = Solid::from_html("rgb(255,0,0)")?;
    ///
    /// assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
    /// assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
    /// assert_eq!(c.to_hex_string(), "#ff0000");
    /// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_html<S: AsRef<str>>(s: S) -> ColorResult<Self> {
        Self::try_from(s.as_ref())
    }

    pub fn to_gradient(&self) -> ColorResult<Gradient> {
        match self.0.clone() {
            ColorValue::Gradient(grad) => Ok(grad),
            _ => Err(Error::new(ErrorKind::InvalidFunction, self.to_string())),
        }
    }

    pub fn to_solid(&self) -> ColorResult<Solid> {
        match self.0.clone() {
            ColorValue::Solid(solid) => Ok(solid),
            _ => Err(Error::new(ErrorKind::InvalidFunction, self.to_string())),
        }
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
        parse(s)
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
