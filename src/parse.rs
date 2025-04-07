// Color parsing functions
// Ported from tinycolor.js

use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use crate::color_space::*;
use crate::ColorFormat;

/// RGB color input result
#[derive(Debug, Clone)]
pub struct RGBInput {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
    pub ok: bool,
    pub format: ColorFormat,
}

impl Default for RGBInput {
    fn default() -> Self {
        RGBInput {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
            ok: false,
            format: ColorFormat::INVALID,
        }
    }
}

/// Given a string or object, convert that input to RGB
/// Possible string inputs:
///
///     "red"
///     "#f00" or "f00"
///     "#ff0000" or "ff0000"
///     "#ff000000" or "ff000000"
///     "rgb 255 0 0" or "rgb (255, 0, 0)"
///     "rgb 1.0 0 0" or "rgb (1, 0, 0)"
///     "rgba (255, 0, 0, 1)" or "rgba 255, 0, 0, 1"
///     "rgba (1.0, 0, 0, 1)" or "rgba 1.0, 0, 0, 1"
///     "hsl(0, 100%, 50%)" or "hsl 0 100% 50%"
///     "hsla(0, 100%, 50%, 1)" or "hsla 0 100% 50%, 1"
///     "hsv(0, 100%, 100%)" or "hsv 0 100% 100%"
///     "lab(50, 50, 0)" or "lab 50 50 0"
///     "lch(50, 50, 0)" or "lch 50 50 0"
///     "oklab(50%, 0.1, 0.1)" or "oklab 50% 0.1 0.1"
///     "oklch(50%, 0.1, 0)" or "oklch 50% 0.1 0"
///     "cmyk(0%, 0%, 0%, 0%)" or "cmyk 0% 0% 0% 0%"
///     "cmyk(100%, 100%, 100%, 100%)" or "cmyk 100% 100% 100% 100%"
pub fn input_to_rgb(color: &str) -> RGBInput {
    let mut rgb = RGBInput::default();

    // Check if it's a color name
    if let Some(color_obj) = string_input_to_object(color) {
        match color_obj {
            ColorInput::RGB(r, g, b) => {
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.ok = true;
                rgb.format = ColorFormat::RGB;
            },
            ColorInput::RGBA(r, g, b, a) => {
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::RGB;
            },
            ColorInput::HSL(h, s, l) => {
                let rgb_val = hsl_to_rgb(h, s, l);
                rgb.r = rgb_val.r;
                rgb.g = rgb_val.g;
                rgb.b = rgb_val.b;
                rgb.ok = true;
                rgb.format = ColorFormat::HSL;
            },
            ColorInput::HSLA(h, s, l, a) => {
                let rgb_val = hsl_to_rgb(h, s, l);
                rgb.r = rgb_val.r;
                rgb.g = rgb_val.g;
                rgb.b = rgb_val.b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::HSL;
            },
            ColorInput::HSV(h, s, v) => {
                let rgb_val = hsv_to_rgb(h, s, v);
                rgb.r = rgb_val.r;
                rgb.g = rgb_val.g;
                rgb.b = rgb_val.b;
                rgb.ok = true;
                rgb.format = ColorFormat::HSV;
            },
            ColorInput::HSVA(h, s, v, a) => {
                let rgb_val = hsv_to_rgb(h, s, v);
                rgb.r = rgb_val.r;
                rgb.g = rgb_val.g;
                rgb.b = rgb_val.b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::HSV;
            },
            ColorInput::HEX(r, g, b) => {
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.ok = true;
                rgb.format = ColorFormat::HEX;
            },
            ColorInput::HEX8(r, g, b, a) => {
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::HEX8;
            },
            ColorInput::NAME(r, g, b) => {
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.ok = true;
                rgb.format = ColorFormat::NAME;
            },
            ColorInput::LAB(l, a, b, alpha) => {
                let lab = Lab { l, a, b, alpha };
                let xyz_d50 = lab_to_xyz_d50(lab);
                let xyz_d65 = xyz_d50_to_xyz_d65(xyz_d50);
                let (r, g, b, a) = xyz_d65_to_rgb(xyz_d65);
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::LAB;
            },
            ColorInput::LCH(l, c, h, alpha) => {
                let lch = LCH { l, c, h, alpha };
                let (r, g, b, a) = lch_to_rgb(lch);
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::LCH;
            },
            ColorInput::OKLAB(l, a, b, alpha) => {
                let oklab = OKLab { l, a, b, alpha };
                let (r, g, b, a) = oklab_to_rgb(oklab);
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::OKLAB;
            },
            ColorInput::OKLCH(l, c, h, alpha) => {
                let oklch = OKLCH { l, c, h, alpha };
                let (r, g, b, a) = oklch_to_rgb(oklch);
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::OKLCH;
            },
            ColorInput::CMYK(c, m, y, k, alpha) => {
                let cmyk = CMYK { c, m, y, k, a: alpha };
                let (r, g, b, a) = cmyk_to_rgb(cmyk);
                rgb.r = r;
                rgb.g = g;
                rgb.b = b;
                rgb.a = a;
                rgb.ok = true;
                rgb.format = ColorFormat::CMYK;
            }
        }
    }

    // Make sure RGB values are clamped to [0, 255]
    rgb.r = rgb.r.min(255).max(0);
    rgb.g = rgb.g.min(255).max(0);
    rgb.b = rgb.b.min(255).max(0);
    
    // Don't allow invalid alpha values
    rgb.a = bound_alpha(rgb.a);

    rgb
}

/// Enum for different color input formats
#[derive(Debug, Clone)]
pub enum ColorInput {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, f32),
    HSL(f32, f32, f32),
    HSLA(f32, f32, f32, f32),
    HSV(f32, f32, f32),
    HSVA(f32, f32, f32, f32),
    HEX(u8, u8, u8),
    HEX8(u8, u8, u8, f32),
    NAME(u8, u8, u8),
    LAB(f32, f32, f32, f32),
    LCH(f32, f32, f32, f32),
    OKLAB(f32, f32, f32, f32),
    OKLCH(f32, f32, f32, f32),
    CMYK(f32, f32, f32, f32, f32),
}

/// Parse a string input into a ColorInput object
fn string_input_to_object(color: &str) -> Option<ColorInput> {
    let color = color.trim().to_lowercase();
    
    // Check for named colors first
    if let Some(hex) = names().get(&color) {
        if let Some(rgb) = parse_hex(hex) {
            return Some(ColorInput::NAME(rgb.0, rgb.1, rgb.2));
        }
    }

    // Check for transparent
    if color == "transparent" {
        return Some(ColorInput::RGBA(0, 0, 0, 0.0));
    }

    // Try to match using regex patterns
    lazy_static! {
        static ref HEX_3: Regex = Regex::new(r"^#?([0-9a-f]{1})([0-9a-f]{1})([0-9a-f]{1})$").unwrap();
        static ref HEX_6: Regex = Regex::new(r"^#?([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$").unwrap();
        static ref HEX_4: Regex = Regex::new(r"^#?([0-9a-f]{1})([0-9a-f]{1})([0-9a-f]{1})([0-9a-f]{1})$").unwrap();
        static ref HEX_8: Regex = Regex::new(r"^#?([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$").unwrap();
        static ref RGB: Regex = Regex::new(r"^rgb\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$").unwrap();
        static ref RGB_PERCENT: Regex = Regex::new(r"^rgb\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)$").unwrap();
        static ref RGBA: Regex = Regex::new(r"^rgba\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*([01]?\.?\d*)\s*\)$").unwrap();
        static ref RGBA_PERCENT: Regex = Regex::new(r"^rgba\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*([01]?\.?\d*)\s*\)$").unwrap();
        static ref HSL: Regex = Regex::new(r"^hsl\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)$").unwrap();
        static ref HSL_SPACE: Regex = Regex::new(r"^(\d+(?:\.\d+)?)\s+(\d+(?:\.\d+)?)%\s+(\d+(?:\.\d+)?)%$").unwrap();
        static ref HSLA: Regex = Regex::new(r"^hsla\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*([01]?\.?\d*)\s*\)$").unwrap();
        static ref HSV: Regex = Regex::new(r"^hsv\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*\)$").unwrap();
        static ref HSVA: Regex = Regex::new(r"^hsva\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*,\s*([01]?\.?\d*)\s*\)$").unwrap();
        static ref HSB: Regex = Regex::new(r"^hsb\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*\)$").unwrap();
        static ref HSBA: Regex = Regex::new(r"^hsba\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*,\s*([01]?\.?\d*)\s*\)$").unwrap();
        static ref LAB: Regex = Regex::new(r"^lab\s*\(\s*(\d+(?:\.\d+)?)\s*,?\s*(-?\d+(?:\.\d+)?)\s*,?\s*(-?\d+(?:\.\d+)?)\s*(?:,\s*([01]?\.?\d+))?\s*\)$").unwrap();
        static ref LAB_WITH_SLASH: Regex = Regex::new(r"^lab\s*\(\s*(\d+(?:\.\d+)?)\s*,?\s*(-?\d+(?:\.\d+)?)\s*,?\s*(-?\d+(?:\.\d+)?)\s*/\s*([01]?\.?\d+)\s*\)$").unwrap();
        static ref LCH: Regex = Regex::new(r"^lch\s*\(\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*(?:,\s*([01]?\.?\d+))?\s*\)$").unwrap();
        static ref LCH_WITH_SLASH: Regex = Regex::new(r"^lch\s*\(\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*/\s*([01]?\.?\d+)\s*\)$").unwrap();
        static ref OKLAB: Regex = Regex::new(r"^oklab\s*\(\s*(\d+(?:\.\d+)?)%\s*,?\s*(-?\d+(?:\.\d+)?)\s*,?\s*(-?\d+(?:\.\d+)?)\s*(?:,\s*([01]?\.?\d+))?\s*\)$").unwrap();
        static ref OKLAB_WITH_SLASH: Regex = Regex::new(r"^oklab\s*\(\s*(\d+(?:\.\d+)?)%\s*,?\s*(-?\d+(?:\.\d+)?)\s*,?\s*(-?\d+(?:\.\d+)?)\s*/\s*([01]?\.?\d+)\s*\)$").unwrap();
        static ref OKLCH: Regex = Regex::new(r"^oklch\s*\(\s*(\d+(?:\.\d+)?)%\s*,?\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*(?:,\s*([01]?\.?\d+))?\s*\)$").unwrap();
        static ref OKLCH_WITH_SLASH: Regex = Regex::new(r"^oklch\s*\(\s*(\d+(?:\.\d+)?)%\s*,?\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*/\s*([01]?\.?\d+)\s*\)$").unwrap();
        static ref OKLCH_DECIMAL: Regex = Regex::new(r"^oklch\s*\(\s*(\d*\.?\d+)\s+(\d*\.?\d+)\s+(\d+(?:\.\d+)?)\s*(?:,\s*([01]?\.?\d+))?\s*\)$").unwrap();
        static ref OKLCH_DECIMAL_WITH_SLASH: Regex = Regex::new(r"^oklch\s*\(\s*(\d*\.?\d+)\s+(\d*\.?\d+)\s+(\d+(?:\.\d+)?)\s*/\s*([01]?\.?\d+)\s*\)$").unwrap();
        static ref CMYK: Regex = Regex::new(r"^cmyk\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*(?:,\s*([01]?\.?\d+))?\s*\)$").unwrap();
        static ref CMYK_WITH_SLASH: Regex = Regex::new(r"^cmyk\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*/\s*([01]?\.?\d+)\s*\)$").unwrap();
    }

    // Try to match hex formats
    if let Some(caps) = HEX_3.captures(&color) {
        let r = caps.get(1).map_or("", |m| m.as_str());
        let g = caps.get(2).map_or("", |m| m.as_str());
        let b = caps.get(3).map_or("", |m| m.as_str());
        
        let r = u8::from_str_radix(&format!("{}{}", r, r), 16).unwrap_or(0);
        let g = u8::from_str_radix(&format!("{}{}", g, g), 16).unwrap_or(0);
        let b = u8::from_str_radix(&format!("{}{}", b, b), 16).unwrap_or(0);
        
        return Some(ColorInput::HEX(r, g, b));
    }
    
    if let Some(caps) = HEX_6.captures(&color) {
        let r = caps.get(1).map_or("", |m| m.as_str());
        let g = caps.get(2).map_or("", |m| m.as_str());
        let b = caps.get(3).map_or("", |m| m.as_str());
        
        let r = u8::from_str_radix(r, 16).unwrap_or(0);
        let g = u8::from_str_radix(g, 16).unwrap_or(0);
        let b = u8::from_str_radix(b, 16).unwrap_or(0);
        
        return Some(ColorInput::HEX(r, g, b));
    }
    
    if let Some(caps) = HEX_4.captures(&color) {
        let r = caps.get(1).map_or("", |m| m.as_str());
        let g = caps.get(2).map_or("", |m| m.as_str());
        let b = caps.get(3).map_or("", |m| m.as_str());
        let a = caps.get(4).map_or("", |m| m.as_str());
        
        let r = u8::from_str_radix(&format!("{}{}", r, r), 16).unwrap_or(0);
        let g = u8::from_str_radix(&format!("{}{}", g, g), 16).unwrap_or(0);
        let b = u8::from_str_radix(&format!("{}{}", b, b), 16).unwrap_or(0);
        let a = u8::from_str_radix(&format!("{}{}", a, a), 16).unwrap_or(0) as f32 / 255.0;
        
        return Some(ColorInput::HEX8(r, g, b, a));
    }
    
    if let Some(caps) = HEX_8.captures(&color) {
        let r = caps.get(1).map_or("", |m| m.as_str());
        let g = caps.get(2).map_or("", |m| m.as_str());
        let b = caps.get(3).map_or("", |m| m.as_str());
        let a = caps.get(4).map_or("", |m| m.as_str());
        
        let r = u8::from_str_radix(r, 16).unwrap_or(0);
        let g = u8::from_str_radix(g, 16).unwrap_or(0);
        let b = u8::from_str_radix(b, 16).unwrap_or(0);
        let a = u8::from_str_radix(a, 16).unwrap_or(0) as f32 / 255.0;
        
        return Some(ColorInput::HEX8(r, g, b, a));
    }
    
    // Try to match RGB formats
    if let Some(caps) = RGB.captures(&color) {
        let r = caps.get(1).map_or("0", |m| m.as_str()).parse::<u8>().unwrap_or(0);
        let g = caps.get(2).map_or("0", |m| m.as_str()).parse::<u8>().unwrap_or(0);
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<u8>().unwrap_or(0);
        
        return Some(ColorInput::RGB(r, g, b));
    }
    
    if let Some(caps) = RGB_PERCENT.captures(&color) {
        let r_pct = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let g_pct = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let b_pct = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        
        let r = (r_pct * 2.55).round() as u8;
        let g = (g_pct * 2.55).round() as u8;
        let b = (b_pct * 2.55).round() as u8;
        
        return Some(ColorInput::RGB(r, g, b));
    }
    
    if let Some(caps) = RGBA.captures(&color) {
        let r = caps.get(1).map_or("0", |m| m.as_str()).parse::<u8>().unwrap_or(0);
        let g = caps.get(2).map_or("0", |m| m.as_str()).parse::<u8>().unwrap_or(0);
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<u8>().unwrap_or(0);
        let a = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::RGBA(r, g, b, a));
    }
    
    if let Some(caps) = RGBA_PERCENT.captures(&color) {
        let r_pct = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let g_pct = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let b_pct = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let a = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        let r = (r_pct * 2.55).round() as u8;
        let g = (g_pct * 2.55).round() as u8;
        let b = (b_pct * 2.55).round() as u8;
        
        return Some(ColorInput::RGBA(r, g, b, a));
    }
    
    // Try to match HSL formats
    if let Some(caps) = HSL.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let l = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        
        return Some(ColorInput::HSL(h / 360.0, s, l));
    }
    
    // Match space-separated HSL format
    if let Some(caps) = HSL_SPACE.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let l = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        
        return Some(ColorInput::HSL(h / 360.0, s, l));
    }
    
    if let Some(caps) = HSLA.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let l = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let a = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::HSLA(h / 360.0, s, l, a));
    }
    
    // Try to match HSV formats
    if let Some(caps) = HSV.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let v = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        
        return Some(ColorInput::HSV(h / 360.0, s, v));
    }
    
    if let Some(caps) = HSVA.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let v = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let a = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::HSVA(h / 360.0, s, v, a));
    }
    
    // Try to match LAB formats
    if let Some(caps) = LAB.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let a = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::LAB(l, a, b, alpha));
    }
    
    if let Some(caps) = LAB_WITH_SLASH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let a = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::LAB(l, a, b, alpha));
    }
    
    // Try to match LCH formats
    if let Some(caps) = LCH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let c = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let h = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::LCH(l, c, h, alpha));
    }
    
    if let Some(caps) = LCH_WITH_SLASH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let c = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let h = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::LCH(l, c, h, alpha));
    }
    
    // Try to match OKLab formats
    if let Some(caps) = OKLAB.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let a = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::OKLAB(l, a, b, alpha));
    }
    
    if let Some(caps) = OKLAB_WITH_SLASH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let a = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::OKLAB(l, a, b, alpha));
    }
    
    // Try to match OKLCH formats
    if let Some(caps) = OKLCH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let c = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let h = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::OKLCH(l, c, h, alpha));
    }
    
    if let Some(caps) = OKLCH_WITH_SLASH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let c = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let h = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::OKLCH(l, c, h, alpha));
    }
    
    if let Some(caps) = OKLCH_DECIMAL.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let c = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let h = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::OKLCH(l, c, h, alpha));
    }
    
    if let Some(caps) = OKLCH_DECIMAL_WITH_SLASH.captures(&color) {
        let l = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let c = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let h = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::OKLCH(l, c, h, alpha));
    }
    
    // Try to match CMYK formats
    if let Some(caps) = CMYK.captures(&color) {
        let c = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let m = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let y = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let k = caps.get(4).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(5).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::CMYK(c, m, y, k, alpha));
    }
    
    if let Some(caps) = CMYK_WITH_SLASH.captures(&color) {
        let c = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let m = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let y = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let k = caps.get(4).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let alpha = caps.get(5).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::CMYK(c, m, y, k, alpha));
    }
    
    // Try to match HSB formats
    if let Some(caps) = HSB.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        
        return Some(ColorInput::HSV(h / 360.0, s, b));
    }
    
    if let Some(caps) = HSBA.captures(&color) {
        let h = caps.get(1).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0);
        let s = caps.get(2).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let b = caps.get(3).map_or("0", |m| m.as_str()).parse::<f32>().unwrap_or(0.0) / 100.0;
        let a = caps.get(4).map_or("1.0", |m| m.as_str()).parse::<f32>().unwrap_or(1.0);
        
        return Some(ColorInput::HSVA(h / 360.0, s, b, a));
    }
    
    None
}

/// Helper function to parse hex values
fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&format!("{}{}", &hex[0..1], &hex[0..1]), 16).ok()?;
            let g = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).ok()?;
            let b = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).ok()?;
            Some((r, g, b))
        },
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        },
        _ => None,
    }
}

/// Color names map (CSS/SVG color names)
pub fn names() -> &'static HashMap<String, &'static str> {
    lazy_static! {
        static ref NAMES: HashMap<String, &'static str> = {
            let mut m = HashMap::new();
            m.insert("aliceblue".to_string(), "f0f8ff");
            m.insert("antiquewhite".to_string(), "faebd7");
            m.insert("aqua".to_string(), "0ff");
            m.insert("aquamarine".to_string(), "7fffd4");
            m.insert("azure".to_string(), "f0ffff");
            m.insert("beige".to_string(), "f5f5dc");
            m.insert("bisque".to_string(), "ffe4c4");
            m.insert("black".to_string(), "000");
            m.insert("blanchedalmond".to_string(), "ffebcd");
            m.insert("blue".to_string(), "00f");
            m.insert("blueviolet".to_string(), "8a2be2");
            m.insert("brown".to_string(), "a52a2a");
            m.insert("burlywood".to_string(), "deb887");
            m.insert("burntsienna".to_string(), "ea7e5d");
            m.insert("cadetblue".to_string(), "5f9ea0");
            m.insert("chartreuse".to_string(), "7fff00");
            m.insert("chocolate".to_string(), "d2691e");
            m.insert("coral".to_string(), "ff7f50");
            m.insert("cornflowerblue".to_string(), "6495ed");
            m.insert("cornsilk".to_string(), "fff8dc");
            m.insert("crimson".to_string(), "dc143c");
            m.insert("cyan".to_string(), "0ff");
            m.insert("darkblue".to_string(), "00008b");
            m.insert("darkcyan".to_string(), "008b8b");
            m.insert("darkgoldenrod".to_string(), "b8860b");
            m.insert("darkgray".to_string(), "a9a9a9");
            m.insert("darkgreen".to_string(), "006400");
            m.insert("darkgrey".to_string(), "a9a9a9");
            m.insert("darkkhaki".to_string(), "bdb76b");
            m.insert("darkmagenta".to_string(), "8b008b");
            m.insert("darkolivegreen".to_string(), "556b2f");
            m.insert("darkorange".to_string(), "ff8c00");
            m.insert("darkorchid".to_string(), "9932cc");
            m.insert("darkred".to_string(), "8b0000");
            m.insert("darksalmon".to_string(), "e9967a");
            m.insert("darkseagreen".to_string(), "8fbc8f");
            m.insert("darkslateblue".to_string(), "483d8b");
            m.insert("darkslategray".to_string(), "2f4f4f");
            m.insert("darkslategrey".to_string(), "2f4f4f");
            m.insert("darkturquoise".to_string(), "00ced1");
            m.insert("darkviolet".to_string(), "9400d3");
            m.insert("deeppink".to_string(), "ff1493");
            m.insert("deepskyblue".to_string(), "00bfff");
            m.insert("dimgray".to_string(), "696969");
            m.insert("dimgrey".to_string(), "696969");
            m.insert("dodgerblue".to_string(), "1e90ff");
            m.insert("firebrick".to_string(), "b22222");
            m.insert("floralwhite".to_string(), "fffaf0");
            m.insert("forestgreen".to_string(), "228b22");
            m.insert("fuchsia".to_string(), "f0f");
            m.insert("gainsboro".to_string(), "dcdcdc");
            m.insert("ghostwhite".to_string(), "f8f8ff");
            m.insert("gold".to_string(), "ffd700");
            m.insert("goldenrod".to_string(), "daa520");
            m.insert("gray".to_string(), "808080");
            m.insert("green".to_string(), "008000");
            m.insert("greenyellow".to_string(), "adff2f");
            m.insert("grey".to_string(), "808080");
            m.insert("honeydew".to_string(), "f0fff0");
            m.insert("hotpink".to_string(), "ff69b4");
            m.insert("indianred".to_string(), "cd5c5c");
            m.insert("indigo".to_string(), "4b0082");
            m.insert("ivory".to_string(), "fffff0");
            m.insert("khaki".to_string(), "f0e68c");
            m.insert("lavender".to_string(), "e6e6fa");
            m.insert("lavenderblush".to_string(), "fff0f5");
            m.insert("lawngreen".to_string(), "7cfc00");
            m.insert("lemonchiffon".to_string(), "fffacd");
            m.insert("lightblue".to_string(), "add8e6");
            m.insert("lightcoral".to_string(), "f08080");
            m.insert("lightcyan".to_string(), "e0ffff");
            m.insert("lightgoldenrodyellow".to_string(), "fafad2");
            m.insert("lightgray".to_string(), "d3d3d3");
            m.insert("lightgreen".to_string(), "90ee90");
            m.insert("lightgrey".to_string(), "d3d3d3");
            m.insert("lightpink".to_string(), "ffb6c1");
            m.insert("lightsalmon".to_string(), "ffa07a");
            m.insert("lightseagreen".to_string(), "20b2aa");
            m.insert("lightskyblue".to_string(), "87cefa");
            m.insert("lightslategray".to_string(), "789");
            m.insert("lightslategrey".to_string(), "789");
            m.insert("lightsteelblue".to_string(), "b0c4de");
            m.insert("lightyellow".to_string(), "ffffe0");
            m.insert("lime".to_string(), "0f0");
            m.insert("limegreen".to_string(), "32cd32");
            m.insert("linen".to_string(), "faf0e6");
            m.insert("magenta".to_string(), "f0f");
            m.insert("maroon".to_string(), "800000");
            m.insert("mediumaquamarine".to_string(), "66cdaa");
            m.insert("mediumblue".to_string(), "0000cd");
            m.insert("mediumorchid".to_string(), "ba55d3");
            m.insert("mediumpurple".to_string(), "9370db");
            m.insert("mediumseagreen".to_string(), "3cb371");
            m.insert("mediumslateblue".to_string(), "7b68ee");
            m.insert("mediumspringgreen".to_string(), "00fa9a");
            m.insert("mediumturquoise".to_string(), "48d1cc");
            m.insert("mediumvioletred".to_string(), "c71585");
            m.insert("midnightblue".to_string(), "191970");
            m.insert("mintcream".to_string(), "f5fffa");
            m.insert("mistyrose".to_string(), "ffe4e1");
            m.insert("moccasin".to_string(), "ffe4b5");
            m.insert("navajowhite".to_string(), "ffdead");
            m.insert("navy".to_string(), "000080");
            m.insert("oldlace".to_string(), "fdf5e6");
            m.insert("olive".to_string(), "808000");
            m.insert("olivedrab".to_string(), "6b8e23");
            m.insert("orange".to_string(), "ffa500");
            m.insert("orangered".to_string(), "ff4500");
            m.insert("orchid".to_string(), "da70d6");
            m.insert("palegoldenrod".to_string(), "eee8aa");
            m.insert("palegreen".to_string(), "98fb98");
            m.insert("paleturquoise".to_string(), "afeeee");
            m.insert("palevioletred".to_string(), "db7093");
            m.insert("papayawhip".to_string(), "ffefd5");
            m.insert("peachpuff".to_string(), "ffdab9");
            m.insert("peru".to_string(), "cd853f");
            m.insert("pink".to_string(), "ffc0cb");
            m.insert("plum".to_string(), "dda0dd");
            m.insert("powderblue".to_string(), "b0e0e6");
            m.insert("purple".to_string(), "800080");
            m.insert("rebeccapurple".to_string(), "663399");
            m.insert("red".to_string(), "f00");
            m.insert("rosybrown".to_string(), "bc8f8f");
            m.insert("royalblue".to_string(), "4169e1");
            m.insert("saddlebrown".to_string(), "8b4513");
            m.insert("salmon".to_string(), "fa8072");
            m.insert("sandybrown".to_string(), "f4a460");
            m.insert("seagreen".to_string(), "2e8b57");
            m.insert("seashell".to_string(), "fff5ee");
            m.insert("sienna".to_string(), "a0522d");
            m.insert("silver".to_string(), "c0c0c0");
            m.insert("skyblue".to_string(), "87ceeb");
            m.insert("slateblue".to_string(), "6a5acd");
            m.insert("slategray".to_string(), "708090");
            m.insert("slategrey".to_string(), "708090");
            m.insert("snow".to_string(), "fffafa");
            m.insert("springgreen".to_string(), "00ff7f");
            m.insert("steelblue".to_string(), "4682b4");
            m.insert("tan".to_string(), "d2b48c");
            m.insert("teal".to_string(), "008080");
            m.insert("thistle".to_string(), "d8bfd8");
            m.insert("tomato".to_string(), "ff6347");
            m.insert("turquoise".to_string(), "40e0d0");
            m.insert("violet".to_string(), "ee82ee");
            m.insert("wheat".to_string(), "f5deb3");
            m.insert("white".to_string(), "fff");
            m.insert("whitesmoke".to_string(), "f5f5f5");
            m.insert("yellow".to_string(), "ff0");
            m.insert("yellowgreen".to_string(), "9acd32");
            m
        };
    }
    &NAMES
}

/// Hex to name mapping
pub fn hex_names() -> &'static HashMap<String, &'static str> {
    lazy_static! {
        static ref HEX_NAMES: HashMap<String, &'static str> = {
            let mut m = HashMap::new();
            for (name, hex) in names().iter() {
                m.insert(hex.to_string(), name.as_str());
            }
            m
        };
    }
    &HEX_NAMES
} 