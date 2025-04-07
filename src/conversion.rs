use crate::BigColor;
use peniko::Color;

/// Converts a BigColor to a peniko::Color
/// 
/// This allows interoperability with the peniko color library and its
/// ecosystem, which is useful for graphics applications.
pub fn to_peniko_color(color: &BigColor) -> Color {
    let rgb = color.to_rgb();
    Color::from_rgba8(rgb.r, rgb.g, rgb.b, (rgb.a * 255.0) as u8)
}

/// Converts a peniko::Color to a BigColor
/// 
/// This allows importing colors from the peniko ecosystem into BigColor
/// for advanced color manipulation.
pub fn from_peniko_color(color: &Color) -> BigColor {
    let rgba = color.to_rgba8();
    BigColor::from_rgb(rgba.r, rgba.g, rgba.b, rgba.a as f32 / 255.0)
} 