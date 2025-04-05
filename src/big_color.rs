use std::str::FromStr;
use std::fmt;
use peniko;
use crate::parser::ParseColorError;

#[cfg(feature = "named-colors")]
use crate::parser::NAMED_COLORS;

/// BigColor is a flexible color manipulation library that combines the functionality
/// of TinyColor with csscolorparser's color parsing capabilities.
#[derive(Debug, Clone, PartialEq)]
pub struct BigColor {
    /// Red value [0..1]
    pub r: f32,
    /// Green value [0..1]
    pub g: f32,
    /// Blue value [0..1]
    pub b: f32,
    /// Alpha value [0..1]
    pub a: f32,
}

/// Blending mode for color operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Normal blending (simple alpha compositing)
    Normal,
    /// Multiply blending mode (multiplies colors together)
    Multiply,
    /// Screen blending mode (inverts, multiplies, then inverts again)
    Screen,
    /// Overlay blending mode (combines Multiply and Screen)
    Overlay,
    /// Darken blending mode (selects darker of base and blend colors)
    Darken,
    /// Lighten blending mode (selects lighter of base and blend colors)
    Lighten,
    /// Color-dodge blending mode (brightens base color)
    ColorDodge,
    /// Color-burn blending mode (darkens base color)
    ColorBurn,
    /// Hard-light blending mode (similar to Overlay, but with blend and base swapped)
    HardLight,
    /// Soft-light blending mode (similar to Overlay, but more subtle)
    SoftLight,
    /// Difference blending mode (subtracts darker from lighter)
    Difference,
    /// Exclusion blending mode (similar to Difference, but with lower contrast)
    Exclusion,
}

/// Color space to use for interpolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationSpace {
    /// RGB interpolation (linear in RGB space)
    RGB,
    /// HSL interpolation (linear in HSL space)
    HSL,
    /// HSV interpolation (linear in HSV space)
    HSV,
    /// LAB interpolation (perceptually linear)
    LAB,
    /// LCH interpolation (perceptually linear with better hue interpolation)
    LCH,
    /// Oklab interpolation (perceptually uniform)
    OKLAB,
}

impl BigColor {
    /// Create a new BigColor instance
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

    /// Parse a color string
    pub fn parse<S: AsRef<str>>(color_str: S) -> Result<Self, ParseColorError> {
        crate::parser::parse(color_str.as_ref())
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

    /// Convert to peniko::Color
    pub fn to_peniko_color(&self) -> peniko::Color {
        let [r, g, b, a] = self.to_rgba8();
        peniko::Color::from_rgba8(r, g, b, a)
    }

    /// Get a contrast color based on the current color
    /// The intensity parameter (0-1) determines the contrast level:
    /// - 0.0: Light gray for dark colors, dark gray for light colors (low contrast)
    /// - 1.0: Pure white for dark colors, pure black for light colors (maximum contrast)
    /// - Values in between create a gradient between these extremes
    pub fn get_contrast(&self, intensity: f32) -> Self {
        // Determine if the color is light or dark
        // We'll use the relative luminance formula for this
        let luminance = self.get_luminance();
        let is_light = luminance > 0.5;

        // Clamp intensity between 0 and 1
        let intensity = intensity.clamp(0.0, 1.0);
        
        if is_light {
            // For light colors, provide a dark contrast
            // At intensity=0, use a medium gray (74,74,74)
            // At intensity=1, use pure black (0,0,0)
            let value = 74.0 * (1.0 - intensity);
            Self::new(value / 255.0, value / 255.0, value / 255.0, 1.0)
        } else {
            // For dark colors, provide a light contrast
            // At intensity=0, use a light gray (201,201,201)
            // At intensity=1, use pure white (255,255,255)
            let value = 201.0 + (255.0 - 201.0) * intensity;
            Self::new(value / 255.0, value / 255.0, value / 255.0, 1.0)
        }
    }

    /// Create a BigColor from RGBA values (0-255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create a new BigColor from HSLA values
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

        let r = hue_to_rgb(p, q, h + 120.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 120.0);

        Self::new(r, g, b, a)
    }

    /// Create a new BigColor from HSVA values
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

    /// Create a new BigColor from Oklab values
    pub fn from_oklaba(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        // Basic conversion from Oklab to sRGB
        // This is a simplified implementation
        let l_ = l.clamp(0.0, 1.0);
        let a_ = a.clamp(-0.4, 0.4);
        let b_ = b.clamp(-0.4, 0.4);

        // Convert to linear RGB (simplified conversion)
        let l_comp = l_ + 0.3963377774 * a_ + 0.2158037573 * b_;
        let m_comp = l_ - 0.1055613458 * a_ - 0.0638541728 * b_;
        let s_comp = l_ - 0.0894841775 * a_ - 1.2914855480 * b_;

        // Ensure non-negative
        let l_comp = l_comp.max(0.0);
        let m_comp = m_comp.max(0.0);
        let s_comp = s_comp.max(0.0);

        // Convert to linear RGB
        let r = 4.0767416621 * l_comp - 3.3077115913 * m_comp + 0.2309699292 * s_comp;
        let g = -1.2684380046 * l_comp + 2.6097574011 * m_comp - 0.3413193965 * s_comp;
        let b = -0.0041960863 * l_comp - 0.7034186147 * m_comp + 1.7076147010 * s_comp;

        // Apply gamma correction for sRGB
        let r = if r <= 0.0031308 { 12.92 * r } else { 1.055 * r.powf(1.0/2.4) - 0.055 };
        let g = if g <= 0.0031308 { 12.92 * g } else { 1.055 * g.powf(1.0/2.4) - 0.055 };
        let b = if b <= 0.0031308 { 12.92 * b } else { 1.055 * b.powf(1.0/2.4) - 0.055 };

        Self::new(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), alpha)
    }

    /// Create a new BigColor from Oklch values
    pub fn from_oklcha(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        // Convert polar coordinates to Cartesian
        let a = c * h.cos();
        let b = c * h.sin();
        
        Self::from_oklaba(l, a, b, alpha)
    }

    /// Create a new BigColor from Lab values
    /// 
    /// - `l`: Lightness [0..100]
    /// - `a`: A axis [-128..127]
    /// - `b`: B axis [-128..127]
    /// - `alpha`: Alpha [0..1]
    #[cfg(feature = "lab")]
    pub fn from_laba(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        // Normalize values
        let l = l.clamp(0.0, 100.0);
        
        // Reference white D65
        let xn = 0.95047;
        let yn = 1.0;
        let zn = 1.08883;
        
        // Convert Lab to XYZ
        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;
        
        let x = if fx.powi(3) > 0.008856 {
            xn * fx.powi(3)
        } else {
            xn * (fx - 16.0 / 116.0) / 7.787
        };
        
        let y = if l > 8.0 {
            yn * fy.powi(3)
        } else {
            yn * l / 903.3
        };
        
        let z = if fz.powi(3) > 0.008856 {
            zn * fz.powi(3)
        } else {
            zn * (fz - 16.0 / 116.0) / 7.787
        };
        
        // XYZ to RGB
        let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
        let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
        
        // Apply gamma correction
        let r = if r > 0.0031308 {
            1.055 * r.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * r
        };
        
        let g = if g > 0.0031308 {
            1.055 * g.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * g
        };
        
        let b = if b > 0.0031308 {
            1.055 * b.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * b
        };
        
        Self::new(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), alpha)
    }
    
    /// Create a new BigColor from LCh values
    /// 
    /// - `l`: Lightness [0..100]
    /// - `c`: Chroma [0..128]
    /// - `h`: Hue angle [0..360]
    /// - `alpha`: Alpha [0..1]
    #[cfg(feature = "lab")]
    pub fn from_lcha(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        // Normalize hue
        let h = normalize_angle(h);
        
        // Convert LCh to Lab
        let h_rad = h * std::f32::consts::PI / 180.0;
        let a = c * h_rad.cos();
        let b = c * h_rad.sin();
        
        Self::from_laba(l, a, b, alpha)
    }

    /// Returns: `[h, s, v, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsva(&self) -> [f32; 4] {
        let (h, s, v) = rgb_to_hsv(self.r, self.g, self.b);
        [h, s, v, self.a]
    }

    /// Returns: `[h, s, l, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla(&self) -> [f32; 4] {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        [h, s, l, self.a]
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

    /// Lighten the color by a percentage (0-100)
    pub fn lighten(&self, amount: u8) -> Self {
        let amount = (amount as f32).min(100.0) / 100.0;
        let mut hsla = self.to_hsla();
        hsla[2] = (hsla[2] + amount).min(1.0);
        Self::from_hsla(hsla[0], hsla[1], hsla[2], hsla[3])
    }

    /// Darken the color by a percentage (0-100)
    pub fn darken(&self, amount: u8) -> Self {
        let amount = (amount as f32).min(100.0) / 100.0;
        let mut hsla = self.to_hsla();
        hsla[2] = (hsla[2] - amount).max(0.0);
        Self::from_hsla(hsla[0], hsla[1], hsla[2], hsla[3])
    }

    /// Saturate the color by a percentage (0-100)
    pub fn saturate(&self, amount: u8) -> Self {
        let amount = (amount as f32).min(100.0) / 100.0;
        let mut hsla = self.to_hsla();
        hsla[1] = (hsla[1] + amount).min(1.0);
        Self::from_hsla(hsla[0], hsla[1], hsla[2], hsla[3])
    }

    /// Desaturate the color by a percentage (0-100)
    pub fn desaturate(&self, amount: u8) -> Self {
        let amount = (amount as f32).min(100.0) / 100.0;
        let mut hsla = self.to_hsla();
        hsla[1] = (hsla[1] - amount).max(0.0);
        Self::from_hsla(hsla[0], hsla[1], hsla[2], hsla[3])
    }

    /// Convert to grayscale
    pub fn greyscale(&self) -> Self {
        let mut hsla = self.to_hsla();
        hsla[1] = 0.0;
        Self::from_hsla(hsla[0], hsla[1], hsla[2], hsla[3])
    }

    /// Alias for greyscale
    pub fn grayscale(&self) -> Self {
        self.greyscale()
    }

    /// Spin the hue by a given amount (-360 to 360)
    pub fn spin(&self, amount: i16) -> Self {
        let mut hsla = self.to_hsla();
        let hue = (hsla[0] * 360.0 + amount as f32) % 360.0;
        hsla[0] = if hue < 0.0 { hue + 360.0 } else { hue } / 360.0;
        Self::from_hsla(hsla[0], hsla[1], hsla[2], hsla[3])
    }

    /// Mix with another color
    pub fn mix(&self, color: &BigColor, amount: u8) -> Self {
        let amount = (amount as f32).min(100.0) / 100.0;
        let rgb1 = self.to_array();
        let rgb2 = color.to_array();
        
        let r = rgb1[0] * (1.0 - amount) + rgb2[0] * amount;
        let g = rgb1[1] * (1.0 - amount) + rgb2[1] * amount;
        let b = rgb1[2] * (1.0 - amount) + rgb2[2] * amount;
        let a = rgb1[3] * (1.0 - amount) + rgb2[3] * amount;
        
        Self::new(r, g, b, a)
    }

    /// Create a complementary color
    pub fn complement(&self) -> Self {
        self.spin(180)
    }

    /// Create a set of analogous colors
    pub fn analogous(&self, results: usize, slices: u16) -> Vec<Self> {
        self.hue_rotation(results, slices)
    }

    /// Create a set of monochromatic colors
    pub fn monochromatic(&self, count: usize) -> Vec<Self> {
        if count <= 1 {
            return vec![self.clone()];
        }
        
        let mut h = 0.0;
        let mut s = 0.0;
        let mut l = 0.0;
        self.to_hsl(&mut h, &mut s, &mut l);
        
        let mut results = Vec::with_capacity(count);
        let step = 1.0 / (count as f32);
        
        for i in 0..count {
            let new_l = (i as f32 * step).clamp(0.0, 1.0);
            results.push(Self::from_hsla(h, s, new_l, self.a));
        }
        
        results
    }

    /// Create a set of triad colors
    pub fn triad(&self) -> [Self; 3] {
        let original = self.clone();
        let color1 = self.spin(120);
        let color2 = self.spin(240);
        [original, color1, color2]
    }

    /// Create a set of tetrad colors
    pub fn tetrad(&self) -> [Self; 4] {
        let original = self.clone();
        let color1 = self.spin(90);
        let color2 = self.spin(180);
        let color3 = self.spin(270);
        [original, color1, color2, color3]
    }

    /// Helper function for hue rotation
    fn hue_rotation(&self, results: usize, slices: u16) -> Vec<Self> {
        let mut colors = Vec::with_capacity(results);
        
        // Include original color
        colors.push(self.clone());
        
        let slice_angle = 360.0 / slices as f32;
        
        for i in 1..results {
            let rotation = slice_angle * i as f32;
            colors.push(self.spin(rotation as i16));
        }
        
        colors
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

    /// Get the brightness value (0-255)
    pub fn brightness(&self) -> u8 {
        let rgb = self.to_rgba8();
        // Using the formula (R*299 + G*587 + B*114) / 1000
        let brightness = (rgb[0] as f32 * 299.0 + rgb[1] as f32 * 587.0 + rgb[2] as f32 * 114.0) / 1000.0;
        brightness as u8
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

    /// Calculate the contrast ratio between two colors
    /// Returns a value between 1 and 21
    pub fn contrast_ratio(&self, other: &Self) -> f32 {
        let l1 = self.get_luminance();
        let l2 = other.get_luminance();
        
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Check if the color is readable on the background color
    pub fn is_readable_on(&self, background: &BigColor, options: Option<ReadableOptions>) -> bool {
        let opts = options.unwrap_or_default();
        let contrast_ratio = self.contrast_ratio(background);
        
        match opts.level.as_str() {
            "AA" => {
                if opts.large {
                    contrast_ratio >= 3.0
                } else {
                    contrast_ratio >= 4.5
                }
            },
            "AAA" => {
                if opts.large {
                    contrast_ratio >= 4.5
                } else {
                    contrast_ratio >= 7.0
                }
            },
            _ => contrast_ratio >= 4.5
        }
    }

    #[cfg(feature = "named-colors")]
    /// Get the color name if it matches a named color
    pub fn name(&self) -> Option<&'static str> {
        let rgb = &self.to_rgba8()[0..3];
        for (&k, &v) in NAMED_COLORS.entries() {
            if v == rgb {
                return Some(k);
            }
        }
        None
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

    /// Brighten the color by a percentage (positive values)
    /// or darken the color (negative values)
    pub fn brighten(&self, amount: f32) -> Self {
        if amount == 0.0 {
            return self.clone();
        }
        
        let mut h = 0.0;
        let mut s = 0.0;
        let mut l = 0.0;
        self.to_hsl(&mut h, &mut s, &mut l);
        
        // Modify lightness
        l = (l + amount / 100.0).clamp(0.0, 1.0);
        
        Self::from_hsla(h, s, l, self.a)
    }

    /// Generate split complementary colors
    /// Returns a vector of [original, complement 1, complement 2]
    pub fn split_complement(&self) -> Vec<Self> {
        let mut h = 0.0;
        let mut s = 0.0;
        let mut l = 0.0;
        self.to_hsl(&mut h, &mut s, &mut l);
        
        let h1 = (h + 72.0) % 360.0;
        let h2 = (h + 216.0) % 360.0;
        
        vec![
            self.clone(),
            Self::from_hsla(h1, s, l, self.a),
            Self::from_hsla(h2, s, l, self.a)
        ]
    }

    /// Convert color to RGB percentages
    pub fn to_percentage_rgb(&self) -> (f32, f32, f32) {
        (self.r * 100.0, self.g * 100.0, self.b * 100.0)
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
                round_to_decimal_places(self.a, 2)
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
                round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("hsl({}, {}%, {}%)", 
                h_rounded,
                s_rounded,
                l_rounded
            )
        }
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
                round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("rgb({}%, {}%, {}%)",
                r_rounded,
                g_rounded,
                b_rounded
            )
        }
    }

    /// Create a new BigColor from CMYK values
    /// 
    /// - c, m, y, k are values in the range [0, 1]
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

    /// Convert color to CMYK values
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
                round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("cmyk({}%, {}%, {}%, {}%)", 
                c_pct, m_pct, y_pct, k_pct
            )
        }
    }

    /// Blend with another color using a specified blending mode
    ///
    /// - `color`: The color to blend with
    /// - `mode`: The blending mode to use
    /// - `amount`: The blending amount (0-100)
    ///
    /// Returns the blended color
    pub fn blend(&self, color: &BigColor, mode: BlendMode, amount: u8) -> Self {
        let amount = (amount as f32).min(100.0) / 100.0;
        
        // Get the blended RGB values based on the selected mode
        let (r, g, b) = match mode {
            BlendMode::Normal => {
                // Simple linear interpolation for normal blending
                let r = self.r * (1.0 - amount) + color.r * amount;
                let g = self.g * (1.0 - amount) + color.g * amount;
                let b = self.b * (1.0 - amount) + color.b * amount;
                (r, g, b)
            },
            BlendMode::Multiply => {
                // Multiply blending mode
                let r = blend_multiply(self.r, color.r, amount);
                let g = blend_multiply(self.g, color.g, amount);
                let b = blend_multiply(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::Screen => {
                // Screen blending mode
                let r = blend_screen(self.r, color.r, amount);
                let g = blend_screen(self.g, color.g, amount);
                let b = blend_screen(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::Overlay => {
                // Overlay blending mode
                let r = blend_overlay(self.r, color.r, amount);
                let g = blend_overlay(self.g, color.g, amount);
                let b = blend_overlay(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::Darken => {
                // Darken blending mode
                let r = blend_darken(self.r, color.r, amount);
                let g = blend_darken(self.g, color.g, amount);
                let b = blend_darken(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::Lighten => {
                // Lighten blending mode
                let r = blend_lighten(self.r, color.r, amount);
                let g = blend_lighten(self.g, color.g, amount);
                let b = blend_lighten(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::ColorDodge => {
                // Color-dodge blending mode
                let r = blend_color_dodge(self.r, color.r, amount);
                let g = blend_color_dodge(self.g, color.g, amount);
                let b = blend_color_dodge(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::ColorBurn => {
                // Color-burn blending mode
                let r = blend_color_burn(self.r, color.r, amount);
                let g = blend_color_burn(self.g, color.g, amount);
                let b = blend_color_burn(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::HardLight => {
                // Hard-light blending mode
                let r = blend_hard_light(self.r, color.r, amount);
                let g = blend_hard_light(self.g, color.g, amount);
                let b = blend_hard_light(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::SoftLight => {
                // Soft-light blending mode
                let r = blend_soft_light(self.r, color.r, amount);
                let g = blend_soft_light(self.g, color.g, amount);
                let b = blend_soft_light(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::Difference => {
                // Difference blending mode
                let r = blend_difference(self.r, color.r, amount);
                let g = blend_difference(self.g, color.g, amount);
                let b = blend_difference(self.b, color.b, amount);
                (r, g, b)
            },
            BlendMode::Exclusion => {
                // Exclusion blending mode
                let r = blend_exclusion(self.r, color.r, amount);
                let g = blend_exclusion(self.g, color.g, amount);
                let b = blend_exclusion(self.b, color.b, amount);
                (r, g, b)
            },
        };
        
        // Calculate alpha
        let a = self.a * (1.0 - amount) + color.a * amount;
        
        Self::new(r, g, b, a)
    }

    /// Interpolate between this color and another color
    /// 
    /// - `color`: The target color to interpolate to
    /// - `amount`: The interpolation amount from 0.0 (this color) to 1.0 (target color)
    /// - `space`: The color space to perform the interpolation in
    pub fn interpolate(&self, color: &BigColor, amount: f32, space: InterpolationSpace) -> Self {
        let t = amount.clamp(0.0, 1.0);
        
        match space {
            InterpolationSpace::RGB => {
                // Simple linear interpolation in RGB space
                let r = self.r * (1.0 - t) + color.r * t;
                let g = self.g * (1.0 - t) + color.g * t;
                let b = self.b * (1.0 - t) + color.b * t;
                let a = self.a * (1.0 - t) + color.a * t;
                
                Self::new(r, g, b, a)
            },
            InterpolationSpace::HSL => {
                // Interpolation in HSL space
                let [h1, s1, l1, a1] = self.to_hsla();
                let [h2, s2, l2, a2] = color.to_hsla();
                
                // Handle hue interpolation specially to go the shorter way around the circle
                let h = interpolate_hue(h1, h2, t);
                let s = s1 * (1.0 - t) + s2 * t;
                let l = l1 * (1.0 - t) + l2 * t;
                let a = a1 * (1.0 - t) + a2 * t;
                
                Self::from_hsla(h, s, l, a)
            },
            InterpolationSpace::HSV => {
                // Interpolation in HSV space
                let [h1, s1, v1, a1] = self.to_hsva();
                let [h2, s2, v2, a2] = color.to_hsva();
                
                // Handle hue interpolation specially to go the shorter way around the circle
                let h = interpolate_hue(h1, h2, t);
                let s = s1 * (1.0 - t) + s2 * t;
                let v = v1 * (1.0 - t) + v2 * t;
                let a = a1 * (1.0 - t) + a2 * t;
                
                Self::from_hsva(h, s, v, a)
            },
            #[cfg(feature = "lab")]
            InterpolationSpace::LAB => {
                // Convert both colors to Lab space
                let (l1, a1, b1) = self.to_lab();
                let (l2, a2, b2) = color.to_lab();
                
                // Interpolate in Lab space
                let l = l1 * (1.0 - t) + l2 * t;
                let a = a1 * (1.0 - t) + a2 * t;
                let b = b1 * (1.0 - t) + b2 * t;
                let alpha = self.a * (1.0 - t) + color.a * t;
                
                // Convert back to RGB
                Self::from_laba(l, a, b, alpha)
            },
            #[cfg(feature = "lab")]
            InterpolationSpace::LCH => {
                // Convert both colors to LCh space
                let (l1, c1, h1) = self.to_lch();
                let (l2, c2, h2) = color.to_lch();
                
                // Interpolate in LCh space with special handling for hue
                let l = l1 * (1.0 - t) + l2 * t;
                let c = c1 * (1.0 - t) + c2 * t;
                let h = interpolate_hue(h1, h2, t);
                let alpha = self.a * (1.0 - t) + color.a * t;
                
                // Convert back to RGB
                Self::from_lcha(l, c, h, alpha)
            },
            InterpolationSpace::OKLAB => {
                // Convert both colors to Oklab space
                let (l1, a1, b1) = self.to_oklab();
                let (l2, a2, b2) = color.to_oklab();
                
                // Interpolate in Oklab space
                let l = l1 * (1.0 - t) + l2 * t;
                let a = a1 * (1.0 - t) + a2 * t;
                let b = b1 * (1.0 - t) + b2 * t;
                let alpha = self.a * (1.0 - t) + color.a * t;
                
                // Convert back to RGB
                Self::from_oklaba(l, a, b, alpha)
            },
            #[cfg(not(feature = "lab"))]
            _ => {
                // Fallback to RGB interpolation if LAB or LCH are not enabled
                let r = self.r * (1.0 - t) + color.r * t;
                let g = self.g * (1.0 - t) + color.g * t;
                let b = self.b * (1.0 - t) + color.b * t;
                let a = self.a * (1.0 - t) + color.a * t;
                
                Self::new(r, g, b, a)
            },
        }
    }

    /// Convert the color to Lab color space
    /// 
    /// Returns (L, a, b) where:
    /// - L: Lightness [0..100]
    /// - a: A axis [-128..127]
    /// - b: B axis [-128..127]
    #[cfg(feature = "lab")]
    pub fn to_lab(&self) -> (f32, f32, f32) {
        // Convert to XYZ first
        let (x, y, z) = self.to_xyz();
        
        // XYZ to Lab
        // Using D65 reference white
        let xn = 0.95047;
        let yn = 1.0;
        let zn = 1.08883;
        
        let x = x / xn;
        let y = y / yn;
        let z = z / zn;
        
        let fx = if x > 0.008856 {
            x.powf(1.0 / 3.0)
        } else {
            (7.787 * x) + (16.0 / 116.0)
        };
        
        let fy = if y > 0.008856 {
            y.powf(1.0 / 3.0)
        } else {
            (7.787 * y) + (16.0 / 116.0)
        };
        
        let fz = if z > 0.008856 {
            z.powf(1.0 / 3.0)
        } else {
            (7.787 * z) + (16.0 / 116.0)
        };
        
        let l = (116.0 * fy) - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);
        
        (l, a, b)
    }

    /// Convert the color to LCh color space
    /// 
    /// Returns (L, C, h) where:
    /// - L: Lightness [0..100]
    /// - C: Chroma [0..128]
    /// - h: Hue angle [0..360]
    #[cfg(feature = "lab")]
    pub fn to_lch(&self) -> (f32, f32, f32) {
        let (l, a, b) = self.to_lab();
        
        let c = (a * a + b * b).sqrt();
        let mut h = b.atan2(a) * (180.0 / std::f32::consts::PI);
        
        if h < 0.0 {
            h += 360.0;
        }
        
        (l, c, h)
    }

    /// Convert the color to XYZ color space
    #[cfg(feature = "lab")]
    fn to_xyz(&self) -> (f32, f32, f32) {
        // Convert from sRGB to linear RGB
        let r_linear = if self.r <= 0.04045 {
            self.r / 12.92
        } else {
            ((self.r + 0.055) / 1.055).powf(2.4)
        };
        
        let g_linear = if self.g <= 0.04045 {
            self.g / 12.92
        } else {
            ((self.g + 0.055) / 1.055).powf(2.4)
        };
        
        let b_linear = if self.b <= 0.04045 {
            self.b / 12.92
        } else {
            ((self.b + 0.055) / 1.055).powf(2.4)
        };
        
        // Linear RGB to XYZ using sRGB/D65 matrix
        let x = 0.4124564 * r_linear + 0.3575761 * g_linear + 0.1804375 * b_linear;
        let y = 0.2126729 * r_linear + 0.7151522 * g_linear + 0.0721750 * b_linear;
        let z = 0.0193339 * r_linear + 0.1191920 * g_linear + 0.9503041 * b_linear;
        
        (x, y, z)
    }

    /// Convert the color to Oklab color space
    pub fn to_oklab(&self) -> (f32, f32, f32) {
        // Convert from sRGB to linear RGB
        let r_linear = if self.r <= 0.04045 {
            self.r / 12.92
        } else {
            ((self.r + 0.055) / 1.055).powf(2.4)
        };
        
        let g_linear = if self.g <= 0.04045 {
            self.g / 12.92
        } else {
            ((self.g + 0.055) / 1.055).powf(2.4)
        };
        
        let b_linear = if self.b <= 0.04045 {
            self.b / 12.92
        } else {
            ((self.b + 0.055) / 1.055).powf(2.4)
        };
        
        // Linear RGB to LMS
        let l = 0.4122214708 * r_linear + 0.5363325363 * g_linear + 0.0514459929 * b_linear;
        let m = 0.2119034982 * r_linear + 0.6806995451 * g_linear + 0.1073969566 * b_linear;
        let s = 0.0883024619 * r_linear + 0.2817188376 * g_linear + 0.6299787005 * b_linear;
        
        // LMS to LAB
        let l_lab = 0.2104542553 * l.powf(1.0/3.0) + 0.7936177850 * m.powf(1.0/3.0) - 0.0040720468 * s.powf(1.0/3.0);
        let a_lab = 1.9779984951 * l.powf(1.0/3.0) - 2.4285922050 * m.powf(1.0/3.0) + 0.4505937099 * s.powf(1.0/3.0);
        let b_lab = 0.0259040371 * l.powf(1.0/3.0) + 0.7827717662 * m.powf(1.0/3.0) - 0.8086757660 * s.powf(1.0/3.0);
        
        (l_lab, a_lab, b_lab)
    }

    /// Convert the color to CSS `lab()` format string
    #[cfg(feature = "lab")]
    pub fn to_lab_string(&self) -> String {
        let (l, a, b) = self.to_lab();
        
        let l_rounded = l.round() as i32;
        let a_rounded = a.round() as i32;
        let b_rounded = b.round() as i32;
        
        if self.a < 1.0 {
            format!("lab({}% {} {} / {})", 
                l_rounded,
                a_rounded,
                b_rounded,
                round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("lab({}% {} {})", 
                l_rounded,
                a_rounded,
                b_rounded
            )
        }
    }
    
    /// Convert the color to CSS `lch()` format string
    #[cfg(feature = "lab")]
    pub fn to_lch_string(&self) -> String {
        let (l, c, h) = self.to_lch();
        
        let l_rounded = l.round() as i32;
        let c_rounded = c.round() as i32;
        let h_rounded = h.round() as i32;
        
        if self.a < 1.0 {
            format!("lch({}% {} {}deg / {})", 
                l_rounded,
                c_rounded,
                h_rounded,
                round_to_decimal_places(self.a, 2)
            )
        } else {
            format!("lch({}% {} {}deg)", 
                l_rounded,
                c_rounded,
                h_rounded
            )
        }
    }
}

/// Options for readability testing
#[derive(Debug, Clone)]
pub struct ReadableOptions {
    /// Compliance level: "AA" or "AAA"
    pub level: String,
    /// Whether the text is large (>= 18pt or 14pt bold)
    pub large: bool,
}

impl Default for ReadableOptions {
    fn default() -> Self {
        Self {
            level: "AA".to_string(),
            large: false,
        }
    }
}

impl FromStr for BigColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse(s)
    }
}

impl Default for BigColor {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl fmt::Display for BigColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGBA({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl From<peniko::Color> for BigColor {
    fn from(color: peniko::Color) -> Self {
        let rgba8 = color.to_rgba8();
        Self::new(
            rgba8.r as f32 / 255.0,
            rgba8.g as f32 / 255.0,
            rgba8.b as f32 / 255.0,
            rgba8.a as f32 / 255.0
        )
    }
}

impl From<BigColor> for peniko::Color {
    fn from(color: BigColor) -> Self {
        color.to_peniko_color()
    }
}

impl From<&BigColor> for peniko::Color {
    fn from(color: &BigColor) -> Self {
        color.to_peniko_color()
    }
}

// Utility functions copied from csscolor.rs to make BigColor independent
fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = (t % 360.0 + 360.0) % 360.0;
    
    if t < 60.0 {
        return p + (q - p) * t / 60.0;
    }
    if t < 180.0 {
        return q;
    }
    if t < 240.0 {
        return p + (q - p) * (240.0 - t) / 60.0;
    }
    
    p
}

// h = 0..360
// s, l = 0..1
// r, g, b = 0..1
#[allow(dead_code)]
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

#[allow(dead_code)]
fn hsv_to_hsl(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let l = (2.0 - s) * v / 2.0;

    let s = if l != 0.0 {
        if l == 1.0 {
            0.0
        } else if l < 0.5 {
            s * v / (l * 2.0)
        } else {
            s * v / (2.0 - l * 2.0)
        }
    } else {
        s
    };

    (h, s, l)
}

#[allow(dead_code)]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let (h, s, l) = hsv_to_hsl(h, s, v);
    hsl_to_rgb(h, s, l)
}

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let v = r.max(g.max(b));
    let d = v - r.min(g.min(b));

    if d == 0.0 {
        return (0.0, 0.0, v);
    }

    let s = d / v;
    let dr = (v - r) / d;
    let dg = (v - g) / d;
    let db = (v - b) / d;

    let h = if r == v {
        db - dg
    } else if g == v {
        2.0 + dr - db
    } else {
        4.0 + dg - dr
    };

    let h = (h * 60.0) % 360.0;
    (normalize_angle(h), s, v)
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let l = (max + min) / 2.0;

    if min == max {
        return (0.0, 0.0, l);
    }

    let d = max - min;

    let s = if l < 0.5 {
        d / (max + min)
    } else {
        d / (2.0 - max - min)
    };

    let dr = (max - r) / d;
    let dg = (max - g) / d;
    let db = (max - b) / d;

    let h = if r == max {
        db - dg
    } else if g == max {
        2.0 + dr - db
    } else {
        4.0 + dg - dr
    };

    let h = (h * 60.0) % 360.0;
    (normalize_angle(h), s, l)
}

#[inline]
fn normalize_angle(t: f32) -> f32 {
    let mut t = t % 360.0;
    if t < 0.0 {
        t += 360.0;
    }
    t
}

#[allow(dead_code)]
fn clamp0_1(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

#[allow(dead_code)]
fn modulo(x: f32, n: f32) -> f32 {
    (x % n + n) % n
}

/// Utility function to round a float to a specific number of decimal places
fn round_to_decimal_places(value: f32, places: u32) -> f32 {
    let multiplier = 10_f32.powi(places as i32);
    (value * multiplier).round() / multiplier
}

// Blend functions for individual channels
fn blend_multiply(base: f32, blend: f32, amount: f32) -> f32 {
    let result = base * blend;
    base * (1.0 - amount) + result * amount
}

fn blend_screen(base: f32, blend: f32, amount: f32) -> f32 {
    let result = 1.0 - (1.0 - base) * (1.0 - blend);
    base * (1.0 - amount) + result * amount
}

fn blend_overlay(base: f32, blend: f32, amount: f32) -> f32 {
    let result = if base < 0.5 {
        2.0 * base * blend
    } else {
        1.0 - 2.0 * (1.0 - base) * (1.0 - blend)
    };
    base * (1.0 - amount) + result * amount
}

fn blend_darken(base: f32, blend: f32, amount: f32) -> f32 {
    let result = base.min(blend);
    base * (1.0 - amount) + result * amount
}

fn blend_lighten(base: f32, blend: f32, amount: f32) -> f32 {
    let result = base.max(blend);
    base * (1.0 - amount) + result * amount
}

fn blend_color_dodge(base: f32, blend: f32, amount: f32) -> f32 {
    let result = if blend >= 1.0 {
        1.0
    } else if base == 0.0 {
        0.0
    } else {
        (base / (1.0 - blend)).min(1.0)
    };
    base * (1.0 - amount) + result * amount
}

fn blend_color_burn(base: f32, blend: f32, amount: f32) -> f32 {
    let result = if blend <= 0.0 {
        0.0
    } else if base >= 1.0 {
        1.0
    } else {
        1.0 - ((1.0 - base) / blend).min(1.0)
    };
    base * (1.0 - amount) + result * amount
}

fn blend_hard_light(base: f32, blend: f32, amount: f32) -> f32 {
    let result = if blend < 0.5 {
        2.0 * base * blend
    } else {
        1.0 - 2.0 * (1.0 - base) * (1.0 - blend)
    };
    base * (1.0 - amount) + result * amount
}

fn blend_soft_light(base: f32, blend: f32, amount: f32) -> f32 {
    let d = if base <= 0.25 {
        ((16.0 * base - 12.0) * base + 4.0) * base
    } else {
        base.sqrt()
    };
    
    let result = if blend <= 0.5 {
        base - (1.0 - 2.0 * blend) * base * (1.0 - base)
    } else {
        base + (2.0 * blend - 1.0) * (d - base)
    };
    
    base * (1.0 - amount) + result * amount
}

fn blend_difference(base: f32, blend: f32, amount: f32) -> f32 {
    let result = (base - blend).abs();
    base * (1.0 - amount) + result * amount
}

fn blend_exclusion(base: f32, blend: f32, amount: f32) -> f32 {
    let result = base + blend - 2.0 * base * blend;
    base * (1.0 - amount) + result * amount
}

/// Interpolate between two hue angles, taking the shorter path around the color wheel
fn interpolate_hue(h1: f32, h2: f32, t: f32) -> f32 {
    let mut delta = h2 - h1;
    
    if delta > 180.0 {
        delta -= 360.0;
    } else if delta < -180.0 {
        delta += 360.0;
    }
    
    let h = (h1 + delta * t) % 360.0;
    if h < 0.0 {
        h + 360.0
    } else {
        h
    }
} 