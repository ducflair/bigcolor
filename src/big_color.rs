use std::str::FromStr;
use std::fmt;
use crate::parser::ParseColorError;
use crate::solid::SolidColor;
use crate::gradient::Gradient;

/// The main color type that can represent a solid color or a gradient
#[derive(Debug, Clone)]
pub enum BigColor {
    /// A solid color with RGBA components
    Solid(SolidColor),
    /// A gradient
    Gradient(Gradient),
}

impl BigColor {
    /// Create a new solid color from RGBA components
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::Solid(SolidColor::new(r, g, b, a))
    }
    
    /// Create a new solid color from a SolidColor
    pub fn from_solid(color: SolidColor) -> Self {
        Self::Solid(color)
    }
    
    /// Create a new gradient color from a Gradient
    pub fn from_gradient(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }

    /// Parse a color string
    pub fn parse<S: AsRef<str>>(color_str: S) -> Result<Self, ParseColorError> {
        crate::parser::parse(color_str.as_ref())
    }
    
    /// Convert to solid color if possible
    pub fn as_solid(&self) -> Option<&SolidColor> {
        match self {
            Self::Solid(color) => Some(color),
            Self::Gradient(_) => None,
        }
    }

    /// Convert to gradient if possible
    pub fn as_gradient(&self) -> Option<&Gradient> {
        match self {
            Self::Solid(_) => None,
            Self::Gradient(gradient) => Some(gradient),
        }
    }
    
    /// Create a BigColor from RGBA values (0-255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Solid(SolidColor::from_rgba8(r, g, b, a))
    }

    /// Create a new BigColor from HSLA values
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self::Solid(SolidColor::from_hsla(h, s, l, a))
    }

    /// Create a new BigColor from HSVA values
    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self::Solid(SolidColor::from_hsva(h, s, v, a))
    }
    
    /// Create a new BigColor from CMYK values
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self::Solid(SolidColor::from_cmyk(c, m, y, k))
    }
    
    /// Get the RGB hexadecimal color string
    pub fn to_hex_string(&self) -> String {
        match self {
            Self::Solid(color) => color.to_hex_string(),
            Self::Gradient(_) => String::from("(gradient)"), // Gradients don't have hex representation
        }
    }
    
    /// Get the CSS `rgb()` format string
    pub fn to_rgb_string(&self) -> String {
        match self {
            Self::Solid(color) => color.to_rgb_string(),
            Self::Gradient(_) => String::from("(gradient)"), // Gradients don't have RGB representation
        }
    }
    
    /// Get the CSS `hsl()` format string
    pub fn to_hsl_string(&self) -> String {
        match self {
            Self::Solid(color) => color.to_hsl_string(),
            Self::Gradient(_) => String::from("(gradient)"), // Gradients don't have HSL representation
        }
    }
}

impl From<SolidColor> for BigColor {
    fn from(color: SolidColor) -> Self {
        Self::Solid(color)
    }
}

impl From<Gradient> for BigColor {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl FromStr for BigColor {
    type Err = ParseColorError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If the string starts with a gradient pattern, try to parse as gradient
        if s.starts_with("linear-gradient(") || 
           s.starts_with("radial-gradient(") || 
           s.starts_with("conic-gradient(") {
            match Gradient::from_css_string(s) {
                Ok(gradient) => Ok(Self::Gradient(gradient)),
                Err(e) => Err(e),
            }
        } else {
            // Otherwise, try to parse as solid color
            match SolidColor::from_str(s) {
                Ok(color) => Ok(Self::Solid(color)),
                Err(e) => Err(e),
            }
        }
    }
}

impl fmt::Display for BigColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Solid(color) => write!(f, "{}", color),
            Self::Gradient(_) => write!(f, "(gradient)"),
        }
    }
}

// Helper functions for color conversions
// These are used by the SolidColor implementation

/// Convert HSL hue to RGB value
/// 
/// Utility function used for HSL to RGB conversion
pub(crate) fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    // Normalize hue to 0-360 range
    let mut t = t;
    if t < 0.0 {
        t += 360.0;
    }
    if t > 360.0 {
        t -= 360.0;
    }
    
    // Convert to 0-1 range
    t /= 360.0;
    
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    
    p
}

/// Convert RGB to HSV
/// 
/// Returns (h, s, v) tuple where:
/// - h is hue [0-360]
/// - s is saturation [0-1]
/// - v is value [0-1]
pub(crate) fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;
    
    let v = max;
    
    // Calculate saturation
    let s = if max == 0.0 {
        0.0
    } else {
        delta / max
    };
    
    // Calculate hue
    let h = if delta == 0.0 {
        0.0 // Achromatic (gray)
    } else if max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    
    // Normalize hue to positive
    let h = if h < 0.0 { h + 360.0 } else { h };
    
    (h, s, v)
}

/// Convert RGB to HSL
/// 
/// Returns (h, s, l) tuple where:
/// - h is hue [0-360]
/// - s is saturation [0-1]
/// - l is lightness [0-1]
pub(crate) fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    
    // Calculate lightness
    let l = (max + min) / 2.0;
    
    if max == min {
        // Achromatic (gray)
        return (0.0, 0.0, l);
    }
    
    let delta = max - min;
    
    // Calculate saturation
    let s = if l > 0.5 {
        delta / (2.0 - max - min)
    } else {
        delta / (max + min)
    };
    
    // Calculate hue
    let h = if max == r {
        (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    
    (h * 60.0, s, l)
}

/// Round a float to a specific number of decimal places
pub(crate) fn round_to_decimal_places(value: f32, places: u32) -> f32 {
    let multiplier = 10.0_f32.powi(places as i32);
    (value * multiplier).round() / multiplier
} 