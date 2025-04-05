use std::{error, fmt};

use crate::BigColor;

#[cfg(feature = "named-colors")]
mod named_colors;

#[cfg(feature = "named-colors")]
pub use named_colors::NAMED_COLORS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseColorError {
    InvalidHex,
    InvalidRgb,
    InvalidHsl,
    InvalidHwb,
    InvalidHsv,
    #[cfg(feature = "lab")]
    InvalidLab,
    #[cfg(feature = "lab")]
    InvalidLch,
    InvalidOklab,
    InvalidOklch,
    InvalidFunction,
    InvalidColorFunc,
    InvalidUnknown,
    InvalidValue,
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidHex => f.write_str("invalid hex format"),
            Self::InvalidRgb => f.write_str("invalid rgb format"),
            Self::InvalidHsl => f.write_str("invalid hsl format"),
            Self::InvalidHwb => f.write_str("invalid hwb format"),
            Self::InvalidHsv => f.write_str("invalid hsv format"),
            #[cfg(feature = "lab")]
            Self::InvalidLab => f.write_str("invalid lab format"),
            #[cfg(feature = "lab")]
            Self::InvalidLch => f.write_str("invalid lch format"),
            Self::InvalidOklab => f.write_str("invalid oklab format"),
            Self::InvalidOklch => f.write_str("invalid oklch format"),
            Self::InvalidFunction => f.write_str("invalid color function"),
            Self::InvalidColorFunc => f.write_str("invalid color() function"),
            Self::InvalidUnknown => f.write_str("invalid unknown format"),
            Self::InvalidValue => f.write_str("invalid value"),
        }
    }
}

impl error::Error for ParseColorError {}

// Helper function for from_str_radix to handle ParseIntError
impl From<std::num::ParseIntError> for ParseColorError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::InvalidHex
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
    let s = s.trim().to_lowercase();

    if s == "transparent" {
        return Ok(BigColor::new(0.0, 0.0, 0.0, 0.0));
    }

    // Named colors
    #[cfg(feature = "named-colors")]
    if let Some([r, g, b]) = NAMED_COLORS.get(&*s) {
        return Ok(BigColor::from_rgba8(*r, *g, *b, 255));
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
                "oklab" | "oklaba" => return parse_oklab(args),
                "oklch" | "oklcha" => return parse_oklch(args),
                "color" => return parse_color_function(args),
                #[cfg(feature = "lab")]
                "lab" | "laba" => return parse_lab(args),
                #[cfg(feature = "lab")]
                "lch" | "lcha" => return parse_lch(args),
                _ => return Err(ParseColorError::InvalidFunction),
            }
        }
    }

    Err(ParseColorError::InvalidUnknown)
}

fn parse_hex(s: &str) -> Result<BigColor, ParseColorError> {
    let n = s.len();

    if n != 3 && n != 4 && n != 6 && n != 8 {
        return Err(ParseColorError::InvalidHex);
    }

    if !s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(ParseColorError::InvalidHex);
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
        _ => unreachable!(),
    };

    Ok(BigColor::from_rgba8(r, g, b, a))
}

fn parse_rgb(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidRgb);
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
                return Err(ParseColorError::InvalidRgb);
            }

            if is_percentage {
                rgb[i] = clamp01(v / 100.0);
            } else {
                rgb[i] = clamp01(v / 255.0);
            }
        }

        let [r, g, b] = rgb;
        Ok(BigColor::new(r, g, b, alpha))
    })
    .map_err(|_| ParseColorError::InvalidRgb)
}

fn parse_hsl(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidHsl);
        }

        let h = parse_hue_or_angle(args[0].trim())?;
        let s = parse_percentage(args[1].trim())?;
        let l = parse_percentage(args[2].trim())?;
        let a = alpha_value.unwrap_or(1.0);

        Ok(BigColor::from_hsla(h, s, l, a))
    })
    .map_err(|_| ParseColorError::InvalidHsl)
}

fn parse_hwb(s: &str) -> Result<BigColor, ParseColorError> {
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
        Ok(BigColor::new(r, g, b, a))
    })
    .map_err(|_| ParseColorError::InvalidHwb)
}

fn parse_hsv(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidHsv);
        }

        let h = parse_hue_or_angle(args[0].trim())?;
        let s = parse_percentage(args[1].trim())?;
        let v = parse_percentage(args[2].trim())?;
        let a = alpha_value.unwrap_or(1.0);

        Ok(BigColor::from_hsva(h, s, v, a))
    })
    .map_err(|_| ParseColorError::InvalidHsv)
}

fn parse_oklab(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidOklab);
        }

        let l = parse_percentage(args[0].trim())?;
        let a = parse_number(args[1].trim())?;
        let b = parse_number(args[2].trim())?;
        let alpha = alpha_value.unwrap_or(1.0);

        Ok(BigColor::from_oklaba(l, a, b, alpha))
    })
    .map_err(|_| ParseColorError::InvalidOklab)
}

fn parse_oklch(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidOklch);
        }

        let l = parse_percentage(args[0].trim())?;
        let c = parse_number(args[1].trim())?;
        let h = parse_hue_or_angle(args[2].trim())?;
        let alpha = alpha_value.unwrap_or(1.0);

        Ok(BigColor::from_oklcha(l, c, h * std::f32::consts::PI / 180.0, alpha))
    })
    .map_err(|_| ParseColorError::InvalidOklch)
}

#[cfg(feature = "lab")]
fn parse_lab(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidLab);
        }

        let l = parse_percentage(args[0].trim())?;
        let a = parse_number(args[1].trim())?;
        let b = parse_number(args[2].trim())?;
        let alpha = alpha_value.unwrap_or(1.0);

        // Map lightness from percentage to 0-100 scale
        let l = l * 100.0;

        Ok(BigColor::from_laba(l, a, b, alpha))
    })
    .map_err(|_| ParseColorError::InvalidLab)
}

#[cfg(feature = "lab")]
fn parse_lch(s: &str) -> Result<BigColor, ParseColorError> {
    parse_color_args(s, |args, alpha_value| {
        if args.len() != 3 {
            return Err(ParseColorError::InvalidLch);
        }

        let l = parse_percentage(args[0].trim())?;
        let c = parse_number(args[1].trim())?;
        let h = parse_hue_or_angle(args[2].trim())?;
        let alpha = alpha_value.unwrap_or(1.0);

        // Map lightness from percentage to 0-100 scale
        let l = l * 100.0;

        Ok(BigColor::from_lcha(l, c, h * std::f32::consts::PI / 180.0, alpha))
    })
    .map_err(|_| ParseColorError::InvalidLch)
}

fn parse_percentage(s: &str) -> Result<f32, ParseColorError> {
    if let Some(s) = s.strip_suffix('%') {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        Ok(v / 100.0)
    } else {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        Ok(v)
    }
}

fn parse_number(s: &str) -> Result<f32, ParseColorError> {
    s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)
}

fn parse_hue_or_angle(s: &str) -> Result<f32, ParseColorError> {
    if let Some(s) = s.strip_suffix("deg") {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        return Ok(v % 360.0);
    }

    if let Some(s) = s.strip_suffix("rad") {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        return Ok(v * 180.0 / std::f32::consts::PI);
    }

    if let Some(s) = s.strip_suffix("grad") {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        return Ok(v * 0.9);
    }

    if let Some(s) = s.strip_suffix("turn") {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        return Ok(v * 360.0);
    }

    parse_number(s)
}

fn parse_unit(s: &str) -> Result<(f32, bool), ParseColorError> {
    if let Some(s) = s.strip_suffix('%') {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
        Ok((v, true))
    } else {
        let v = s.parse::<f32>().map_err(|_| ParseColorError::InvalidValue)?;
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

// Helper functions for color conversion
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

/// Parse the CSS Color 4 `color()` function
/// Format: color(colorspace c1 c2 c3[ / alpha])
/// Examples:
///   color(srgb 1 0 0)
///   color(srgb 1 0 0 / 0.5)
///   color(display-p3 1 0.5 0)
///   color(a98-rgb 1 0 0)
///   color(prophoto-rgb 1 0 0)
///   color(rec2020 1 0 0)
fn parse_color_function(s: &str) -> Result<BigColor, ParseColorError> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(ParseColorError::InvalidColorFunc);
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
        return Err(ParseColorError::InvalidColorFunc);
    }
    
    // Convert to RGB based on color space
    match color_space {
        "srgb" => {
            // sRGB color space (same as our internal representation)
            let r = components[0].clamp(0.0, 1.0);
            let g = components[1].clamp(0.0, 1.0);
            let b = components[2].clamp(0.0, 1.0);
            Ok(BigColor::new(r, g, b, alpha))
        },
        "display-p3" => {
            // Display P3 color space (approximate conversion)
            // This is a simplified conversion, a proper implementation would use color profiles
            let r = 1.0483 * components[0] - 0.0483 * components[1] - 0.0000 * components[2];
            let g = -0.0000 * components[0] + 1.0121 * components[1] - 0.0121 * components[2];
            let b = -0.0000 * components[0] - 0.0181 * components[1] + 1.0181 * components[2];
            Ok(BigColor::new(
                r.clamp(0.0, 1.0),
                g.clamp(0.0, 1.0),
                b.clamp(0.0, 1.0),
                alpha
            ))
        },
        "a98-rgb" | "prophoto-rgb" | "rec2020" => {
            // These color spaces have wider gamuts than sRGB
            // For now, we'll do a simple normalization as an approximation
            // A proper implementation would use color profiles and proper conversion
            let r = components[0].clamp(0.0, 1.0);
            let g = components[1].clamp(0.0, 1.0);
            let b = components[2].clamp(0.0, 1.0);
            Ok(BigColor::new(r, g, b, alpha))
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
            
            Ok(BigColor::new(
                r.clamp(0.0, 1.0),
                g.clamp(0.0, 1.0),
                b.clamp(0.0, 1.0),
                alpha
            ))
        },
        _ => Err(ParseColorError::InvalidColorFunc),
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
