#![allow(unused_imports)]
use crate::Error;
use crate::ErrorKind;
use crate::Result;
use std::path::{Component, Path, PathBuf};
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::FALSE;
use windows_sys::Win32::Graphics::Dwm::DwmGetColorizationColor;

use crate::{Hsla, Solid};

fn hue_to_rgb(n1: f32, n2: f32, h: f32) -> f32 {
    let h = modulo(h, 6.0);

    if h < 1.0 {
        return n1 + ((n2 - n1) * h);
    }

    if h < 3.0 {
        return n2;
    }

    if h < 4.0 {
        return n1 + ((n2 - n1) * (4.0 - h));
    }

    n1
}

// h = 0..360
// s, l = 0..1
// r, g, b = 0..1
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let n2 = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - (l * s)
    };

    let n1 = 2.0 * l - n2;
    let h = h / 60.0;
    let r = hue_to_rgb(n1, n2, h + 2.0);
    let g = hue_to_rgb(n1, n2, h);
    let b = hue_to_rgb(n1, n2, h - 2.0);
    (r, g, b)
}

fn _get_min(rgb: &[f32]) -> f32 {
    rgb.iter().fold(f32::MAX, |a, &b| a.min(b))
}

fn _get_max(rgb: &[f32]) -> f32 {
    rgb.iter().fold(f32::MIN, |a, &b| a.max(b))
}

pub fn _rgb_to_hsl_other(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let rgb_arr: Vec<f32> = [r, g, b].iter().map(|p| p / 255.0).collect();
    let max = _get_max(&rgb_arr);
    let min = _get_min(&rgb_arr);
    let luminace = (max + min) / 2.0;

    if max.eq(&min) {
        return (0.0, 0.0, luminace * 100.0);
    }

    let max_min_delta = max - min;
    let saturation = if luminace > 0.5 {
        max_min_delta / (2.0 - max - min)
    } else {
        max_min_delta / (max + min)
    };

    let red = rgb_arr[0];
    let green = rgb_arr[1];
    let blue = rgb_arr[2];

    let hue = if red.eq(&max) {
        let x = if g < b { 6.0 } else { 0.0 };
        (green - blue) / max_min_delta + x
    } else if green.eq(&max) {
        (blue - red) / max_min_delta + 2.0
    } else {
        (red - green) / max_min_delta + 4.0
    };

    (hue * 60.0, saturation, luminace)
}

#[allow(clippy::float_cmp)]
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let l = (max + min) / 2.0;

    if min == max {
        return (0.0, 0.0, l);
    }

    let d = max - min;

    let s = if l < 0.5 {
        d / (max + min)
    } else {
        d / (2.0 - max - min)
    };

    let dr = (max - r) / d;
    let dg = (max - g) / d;
    let db = (max - b) / d;

    let h = if r == max {
        db - dg
    } else if g == max {
        2.0 + dr - db
    } else {
        4.0 + dg - dr
    };

    let h = (h * 60.0) % 360.0;
    (normalize_angle(h), s, l)
}

#[inline]
pub fn normalize_angle(t: f32) -> f32 {
    let mut t = t % 360.0;
    if t < 0.0 {
        t += 360.0;
    }
    t
}

#[inline]
pub fn clamp0_1(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

#[inline]
pub fn modulo(x: f32, n: f32) -> f32 {
    (x % n + n) % n
}

pub fn strip_string(input: String, prefixes: &[&str], suffix: char) -> String {
    let mut result = input;

    // Remove matching prefix (if any)
    for &prefix in prefixes {
        if let Some(stripped) = result.strip_prefix(prefix) {
            result = stripped.to_string();
            break; // Only remove the first matching prefix
        }
    }

    // Remove suffix (if it exists)
    result.strip_suffix(suffix).unwrap_or(&result).to_string()
}

pub fn get_accent(active: bool) -> Result<Solid> {
    #[cfg(windows)]
    {
        let mut pcr_colorization: u32 = 0;
        let mut pf_opaqueblend: BOOL = FALSE;
        unsafe { DwmGetColorizationColor(&mut pcr_colorization, &mut pf_opaqueblend) };

        let r = ((pcr_colorization & 0x00FF0000) >> 16) as f32 / 255.0;
        let g = ((pcr_colorization & 0x0000FF00) >> 8) as f32 / 255.0;
        let b = (pcr_colorization & 0x000000FF) as f32 / 255.0;
        let avg = (r + g + b) / 3.0;

        match active {
            true => Ok(Solid::from([r, g, b, 1.0])),
            false => Ok(Solid::from([
                avg / 1.5 + r / 10.0,
                avg / 1.5 + g / 10.0,
                avg / 1.5 + b / 10.0,
                1.0,
            ])),
        }
    }

    #[cfg(not(windows))]
    {
        Err(Error::new(
            ErrorKind::InvalidFunction,
            "accent is only available on windows platform",
        ))
    }
}

#[cfg(feature = "theme")]
/// The Clean trait implements a `clean` method.
pub trait PathClean {
    fn clean(&self) -> PathBuf;
}

#[cfg(feature = "theme")]
/// PathClean implemented for `Path`
impl PathClean for Path {
    fn clean(&self) -> PathBuf {
        clean(self)
    }
}

#[cfg(feature = "theme")]
/// PathClean implemented for `PathBuf`
impl PathClean for PathBuf {
    fn clean(&self) -> PathBuf {
        clean(self)
    }
}

#[cfg(feature = "theme")]
/// The core implementation. It performs the following, lexically:
/// 1. Reduce multiple slashes to a single slash.
/// 2. Eliminate `.` path name elements (the current directory).
/// 3. Eliminate `..` path name elements (the parent directory) and the non-`.` non-`..`, element that precedes them.
/// 4. Eliminate `..` elements that begin a rooted path, that is, replace `/..` by `/` at the beginning of a path.
/// 5. Leave intact `..` elements that begin a non-rooted path.
///
/// If the result of this process is an empty string, return the string `"."`, representing the current directory.
pub fn clean<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut out = Vec::new();

    for comp in path.as_ref().components() {
        match comp {
            Component::CurDir => (),
            Component::ParentDir => match out.last() {
                Some(Component::RootDir) => (),
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                None
                | Some(Component::CurDir)
                | Some(Component::ParentDir)
                | Some(Component::Prefix(_)) => out.push(comp),
            },
            comp => out.push(comp),
        }
    }

    if !out.is_empty() {
        out.iter().collect()
    } else {
        PathBuf::from(".")
    }
}
