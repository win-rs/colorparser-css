use crate::Color;
use crate::ColorValue;
use crate::ErrorKind;
use crate::Solid;
use crate::gradient::Gradient;
use crate::gradient::GradientCoordinates;
use crate::gradient::is_valid_direction;
use crate::utils::darken;
use crate::utils::get_accent;
use crate::utils::lighten;
use crate::utils::strip_string;

mod named_colors;
use named_colors::COLOR_REGEX;
use named_colors::DARKEN_LIGHTEN_REGEX;
#[cfg(feature = "named-colors")]
pub use named_colors::NAMED_COLORS;
#[cfg(feature = "hash-colors")]
pub use named_colors::NAMED_COLORS_MAP;

pub use crate::Error;
pub use crate::Result;

/// Parse CSS color string
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("#ff0")?;
///
/// assert_eq!(c.to_array(), [1.0, 1.0, 0.0, 1.0]);
/// assert_eq!(c.to_rgba8(), [255, 255, 0, 255]);
/// assert_eq!(c.to_hex_string(), "#ffff00");
/// assert_eq!(c.to_rgb_string(), "rgb(255,255,0)");
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("hsl(360deg,100%,50%)")?;
///
/// assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
/// assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
/// assert_eq!(c.to_hex_string(), "#ff0000");
/// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
/// # Ok(())
/// # }
/// ```
pub fn parse_solid(s: &str) -> Result<Solid> {
    let s = s.trim().to_lowercase();

    if s == "transparent" {
        return Ok(Solid::new(0.0, 0.0, 0.0, 0.0));
    }

    if s == "accent" {
        return get_accent(true);
    }

    if s == "accent_inactive" {
        return get_accent(false);
    }

    // Named colors
    #[cfg(feature = "hash-colors")]
    if let Some([r, g, b]) = NAMED_COLORS_MAP.get(&*s) {
        return Ok(Solid::from_rgba8(*r, *g, *b, 255));
    }

    #[cfg(feature = "named-colors")]
    if let Some([r, g, b]) = NAMED_COLORS.get(&*s) {
        return Ok(Solid::from_rgba8(*r, *g, *b, 255));
    }

    // Hex format
    if let Some(s) = s.strip_prefix('#') {
        return parse_hex(s);
    }

    if s.starts_with("darken(") || s.starts_with("lighten(") {
        return parse_darken_or_lighten(&s);
    }

    if let (Some(i), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        let fname = &s[..i].trim_end();
        let s = &s[i + 1..].replace([',', '/'], " ");
        let params = s.split_whitespace().collect::<Vec<&str>>();
        let p_len = params.len();

        match *fname {
            "rgb" | "rgba" => {
                if p_len != 3 && p_len != 4 {
                    return Err(Error::new(ErrorKind::InvalidRgb, s));
                }

                let r = parse_percent_or_255(params[0]);
                let g = parse_percent_or_255(params[1]);
                let b = parse_percent_or_255(params[2]);

                let a = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some((1.0, true))
                };

                if let (Some((r, r_fmt)), Some((g, g_fmt)), Some((b, b_fmt)), Some((a, _))) =
                    (r, g, b, a)
                {
                    if r_fmt == g_fmt && g_fmt == b_fmt {
                        return Ok(Solid::new(
                            r.clamp(0.0, 1.0),
                            g.clamp(0.0, 1.0),
                            b.clamp(0.0, 1.0),
                            a.clamp(0.0, 1.0),
                        ));
                    }
                }

                return Err(Error::new(ErrorKind::InvalidRgb, s));
            }
            "hsl" | "hsla" => {
                if p_len != 3 && p_len != 4 {
                    return Err(Error::new(ErrorKind::InvalidHsl, s));
                }

                let original_s = s;
                let h = parse_angle(params[0]);
                let s = parse_percent_or_float(params[1]);
                let l = parse_percent_or_float(params[2]);

                let a = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some((1.0, true))
                };

                if let (Some(h), Some((s, s_fmt)), Some((l, l_fmt)), Some((a, _))) = (h, s, l, a) {
                    if s_fmt == l_fmt {
                        return Ok(Solid::from_normalized_hsla(h, s, l, a));
                    }
                }

                return Err(Error::new(ErrorKind::InvalidHsl, original_s));
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidFunction, s));
            }
        }
    }

    // Hex format without prefix '#'
    if let Ok(c) = parse_hex(&s) {
        return Ok(c);
    }

    Err(Error::new(ErrorKind::InvalidUnknown, s))
}

pub fn parse_gradient(s: &str) -> Result<Gradient> {
    if !s.starts_with("gradient(") {
        return Err(Error::new(ErrorKind::InvalidGradient, s));
    }

    let binding = strip_string(s.to_string(), &["gradient("], ')');

    let color_matches = COLOR_REGEX
        .captures_iter(&binding)
        .filter_map(|cap| cap.get(0).map(|m| m.as_str()))
        .collect::<Vec<&str>>();

    let remaining_input = s
        [s.rfind(color_matches.last().unwrap()).unwrap() + color_matches.last().unwrap().len()..]
        .trim_start();

    let remaining_input_arr = remaining_input
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .collect::<Vec<&str>>();

    let direction = remaining_input_arr
        .iter()
        .find(|&&input| is_valid_direction(input))
        .map(|&s| s.to_string())
        .unwrap_or_else(|| "to right".to_string());

    let colors = color_matches
        .iter()
        .filter_map(|&color| parse_solid(color).ok()) // Only keep Ok values
        .collect::<Vec<Solid>>();

    let direction = GradientCoordinates::try_from(direction.as_str())?;

    Ok(Gradient { direction, colors })
}

pub fn parse(s: &str) -> Result<Color> {
    if s.starts_with("gradient(") {
        parse_gradient(s).map(|res| Color(ColorValue::Gradient(res)))
    } else {
        parse_solid(s).map(|res| Color(ColorValue::Solid(res)))
    }
}

fn parse_darken_or_lighten(s: &str) -> Result<Solid> {
    let is_darken = s.starts_with("darken(");
    if let Some(caps) = DARKEN_LIGHTEN_REGEX.captures(s) {
        if caps.len() != 4 {
            return match is_darken {
                true => Err(Error::new(ErrorKind::InvalidDarken, s)),
                false => Err(Error::new(ErrorKind::InvalidLighten, s)),
            };
        }
        let dark_or_lighten = &caps[1];
        let color_str = &caps[2];
        let percentage = &caps[3].parse::<f32>().unwrap_or(10.0);

        let color = parse_solid(color_str)?;

        let color_res = match dark_or_lighten {
            "darken" => darken(color.to_hsla(), *percentage),
            "lighten" => lighten(color.to_hsla(), *percentage),
            _ => color,
        };

        return Ok(color_res);
    }

    match is_darken {
        true => Err(Error::new(ErrorKind::InvalidDarken, s)),
        false => Err(Error::new(ErrorKind::InvalidLighten, s)),
    }
}

fn parse_hex(s: &str) -> Result<Solid> {
    if !matches!(s.len(), 3 | 4 | 6 | 8) || !s[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Error::new(ErrorKind::InvalidHex, s));
    }

    let n = s.len();

    let parse_digit = |digit: &str| -> Result<u8> {
        u8::from_str_radix(digit, 16)
            .map(|n| if digit.len() == 1 { (n << 4) | n } else { n })
            .map_err(|_| Error::new(ErrorKind::InvalidHex, s))
    };

    if n == 3 || n == 4 {
        let r = parse_digit(&s[0..1])?;
        let g = parse_digit(&s[1..2])?;
        let b = parse_digit(&s[2..3])?;

        let a = if n == 4 { parse_digit(&s[3..4])? } else { 255 };

        Ok(Solid::from_rgba8(r, g, b, a))
    } else if n == 6 || n == 8 {
        let r = parse_digit(&s[0..2])?;
        let g = parse_digit(&s[2..4])?;
        let b = parse_digit(&s[4..6])?;

        let a = if n == 8 { parse_digit(&s[6..8])? } else { 255 };

        Ok(Solid::from_rgba8(r, g, b, a))
    } else {
        Err(Error::new(ErrorKind::InvalidHex, s))
    }
}

fn parse_percent_or_float(s: &str) -> Option<(f32, bool)> {
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| (t / 100.0, true)))
        .or_else(|| s.parse().ok().map(|t| (t, false)))
}

fn parse_percent_or_255(s: &str) -> Option<(f32, bool)> {
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| (t / 100.0, true)))
        .or_else(|| s.parse().ok().map(|t: f32| (t / 255.0, false)))
}

fn parse_angle(s: &str) -> Option<f32> {
    s.strip_suffix("deg")
        .and_then(|s| s.parse().ok())
        .or_else(|| {
            s.strip_suffix("grad")
                .and_then(|s| s.parse().ok())
                .map(|t: f32| t * 360.0 / 400.0)
        })
        .or_else(|| {
            s.strip_suffix("rad")
                .and_then(|s| s.parse().ok())
                .map(|t: f32| t.to_degrees())
        })
        .or_else(|| {
            s.strip_suffix("turn")
                .and_then(|s| s.parse().ok())
                .map(|t: f32| t * 360.0)
        })
        .or_else(|| s.parse().ok())
}
