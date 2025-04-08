// Color space conversion functions
// Ported from tinycolor.js

use crate::matrix_utils::*;
use std::f32::consts::PI;

/// RGB color
#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

/// HSL color
#[derive(Debug, Clone, Copy)]
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

/// HSV color
#[derive(Debug, Clone, Copy)]
pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

/// Percentage RGB color
#[derive(Debug, Clone, Copy)]
pub struct PercentageRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// XYZ D65 color space
#[derive(Debug, Clone, Copy)]
pub struct XyzD65 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
}

/// XYZ D50 color space
#[derive(Debug, Clone, Copy)]
pub struct XyzD50 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
}

/// Lab color space
#[derive(Debug, Clone, Copy)]
pub struct Lab {
    pub l: f32, // Lightness: 0-100
    pub a: f32, // a axis: -125 to 125
    pub b: f32, // b axis: -125 to 125
    pub alpha: f32,
}

/// LCH color space
#[derive(Debug, Clone, Copy)]
pub struct LCH {
    pub l: f32, // Lightness: 0-100
    pub c: f32, // Chroma: 0-150+
    pub h: f32, // Hue: 0-360 degrees
    pub alpha: f32,
}

/// OKLab color space
#[derive(Debug, Clone, Copy)]
pub struct OKLab {
    pub l: f32, // Lightness: 0-1
    pub a: f32, // a axis: -0.4 to 0.4
    pub b: f32, // b axis: -0.4 to 0.4
    pub alpha: f32,
}

/// OKLCH color space
#[derive(Debug, Clone, Copy)]
pub struct OKLCH {
    pub l: f32, // Lightness: 0-1
    pub c: f32, // Chroma: 0-0.4+
    pub h: f32, // Hue: 0-360 degrees
    pub alpha: f32,
}

/// CMYK color model
#[derive(Debug, Clone, Copy)]
pub struct CMYK {
    pub c: f32, // Cyan: 0-100%
    pub m: f32, // Magenta: 0-100%
    pub y: f32, // Yellow: 0-100%
    pub k: f32, // Key (Black): 0-100%
    pub a: f32, // Alpha: 0-1
}

/// Converts an RGB color to RGB
/// Ensures proper bounds and handling of percentages
/// Assumes r, g, b in [0, 255] or [0, 1]
/// Returns { r, g, b } in [0, 255]
pub fn rgb_to_rgb(r: u8, g: u8, b: u8) -> RGB {
    RGB {
        r: (bound_01(r as f32, 255.0) * 255.0) as u8,
        g: (bound_01(g as f32, 255.0) * 255.0) as u8,
        b: (bound_01(b as f32, 255.0) * 255.0) as u8,
        a: 1.0,
    }
}

/// Converts an RGB color to HSL
/// Assumes r, g, and b are contained in [0, 255] or [0, 1]
/// Returns { h, s, l } in [0,1]
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> HSL {
    let r = bound_01(r as f32, 255.0);
    let g = bound_01(g as f32, 255.0);
    let b = bound_01(b as f32, 255.0);

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let mut h = 0.0;
    let s;
    let l = (max + min) / 2.0;

    if max == min {
        h = 0.0; // achromatic
        s = 0.0;
    } else {
        let d = max - min;
        s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
        
        if max == r {
            h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
        } else if max == g {
            h = (b - r) / d + 2.0;
        } else if max == b {
            h = (r - g) / d + 4.0;
        }
        
        h /= 6.0;
    }

    HSL {
        h,
        s,
        l,
        a: 1.0,
    }
}

/// Converts an HSL color value to RGB
/// Assumes h is contained in [0, 1] or [0, 360] and s and l are contained [0, 1] or [0, 100]
/// Returns { r, g, b } in the set [0, 255]
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> RGB {
    // Normalize inputs to [0, 1]
    let h_norm = if h > 1.0 { h / 360.0 } else { h };
    let s_norm = if s > 1.0 { s / 100.0 } else { s };
    let l_norm = if l > 1.0 { l / 100.0 } else { l };
    
    let h = h_norm;
    let s = s_norm;
    let l = l_norm;

    let hue_to_rgb = |p: f32, q: f32, mut t: f32| -> f32 {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        
        if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0/2.0 { return q; }
        if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
        
        p
    };

    if s == 0.0 {
        // achromatic
        let rgb_val = (l * 255.0) as u8;
        return RGB {
            r: rgb_val,
            g: rgb_val,
            b: rgb_val,
            a: 1.0,
        };
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    
    let r = (hue_to_rgb(p, q, h + 1.0/3.0) * 255.0) as u8;
    let g = (hue_to_rgb(p, q, h) * 255.0) as u8;
    let b = (hue_to_rgb(p, q, h - 1.0/3.0) * 255.0) as u8;
    
    RGB {
        r,
        g,
        b,
        a: 1.0,
    }
}

/// Converts an RGB color value to HSV
/// Assumes r, g, and b are contained in the set [0, 255] or [0, 1]
/// Returns { h, s, v } in [0,1]
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> HSV {
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let mut h = 0.0;
    let s;
    let v = max;

    let d = max - min;
    s = if max == 0.0 { 0.0 } else { d / max };

    if max == min {
        h = 0.0; // achromatic
    } else {
        if max == r_norm {
            h = (g_norm - b_norm) / d + (if g_norm < b_norm { 6.0 } else { 0.0 });
        } else if max == g_norm {
            h = (b_norm - r_norm) / d + 2.0;
        } else if max == b_norm {
            h = (r_norm - g_norm) / d + 4.0;
        }
        
        h /= 6.0;
    }
    
    HSV {
        h,
        s,
        v,
        a: 1.0,
    }
}

/// Converts an HSV color value to RGB
/// Assumes h is contained in [0, 1] or [0, 360] and s and v are contained in [0, 1] or [0, 100]
/// Returns { r, g, b } in the set [0, 255]
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RGB {
    // Normalize inputs to [0, 1]
    let h_norm = if h > 1.0 { h / 360.0 } else { h };
    let s_norm = if s > 1.0 { s / 100.0 } else { s };
    let v_norm = if v > 1.0 { v / 100.0 } else { v };
    
    // Handle zero saturation case (achromatic - grayscale)
    if s_norm < 0.00001 {
        let v8 = (v_norm * 255.0).round() as u8;
        return RGB { r: v8, g: v8, b: v8, a: 1.0 };
    }
    
    // Convert to the [0, 6) range for the algorithm
    let h = h_norm * 6.0;
    
    // Calculate the primary color components
    let i = h.floor() as i32;
    let f = h - i as f32; // fractional part
    
    let p = v_norm * (1.0 - s_norm);
    let q = v_norm * (1.0 - f * s_norm);
    let t = v_norm * (1.0 - (1.0 - f) * s_norm);
    
    // Select the appropriate RGB values based on the hue sector
    let (r, g, b) = match i % 6 {
        0 => (v_norm, t, p),
        1 => (q, v_norm, p),
        2 => (p, v_norm, t),
        3 => (p, q, v_norm),
        4 => (t, p, v_norm),
        _ => (v_norm, p, q), // 5 or potentially negative values
    };
    
    // Convert to 8-bit RGB values
    RGB {
        r: (r * 255.0).round() as u8,
        g: (g * 255.0).round() as u8,
        b: (b * 255.0).round() as u8,
        a: 1.0,
    }
}

/// Converts an RGB color to a hex string
/// Assumes r, g, and b are contained in the set [0, 255]
/// Returns a 3 or 6 character hex
pub fn rgb_to_hex(r: u8, g: u8, b: u8, allow_3_char: bool) -> String {
    let hex = vec![
        format!("{:02x}", r),
        format!("{:02x}", g),
        format!("{:02x}", b),
    ];

    // Return a 3 character hex if possible
    if allow_3_char && 
       hex[0].chars().nth(0) == hex[0].chars().nth(1) &&
       hex[1].chars().nth(0) == hex[1].chars().nth(1) &&
       hex[2].chars().nth(0) == hex[2].chars().nth(1) {
        return format!("{}{}{}", 
            hex[0].chars().nth(0).unwrap(),
            hex[1].chars().nth(0).unwrap(),
            hex[2].chars().nth(0).unwrap()
        );
    }

    hex.join("")
}

/// Converts an RGBA color plus alpha transparency to hex
/// Assumes r, g, b are contained in the set [0, 255] and
/// a in [0, 1]. Returns a 4 or 8 character rgba hex
pub fn rgba_to_hex(r: u8, g: u8, b: u8, a: f32, allow_4_char: bool) -> String {
    let alpha_byte = (a * 255.0).round() as u8;
    
    let hex = vec![
        format!("{:02x}", r),
        format!("{:02x}", g),
        format!("{:02x}", b),
        format!("{:02x}", alpha_byte),
    ];

    // Return a 4 character hex if possible
    if allow_4_char && 
       hex[0].chars().nth(0) == hex[0].chars().nth(1) &&
       hex[1].chars().nth(0) == hex[1].chars().nth(1) &&
       hex[2].chars().nth(0) == hex[2].chars().nth(1) &&
       hex[3].chars().nth(0) == hex[3].chars().nth(1) {
        return format!("{}{}{}{}", 
            hex[0].chars().nth(0).unwrap(),
            hex[1].chars().nth(0).unwrap(),
            hex[2].chars().nth(0).unwrap(),
            hex[3].chars().nth(0).unwrap()
        );
    }

    hex.join("")
}

/// Converts an RGBA color to an ARGB Hex8 string
/// Rarely used, but required for "toFilter()"
pub fn rgba_to_argb_hex(r: u8, g: u8, b: u8, a: f32) -> String {
    let alpha_byte = (a * 255.0).round() as u8;
    
    let hex = vec![
        format!("{:02x}", alpha_byte),
        format!("{:02x}", r),
        format!("{:02x}", g),
        format!("{:02x}", b),
    ];

    hex.join("")
}

// Constants for Lab conversions
const EPSILON: f32 = 216.0 / 24389.0; // 6^3/29^3 == (24/116)^3
const EPSILON3: f32 = 24.0 / 116.0;
const KAPPA: f32 = 24389.0 / 27.0; // 29^3/3^3

/// Convert RGB to XYZ D65
pub fn rgb_to_xyz_d65(r: u8, g: u8, b: u8, a: f32) -> XyzD65 {
    // sRGB to linear RGB
    let r_linear = srgb_to_linear(r as f32 / 255.0);
    let g_linear = srgb_to_linear(g as f32 / 255.0);
    let b_linear = srgb_to_linear(b as f32 / 255.0);

    // Linear RGB to XYZ D65
    // sRGB uses D65 as reference white
    let xyz = [
        0.4124564 * r_linear + 0.3575761 * g_linear + 0.1804375 * b_linear,
        0.2126729 * r_linear + 0.7151522 * g_linear + 0.0721750 * b_linear,
        0.0193339 * r_linear + 0.1191920 * g_linear + 0.9503041 * b_linear,
    ];

    XyzD65 {
        x: xyz[0],
        y: xyz[1],
        z: xyz[2],
        a,
    }
}

/// Convert XYZ D65 to RGB
pub fn xyz_d65_to_rgb(xyz: XyzD65) -> (u8, u8, u8, f32) {
    let rgb_linear = [
        3.2404542 * xyz.x - 1.5371385 * xyz.y - 0.4985314 * xyz.z,
        -0.9692660 * xyz.x + 1.8760108 * xyz.y + 0.0415560 * xyz.z,
        0.0556434 * xyz.x - 0.2040259 * xyz.y + 1.0572252 * xyz.z,
    ];

    // Linear RGB to sRGB
    let r = (linear_to_srgb(rgb_linear[0]) * 255.0).round() as u8;
    let g = (linear_to_srgb(rgb_linear[1]) * 255.0).round() as u8;
    let b = (linear_to_srgb(rgb_linear[2]) * 255.0).round() as u8;

    (r, g, b, xyz.a)
}

/// Convert XYZ D65 to XYZ D50
pub fn xyz_d65_to_xyz_d50(xyz: XyzD65) -> XyzD50 {
    let xyz_vec = [xyz.x, xyz.y, xyz.z];
    let xyz_d50 = adapt_xyz(xyz_vec, WHITE_D65, WHITE_D50);
    
    XyzD50 {
        x: xyz_d50[0],
        y: xyz_d50[1],
        z: xyz_d50[2],
        a: xyz.a,
    }
}

/// Convert XYZ D50 to XYZ D65
pub fn xyz_d50_to_xyz_d65(xyz: XyzD50) -> XyzD65 {
    let xyz_vec = [xyz.x, xyz.y, xyz.z];
    let xyz_d65 = adapt_xyz(xyz_vec, WHITE_D50, WHITE_D65);
    
    XyzD65 {
        x: xyz_d65[0],
        y: xyz_d65[1],
        z: xyz_d65[2],
        a: xyz.a,
    }
}

/// Convert XYZ D50 to Lab
pub fn xyz_d50_to_lab(xyz: XyzD50) -> Lab {
    // Scale relative to D50 white point
    let xyz_rel = [
        xyz.x / WHITE_D50[0],
        xyz.y / WHITE_D50[1],
        xyz.z / WHITE_D50[2],
    ];
    
    // Apply non-linear transformation
    let f = xyz_rel.map(|value| {
        if value > EPSILON {
            value.cbrt()
        } else {
            (KAPPA * value + 16.0) / 116.0
        }
    });
    
    let l = 116.0 * f[1] - 16.0;
    let a = 500.0 * (f[0] - f[1]);
    let b = 200.0 * (f[1] - f[2]);
    
    Lab {
        l,
        a,
        b,
        alpha: xyz.a,
    }
}

/// Convert Lab to XYZ D50
pub fn lab_to_xyz_d50(lab: Lab) -> XyzD50 {
    // Calculate f
    let f1 = (lab.l + 16.0) / 116.0;
    let f0 = lab.a / 500.0 + f1;
    let f2 = f1 - lab.b / 200.0;
    
    // Calculate xyz relative to white
    let xyz_rel = [
        if f0 > EPSILON3 { f0.powi(3) } else { (116.0 * f0 - 16.0) / KAPPA },
        if lab.l > 8.0 { ((lab.l + 16.0) / 116.0).powi(3) } else { lab.l / KAPPA },
        if f2 > EPSILON3 { f2.powi(3) } else { (116.0 * f2 - 16.0) / KAPPA },
    ];
    
    // Scale by white point
    let x = xyz_rel[0] * WHITE_D50[0];
    let y = xyz_rel[1] * WHITE_D50[1];
    let z = xyz_rel[2] * WHITE_D50[2];
    
    XyzD50 {
        x,
        y,
        z,
        a: lab.alpha,
    }
}

/// Convert Lab to LCH
pub fn lab_to_lch(lab: Lab) -> LCH {
    let c = (lab.a.powi(2) + lab.b.powi(2)).sqrt();
    
    // Calculate hue angle, handling special cases
    let h = if lab.a.abs() < 1e-10 && lab.b.abs() < 1e-10 {
        // Achromatic case: hue is undefined, so return 0
        0.0
    } else {
        // Normal case
        let h_rad = lab.b.atan2(lab.a);
        let h_deg = h_rad * 180.0 / PI;
        constrain_angle(h_deg)
    };
    
    LCH {
        l: lab.l,
        c,
        h,
        alpha: lab.alpha,
    }
}

/// Convert LCH to Lab
pub fn lch_to_lab(lch: LCH) -> Lab {
    let h_rad = lch.h * PI / 180.0;
    let a = lch.c * h_rad.cos();
    let b = lch.c * h_rad.sin();
    
    Lab {
        l: lch.l,
        a,
        b,
        alpha: lch.alpha,
    }
}

/// Convert XYZ D65 to OKLab
pub fn xyz_d65_to_oklab(xyz: XyzD65) -> OKLab {
    let xyz_vec = [xyz.x, xyz.y, xyz.z];
    
    // Convert XYZ to LMS
    let lms = multiply_v3_m3x3(xyz_vec, XYZ_TO_LMS_M);
    
    // Non-linearity (cube root)
    let lms_cube = [lms[0].cbrt(), lms[1].cbrt(), lms[2].cbrt()];
    
    // Convert to Lab
    let lab = multiply_v3_m3x3(lms_cube, LMS_TO_LAB_M);
    
    OKLab {
        l: lab[0],
        a: lab[1],
        b: lab[2],
        alpha: xyz.a,
    }
}

/// Convert OKLab to XYZ D65
pub fn oklab_to_xyz_d65(oklab: OKLab) -> XyzD65 {
    let lab_vec = [oklab.l, oklab.a, oklab.b];
    
    // Convert to LMS
    let lms_cube = multiply_v3_m3x3(lab_vec, LAB_TO_LMS_M);
    
    // Apply non-linearity (cube)
    let lms = [lms_cube[0].powi(3), lms_cube[1].powi(3), lms_cube[2].powi(3)];
    
    // Convert to XYZ
    let xyz = multiply_v3_m3x3(lms, LMS_TO_XYZ_M);
    
    XyzD65 {
        x: xyz[0],
        y: xyz[1],
        z: xyz[2],
        a: oklab.alpha,
    }
}

/// Convert OKLab to OKLCH
pub fn oklab_to_oklch(oklab: OKLab) -> OKLCH {
    let c = (oklab.a.powi(2) + oklab.b.powi(2)).sqrt();
    
    // Calculate hue angle, handling special cases
    let h = if oklab.a.abs() < 1e-10 && oklab.b.abs() < 1e-10 {
        // Achromatic case: hue is undefined, so return 0
        0.0
    } else {
        // Normal case
        let h_rad = oklab.b.atan2(oklab.a);
        let h_deg = h_rad * 180.0 / PI;
        constrain_angle(h_deg)
    };
    
    OKLCH {
        l: oklab.l,
        c,
        h,
        alpha: oklab.alpha,
    }
}

/// Convert OKLCH to OKLab
pub fn oklch_to_oklab(oklch: OKLCH) -> OKLab {
    let h_rad = oklch.h * PI / 180.0;
    let a = oklch.c * h_rad.cos();
    let b = oklch.c * h_rad.sin();
    
    OKLab {
        l: oklch.l,
        a,
        b,
        alpha: oklch.alpha,
    }
}

/// Convert OKLab to RGB directly
pub fn oklab_to_rgb(oklab: OKLab) -> (u8, u8, u8, f32) {
    let xyz_d65 = oklab_to_xyz_d65(oklab);
    xyz_d65_to_rgb(xyz_d65)
}

/// Convert sRGB to linear RGB
fn srgb_to_linear(srgb: f32) -> f32 {
    if srgb <= 0.04045 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear RGB to sRGB
fn linear_to_srgb(linear: f32) -> f32 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

/// Convert RGB to OKLCH
pub fn rgb_to_oklch(r: u8, g: u8, b: u8, a: f32) -> OKLCH {
    let xyz_d65 = rgb_to_xyz_d65(r, g, b, a);
    let oklab = xyz_d65_to_oklab(xyz_d65);
    oklab_to_oklch(oklab)
}

/// Convert OKLCH to RGB
pub fn oklch_to_rgb(oklch: OKLCH) -> (u8, u8, u8, f32) {
    let oklab = oklch_to_oklab(oklch);
    let xyz_d65 = oklab_to_xyz_d65(oklab);
    xyz_d65_to_rgb(xyz_d65)
}

/// Convert RGB to LCH
pub fn rgb_to_lch(r: u8, g: u8, b: u8, a: f32) -> LCH {
    let xyz_d65 = rgb_to_xyz_d65(r, g, b, a);
    let xyz_d50 = xyz_d65_to_xyz_d50(xyz_d65);
    let lab = xyz_d50_to_lab(xyz_d50);
    lab_to_lch(lab)
}

/// Convert LCH to RGB
pub fn lch_to_rgb(lch: LCH) -> (u8, u8, u8, f32) {
    let lab = lch_to_lab(lch);
    let xyz_d50 = lab_to_xyz_d50(lab);
    let xyz_d65 = xyz_d50_to_xyz_d65(xyz_d50);
    xyz_d65_to_rgb(xyz_d65)
}

/// Helper functions

/// Take input from [0, n] and return it as [0, 1]
pub fn bound_01(n: f32, max: f32) -> f32 {
    // Check if it's already a valid percentage 1.0 = 100%
    if is_one_point_zero(n) { 
        return 1.0;
    }
    
    let percent = is_percentage(n);
    let value = if percent {
        // In JavaScript, this would be parsing a string percentage
        // In Rust, we just need to normalize the value if it's a percentage
        n / 100.0 * max
    } else {
        n
    };
    
    value.min(max).max(0.0) / max
}

/// Return a valid alpha value [0,1] with all invalid values being set to 1
pub fn bound_alpha(a: f32) -> f32 {
    if a.is_nan() || a < 0.0 || a > 1.0 {
        1.0
    } else {
        a
    }
}

/// Force a number between 0 and 1
pub fn clamp_01(val: f32) -> f32 {
    val.min(1.0).max(0.0)
}

/// Need to handle 1.0 as 100%, since once it is a number, there is no difference between it and 1
fn is_one_point_zero(n: f32) -> bool {
    n.abs() - 1.0 < std::f32::EPSILON
}

/// Check to see if the value is a percentage (between 0-100)
fn is_percentage(n: f32) -> bool {
    n >= 0.0 && n <= 100.0
}

/// Force a hex value to have 2 characters
pub fn pad2(c: &str) -> String {
    if c.len() == 1 {
        format!("0{}", c)
    } else {
        c.to_string()
    }
}

/// Replace a decimal with its percentage value
pub fn convert_to_percentage(n: f32) -> String {
    format!("{}%", (n * 100.0).round())
}

/// Converts a decimal to a hex value
pub fn convert_decimal_to_hex(d: f32) -> String {
    format!("{:02x}", (d * 255.0).round() as i32)
}

/// Converts a hex value to a decimal
pub fn convert_hex_to_decimal(h: &str) -> f32 {
    i32::from_str_radix(h, 16).unwrap_or(0) as f32 / 255.0
}

/// Convert RGB to CMYK
pub fn rgb_to_cmyk(r: u8, g: u8, b: u8, a: f32) -> CMYK {
    let r_normalized = r as f32 / 255.0;
    let g_normalized = g as f32 / 255.0;
    let b_normalized = b as f32 / 255.0;
    
    let k = 1.0 - f32::max(f32::max(r_normalized, g_normalized), b_normalized);
    
    let c = if k == 1.0 { 0.0 } else { (1.0 - r_normalized - k) / (1.0 - k) };
    let m = if k == 1.0 { 0.0 } else { (1.0 - g_normalized - k) / (1.0 - k) };
    let y = if k == 1.0 { 0.0 } else { (1.0 - b_normalized - k) / (1.0 - k) };
    
    CMYK {
        c: c * 100.0,
        m: m * 100.0,
        y: y * 100.0,
        k: k * 100.0,
        a,
    }
}

/// Convert CMYK to RGB
pub fn cmyk_to_rgb(cmyk: CMYK) -> (u8, u8, u8, f32) {
    let c = cmyk.c / 100.0;
    let m = cmyk.m / 100.0;
    let y = cmyk.y / 100.0;
    let k = cmyk.k / 100.0;
    
    let r = 255.0 * (1.0 - c) * (1.0 - k);
    let g = 255.0 * (1.0 - m) * (1.0 - k);
    let b = 255.0 * (1.0 - y) * (1.0 - k);
    
    (r.round() as u8, g.round() as u8, b.round() as u8, cmyk.a)
} 