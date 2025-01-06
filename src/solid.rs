use std::convert::TryFrom;
use std::fmt;
use std::ops::Mul;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

use crate::ColorspaceImpl;
use crate::Error;
use crate::Hsla;
use crate::NormalizedHsla;
use crate::NormalizedRgba;
use crate::Result as SolidResult;
use crate::Rgba;
use crate::Rgba16;
#[cfg(feature = "named-colors")]
use crate::parser::NAMED_COLORS;
use crate::parser::parse_solid;
use crate::utils::{clamp0_1, hsl_to_rgb, normalize_angle, rgb_to_hsl};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Solid(f32, f32, f32, f32);

impl Solid {
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(r, g, b, a)
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn to_rgba(&self) -> Rgba {
        Rgba {
            r: self.0.mul_add(255.0, 0.5) as u8,
            g: self.1.mul_add(255.0, 0.5) as u8,
            b: self.2.mul_add(255.0, 0.5) as u8,
            a: self.3,
        }
    }

    pub fn to_normalized_rgba(&self) -> NormalizedRgba {
        NormalizedRgba {
            r: self.0,
            g: self.1,
            b: self.2,
            a: self.3,
        }
    }

    pub fn to_rgba16(&self) -> Rgba16 {
        Rgba16 {
            r: self.0.mul_add(65535.0, 0.5) as u16,
            g: self.1.mul_add(65535.0, 0.5) as u16,
            b: self.2.mul_add(65535.0, 0.5) as u16,
            a: self.3,
        }
    }

    pub fn clamp(&self) -> Self {
        Self(
            self.0.clamp(0.0, 1.0),
            self.1.clamp(0.0, 1.0),
            self.2.clamp(0.0, 1.0),
            self.3.clamp(0.0, 1.0),
        )
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..65535]
    /// * `g`: Green value [0..65535]
    /// * `b`: Blue value [0..65535]
    /// * `a`: Alpha value [0..65535]
    pub fn from_rgba16(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self::new(
            r as f32 / 65535.0,
            g as f32 / 65535.0,
            b as f32 / 65535.0,
            a as f32 / 65535.0,
        )
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..1]
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_normalized_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_normalized_linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
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
        Self::from_normalized_linear_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_linear_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_normalized_linear_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..65535]
    /// * `g`: Green value [0..65535]
    /// * `b`: Blue value [0..65535]
    /// * `a`: Alpha value [0..65535]
    pub fn from_linear_rgba16(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self::from_normalized_linear_rgba(
            r as f32 / 65535.0,
            g as f32 / 65535.0,
            b as f32 / 65535.0,
            a as f32 / 65535.0,
        )
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
    pub fn to_hsla(&self) -> Hsla {
        let (h, s, l) = rgb_to_hsl(self.0, self.1, self.2);
        Hsla {
            h,
            s: s.mul(100.0),
            l: l.mul(100.0),
            a: self.3,
        }
    }

    /// Returns: `[h, s, l, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_normalized_hsla(&self) -> NormalizedHsla {
        let (h, s, l) = rgb_to_hsl(self.0, self.1, self.2);
        NormalizedHsla { h, s, l, a: self.3 }
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn to_normalized_linear_rgba(&self) -> NormalizedRgba {
        fn to_linear(x: f32) -> f32 {
            if x >= 0.04045 {
                return ((x + 0.055) / 1.055).powf(2.4);
            }
            x / 12.92
        }
        NormalizedRgba {
            r: to_linear(self.0),
            g: to_linear(self.1),
            b: to_linear(self.2),
            a: self.3,
        }
    }

    /// Returns: `[r, g, b, a]`
    /// * Red, green, blue in the range [0..255]
    /// * Alpha in the range [0..1]
    pub fn to_linear_rgba(&self) -> Rgba {
        let n_rgba = self.to_normalized_linear_rgba();
        Rgba {
            r: n_rgba.r.mul(255.0).round() as u8,
            g: n_rgba.g.mul(255.0).round() as u8,
            b: n_rgba.b.mul(255.0).round() as u8,
            a: n_rgba.a,
        }
    }

    /// Get the RGB hexadecimal color string.
    pub fn to_hex_string(&self) -> String {
        let rgba = self.to_rgba();
        let alpha = rgba.a.mul_add(255.0, 0.5).round() as u8;

        if alpha < 255 {
            return format!("#{:02x}{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b, alpha);
        }

        format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
    }

    /// Get the CSS `rgb()` format string.
    pub fn to_rgb_string(&self) -> String {
        let rgba = self.to_rgba();

        if self.3 < 1.0 {
            return format!("rgba({},{},{},{})", rgba.r, rgba.g, rgba.b, self.3);
        }

        format!("rgb({},{},{})", rgba.r, rgba.g, rgba.b)
    }

    /// Blend this color with the other one, in the RGB color-space. `t` in the range [0..1].
    pub fn interpolate_rgb(&self, other: &Solid, t: f32) -> Self {
        Self::new(
            self.0 + t * (other.0 - self.0),
            self.1 + t * (other.1 - self.1),
            self.2 + t * (other.2 - self.2),
            self.3 + t * (other.3 - self.3),
        )
    }

    /// Blend this color with the other one, in the linear RGB color-space. `t` in the range [0..1].
    pub fn interpolate_linear_rgb(&self, other: &Solid, t: f32) -> Self {
        let rgba_1 = self.to_normalized_linear_rgba();
        let rgba_2 = other.to_normalized_linear_rgba();
        Self::from_normalized_linear_rgba(
            rgba_1.r + t * (rgba_2.r - rgba_1.r),
            rgba_1.g + t * (rgba_2.g - rgba_1.g),
            rgba_1.b + t * (rgba_2.b - rgba_1.b),
            rgba_1.a + t * (rgba_2.a - rgba_1.a),
        )
    }

    pub fn darken(&self, percentage: f32) -> Self {
        let rgba = self.to_normalized_rgba().darken(percentage);
        Self::from(rgba)
    }

    pub fn lighten(&self, percentage: f32) -> Self {
        let rgba = self.to_normalized_rgba().lighten(percentage);
        Self::from(rgba)
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl fmt::Display for Solid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solid({}, {}, {}, {})", self.0, self.1, self.2, self.3)
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
        Self::new(r, g, b, a)
    }
}

impl From<(f32, f32, f32)> for Solid {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b, 1.0)
    }
}

impl From<[f32; 4]> for Solid {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self::new(r, g, b, a)
    }
}

impl From<[f32; 3]> for Solid {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::new(r, g, b, 1.0)
    }
}

impl From<[f64; 4]> for Solid {
    fn from([r, g, b, a]: [f64; 4]) -> Self {
        Self::new(r as f32, g as f32, b as f32, a as f32)
    }
}

impl From<[f64; 3]> for Solid {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Self::new(r as f32, g as f32, b as f32, 1.0)
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

impl From<(u8, u8, u8, f32)> for Solid {
    fn from((r, g, b, a): (u8, u8, u8, f32)) -> Self {
        Self::from_rgba(r, g, b, a)
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

impl From<(u16, u16, u16, u16)> for Solid {
    fn from((r, g, b, a): (u16, u16, u16, u16)) -> Self {
        Self::from_rgba16(r, g, b, a)
    }
}

impl From<(u16, u16, u16)> for Solid {
    fn from((r, g, b): (u16, u16, u16)) -> Self {
        Self::from_rgba16(r, g, b, 65535)
    }
}

impl From<(u16, u16, u16, f32)> for Solid {
    fn from((r, g, b, a): (u16, u16, u16, f32)) -> Self {
        Self::from_rgba16(r, g, b, (a * 65535.0 + 0.5).round() as u16)
    }
}

impl From<[u16; 4]> for Solid {
    fn from([r, g, b, a]: [u16; 4]) -> Self {
        Self::from_rgba16(r, g, b, a)
    }
}

impl From<[u16; 3]> for Solid {
    fn from([r, g, b]: [u16; 3]) -> Self {
        Self::from_rgba16(r, g, b, 65535)
    }
}

impl From<Rgba> for Solid {
    fn from(rgba: Rgba) -> Self {
        Self::from_rgba(rgba.r, rgba.g, rgba.b, rgba.a)
    }
}

impl From<Rgba16> for Solid {
    fn from(rgba: Rgba16) -> Self {
        Self::from_rgba16(rgba.r, rgba.g, rgba.b, (rgba.a * 65535.0).round() as u16)
    }
}

impl From<NormalizedRgba> for Solid {
    fn from(rgba: NormalizedRgba) -> Self {
        Self::from_normalized_rgba(rgba.r, rgba.g, rgba.b, rgba.a)
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
