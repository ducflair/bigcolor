use std::fmt;
use std::num::ParseFloatError;

use crate::BigColor;

#[cfg(feature = "named-colors")]
mod named_colors;

#[cfg(feature = "named-colors")]
pub use named_colors::NAMED_COLORS;

/// Internal RGBA color structure used for parsing
#[derive(Debug, Clone, Copy)]
pub(crate) struct RgbaColor {
    r: f32,
    g: f32,
    b: f32, 
    a: f32,
}

/// Possible errors when parsing a color string
#[derive(Debug, PartialEq)]
pub enum ParseColorError {
    /// Empty color string
    EmptyColorString,
    /// Invalid hex color format
    InvalidHexColor,
    /// Invalid RGB color format
    InvalidRgbColor,
    /// Invalid HSL color format
    InvalidHslColor,
    /// Invalid HSV color format
    InvalidHsvColor,
    /// Invalid HWB color format
    InvalidHwb,
    /// Invalid named color
    InvalidNamedColor,
    /// Invalid CMYK color format
    InvalidCmykColor,
    /// Invalid LAB color format
    InvalidLabColor,
    /// Invalid XYZ color format
    InvalidXyzColor,
    /// Invalid CSS color function
    InvalidColorFunction,
    /// Invalid CSS function
    InvalidFunction,
    /// Invalid number format when parsing component
    InvalidNumberFormat(ParseFloatError),
    /// Invalid gradient format
    InvalidGradient,
    /// Invalid value
    InvalidValue,
    /// Invalid unknown format
    InvalidUnknown,
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyColorString => write!(f, "Empty color string"),
            Self::InvalidHexColor => write!(f, "Invalid hex color format"),
            Self::InvalidRgbColor => write!(f, "Invalid RGB color format"),
            Self::InvalidHslColor => write!(f, "Invalid HSL color format"),
            Self::InvalidHsvColor => write!(f, "Invalid HSV color format"),
            Self::InvalidHwb => write!(f, "Invalid HWB color format"),
            Self::InvalidNamedColor => write!(f, "Invalid color name"),
            Self::InvalidCmykColor => write!(f, "Invalid CMYK color format"),
            Self::InvalidLabColor => write!(f, "Invalid LAB color format"),
            Self::InvalidXyzColor => write!(f, "Invalid XYZ color format"),
            Self::InvalidColorFunction => write!(f, "Invalid CSS color function"),
            Self::InvalidFunction => write!(f, "Invalid CSS function"),
            Self::InvalidNumberFormat(e) => write!(f, "Invalid number format: {}", e),
            Self::InvalidGradient => write!(f, "Invalid gradient format"),
            Self::InvalidValue => write!(f, "Invalid value"),
            Self::InvalidUnknown => write!(f, "Invalid unknown format"),
        }
    }
}

impl From<ParseFloatError> for ParseColorError {
    fn from(err: ParseFloatError) -> Self {
        Self::InvalidNumberFormat(err)
    }
}

// Helper function for from_str_radix to handle ParseIntError
impl From<std::num::ParseIntError> for ParseColorError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::InvalidHexColor
    }
}

// Add conversion from () to ParseColorError for our internal error handling
impl From<()> for ParseColorError {
    fn from(_: ()) -> Self {
        Self::InvalidValue
    }
}

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
pub fn parse(s: &str) -> Result<BigColor, ParseColorError> {
    let rgba = parse_internal(s)?;
    Ok(BigColor::new(rgba.r, rgba.g, rgba.b, rgba.a))
}

/// Internal parse function that returns RgbaColor
pub(crate) fn parse_internal(s: &str) -> Result<RgbaColor, ParseColorError> {
    let s = s.trim().to_lowercase();

    if s == "transparent" {
        return Ok(RgbaColor { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
    }

    // Named colors
    #[cfg(feature = "named-colors")]
    if let Some([r, g, b]) = NAMED_COLORS.get(&*s) {
        return Ok(RgbaColor {
            r: *r as f32 / 255.0,
            g: *g as f32 / 255.0,
            b: *b as f32 / 255.0,
            a: 1.0,
        });
    }

    // Hex format
    if let Some(s) = s.strip_prefix('#') {
        return parse_hex(s);
    }

    if let (Some(i), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        if i > 0 {
            let fname = &s[..i];
            let args = &s[i + 1..];
            
            match fname {
                "rgb" | "rgba" => return parse_rgb(args),
                "hsl" | "hsla" => return parse_hsl(args),
                "hwb" | "hwba" => return parse_hwb(args),
                "hsv" | "hsva" => return parse_hsv(args),
                "color" => return parse_color_function(args),
                #[cfg(feature = "lab")]
                "lab" | "laba" => return parse_lab(args),
                _ => return Err(ParseColorError::InvalidFunction),
            }
        }
    }

    Err(ParseColorError::InvalidUnknown)
}

fn parse_hex(s: &str) -> Result<RgbaColor, ParseColorError> {
    let n = s.len();

    if n != 3 && n != 4 && n != 6 && n != 8 {
        return Err(ParseColorError::InvalidHexColor);
    }

    if !s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(ParseColorError::InvalidHexColor);
    }

    let (r, g, b, a) = match n {
        3 => {
            let r = u8::from_str_radix(&s[0..1], 16)?;
            let g = u8::from_str_radix(&s[1..2], 16)?;
            let b = u8::from_str_radix(&s[2..3], 16)?;
            (r * 17, g * 17, b * 17, 255)
        }
        4 => {
            let r = u8::from_str_radix(&s[0..1], 16)?;
            let g = u8::from_str_radix(&s[1..2], 16)?;
            let b = u8::from_str_radix(&s[2..3], 16)?;
            let a = u8::from_str_radix(&s[3..4], 16)?;
            (r * 17, g * 17, b * 17, a * 17)
        }
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16)?;
            let g = u8::from_str_radix(&s[2..4], 16)?;
            let b = u8::from_str_radix(&s[4..6], 16)?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16)?;
            let g = u8::from_str_radix(&s[2..4], 16)?;
            let b = u8::from_str_radix(&s[4..6], 16)?;
            let a = u8::from_str_radix(&s[6..8], 16)?;
            (r, g, b, a)
        }
        _ => panic!("Invalid hex color format, this should never happen"),
    };

    Ok(RgbaColor {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    })
}

fn parse_rgb(s: &str) -> Result<RgbaColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidRgbColor);
        }

        let mut is_percentage = false;
        let alpha = alpha_value.unwrap_or(1.0);

        // RGB values
        let mut rgb = [0.0, 0.0, 0.0];

        for (i, a) in args.iter().enumerate() {
            let a = a.trim();
            let (v, pct) = parse_unit(a)?;

            if i == 0 {
                is_percentage = pct;
            } else if is_percentage != pct {
                return Err(ParseColorError::InvalidRgbColor);
            }

            if is_percentage {
                rgb[i] = clamp01(v / 100.0);
                } else {
                rgb[i] = clamp01(v / 255.0);
            }
        }

        let [r, g, b] = rgb;
        Ok(RgbaColor {
            r,
            g,
            b,
            a: alpha,
        })
    })
    .map_err(|_| ParseColorError::InvalidRgbColor)
}

fn parse_hsl(s: &str) -> Result<RgbaColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidHslColor);
        }

        let h = parse_hue_or_angle(args[0].trim())?;
        let s = parse_percentage(args[1].trim())?;
        let l = parse_percentage(args[2].trim())?;
        let a = alpha_value.unwrap_or(1.0);

        Ok(RgbaColor {
            r: hsl_to_rgb(h, s, l).0,
            g: hsl_to_rgb(h, s, l).1,
            b: hsl_to_rgb(h, s, l).2,
            a,
        })
    })
    .map_err(|_| ParseColorError::InvalidHslColor)
}

fn parse_hwb(s: &str) -> Result<RgbaColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
                    return Err(ParseColorError::InvalidHwb);
                }

        let h = parse_hue_or_angle(args[0].trim())?;
        let w = parse_percentage(args[1].trim())?;
        let b = parse_percentage(args[2].trim())?;
        let a = alpha_value.unwrap_or(1.0);

        // HWB is not as common, so we'll convert to HSL first
        let (r, g, b) = hwb_to_rgb(h, w, b);
        Ok(RgbaColor {
            r,
            g,
            b,
            a,
        })
    })
    .map_err(|_| ParseColorError::InvalidHwb)
}

fn parse_hsv(s: &str) -> Result<RgbaColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidHsvColor);
        }

        let h = parse_hue_or_angle(args[0].trim())?;
        let s = parse_percentage(args[1].trim())?;
        let v = parse_percentage(args[2].trim())?;
        let a = alpha_value.unwrap_or(1.0);

        Ok(RgbaColor {
            r: hsv_to_rgb(h, s, v).0,
            g: hsv_to_rgb(h, s, v).1,
            b: hsv_to_rgb(h, s, v).2,
            a,
        })
    })
    .map_err(|_| ParseColorError::InvalidHsvColor)
}

fn parse_color_function(s: &str) -> Result<RgbaColor, ParseColorError> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(ParseColorError::InvalidColorFunction);
    }
    
    // Extract color space
    let color_space = parts[0];
    
    // Extract color components and alpha
    let mut components = Vec::new();
    let mut alpha = 1.0;
    let mut in_alpha = false;
    
    for (_, part) in parts.iter().enumerate().skip(1) {
        if *part == "/" {
            in_alpha = true;
            continue;
        }
        
        if in_alpha {
            // Parse alpha value
            let trimmed = part.trim_matches(|c: char| c == ',' || c.is_whitespace());
            alpha = parse_number(trimmed)?;
            alpha = alpha.clamp(0.0, 1.0);
            break;
        } else {
            // Parse color component
            let trimmed = part.trim_matches(|c: char| c == ',' || c.is_whitespace());
            if !trimmed.is_empty() {
                let value = parse_number(trimmed)?;
                components.push(value);
            }
        }
    }
    
    // Ensure we have enough components
    if components.len() < 3 {
        return Err(ParseColorError::InvalidColorFunction);
    }
    
    // Convert to RGB based on color space
    match color_space {
        "srgb" => {
            // sRGB color space (same as our internal representation)
            let r = components[0].clamp(0.0, 1.0);
            let g = components[1].clamp(0.0, 1.0);
            let b = components[2].clamp(0.0, 1.0);
            Ok(RgbaColor {
                r,
                g,
                b,
                a: alpha,
            })
        },
        "display-p3" => {
            // Display P3 color space (approximate conversion)
            // This is a simplified conversion, a proper implementation would use color profiles
            let r = 1.0483 * components[0] - 0.0483 * components[1] - 0.0000 * components[2];
            let g = -0.0000 * components[0] + 1.0121 * components[1] - 0.0121 * components[2];
            let b = -0.0000 * components[0] - 0.0181 * components[1] + 1.0181 * components[2];
            Ok(RgbaColor {
                r: r.clamp(0.0, 1.0),
                g: g.clamp(0.0, 1.0),
                b: b.clamp(0.0, 1.0),
                a: alpha,
            })
        },
        "a98-rgb" | "prophoto-rgb" | "rec2020" => {
            // These color spaces have wider gamuts than sRGB
            // For now, we'll do a simple normalization as an approximation
            // A proper implementation would use color profiles and proper conversion
            let r = components[0].clamp(0.0, 1.0);
            let g = components[1].clamp(0.0, 1.0);
            let b = components[2].clamp(0.0, 1.0);
            Ok(RgbaColor {
                r,
                g,
                b,
                a: alpha,
            })
        },
        "xyz" | "xyz-d50" | "xyz-d65" => {
            // Convert XYZ to sRGB (approximate D65 conversion)
            // This is a simplified version, proper implementation would handle different illuminants
            let x = components[0];
            let y = components[1];
            let z = components[2];
            
            // XYZ to sRGB matrix (D65)
            let r =  3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
            let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
            let b =  0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
            
            // Apply gamma correction
            let r = if r <= 0.0031308 { 12.92 * r } else { 1.055 * r.powf(1.0/2.4) - 0.055 };
            let g = if g <= 0.0031308 { 12.92 * g } else { 1.055 * g.powf(1.0/2.4) - 0.055 };
            let b = if b <= 0.0031308 { 12.92 * b } else { 1.055 * b.powf(1.0/2.4) - 0.055 };
            
            Ok(RgbaColor {
                r: r.clamp(0.0, 1.0),
                g: g.clamp(0.0, 1.0),
                b: b.clamp(0.0, 1.0),
                a: alpha,
            })
        },
        _ => Err(ParseColorError::InvalidColorFunction),
    }
}

/// Parse a color string into a SolidColor
/// This is delegated to from the SolidColor::from_str implementation
pub fn parse_solid_color(s: &str) -> Result<crate::solid::SolidColor, ParseColorError> {
    // First try to parse using our internal color parser
    let result = parse_internal(s);
    
    // If it was successfully parsed, convert to SolidColor
    match result {
        Ok(rgba) => Ok(crate::solid::SolidColor::new(rgba.r, rgba.g, rgba.b, rgba.a)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_percentage() {
        let test_data = [
            ("0%", Ok(0.0)),
            ("100%", Ok(1.0)),
            ("50%", Ok(0.5)),
            ("0", Ok(0.0)),
            ("1", Ok(1.0)),
            ("0.5", Ok(0.5)),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_percentage(s), expected);
        }
    }

    #[test]
    fn test_parse_number() {
        let test_data = [
            ("0", Ok(0.0)),
            ("255", Ok(255.0)),
            ("127.5", Ok(127.5)),
            ("-100", Ok(-100.0)),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_number(s), expected);
        }
    }

    #[test]
    fn test_parse_hue_or_angle() {
        let test_data = [
            ("360", Ok(360.0)),
            ("127.356", Ok(127.356)),
            ("+120deg", Ok(120.0)),
            ("90deg", Ok(90.0)),
            ("-127deg", Ok(-127.0)),
            ("100grad", Ok(90.0)),
            ("1.5707963267948966rad", Ok(90.0)),
            ("0.25turn", Ok(90.0)),
            ("-0.25turn", Ok(-90.0)),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_hue_or_angle(s), expected);
        }
    }
}

// Add missing conversion functions
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (v, v, v);
    }

    let h = h / 60.0; // sector 0 to 5
    let i = h.floor();
    let f = h - i; // factorial part of h
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

// Add LAB function if needed
#[cfg(feature = "lab")]
fn parse_lab(s: &str) -> Result<RgbaColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidLabColor);
        }

        let l_str = args[0].trim();
        let a_str = args[1].trim();
        let b_str = args[2].trim();
        
        // Handle lightness (L) - must be between 0-100
        let l = if l_str.ends_with('%') {
            // If percentage, use as is but clamp
            let val = l_str.trim_end_matches('%').parse::<f32>()?;
            val.clamp(0.0, 100.0)
        } else {
            // If number, use directly but clamp
            l_str.parse::<f32>()?.clamp(0.0, 100.0)
        };
        
        // Handle a and b components
        let a = a_str.parse::<f32>()?;
        let b = b_str.parse::<f32>()?;
        
        let alpha = alpha_value.unwrap_or(1.0);

        Ok(RgbaColor {
            r: lab_to_rgb(l, a, b).0,
            g: lab_to_rgb(l, a, b).1,
            b: lab_to_rgb(l, a, b).2,
            a: alpha,
        })
    })
    .map_err(|_| ParseColorError::InvalidLabColor)
}

#[cfg(feature = "lab")]
fn lab_to_rgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    use palette::{rgb::Srgb, Lab, FromColor};
    
    // Convert l from 0-100 range to 0-1 range for palette
    let normalized_l = l / 100.0;
    
    // Create a Lab color
    let lab = Lab::new(normalized_l, a, b);
    
    // Convert to RGB
    let rgb = Srgb::from_color(lab);
    
    // Get the RGB components and clamp to valid range
    let r = rgb.red.clamp(0.0, 1.0);
    let g = rgb.green.clamp(0.0, 1.0);
    let b = rgb.blue.clamp(0.0, 1.0);
    
    (r, g, b)
}

fn hue_to_rgb(n1: f32, n2: f32, h: f32) -> f32 {
    let h = ((h % 6.0) + 6.0) % 6.0;

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

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
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

fn hwb_to_rgb(h: f32, w: f32, b: f32) -> (f32, f32, f32) {
    let sum = w + b;
    if sum >= 1.0 {
        let gray = w / sum;
        return (gray, gray, gray);
    }

    let (r, g, b) = hsl_to_rgb(h, 1.0, 0.5);
    let r = r * (1.0 - w - b) + w;
    let g = g * (1.0 - w - b) + w;
    let b = b * (1.0 - w - b) + w;
    (r, g, b)
}

fn parse_percentage(s: &str) -> Result<f32, ParseColorError> {
    if let Some(s) = s.strip_suffix('%') {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        Ok(v / 100.0)
                } else {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        Ok(v)
    }
}

fn parse_number(s: &str) -> Result<f32, ParseColorError> {
    s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))
}

fn parse_hue_or_angle(s: &str) -> Result<f32, ParseColorError> {
    if let Some(s) = s.strip_suffix("deg") {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        return Ok(v % 360.0);
    }

    if let Some(s) = s.strip_suffix("rad") {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        return Ok(v * 180.0 / std::f32::consts::PI);
    }

    if let Some(s) = s.strip_suffix("grad") {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        return Ok(v * 0.9);
    }

    if let Some(s) = s.strip_suffix("turn") {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        return Ok(v * 360.0);
    }

    parse_number(s)
}

fn parse_unit(s: &str) -> Result<(f32, bool), ParseColorError> {
    if let Some(s) = s.strip_suffix('%') {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        Ok((v, true))
    } else {
        let v = s.parse::<f32>().map_err(|e| ParseColorError::InvalidNumberFormat(e))?;
        Ok((v, false))
    }
}

/// Split color args and parse.
///
/// # Examples
///
/// ```
/// // "1, 2, 3, 0.5" => [1, 2, 3], Some(0.5)
/// // "1 2 3" => [1, 2, 3], None
/// // "1 2 3 / 0.5" => [1, 2, 3], Some(0.5)
/// ```
fn parse_color_args<F, T, E>(
    s: &str,
    f: F,
) -> Result<T, E>
where
    F: FnOnce(Vec<&str>, Option<f32>) -> Result<T, E>,
    E: From<()>,
{
    let mut alpha = None;

    // The slash separator: "1 2 3 / 0.5"
    let mut parts: Vec<&str> = if let Some(i) = s.find('/') {
        let alpha_str = s[i + 1..].trim();
        let alpha_value = alpha_str.parse::<f32>().map_err(|_| ())?;
        alpha = Some(alpha_value.clamp(0.0, 1.0));
        s[..i].split_whitespace().collect()
    } else if s.contains(',') {
        // The comma separator: "1, 2, 3, 0.5"
        let mut parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 4 {
            let alpha_str = parts.pop().unwrap().trim();
            let alpha_value = alpha_str.parse::<f32>().map_err(|_| ())?;
            alpha = Some(alpha_value.clamp(0.0, 1.0));
        }
        parts
                } else {
        // The whitespace separator: "1 2 3"
        s.split_whitespace().collect()
    };

    if parts.is_empty() {
        parts = s.split(',').collect();
    }

    f(parts, alpha)
}

#[inline]
fn clamp01(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}
