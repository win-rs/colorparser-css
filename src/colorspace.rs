use core::fmt;
use std::ops::Mul;

use crate::utils::{hsl_to_rgb, normalize_angle, rgb_to_hsl};

pub trait ColorspaceImpl {
    fn to_rgba(&self) -> Rgba;
    fn to_rgba16(&self) -> Rgba16;
    fn to_normalized_rgba(&self) -> NormalizedRgba;
    fn to_hsla(&self) -> Hsla;
    fn to_normalized_hsla(&self) -> NormalizedHsla;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl ColorspaceImpl for Rgba {
    fn to_rgba(&self) -> Rgba {
        *self
    }

    fn to_normalized_rgba(&self) -> NormalizedRgba {
        NormalizedRgba {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
            a: self.a,
        }
    }

    fn to_rgba16(&self) -> Rgba16 {
        Rgba16 {
            r: (self.r as f32 * 257.0) as u16,
            g: (self.g as f32 * 257.0) as u16,
            b: (self.b as f32 * 257.0) as u16,
            a: self.a,
        }
    }

    fn to_normalized_hsla(&self) -> NormalizedHsla {
        let (h, s, l) = rgb_to_hsl(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        );
        NormalizedHsla { h, s, l, a: self.a }
    }

    fn to_hsla(&self) -> Hsla {
        let (h, s, l) = rgb_to_hsl(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        );
        Hsla {
            h,
            s: s.mul(100.0),
            l: l.mul(100.0),
            a: self.a,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct Rgba16 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: f32,
}

impl ColorspaceImpl for Rgba16 {
    fn to_rgba(&self) -> Rgba {
        Rgba {
            r: (self.r as f32 / 257.0).round() as u8,
            g: (self.g as f32 / 257.0).round() as u8,
            b: (self.b as f32 / 257.0).round() as u8,
            a: self.a,
        }
    }

    fn to_normalized_rgba(&self) -> NormalizedRgba {
        NormalizedRgba {
            r: self.r as f32 / 65535.0,
            g: self.g as f32 / 65535.0,
            b: self.b as f32 / 65535.0,
            a: self.a,
        }
    }

    fn to_rgba16(&self) -> Rgba16 {
        *self
    }

    fn to_normalized_hsla(&self) -> NormalizedHsla {
        let (h, s, l) = rgb_to_hsl(
            self.r as f32 / 65535.0,
            self.g as f32 / 65535.0,
            self.b as f32 / 65535.0,
        );
        NormalizedHsla { h, s, l, a: self.a }
    }

    fn to_hsla(&self) -> Hsla {
        let (h, s, l) = rgb_to_hsl(
            self.r as f32 / 65535.0,
            self.g as f32 / 65535.0,
            self.b as f32 / 65535.0,
        );
        Hsla {
            h,
            s: s.mul(100.0),
            l: l.mul(100.0),
            a: self.a,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct NormalizedRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorspaceImpl for NormalizedRgba {
    fn to_rgba(&self) -> Rgba {
        Rgba {
            r: self.r.mul_add(255.0, 0.5).round() as u8,
            g: self.g.mul_add(255.0, 0.5).round() as u8,
            b: self.b.mul_add(255.0, 0.5).round() as u8,
            a: self.a,
        }
    }

    fn to_normalized_rgba(&self) -> NormalizedRgba {
        *self
    }

    fn to_rgba16(&self) -> Rgba16 {
        Rgba16 {
            r: self.r.mul_add(65535.0, 0.5).round() as u16,
            g: self.g.mul_add(65535.0, 0.5).round() as u16,
            b: self.b.mul_add(65535.0, 0.5).round() as u16,
            a: self.a,
        }
    }

    fn to_normalized_hsla(&self) -> NormalizedHsla {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        NormalizedHsla { h, s, l, a: self.a }
    }

    fn to_hsla(&self) -> Hsla {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        Hsla {
            h,
            s: s.mul(100.0),
            l: l.mul(100.0),
            a: self.a,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct Hsla {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

impl ColorspaceImpl for Hsla {
    fn to_rgba(&self) -> Rgba {
        let (r, g, b) = hsl_to_rgb(normalize_angle(self.h), self.s / 100.0, self.l / 100.0);
        Rgba {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8,
            a: self.a,
        }
    }

    fn to_normalized_rgba(&self) -> NormalizedRgba {
        let (r, g, b) = hsl_to_rgb(normalize_angle(self.h), self.s / 100.0, self.l / 100.0);
        NormalizedRgba { r, g, b, a: self.a }
    }

    fn to_rgba16(&self) -> Rgba16 {
        let (r, g, b) = hsl_to_rgb(normalize_angle(self.h), self.s / 100.0, self.l / 100.0);
        Rgba16 {
            r: (r * 65535.0).round() as u16,
            g: (g * 65535.0).round() as u16,
            b: (b * 65535.0).round() as u16,
            a: self.a,
        }
    }

    fn to_normalized_hsla(&self) -> NormalizedHsla {
        NormalizedHsla {
            h: self.h,
            s: self.s / 100.0,
            l: self.l / 100.0,
            a: self.a,
        }
    }

    fn to_hsla(&self) -> Self {
        *self
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct NormalizedHsla {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

impl ColorspaceImpl for NormalizedHsla {
    fn to_rgba(&self) -> Rgba {
        let (r, g, b) = hsl_to_rgb(normalize_angle(self.h), self.s / 100.0, self.l / 100.0);
        Rgba {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8,
            a: self.a,
        }
    }

    fn to_normalized_rgba(&self) -> NormalizedRgba {
        let (r, g, b) = hsl_to_rgb(normalize_angle(self.h), self.s / 100.0, self.l / 100.0);
        NormalizedRgba { r, g, b, a: self.a }
    }

    fn to_rgba16(&self) -> Rgba16 {
        let (r, g, b) = hsl_to_rgb(normalize_angle(self.h), self.s / 100.0, self.l / 100.0);
        Rgba16 {
            r: (r * 65535.0).round() as u16,
            g: (g * 65535.0).round() as u16,
            b: (b * 65535.0).round() as u16,
            a: self.a,
        }
    }

    fn to_normalized_hsla(&self) -> Self {
        *self
    }

    fn to_hsla(&self) -> Hsla {
        Hsla {
            h: self.h,
            s: self.s * 100.0,
            l: self.l * 100.0,
            a: self.a,
        }
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rgba({}, {}, {}, {})", // Format floating-point values to 2 decimal places
            self.r, self.g, self.b, self.a
        )
    }
}

impl fmt::Display for Rgba16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rgba({}, {}, {}, {})", // Format floating-point values to 2 decimal places
            self.r, self.g, self.b, self.a
        )
    }
}

impl fmt::Display for NormalizedRgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rgba({}, {}, {}, {})", // Format floating-point values to 2 decimal places
            self.r, self.g, self.b, self.a
        )
    }
}

impl fmt::Display for Hsla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hsla({}, {}, {}, {})", // Format floating-point values to 2 decimal places
            self.h, self.s, self.l, self.a
        )
    }
}

impl fmt::Display for NormalizedHsla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hsla({}, {}, {}, {})", // Format floating-point values to 2 decimal places
            self.h, self.s, self.l, self.a
        )
    }
}
