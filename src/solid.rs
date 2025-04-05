use std::fmt;
use std::str::FromStr;
use crate::parser::ParseColorError;

/// Solid color with RGB and alpha components in the [0.0, 1.0] range
#[derive(Debug, Clone, PartialEq)]
pub struct SolidColor {
    /// Red value [0..1]
    pub r: f32,
    /// Green value [0..1]
    pub g: f32,
    /// Blue value [0..1]
    pub b: f32,
    /// Alpha value [0..1]
    pub a: f32,
}

impl SolidColor {
    /// Create a new SolidColor instance
    ///
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to RGBA array
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Convert to RGBA u8 array
    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r * 255.0 + 0.5) as u8,
            (self.g * 255.0 + 0.5) as u8,
            (self.b * 255.0 + 0.5) as u8,
            (self.a * 255.0 + 0.5) as u8,
        ]
    }

    /// Create a SolidColor from RGBA values (0-255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create a new SolidColor from HSLA values
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let h = (h % 360.0 + 360.0) % 360.0;
        
        if s == 0.0 {
            // Achromatic (gray)
            return Self::new(l, l, l, a);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let r = crate::big_color::hue_to_rgb(p, q, h + 120.0);
        let g = crate::big_color::hue_to_rgb(p, q, h);
        let b = crate::big_color::hue_to_rgb(p, q, h - 120.0);

        Self::new(r, g, b, a)
    }

    /// Create a new SolidColor from HSVA values
    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        let h = (h % 360.0 + 360.0) % 360.0;
        
        if s == 0.0 {
            // Achromatic (gray)
            return Self::new(v, v, v, a);
        }

        let h = h / 60.0; // sector 0 to 5
        let i = h.floor();
        let f = h - i; // factorial part of h
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        let (r, g, b) = match i as i32 % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        Self::new(r, g, b, a)
    }

    /// Create a new SolidColor from CMYK values
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32) -> Self {
        let c = c.clamp(0.0, 1.0);
        let m = m.clamp(0.0, 1.0);
        let y = y.clamp(0.0, 1.0);
        let k = k.clamp(0.0, 1.0);

        let r = (1.0 - c) * (1.0 - k);
        let g = (1.0 - m) * (1.0 - k);
        let b = (1.0 - y) * (1.0 - k);

        Self::new(r, g, b, 1.0)
    }

    /// Convert to CMYK values
    /// 
    /// Returns [c, m, y, k] where each value is in the range [0, 1]
    pub fn to_cmyk(&self) -> [f32; 4] {
        let k = 1.0 - self.r.max(self.g.max(self.b));
        
        if k >= 1.0 {
            return [0.0, 0.0, 0.0, 1.0];
        }
        
        let c = (1.0 - self.r - k) / (1.0 - k);
        let m = (1.0 - self.g - k) / (1.0 - k);
        let y = (1.0 - self.b - k) / (1.0 - k);
        
        [c.clamp(0.0, 1.0), m.clamp(0.0, 1.0), y.clamp(0.0, 1.0), k.clamp(0.0, 1.0)]
    }

    /// Returns: `[h, s, v, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsva(&self) -> [f32; 4] {
        let (h, s, v) = crate::big_color::rgb_to_hsv(self.r, self.g, self.b);
        [h, s, v, self.a]
    }

    /// Returns: `[h, s, l, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla(&self) -> [f32; 4] {
        let (h, s, l) = crate::big_color::rgb_to_hsl(self.r, self.g, self.b);
        [h, s, l, self.a]
    }

    /// Check if the color is considered "dark"
    pub fn is_dark(&self) -> bool {
        let mut h = 0.0;
        let mut s = 0.0;
        let mut l = 0.0;
        self.to_hsl(&mut h, &mut s, &mut l);
        l < 0.5
    }

    /// Check if the color is considered "light"
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    /// Convert color to HSL components
    pub fn to_hsl(&self, h: &mut f32, s: &mut f32, l: &mut f32) {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        
        *l = (max + min) / 2.0;
        
        if max == min {
            // Achromatic
            *h = 0.0;
            *s = 0.0;
        } else {
            let d = max - min;
            *s = if *l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };
            
            if max == r {
                *h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
            } else if max == g {
                *h = (b - r) / d + 2.0;
            } else {
                *h = (r - g) / d + 4.0;
            }
            
            *h *= 60.0;
        }
    }
    
    /// Convert color to HSV components
    pub fn to_hsv(&self, h: &mut f32, s: &mut f32, v: &mut f32) {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let d = max - min;
        
        *v = max;
        *s = if max == 0.0 { 0.0 } else { d / max };
        
        if max == min {
            // Achromatic
            *h = 0.0;
        } else {
            if max == r {
                *h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
            } else if max == g {
                *h = (b - r) / d + 2.0;
            } else {
                *h = (r - g) / d + 4.0;
            }
            
            *h *= 60.0;
        }
    }

    /// Calculate the relative luminance of the color (WCAG formula)
    /// Returns a value between 0 (darkest black) and 1 (brightest white)
    pub fn get_luminance(&self) -> f32 {
        // Convert RGB to linear values first
        let r = if self.r <= 0.03928 {
            self.r / 12.92
        } else {
            ((self.r + 0.055) / 1.055).powf(2.4)
        };
        
        let g = if self.g <= 0.03928 {
            self.g / 12.92
        } else {
            ((self.g + 0.055) / 1.055).powf(2.4)
        };
        
        let b = if self.b <= 0.03928 {
            self.b / 12.92
        } else {
            ((self.b + 0.055) / 1.055).powf(2.4)
        };
        
        // Calculate luminance using the WCAG formula
        // L = 0.2126 * R + 0.7152 * G + 0.0722 * B
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Get the RGB hexadecimal color string
    pub fn to_hex_string(&self) -> String {
        let [r, g, b, a] = self.to_rgba8();

        if a < 255 {
            return format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a);
        }

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Get the CSS `rgb()` format string
    pub fn to_rgb_string(&self) -> String {
        let [r, g, b, _] = self.to_rgba8();

        if self.a < 1.0 {
            return format!("rgba({},{},{},{})", r, g, b, self.a);
        }

        format!("rgb({},{},{})", r, g, b)
    }

    /// Get the CSS `cmyk()` format string
    pub fn to_cmyk_string(&self) -> String {
        let [c, m, y, k] = self.to_cmyk();
        
        let c_pct = (c * 100.0).round() as i32;
        let m_pct = (m * 100.0).round() as i32;
        let y_pct = (y * 100.0).round() as i32;
        let k_pct = (k * 100.0).round() as i32;
        
        if self.a < 1.0 {
            format!("cmyka({}%, {}%, {}%, {}%, {})", 
                c_pct, m_pct, y_pct, k_pct, 
                crate::big_color::round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("cmyk({}%, {}%, {}%, {}%)", 
                c_pct, m_pct, y_pct, k_pct
            )
        }
    }

    /// Convert to HSV string format
    pub fn to_hsv_string(&self) -> String {
        let mut h = 0.0;
        let mut s = 0.0;
        let mut v = 0.0;
        self.to_hsv(&mut h, &mut s, &mut v);
        
        let h_rounded = h as i32;
        let s_rounded = (s * 100.0) as i32;
        let v_rounded = (v * 100.0) as i32;
        
        if self.a < 1.0 {
            format!("hsva({}, {}%, {}%, {})", 
                h_rounded,
                s_rounded,
                v_rounded,
                crate::big_color::round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("hsv({}, {}%, {}%)", 
                h_rounded,
                s_rounded,
                v_rounded
            )
        }
    }

    /// Convert to HSL string format
    pub fn to_hsl_string(&self) -> String {
        let mut h = 0.0;
        let mut s = 0.0;
        let mut l = 0.0;
        self.to_hsl(&mut h, &mut s, &mut l);
        
        let h_rounded = h as i32;
        let s_rounded = (s * 100.0) as i32;
        let l_rounded = (l * 100.0) as i32;
        
        if self.a < 1.0 {
            format!("hsla({}, {}%, {}%, {})", 
                h_rounded,
                s_rounded,
                l_rounded,
                crate::big_color::round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("hsl({}, {}%, {}%)", 
                h_rounded,
                s_rounded,
                l_rounded
            )
        }
    }

    /// Convert color to RGB percentages
    pub fn to_percentage_rgb(&self) -> (f32, f32, f32) {
        (self.r * 100.0, self.g * 100.0, self.b * 100.0)
    }

    /// Convert to percentage RGB string format
    pub fn to_percentage_rgb_string(&self) -> String {
        let (r, g, b) = self.to_percentage_rgb();
        
        let r_rounded = r as i32;
        let g_rounded = g as i32;
        let b_rounded = b as i32;
        
        if self.a < 1.0 {
            format!("rgba({}%, {}%, {}%, {})",
                r_rounded,
                g_rounded,
                b_rounded,
                crate::big_color::round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("rgb({}%, {}%, {}%)",
                r_rounded,
                g_rounded,
                b_rounded
            )
        }
    }
}

impl fmt::Display for SolidColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGBA({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl FromStr for SolidColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse_solid_color(s)
    }
} 