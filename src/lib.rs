// `csscolors`: A very fast CSS color parser for Rust.
// Copyright: 2020, Joonas Javanainen <joonas.javanainen@gmail.com>
// License: MIT/Apache-2.0

//! `bigcolor`: Advanced color manipulation library for Rust.
//!
//! Inspired by TinyColor.js with extended features like CMYK support, blending modes, 
//! color interpolation, and support for CSS4 color functions.
//!
//! # Examples
//!
//! ```
//! # use bigcolor::BigColor;
//! # use std::str::FromStr;
//! assert_eq!("#abc", BigColor::from_str("#abc").unwrap().to_hex_string());
//! assert_eq!("#aabbcc", BigColor::from_str("#aabbcc").unwrap().to_hex_string());
//! assert_eq!("#11223344", BigColor::from_str("#11223344").unwrap().to_hex_string());
//! assert_eq!("rgb(1,2,3)", BigColor::from_str("rgb(1, 2, 3)").unwrap().to_rgb_string());
//! assert_eq!("rgba(1,2,3,0.5)", BigColor::from_str("rgba(1, 2, 3, 0.5)").unwrap().to_rgb_string());
//! assert_eq!("hsl(1, 2%, 3%)", BigColor::from_str("hsl(1, 2%, 3%)").unwrap().to_hsl_string());
//! assert_eq!("hsla(1, 2%, 3%, 0.5)", BigColor::from_str("hsla(1, 2%, 3%, 0.5)").unwrap().to_hsl_string());
//! assert_eq!("#000000", BigColor::from_str("black").unwrap().to_hex_string());
//! ```

#[cfg(feature = "lab")]
extern crate lab;

#[cfg(feature = "lab")]
extern crate palette;

mod parser;
mod big_color;
mod solid;
mod gradient;

#[cfg(feature = "cint")]
mod cint;

pub use parser::{ParseColorError, NAMED_COLORS};
pub use big_color::BigColor;
pub use solid::SolidColor;
pub use gradient::{Gradient, ColorStop, GradientType, GradientExtend};
