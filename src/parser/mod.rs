use crate::Color;
use crate::ColorValue;
use crate::ErrorKind;
use crate::Solid;
use crate::gradient::Gradient;
use crate::gradient::GradientCoordinates;
use crate::gradient::is_valid_direction;
use crate::utils::get_accent;
use crate::utils::strip_string;
#[cfg(feature = "theme")]
use crate::{Theme, utils::PathClean};

use named_colors::ACCENT_TRANSPARENT_PATTERN;
use named_colors::HEX_PATTERN;
use named_colors::HSLA_PATTERN;
use named_colors::RGBA_PATTERN;
#[cfg(feature = "named-colors")]
pub use named_colors::{NAMED_COLOR_PATTERN, NAMED_COLORS};
use regex::Regex;
#[cfg(feature = "theme")]
use std::{
    env::current_dir,
    fs::{metadata, read_to_string},
    path::PathBuf,
    sync::{LazyLock, RwLock},
    time::SystemTime,
};

pub use crate::Error;
pub use crate::Result;

mod named_colors;

#[cfg(feature = "theme")]
type ThemeCache = (String, Theme, SystemTime);
#[cfg(feature = "theme")]
static THEME_CACHE: LazyLock<RwLock<Option<ThemeCache>>> = LazyLock::new(|| RwLock::new(None));

/// Parse CSS color string to solid (with optional theme)
pub fn parse_solid(s: &str, file_path: Option<&str>) -> Result<Solid> {
    let s = s.trim().to_ascii_lowercase();

    match s.as_str() {
        "transparent" => return Ok(Solid::new(0.0, 0.0, 0.0, 0.0)),
        "accent" => return get_accent(true),
        "accent_inactive" => return get_accent(false),
        _ => {}
    }

    // Custom theme
    #[cfg(feature = "theme")]
    if let Some(file_path) = file_path {
        if let Some(color) = parse_custom_theme(file_path)?.get_color(&s) {
            return parse_solid(color.as_str(), None);
        }
    }

    // Named colors
    #[cfg(feature = "named-colors")]
    if let Some([r, g, b]) = NAMED_COLORS.get(&*s) {
        return Ok(Solid::from_rgba8(*r, *g, *b, 255));
    }

    // Hex format
    if let Some(s) = s.strip_prefix('#') {
        return parse_hex(s);
    }

    let original_s = s.clone();

    if let (Some(i), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        let fname = &s[..i].trim_end();
        let s = &s[i + 1..].replace([',', '/'], " ");
        let params = s.split_whitespace().collect::<Vec<&str>>();

        return match *fname {
            "rgb" | "rgba" => parse_rgb_or_rgba(params, original_s.as_str()),
            "hsl" | "hsla" => parse_hsl_or_hsla(params, original_s.as_str()),
            _ => Err(Error::new(ErrorKind::InvalidFunction, s)),
        };
    }

    // Hex format without prefix '#'
    if let Ok(c) = parse_hex(&s) {
        return Ok(c);
    }

    Err(Error::new(ErrorKind::InvalidUnknown, s))
}

pub fn parse_gradient(s: &str, file_path: Option<&str>) -> Result<Gradient> {
    if !s.starts_with("gradient(") {
        return Err(Error::new(ErrorKind::InvalidGradient, s));
    }

    let binding = strip_string(s.to_string(), &["gradient("], ')');

    let base_pattern = format!(
        r"(?i){}|{}|{}|{}",
        HEX_PATTERN, RGBA_PATTERN, HSLA_PATTERN, ACCENT_TRANSPARENT_PATTERN
    );

    let mut color_regex: Regex = Regex::new(base_pattern.as_str()).unwrap();

    #[cfg(feature = "theme")]
    {
        if let Some(file_path) = file_path {
            if let Ok(theme_data) = parse_custom_theme(file_path) {
                // Get the keys from the theme_data (assuming it's a map-like structure)
                let theme_keys: Vec<_> = theme_data.colors();
                let escaped_keys: Vec<String> = theme_keys
                    .iter()
                    .map(|key| key.replace('.', r"\."))
                    .collect();

                // Join the keys into a single pattern string separated by "|"
                let theme_pattern_base = escaped_keys.join("|");
                let theme_pattern = format!(r"\b(?:{})\b", theme_pattern_base);

                // If `named-colors` is also enabled, include NAMED_COLOR_PATTERN as well
                #[cfg(feature = "named-colors")]
                {
                    color_regex = Regex::new(
                        format!(
                            r"(?i){}|{}|{}",
                            base_pattern, theme_pattern, NAMED_COLOR_PATTERN
                        )
                        .as_str(),
                    )
                    .unwrap();
                }

                // If only `theme` is enabled (no `named-colors`), use just `theme_pattern`
                #[cfg(not(feature = "named-colors"))]
                {
                    color_regex =
                        Regex::new(format!(r"(?i){}|{}", base_pattern, theme_pattern).as_str())
                            .unwrap();
                }
            }
        }
    }

    #[cfg(feature = "named-colors")]
    #[cfg(not(feature = "theme"))]
    {
        color_regex =
            Regex::new(format!(r"(?i){}|{}", base_pattern, NAMED_COLOR_PATTERN).as_str()).unwrap();
    }

    let color_matches = color_regex
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
        .filter_map(|&color| parse_solid(color, file_path).ok()) // Only keep Ok values
        .collect::<Vec<Solid>>();

    let direction = GradientCoordinates::try_from(direction.as_str())?;

    Ok(Gradient { direction, colors })
}

pub fn parse(s: &str, file_path: Option<&str>) -> Result<Color> {
    if s.starts_with("gradient(") {
        parse_gradient(s, file_path).map(|res| Color(ColorValue::Gradient(res)))
    } else {
        parse_solid(s, file_path).map(|res| Color(ColorValue::Solid(res)))
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

fn parse_rgb_or_rgba(params: Vec<&str>, original_s: &str) -> Result<Solid> {
    if params.len() != 3 && params.len() != 4 {
        return Err(Error::new(ErrorKind::InvalidRgb, original_s));
    }

    let r = parse_percent_or_255(params[0]);
    let g = parse_percent_or_255(params[1]);
    let b = parse_percent_or_255(params[2]);
    let a = if params.len() == 4 {
        parse_percent_or_float(params[3])
    } else {
        Some((1.0, true))
    };

    if let (Some((r, r_fmt)), Some((g, g_fmt)), Some((b, b_fmt)), Some((a, _))) = (r, g, b, a) {
        if r_fmt == g_fmt && g_fmt == b_fmt {
            return Ok(Solid::new(
                r.clamp(0.0, 1.0),
                g.clamp(0.0, 1.0),
                b.clamp(0.0, 1.0),
                a.clamp(0.0, 1.0),
            ));
        }
    }

    Err(Error::new(ErrorKind::InvalidRgb, original_s))
}

fn parse_hsl_or_hsla(params: Vec<&str>, original_s: &str) -> Result<Solid> {
    if params.len() != 3 && params.len() != 4 {
        return Err(Error::new(ErrorKind::InvalidHsl, original_s));
    }

    let h = parse_angle(params[0]);
    let s = parse_percent_or_float(params[1]);
    let l = parse_percent_or_float(params[2]);
    let a = if params.len() == 4 {
        parse_percent_or_float(params[3])
    } else {
        Some((1.0, true))
    };

    if let (Some(h), Some((s, s_fmt)), Some((l, l_fmt)), Some((a, _))) = (h, s, l, a) {
        if s_fmt == l_fmt {
            return Ok(Solid::from_normalized_hsla(h, s, l, a));
        }
    }

    Err(Error::new(ErrorKind::InvalidHsl, original_s))
}

fn parse_percent_or_float(s: &str) -> Option<(f32, bool)> {
    match s.strip_suffix('%') {
        Some(num) => num.parse().ok().map(|t: f32| (t / 100.0, true)),
        None => s.parse().ok().map(|t| (t, false)),
    }
}

fn parse_percent_or_255(s: &str) -> Option<(f32, bool)> {
    match s.strip_suffix('%') {
        Some(num) => num.parse().ok().map(|t: f32| (t / 100.0, true)),
        None => s.parse().ok().map(|t: f32| (t / 255.0, false)),
    }
}

fn parse_angle(s: &str) -> Option<f32> {
    if let Some(s) = s.strip_suffix("deg") {
        return s.parse().ok();
    }
    if let Some(s) = s.strip_suffix("grad") {
        return s.parse::<f32>().ok().map(|t| t * 360.0 / 400.0);
    }
    if let Some(s) = s.strip_suffix("rad") {
        return s.parse::<f32>().ok().map(|t| t.to_degrees());
    }
    if let Some(s) = s.strip_suffix("turn") {
        return s.parse::<f32>().ok().map(|t| t * 360.0);
    }
    
    s.parse().ok()
}

#[cfg(feature = "theme")]
fn parse_custom_theme(file_path: &str) -> Result<Theme> {
    let mut full_path = PathBuf::from(file_path).clean();

    // If the path is relative, join it with the current working directory
    if full_path.is_relative() {
        let cwd =
            current_dir().map_err(|e| Error::new(ErrorKind::InvalidUnknown, format!("{:?}", e)))?;
        full_path = cwd.join(full_path).clean(); // Fix here: Update full_path
    }

    let current_metadata = metadata(&full_path)
        .map_err(|e| Error::new(ErrorKind::InvalidUnknown, format!("{:?}", e)))?;
    let current_modified = current_metadata
        .modified()
        .map_err(|e| Error::new(ErrorKind::InvalidUnknown, format!("{:?}", e)))?;

    // If the cache exists and the path has not changed, check the modification timestamp.
    if let Some((cached_path, theme_data, cached_time)) = THEME_CACHE.read().unwrap().as_ref() {
        if cached_path == &full_path.to_string_lossy().into_owned()
            && current_modified != *cached_time
        {
            // If path matches and the file has been modified, reload and update the cache.
            return Ok(theme_data.clone());
        }
    }

    // If no cache or path changed, reload the theme.
    reload_and_cache_theme(full_path)
}

#[cfg(feature = "theme")]
fn reload_and_cache_theme(file_path: PathBuf) -> Result<Theme> {
    let path = file_path.as_path();
    let contents = read_to_string(path)
        .map_err(|e| Error::new(ErrorKind::InvalidUnknown, format!("{:?}", e)))?;

    let theme = Theme::parse_theme(contents.as_str()).unwrap();

    let current_metadata = std::fs::metadata(path)
        .map_err(|e| Error::new(ErrorKind::InvalidUnknown, format!("{:?}", e)))?;
    let current_modified = current_metadata
        .modified()
        .map_err(|e| Error::new(ErrorKind::InvalidUnknown, format!("{:?}", e)))?;

    // Cache the theme with the path, theme content, and last modified timestamp
    let mut cache = THEME_CACHE.write().unwrap();
    let path_string = path.to_string_lossy().into_owned();
    *cache = Some((path_string, theme.clone(), current_modified));

    Ok(theme)
}
