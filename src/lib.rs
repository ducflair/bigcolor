// Port of tinycolor.js to Rust with OKLCH foundation
// Original: https://github.com/bgrins/TinyColor
// Brian Grinstead, MIT License

mod matrix_utils;
pub mod color_space;
mod parse;
pub mod conversion;
pub mod accessibility;

use std::fmt;
use color_space::*;
use parse::*;
use crate::accessibility::{get_contrast_color as get_contrast_color_impl, get_contrast_ratio as get_contrast_ratio_impl};
pub use peniko;

/// BigColor struct represents a color with various formats
/// Using OKLCH as the foundation
#[derive(Debug, Clone)]
pub struct BigColor {
    // OKLCH values serve as the foundation
    oklch: OKLCH,
    original_input: String,
    format: ColorFormat,
    ok: bool,
}

/// Format types for color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFormat {
    RGB,
    PRGB,
    HEX,
    HEX3,
    HEX6,
    HEX8,
    HSL,
    HSV,
    HSB,
    LAB,
    LCH,
    OKLAB,
    OKLCH,
    CMYK,
    NAME,
    INVALID,
}

impl PartialEq for BigColor {
    fn eq(&self, other: &Self) -> bool {
        self.oklch.l == other.oklch.l &&
        self.oklch.c == other.oklch.c &&
        self.oklch.h == other.oklch.h &&
        self.oklch.alpha == other.oklch.alpha
    }
}

impl Default for BigColor {
    fn default() -> Self {
        BigColor {
            oklch: OKLCH {
                l: 0.0,
                c: 0.0,
                h: 0.0,
                alpha: 1.0,
            },
            original_input: String::new(),
            format: ColorFormat::INVALID,
            ok: false,
        }
    }
}

impl BigColor {
    /// Create a new BigColor instance from various inputs
    pub fn new<T: Into<String>>(color: T) -> Self {
        let color_str = color.into();
        if color_str.is_empty() {
            return BigColor::default();
        }

        // If input is already a BigColor, return a copy
        // (In Rust, we'll just handle this with the Clone trait)

        let rgb = input_to_rgb(&color_str);
        
        if !rgb.ok {
            return BigColor::default();
        }
        
        // Convert RGB to OKLCH to set our foundation
        let oklch = rgb_to_oklch(rgb.r, rgb.g, rgb.b, rgb.a);
        
        BigColor {
            oklch,
            original_input: color_str,
            format: rgb.format,
            ok: rgb.ok,
        }
    }

    /// Alternative constructor for compatibility with old API
    pub fn from_string<T: Into<String>>(input: T) -> Result<Self, String> {
        let color = Self::new(input);
        if color.is_valid() {
            Ok(color)
        } else {
            Err(format!("Invalid color: {}", color.get_original_input()))
        }
    }

    /// Returns true if the color is dark
    pub fn is_dark(&self) -> bool {
        // With OKLCH, we can use L directly to determine darkness
        // Values less than 0.5 are generally considered dark
        self.oklch.l < 0.5
    }

    /// Returns true if the color is light
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    /// Returns true if the color is valid
    pub fn is_valid(&self) -> bool {
        self.ok
    }

    /// Returns the original input
    pub fn get_original_input(&self) -> &str {
        &self.original_input
    }

    /// Returns the format of the color
    pub fn get_format(&self) -> ColorFormat {
        self.format
    }

    /// Returns the alpha value
    pub fn get_alpha(&self) -> f32 {
        self.oklch.alpha
    }

    /// Returns the brightness value
    pub fn get_brightness(&self) -> f32 {
        // In OKLCH, we can use L directly but normalize to 0-255 range
        self.oklch.l * 255.0
    }

    /// Returns the luminance value
    pub fn get_luminance(&self) -> f32 {
        // Convert to sRGB and calculate standard luminance
        let rgb = self.to_rgb();
        let r_srgb = rgb.r as f32 / 255.0;
        let g_srgb = rgb.g as f32 / 255.0;
        let b_srgb = rgb.b as f32 / 255.0;

        let r = if r_srgb <= 0.03928 {
            r_srgb / 12.92
        } else {
            ((r_srgb + 0.055) / 1.055).powf(2.4)
        };
        
        let g = if g_srgb <= 0.03928 {
            g_srgb / 12.92
        } else {
            ((g_srgb + 0.055) / 1.055).powf(2.4)
        };
        
        let b = if b_srgb <= 0.03928 {
            b_srgb / 12.92
        } else {
            ((b_srgb + 0.055) / 1.055).powf(2.4)
        };

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Sets the alpha value
    pub fn set_alpha(&mut self, value: f32) -> &mut Self {
        self.oklch.alpha = bound_alpha(value);
        self
    }

    /// Converts the color to HSV
    pub fn to_hsv(&self) -> HSV {
        // Convert to RGB first, then to HSV
        let (r, g, b, _) = oklch_to_rgb(self.oklch);
        let hsv = rgb_to_hsv(r, g, b);
        HSV {
            h: hsv.h * 360.0,
            s: hsv.s,
            v: hsv.v,
            a: self.oklch.alpha,
        }
    }

    /// Converts the color to HSV string
    pub fn to_hsv_string(&self) -> String {
        let hsv = self.to_hsv();
        let h = hsv.h.round() as i32;
        let s = (hsv.s * 100.0).round() as i32;
        let v = (hsv.v * 100.0).round() as i32;
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("hsv({}, {}%, {}%)", h, s, v)
        } else {
            format!("hsva({}, {}%, {}%, {})", h, s, v, (self.oklch.alpha * 100.0).round() / 100.0)
        }
    }

    /// Converts the color to HSL
    pub fn to_hsl(&self) -> HSL {
        // Convert to RGB first, then manually calculate HSL
        let (r, g, b, _) = oklch_to_rgb(self.oklch);
        
        let r_norm = r as f32 / 255.0;
        let g_norm = g as f32 / 255.0;
        let b_norm = b as f32 / 255.0;
        
        let max = r_norm.max(g_norm).max(b_norm);
        let min = r_norm.min(g_norm).min(b_norm);
        let mut h = 0.0;
        let mut s = 0.0;
        let l = (max + min) / 2.0;
        
        if max != min {
            let d = max - min;
            s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
            
            if max == r_norm {
                h = (g_norm - b_norm) / d + (if g_norm < b_norm { 6.0 } else { 0.0 });
            } else if max == g_norm {
                h = (b_norm - r_norm) / d + 2.0;
            } else if max == b_norm {
                h = (r_norm - g_norm) / d + 4.0;
            }
            
            h /= 6.0;
        }
        
        HSL {
            h: h * 360.0,
            s,
            l,
            a: self.oklch.alpha,
        }
    }

    /// Converts the color to HSL string
    pub fn to_hsl_string(&self) -> String {
        let hsl = self.to_hsl();
        let h = hsl.h.round() as i32;
        let s = (hsl.s * 100.0).round() as i32;
        let l = (hsl.l * 100.0).round() as i32;
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("hsl({}, {}%, {}%)", h, s, l)
        } else {
            format!("hsla({}, {}%, {}%, {})", h, s, l, (self.oklch.alpha * 100.0).round() / 100.0)
        }
    }

    /// Converts the color to OKLCH
    pub fn to_oklch(&self) -> OKLCH {
        self.oklch
    }

    /// Converts the color to OKLCH string
    pub fn to_oklch_string(&self) -> String {
        let l = (self.oklch.l * 100.0).round() / 100.0;
        let c = (self.oklch.c * 100.0).round() / 100.0;
        let h = self.oklch.h.round();
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("oklch({}% {} {})", l * 100.0, c, h)
        } else {
            format!("oklch({}% {} {} / {})", l * 100.0, c, h, self.oklch.alpha)
        }
    }

    /// Converts the color to OKLab
    pub fn to_oklab(&self) -> OKLab {
        oklch_to_oklab(self.oklch)
    }

    /// Converts the color to OKLab string
    pub fn to_oklab_string(&self) -> String {
        let oklab = self.to_oklab();
        let l = (oklab.l * 100.0).round() / 100.0;
        let a = (oklab.a * 100.0).round() / 100.0;
        let b = (oklab.b * 100.0).round() / 100.0;
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("oklab({}% {} {})", l * 100.0, a, b)
        } else {
            format!("oklab({}% {} {} / {})", l * 100.0, a, b, self.oklch.alpha)
        }
    }

    /// Converts the color to LCH
    pub fn to_lch(&self) -> LCH {
        let (r, g, b, _) = oklch_to_rgb(self.oklch);
        rgb_to_lch(r, g, b, self.oklch.alpha)
    }

    /// Converts the color to LCH string
    pub fn to_lch_string(&self) -> String {
        let lch = self.to_lch();
        let l = lch.l.round();
        let c = lch.c.round();
        let h = lch.h.round();
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("lch({} {} {})", l, c, h)
        } else {
            format!("lch({} {} {} / {})", l, c, h, self.oklch.alpha)
        }
    }

    /// Converts the color to Lab
    pub fn to_lab(&self) -> Lab {
        let (r, g, b, _) = oklch_to_rgb(self.oklch);
        let xyz_d65 = rgb_to_xyz_d65(r, g, b, self.oklch.alpha);
        let xyz_d50 = xyz_d65_to_xyz_d50(xyz_d65);
        xyz_d50_to_lab(xyz_d50)
    }

    /// Converts the color to Lab string
    pub fn to_lab_string(&self) -> String {
        let lab = self.to_lab();
        let l = lab.l.round();
        let a = lab.a.round();
        let b = lab.b.round();
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("lab({} {} {})", l, a, b)
        } else {
            format!("lab({} {} {} / {})", l, a, b, self.oklch.alpha)
        }
    }

    /// Converts the color to HEX
    pub fn to_hex(&self, allow_3_char: bool) -> String {
        let (r, g, b, _) = oklch_to_rgb(self.oklch);
        rgb_to_hex(r, g, b, allow_3_char)
    }

    /// Converts the color to HEX string
    pub fn to_hex_string(&self, allow_3_char: bool) -> String {
        format!("#{}", self.to_hex(allow_3_char))
    }

    /// Converts the color to HEX8
    pub fn to_hex8(&self, allow_4_char: bool) -> String {
        let (r, g, b, _) = oklch_to_rgb(self.oklch);
        rgba_to_hex(r, g, b, self.oklch.alpha, allow_4_char)
    }

    /// Converts the color to HEX8 string
    pub fn to_hex8_string(&self, allow_4_char: bool) -> String {
        format!("#{}", self.to_hex8(allow_4_char))
    }

    /// Converts the color to RGB
    pub fn to_rgb(&self) -> RGB {
        let (r, g, b, a) = oklch_to_rgb(self.oklch);
        RGB {
            r,
            g,
            b,
            a,
        }
    }

    /// Converts the color to RGB string
    pub fn to_rgb_string(&self) -> String {
        let rgb = self.to_rgb();
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b)
        } else {
            format!("rgba({}, {}, {}, {})", rgb.r, rgb.g, rgb.b, (rgb.a * 100.0).round() / 100.0)
        }
    }

    /// Converts the color to percentage RGB
    pub fn to_percentage_rgb(&self) -> PercentageRGB {
        let rgb = self.to_rgb();
        PercentageRGB {
            r: (bound_01(rgb.r as f32, 255.0) * 100.0).round(),
            g: (bound_01(rgb.g as f32, 255.0) * 100.0).round(),
            b: (bound_01(rgb.b as f32, 255.0) * 100.0).round(),
            a: rgb.a,
        }
    }

    /// Converts the color to percentage RGB string
    pub fn to_percentage_rgb_string(&self) -> String {
        let prgb = self.to_percentage_rgb();
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("rgb({}%, {}%, {}%)", prgb.r, prgb.g, prgb.b)
        } else {
            format!("rgba({}%, {}%, {}%, {})", prgb.r, prgb.g, prgb.b, (prgb.a * 100.0).round() / 100.0)
        }
    }

    /// Converts the color to a name if possible
    pub fn to_name(&self) -> Option<&'static str> {
        if self.oklch.alpha == 0.0 {
            Some("transparent")
        } else if self.oklch.alpha < 1.0 {
            None
        } else {
            let (r, g, b, _) = oklch_to_rgb(self.oklch);
            let hex = rgb_to_hex(r, g, b, true);
            hex_names().get(&hex).map(|&name| name)
        }
    }

    /// Converts the color to a string format
    pub fn to_string(&self, format: Option<ColorFormat>) -> String {
        let format = format.unwrap_or(self.format);
        
        let has_alpha = self.oklch.alpha < 1.0 && self.oklch.alpha >= 0.0;
        let needs_alpha_format = 
            format == ColorFormat::HEX || 
            format == ColorFormat::HEX6 || 
            format == ColorFormat::HEX3 || 
            format == ColorFormat::HEX8 || 
            format == ColorFormat::NAME;
        
        if has_alpha && needs_alpha_format {
            // Special case for "transparent", all other non-alpha formats
            // will return rgba when there is transparency
            if format == ColorFormat::NAME && self.oklch.alpha == 0.0 {
                if let Some(name) = self.to_name() {
                    return name.to_string();
                }
            }
            return self.to_rgb_string();
        }
        
        match format {
            ColorFormat::RGB => self.to_rgb_string(),
            ColorFormat::PRGB => self.to_percentage_rgb_string(),
            ColorFormat::HEX | ColorFormat::HEX6 => self.to_hex_string(false),
            ColorFormat::HEX3 => self.to_hex_string(true),
            ColorFormat::HEX8 => self.to_hex8_string(false),
            ColorFormat::NAME => {
                if let Some(name) = self.to_name() {
                    name.to_string()
                } else {
                    self.to_hex_string(false)
                }
            },
            ColorFormat::HSL => self.to_hsl_string(),
            ColorFormat::HSV => self.to_hsv_string(),
            ColorFormat::HSB => self.to_hsb_string(),
            ColorFormat::LAB => self.to_lab_string(),
            ColorFormat::LCH => self.to_lch_string(),
            ColorFormat::OKLAB => self.to_oklab_string(),
            ColorFormat::OKLCH => self.to_oklch_string(),
            ColorFormat::CMYK => self.to_cmyk_string(),
            _ => self.to_hex_string(false),
        }
    }

    /// Creates a clone of the color
    pub fn clone_color(&self) -> Self {
        BigColor::new(self.to_string(None))
    }

    /// Lightens the color
    pub fn lighten(&mut self, amount: Option<f32>) -> &mut Self {
        let amount = amount.unwrap_or(10.0);
        // Direct manipulation in OKLCH space
        self.oklch.l = (self.oklch.l + amount / 100.0).min(1.0).max(0.0);
        self
    }

    /// Brightens the color
    pub fn brighten(&mut self, amount: Option<f32>) -> &mut Self {
        let amount = amount.unwrap_or(10.0);
        // Convert to RGB, apply brightening, then back to OKLCH
        let (r, g, b, a) = oklch_to_rgb(self.oklch);
        let r_new = (r as f32 - (255.0 * -(amount / 100.0))).round().max(0.0).min(255.0) as u8;
        let g_new = (g as f32 - (255.0 * -(amount / 100.0))).round().max(0.0).min(255.0) as u8;
        let b_new = (b as f32 - (255.0 * -(amount / 100.0))).round().max(0.0).min(255.0) as u8;
        
        self.oklch = rgb_to_oklch(r_new, g_new, b_new, a);
        self
    }

    /// Darkens the color
    pub fn darken(&mut self, amount: Option<f32>) -> &mut Self {
        let amount = amount.unwrap_or(10.0);
        // Direct manipulation in OKLCH space
        self.oklch.l = (self.oklch.l - amount / 100.0).min(1.0).max(0.0);
        self
    }

    /// Desaturates the color
    pub fn desaturate(&mut self, amount: Option<f32>) -> &mut Self {
        let amount = amount.unwrap_or(10.0);
        // Direct manipulation in OKLCH space
        self.oklch.c = (self.oklch.c - amount / 100.0).max(0.0);
        self
    }

    /// Saturates the color
    pub fn saturate(&mut self, amount: Option<f32>) -> &mut Self {
        let amount = amount.unwrap_or(10.0);
        // Direct manipulation in OKLCH space
        self.oklch.c = self.oklch.c + amount / 100.0;
        self
    }

    /// Converts the color to grayscale
    pub fn greyscale(&mut self) -> &mut Self {
        // Direct manipulation in OKLCH space - set chroma to 0
        self.oklch.c = 0.0;
        self
    }

    /// Spins the hue of the color
    pub fn spin(&mut self, amount: f32) -> &mut Self {
        // Direct manipulation in OKLCH space
        let mut h = (self.oklch.h + amount) % 360.0;
        if h < 0.0 {
            h += 360.0;
        }
        self.oklch.h = h;
        self
    }

    /// Creates analogous colors
    pub fn analogous(&self, results: Option<usize>, slices: Option<usize>) -> Vec<BigColor> {
        let results = results.unwrap_or(6);
        let slices = slices.unwrap_or(30);
        
        let part = 360.0 / slices as f32;
        let mut ret = vec![self.clone()];
        
        let mut h = (self.oklch.h - ((part * results as f32) / 2.0) + 720.0) % 360.0;
        for _ in 0..results-1 {
            h = (h + part) % 360.0;
            let mut new_color = self.clone();
            new_color.oklch.h = h;
            ret.push(new_color);
        }
        
        ret
    }

    /// Creates a complement color
    pub fn complement(&self) -> BigColor {
        let mut new_color = self.clone();
        new_color.oklch.h = (new_color.oklch.h + 180.0) % 360.0;
        new_color
    }

    /// Creates monochromatic colors
    pub fn monochromatic(&self, results: Option<usize>) -> Vec<BigColor> {
        let results = results.unwrap_or(6);
        let step = 1.0 / results as f32;
        
        let mut ret = Vec::new();
        let mut l: f32 = 0.0;
        
        for _ in 0..results {
            let mut new_color = self.clone();
            new_color.oklch.l = l.min(1.0);
            ret.push(new_color);
            l += step;
        }
        
        ret
    }

    /// Creates split complement colors
    pub fn split_complement(&self) -> Vec<BigColor> {
        let h = self.oklch.h;
        
        let mut color1 = self.clone();
        color1.oklch.h = (h + 72.0) % 360.0;
        
        let mut color2 = self.clone();
        color2.oklch.h = (h + 216.0) % 360.0;
        
        vec![self.clone(), color1, color2]
    }

    /// Creates a triad of colors
    pub fn triad(&self) -> Vec<BigColor> {
        self.polyad(3)
    }

    /// Creates a tetrad of colors
    pub fn tetrad(&self) -> Vec<BigColor> {
        self.polyad(4)
    }

    /// Creates polyad colors
    pub fn polyad(&self, number: usize) -> Vec<BigColor> {
        if number == 0 {
            panic!("Argument to polyad must be a positive number");
        }
        
        let mut result = vec![self.clone()];
        let step = 360.0 / number as f32;
        
        for i in 1..number {
            let mut new_color = self.clone();
            new_color.oklch.h = (self.oklch.h + i as f32 * step) % 360.0;
            result.push(new_color);
        }
        
        result
    }

    // Factory methods for creating a BigColor from different formats
    
    /// Creates a BigColor from RGB values
    pub fn from_rgb(r: u8, g: u8, b: u8, a: f32) -> Self {
        let input = format!("rgba({},{},{},{})", r, g, b, a);
        BigColor::new(input)
    }
    
    /// Creates a BigColor from HSL values
    pub fn from_hsl(h: f32, s: f32, l: f32, a: f32) -> Self {
        // Use the hsl_to_rgb function directly
        let rgb = hsl_to_rgb(h/360.0, s, l);
        BigColor::from_rgb(rgb.r, rgb.g, rgb.b, a)
    }
    
    /// Creates a BigColor from HSV values
    pub fn from_hsv(h: f32, s: f32, v: f32, a: f32) -> Self {
        // Convert HSV directly to RGB
        let rgb = hsv_to_rgb(h/360.0, s, v);
        BigColor::from_rgb(rgb.r, rgb.g, rgb.b, a)
    }
    
    /// Creates a BigColor from LCH values
    pub fn from_lch(l: f32, c: f32, h: f32, a: f32) -> Self {
        let lch = LCH { l, c, h, alpha: a };
        let (r, g, b, a) = lch_to_rgb(lch);
        BigColor::from_rgb(r, g, b, a)
    }
    
    /// Creates a BigColor from OKLCH values directly
    pub fn from_oklch(l: f32, c: f32, h: f32, a: f32) -> Self {
        let oklch = OKLCH { l, c, h, alpha: a };
        let mut color = BigColor::default();
        color.oklch = oklch;
        color.ok = true;
        color.format = ColorFormat::OKLCH;
        color
    }
    
    /// Creates a BigColor from a ratio
    pub fn from_ratio(color: &str) -> Self {
        // This is a simplified version that just passes through to new
        // In a full implementation, you'd parse the ratio properly
        BigColor::new(color)
    }

    /// Converts the color to HSB (same as HSV)
    pub fn to_hsb(&self) -> HSV {
        self.to_hsv()
    }

    /// Converts the color to HSB string
    pub fn to_hsb_string(&self) -> String {
        let hsv = self.to_hsv();
        let h = hsv.h.round() as i32;
        let s = (hsv.s * 100.0).round() as i32;
        let b = (hsv.v * 100.0).round() as i32;
        
        if (self.oklch.alpha - 1.0).abs() < f32::EPSILON {
            format!("hsb({}, {}%, {}%)", h, s, b)
        } else {
            format!("hsba({}, {}%, {}%, {})", h, s, b, (self.oklch.alpha * 100.0).round() / 100.0)
        }
    }

    /// Creates a BigColor from HSB values (same as HSV)
    pub fn from_hsb(h: f32, s: f32, b: f32, a: f32) -> Self {
        Self::from_hsv(h, s, b, a)
    }

    /// Converts the color to CMYK
    pub fn to_cmyk(&self) -> CMYK {
        let rgb = self.to_rgb();
        rgb_to_cmyk(rgb.r, rgb.g, rgb.b, rgb.a)
    }
    
    /// Converts the color to CMYK string
    pub fn to_cmyk_string(&self) -> String {
        let cmyk = self.to_cmyk();
        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            cmyk.c.round() as i32,
            cmyk.m.round() as i32,
            cmyk.y.round() as i32,
            cmyk.k.round() as i32
        )
    }
    
    /// Create a new BigColor from CMYK values
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self {
        let (r, g, b, a) = cmyk_to_rgb(CMYK { c, m, y, k, a });
        let oklch = rgb_to_oklch(r, g, b, a);
        
        BigColor {
            oklch,
            original_input: format!("cmyk({}%, {}%, {}%, {}%)", c, m, y, k),
            format: ColorFormat::CMYK,
            ok: true,
        }
    }

    /// Returns the color as a CSS-compatible string in the specified format
    pub fn to(&self, format: ColorFormat) -> String {
        if !self.is_valid() {
            return String::from("invalid");
        }

        match format {
            ColorFormat::HEX => self.to_hex_string(false),
            ColorFormat::HEX3 => self.to_hex_string(true),
            ColorFormat::HEX6 => self.to_hex_string(false),
            ColorFormat::HEX8 => self.to_hex8_string(false),
            ColorFormat::RGB => self.to_rgb_string(),
            ColorFormat::PRGB => self.to_percentage_rgb_string(),
            ColorFormat::HSL => self.to_hsl_string(),
            ColorFormat::HSV => self.to_hsv_string(),
            ColorFormat::HSB => self.to_hsb_string(),
            ColorFormat::OKLAB => self.to_oklab_string(),
            ColorFormat::OKLCH => self.to_oklch_string(),
            ColorFormat::LAB => self.to_lab_string(),
            ColorFormat::LCH => self.to_lch_string(),
            ColorFormat::CMYK => self.to_cmyk_string(),
            ColorFormat::NAME => self.to_name().unwrap_or(&self.original_input).to_string(),
            ColorFormat::INVALID => String::from("invalid"),
        }
    }

    /// Gets a contrast color with a specified intensity (0.0 to 1.0)
    /// 
    /// The intensity parameter controls how strong the contrast will be:
    /// - 0.0: Minimum contrast (slight difference)
    /// - 0.5: Medium contrast
    /// - 1.0: Maximum contrast (black or white)
    pub fn get_contrast_color(&self, intensity: f32) -> BigColor {
        get_contrast_color_impl(self, intensity)
    }

    /// Gets the contrast ratio between this color and another color
    /// according to WCAG standards. The ratio ranges from 1:1 (no contrast)
    /// to 21:1 (maximum contrast).
    pub fn get_contrast_ratio(&self, other: &BigColor) -> f32 {
        get_contrast_ratio_impl(self, other)
    }
}

/// Creates a random color
pub fn random() -> BigColor {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    BigColor::from_rgb(
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        1.0
    )
}

/// Checks if two colors are equal
pub fn equals(color1: &BigColor, color2: &BigColor) -> bool {
    color1.to_rgb_string() == color2.to_rgb_string()
}

/// Mixes two colors
pub fn mix(color1: &BigColor, color2: &BigColor, amount: Option<f32>) -> BigColor {
    let amount = amount.unwrap_or(50.0);
    
    let rgb1 = color1.to_rgb();
    let rgb2 = color2.to_rgb();
    
    let p = amount / 100.0;
    
    let r = (rgb2.r as f32 - rgb1.r as f32) * p + rgb1.r as f32;
    let g = (rgb2.g as f32 - rgb1.g as f32) * p + rgb1.g as f32;
    let b = (rgb2.b as f32 - rgb1.b as f32) * p + rgb1.b as f32;
    let a = (rgb2.a - rgb1.a) * p + rgb1.a;
    
    BigColor::from_rgb(r as u8, g as u8, b as u8, a)
}

/// Analyzes the readability between two colors
pub fn readability(color1: &BigColor, color2: &BigColor) -> f32 {
    let l1 = color1.get_luminance();
    let l2 = color2.get_luminance();
    
    let max = l1.max(l2);
    let min = l1.min(l2);
    
    (max + 0.05) / (min + 0.05)
}

/// WCAG2 parameters
#[derive(Debug, Clone, Copy)]
pub struct WCAG2Params {
    pub level: WCAG2Level,
    pub size: WCAG2Size,
}

/// WCAG2 levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WCAG2Level {
    AA,
    AAA,
}

/// WCAG2 sizes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WCAG2Size {
    Small,
    Large,
}

impl Default for WCAG2Params {
    fn default() -> Self {
        WCAG2Params {
            level: WCAG2Level::AA,
            size: WCAG2Size::Small,
        }
    }
}

/// Validates WCAG2 parameters
fn validate_wcag2_params(params: Option<WCAG2Params>) -> WCAG2Params {
    params.unwrap_or_default()
}

/// Checks if a color is readable against another color
pub fn is_readable(color1: &BigColor, color2: &BigColor, wcag2: Option<WCAG2Params>) -> bool {
    let readability_value = readability(color1, color2);
    let wcag2_params = validate_wcag2_params(wcag2);
    
    match (wcag2_params.level, wcag2_params.size) {
        (WCAG2Level::AA, WCAG2Size::Small) | (WCAG2Level::AAA, WCAG2Size::Large) => {
            readability_value >= 4.5
        },
        (WCAG2Level::AA, WCAG2Size::Large) => {
            readability_value >= 3.0
        },
        (WCAG2Level::AAA, WCAG2Size::Small) => {
            readability_value >= 7.0
        },
    }
}

/// Arguments for the most_readable function
#[derive(Debug, Clone, Copy)]
pub struct MostReadableArgs {
    pub include_fallback_colors: bool,
    pub wcag2: WCAG2Params,
}

impl Default for MostReadableArgs {
    fn default() -> Self {
        MostReadableArgs {
            include_fallback_colors: false,
            wcag2: WCAG2Params::default(),
        }
    }
}

/// Finds the most readable color against a base color
pub fn most_readable(
    base_color: &BigColor,
    color_list: &[BigColor],
    args: Option<MostReadableArgs>,
) -> BigColor {
    let args = args.unwrap_or_default();
    
    let mut best_color = None;
    let mut best_score = 0.0;
    
    for color in color_list {
        let readability_value = readability(base_color, color);
        if readability_value > best_score {
            best_score = readability_value;
            best_color = Some(color.clone());
        }
    }
    
    if let Some(best) = best_color {
        if is_readable(base_color, &best, Some(args.wcag2)) || !args.include_fallback_colors {
            best
        } else {
            // Create white and black colors for fallback
            let white = BigColor::new("#fff");
            let black = BigColor::new("#000");
            
            let mut args_without_fallback = args;
            args_without_fallback.include_fallback_colors = false;
            
            most_readable(base_color, &[white, black], Some(args_without_fallback))
        }
    } else {
        // Fallback to black if no colors provided
        BigColor::new("#000")
    }
}

// Common display implementation for BigColor to allow string conversion
impl fmt::Display for BigColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string(None))
    }
}

