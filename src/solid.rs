use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

use crate::Error;
use crate::Result as SolidResult;
use crate::parser::parse_solid;

#[cfg(feature = "named-colors")]
pub use crate::parser::NAMED_COLORS;

use crate::utils::{clamp0_1, hsl_to_rgb, normalize_angle, rgb_to_hsl};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// The color
pub struct Solid {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
    /// Alpha
    pub a: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Hsla<T> {
    pub h: T,
    pub s: T,
    pub l: T,
    pub a: f32,
}

impl Solid {
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba {
            r: (self.r * 255.0 + 0.5) as u8,
            g: (self.g * 255.0 + 0.5) as u8,
            b: (self.b * 255.0 + 0.5) as u8,
            a: self.a,
        }
    }

    pub fn to_rgba_f(&self) -> Rgba<f32> {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    pub fn to_rgba16(&self) -> Rgba<u16> {
        Rgba {
            r: (self.r * 65535.0 + 0.5) as u16,
            g: (self.g * 65535.0 + 0.5) as u16,
            b: (self.b * 65535.0 + 0.5) as u16,
            a: self.a,
        }
    }

    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..1]
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_linear_rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        fn from_linear(x: f32) -> f32 {
            if x >= 0.0031308 {
                return 1.055 * x.powf(1.0 / 2.4) - 0.055;
            }
            12.92 * x
        }
        Self::new(from_linear(r), from_linear(g), from_linear(b), a)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..1]
    pub fn from_linear_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        fn from_linear(x: f32) -> f32 {
            if x >= 0.0031308 {
                return 1.055 * x.powf(1.0 / 2.4) - 0.055;
            }
            12.92 * x
        }
        Self::new(
            from_linear(r as f32 / 255.0),
            from_linear(g as f32 / 255.0),
            from_linear(b as f32 / 255.0),
            a,
        )
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_linear_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_linear_rgba(r, g, b, a as f32 / 255.0)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(l));
        Self::new(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

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
    pub fn from_html<S: AsRef<str>>(s: S) -> SolidResult<Self> {
        parse_solid(s.as_ref())
    }

    #[cfg(feature = "named-colors")]
    pub fn name(&self) -> Option<&'static str> {
        let rgb = self.to_rgba();
        for (&k, &v) in NAMED_COLORS.entries() {
            if v[0] == rgb.r && v[1] == rgb.b && v[2] == rgb.r {
                return Some(k);
            }
        }
        None
    }

    /// Returns: `[h, s, l, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..100]
    /// * `l`: Lightness [0..100]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla(&self) -> Hsla<u8> {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);

        Hsla {
            h: h as u8,
            s: (s * 100.0).round() as u8,
            l: (l * 100.0).round() as u8,
            a: self.a,
        }
    }

    /// Returns: `[h, s, l, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla_f(&self) -> Hsla<f32> {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);

        Hsla { h, s, l, a: self.a }
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn to_linear_rgba_f(&self) -> Rgba<f32> {
        fn to_linear(x: f32) -> f32 {
            if x >= 0.04045 {
                return ((x + 0.055) / 1.055).powf(2.4);
            }
            x / 12.92
        }
        Rgba {
            r: to_linear(self.r),
            g: to_linear(self.g),
            b: to_linear(self.b),
            a: self.a,
        }
    }

    /// Returns: `[r, g, b, a]`
    /// * Red, green, blue in the range [0..255]
    /// * Alpha in the range [0..1]
    pub fn to_linear_rgba(&self) -> Rgba<u8> {
        let f_rgba = self.to_linear_rgba_f();
        Rgba {
            r: (f_rgba.r * 255.0).round() as u8,
            g: (f_rgba.g * 255.0).round() as u8,
            b: (f_rgba.b * 255.0).round() as u8,
            a: f_rgba.a,
        }
    }

    /// Get the RGB hexadecimal color string.
    pub fn to_hex_string(&self) -> String {
        let rgba = self.to_rgba();
        let alpha = (rgba.a * 255.0).round() as u8;

        if alpha < 255 {
            return format!("#{:02x}{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b, alpha);
        }

        format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
    }

    /// Get the CSS `rgb()` format string.
    pub fn to_rgb_string(&self) -> String {
        let rgba = self.to_rgba();

        if self.a < 1.0 {
            return format!("rgba({},{},{},{})", rgba.r, rgba.g, rgba.b, self.a);
        }

        format!("rgb({},{},{})", rgba.r, rgba.g, rgba.b)
    }

    /// Blend this color with the other one, in the RGB color-space. `t` in the range [0..1].
    pub fn interpolate_rgb(&self, other: &Solid, t: f32) -> Self {
        Self {
            r: self.r + t * (other.r - self.r),
            g: self.g + t * (other.g - self.g),
            b: self.b + t * (other.b - self.b),
            a: self.a + t * (other.a - self.a),
        }
    }

    /// Blend this color with the other one, in the linear RGB color-space. `t` in the range [0..1].
    pub fn interpolate_linear_rgb(&self, other: &Solid, t: f32) -> Self {
        let rgba_1 = self.to_linear_rgba_f();
        let rgba_2 = other.to_linear_rgba_f();
        Self::from_linear_rgba_f(
            rgba_1.r + t * (rgba_2.r - rgba_1.r),
            rgba_1.g + t * (rgba_2.g - rgba_1.g),
            rgba_1.b + t * (rgba_2.b - rgba_1.b),
            rgba_1.a + t * (rgba_2.a - rgba_1.a),
        )
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl fmt::Display for Solid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solid({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl fmt::Display for Rgba<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rgba({}, {}, {}, {:.2})", // Format floating-point values to 2 decimal places
            self.r, self.g, self.b, self.a
        )
    }
}

impl fmt::Display for Rgba<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rgba({:.2}, {:.2}, {:.2}, {:.2})", // Format floating-point values to 2 decimal places
            self.r, self.g, self.b, self.a
        )
    }
}

impl fmt::Display for Hsla<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hsla({}, {}, {}, {:.2})", // Format floating-point values to 2 decimal places
            self.h, self.s, self.l, self.a
        )
    }
}

impl fmt::Display for Hsla<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hsla({:.2}, {:.2}, {:.2}, {:.2})", // Format floating-point values to 2 decimal places
            self.h, self.s, self.l, self.a
        )
    }
}

impl FromStr for Solid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_solid(s)
    }
}

impl TryFrom<&str> for Solid {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse_solid(s)
    }
}

impl From<(f32, f32, f32, f32)> for Solid {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32)> for Solid {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

impl From<[f32; 4]> for Solid {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<[f32; 3]> for Solid {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

impl From<[f64; 4]> for Solid {
    fn from([r, g, b, a]: [f64; 4]) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        }
    }
}

impl From<[f64; 3]> for Solid {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: 1.0,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Solid {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::from_rgba8(r, g, b, a)
    }
}

impl From<(u8, u8, u8)> for Solid {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }
}

impl From<[u8; 4]> for Solid {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self::from_rgba8(r, g, b, a)
    }
}

impl From<[u8; 3]> for Solid {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }
}

impl From<(u8, u8, u8, f32)> for Solid {
    fn from((r, g, b, a): (u8, u8, u8, f32)) -> Self {
        Self::from_rgba(r, g, b, a)
    }
}

/// Implement Serde serialization into HEX string
#[cfg(feature = "serde")]
impl Serialize for Solid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex_string())
    }
}

/// Implement Serde deserialization from string
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Solid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(SolidVisitor)
    }
}

#[cfg(feature = "serde")]
struct SolidVisitor;

#[cfg(feature = "serde")]
impl Visitor<'_> for SolidVisitor {
    type Value = Solid;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a valid css color")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Solid::from_str(v).map_err(serde::de::Error::custom)
    }
}
