//! # Overview
//!
//! Rust library for parsing CSS color string as defined in the W3C's [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/).
//!
//! ## Supported Color Format
//!
//! * [Named colors](https://www.w3.org/TR/css-color-4/#named-colors)
//! * RGB hexadecimal (with and without `#` prefix)
//!      + Short format `#rgb`
//!      + Short format with alpha `#rgba`
//!      + Long format `#rrggbb`
//!      + Long format with alpha `#rrggbbaa`
//! * `rgb()` and `rgba()`
//! * `hsl()` and `hsla()`
//! * `gradient()`
//!
//! ### Example Color Format
//!
//! <details>
//! <summary>Click to expand!</summary>
//!
//! ```text
//! transparent
//! gold
//! rebeccapurple
//! lime
//! accent
//! accent_inactive
//! #0f0
//! #0f0f
//! #00ff00
//! #00ff00ff
//! rgb(0,255,0)
//! rgb(0% 100% 0%)
//! rgb(0 255 0 / 100%)
//! rgba(0,255,0,1)
//! hsl(120,100%,50%)
//! hsl(120deg 100% 50%)
//! hsl(-240 100% 50%)
//! hsl(-240deg 100% 50%)
//! hsl(0.3333turn 100% 50%)
//! hsl(133.333grad 100% 50%)
//! hsl(2.0944rad 100% 50%)
//! hsla(120,100%,50%,100%)
//! gradient(rgb(0, 255, 0), #0f0, to right)
//! ```
//! </details>
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`
//!
//! ```toml
//! colorparser_css = "0.1.0"
//! ```
//!
//! ## Default Feature
//!
//! * `named-colors`: Enables parsing from [named colors](https://www.w3.org/TR/css-color-4/#named-colors). Requires [`rustc-hash`](https://crates.io/crates/rustc-hash).
//! * `theme`: Enables to add custom theme.
//!
//! ## Optional Features
//!
//! * `serde`: Enables serializing (into HEX string) and deserializing (from any supported string color format) using [`serde`](https://serde.rs/) framework.

mod color;
mod colorspace;
mod error;
mod gradient;
mod parser;
mod solid;
#[cfg(feature = "theme")]
mod theme;
mod utils;

pub use color::Color;
pub use color::ColorValue;
pub use colorspace::ColorspaceImpl;
pub use colorspace::Hsla;
pub use colorspace::NormalizedHsla;
pub use colorspace::NormalizedRgba;
pub use colorspace::Rgba;
pub use colorspace::Rgba16;
pub use error::Error;
pub use error::ErrorKind;
pub use error::Result;
pub use gradient::Gradient;
pub use gradient::GradientCoordinates;
pub use parser::parse;
pub use solid::Solid;

#[cfg(feature = "named-colors")]
pub use parser::NAMED_COLORS;

#[cfg(feature = "theme")]
pub use theme::{Theme, ThemeValue};
