use crate::parser::ParseColorError;
use crate::solid::SolidColor;
use std::fmt;
use std::str::FromStr;

/// Represents a color stop for gradients
#[derive(Debug, Clone)]
pub struct ColorStop {
    /// The color of the stop
    pub color: SolidColor,
    /// The position of the stop (0.0 to 1.0)
    pub position: f32,
}

impl ColorStop {
    /// Create a new color stop
    pub fn new(color: SolidColor, position: f32) -> Self {
        Self {
            color,
            position: position.clamp(0.0, 1.0),
        }
    }
}

/// Type of gradient
#[derive(Debug, Clone, PartialEq)]
pub enum GradientType {
    /// Linear gradient
    Linear,
    /// Radial gradient
    Radial,
    /// Conic gradient
    Conic,
}

/// How the gradient extends beyond its bounds
#[derive(Debug, Clone, PartialEq)]
pub enum GradientExtend {
    /// Extend the final color
    Pad,
    /// Repeat the gradient
    Repeat,
    /// Mirror the gradient
    Reflect,
}

/// Represents a gradient
#[derive(Debug, Clone)]
pub struct Gradient {
    /// The type of gradient
    pub gradient_type: GradientType,
    /// Color stops
    pub stops: Vec<ColorStop>,
    /// Start point (for linear gradients)
    pub start_point: (f32, f32),
    /// End point (for linear gradients)
    pub end_point: (f32, f32),
    /// Center (for radial and conic gradients)
    pub center: (f32, f32),
    /// Radius (for radial gradients)
    pub radius: f32,
    /// Start angle in degrees (for conic gradients)
    pub angle: f32,
    /// How the gradient extends
    pub extend: GradientExtend,
}

impl Gradient {
    /// Create a new linear gradient
    pub fn linear(
        stops: Vec<ColorStop>,
        start_point: (f32, f32),
        end_point: (f32, f32),
        extend: GradientExtend,
    ) -> Self {
        Self {
            gradient_type: GradientType::Linear,
            stops,
            start_point,
            end_point,
            center: (0.5, 0.5),
            radius: 0.5,
            angle: 0.0,
            extend,
        }
    }

    /// Create a new radial gradient
    pub fn radial(
        stops: Vec<ColorStop>,
        center: (f32, f32),
        radius: f32,
        extend: GradientExtend,
    ) -> Self {
        Self {
            gradient_type: GradientType::Radial,
            stops,
            start_point: (0.0, 0.0),
            end_point: (1.0, 1.0),
            center,
            radius,
            angle: 0.0,
            extend,
        }
    }

    /// Create a new conic gradient
    pub fn conic(
        stops: Vec<ColorStop>,
        center: (f32, f32),
        angle: f32,
        extend: GradientExtend,
    ) -> Self {
        Self {
            gradient_type: GradientType::Conic,
            stops,
            start_point: (0.0, 0.0),
            end_point: (1.0, 1.0),
            center,
            radius: 0.5,
            angle,
            extend,
        }
    }

    /// Create a circular gradient (alias for radial)
    pub fn circular(
        stops: Vec<ColorStop>,
        center: (f32, f32),
        radius: f32,
        extend: GradientExtend,
    ) -> Self {
        Self::radial(stops, center, radius, extend)
    }

    /// Parse a CSS gradient string
    pub fn from_css_string(css_string: &str) -> Result<Self, crate::parser::ParseColorError> {
        // Basic parsing of CSS gradient strings
        let css_string = css_string.trim();
        
        if css_string.starts_with("linear-gradient(") {
            // Very basic implementation for demo purposes
            let mut stops = Vec::new();
            stops.push(ColorStop::new(SolidColor::new(1.0, 0.0, 0.0, 1.0), 0.0)); // Red at 0%
            stops.push(ColorStop::new(SolidColor::new(0.0, 0.0, 1.0, 1.0), 1.0)); // Blue at 100%
            
            Ok(Self::linear(
                stops,
                (0.0, 0.0),
                (1.0, 1.0),
                GradientExtend::Pad,
            ))
        } else if css_string.starts_with("radial-gradient(") {
            // Very basic implementation for demo purposes
            let mut stops = Vec::new();
            stops.push(ColorStop::new(SolidColor::new(1.0, 1.0, 0.0, 1.0), 0.0)); // Yellow at 0%
            stops.push(ColorStop::new(SolidColor::new(1.0, 0.0, 0.0, 1.0), 1.0)); // Red at 100%
            
            Ok(Self::radial(
                stops,
                (0.5, 0.5),
                0.5,
                GradientExtend::Pad,
            ))
        } else if css_string.starts_with("conic-gradient(") {
            // Very basic implementation for demo purposes
            let mut stops = Vec::new();
            stops.push(ColorStop::new(SolidColor::new(1.0, 0.0, 0.0, 1.0), 0.0)); // Red at 0%
            stops.push(ColorStop::new(SolidColor::new(1.0, 1.0, 0.0, 1.0), 0.33)); // Yellow at 33%
            stops.push(ColorStop::new(SolidColor::new(0.0, 0.0, 1.0, 1.0), 0.66)); // Blue at 66%
            stops.push(ColorStop::new(SolidColor::new(1.0, 0.0, 0.0, 1.0), 1.0)); // Red at 100%
            
            Ok(Self::conic(
                stops,
                (0.5, 0.5),
                0.0,
                GradientExtend::Pad,
            ))
        } else {
            Err(crate::parser::ParseColorError::InvalidGradient)
        }
    }

    /// Get a solid color representation at a specific position
    pub fn color_at(&self, position: f32) -> SolidColor {
        if self.stops.is_empty() {
            return SolidColor::new(0.0, 0.0, 0.0, 0.0);
        }
        
        if self.stops.len() == 1 {
            return self.stops[0].color.clone();
        }
        
        // Find the correct stops to interpolate between
        let mut lower_stop = &self.stops[0];
        let mut upper_stop = &self.stops[self.stops.len() - 1];
        
        for i in 0..self.stops.len() - 1 {
            if position >= self.stops[i].position && position <= self.stops[i + 1].position {
                lower_stop = &self.stops[i];
                upper_stop = &self.stops[i + 1];
                break;
            }
        }
        
        // Calculate the interpolation factor
        let range = upper_stop.position - lower_stop.position;
        let factor = if range == 0.0 { 0.0 } else { (position - lower_stop.position) / range };
        
        // Interpolate the color
        SolidColor::new(
            lower_stop.color.r * (1.0 - factor) + upper_stop.color.r * factor,
            lower_stop.color.g * (1.0 - factor) + upper_stop.color.g * factor,
            lower_stop.color.b * (1.0 - factor) + upper_stop.color.b * factor,
            lower_stop.color.a * (1.0 - factor) + upper_stop.color.a * factor,
        )
    }
    
    /// Get a complementary gradient (with colors from the opposite side of the color wheel)
    pub fn complementary(&self) -> Self {
        let mut new_stops = Vec::new();
        
        for stop in &self.stops {
            // Get HSL values for the color
            let [h, s, l, a] = stop.color.to_hsla();
            // Shift hue by 180 degrees
            let new_h = (h + 180.0) % 360.0;
            // Create new color with shifted hue
            let comp_color = SolidColor::from_hsla(new_h, s, l, a);
            new_stops.push(ColorStop::new(comp_color, stop.position));
        }
        
        Self {
            gradient_type: self.gradient_type.clone(),
            stops: new_stops,
            start_point: self.start_point,
            end_point: self.end_point,
            center: self.center,
            radius: self.radius,
            angle: self.angle,
            extend: self.extend.clone(),
        }
    }
}

#[cfg(feature = "peniko")]
impl Gradient {
    /// Convert to a peniko gradient (when the peniko feature is enabled)
    pub fn to_peniko_gradient(&self) -> peniko::Gradient {
        use peniko::{Color, Extend, Gradient, Point};
        
        // Convert color stops
        let peniko_stops: Vec<_> = self.stops
            .iter()
            .map(|stop| {
                let rgba = stop.color.to_rgba8();
                (stop.position, Color::rgba8(rgba.0, rgba.1, rgba.2, rgba.3))
            })
            .collect();
        
        // Convert extend mode
        let extend = match self.extend {
            GradientExtend::Pad => Extend::Pad,
            GradientExtend::Repeat => Extend::Repeat,
            GradientExtend::Reflect => Extend::Reflect,
        };
        
        // Create the appropriate gradient type
        match self.gradient_type {
            GradientType::Linear => Gradient::Linear {
                start: Point::new(self.start_point.0, self.start_point.1),
                end: Point::new(self.end_point.0, self.end_point.1),
                stops: peniko_stops,
                extend,
            },
            GradientType::Radial => Gradient::Radial {
                center: Point::new(self.center.0, self.center.1),
                radius: self.radius,
                stops: peniko_stops,
                extend,
            },
            GradientType::Conic => Gradient::Sweep {
                center: Point::new(self.center.0, self.center.1),
                angle: self.angle,
                stops: peniko_stops,
                extend,
            },
        }
    }
}

impl fmt::Display for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.gradient_type {
            GradientType::Linear => {
                let prefix = if self.extend == GradientExtend::Repeat {
                    "repeating-linear-gradient("
                } else {
                    "linear-gradient("
                };
                
                // Calculate angle from points
                let dx = self.end_point.0 - self.start_point.0;
                let dy = self.start_point.1 - self.end_point.1; // Y is flipped in CSS
                let angle_rad = f32::atan2(dx, dy);
                let angle_deg = angle_rad * 180.0 / std::f32::consts::PI;
                
                write!(f, "{}{}deg", prefix, angle_deg as i32)?;
                
                for stop in &self.stops {
                    write!(f, ", {} {}%", 
                        stop.color.to_rgb_string(), 
                        (stop.position * 100.0) as i32
                    )?;
                }
                
                write!(f, ")")
            },
            GradientType::Radial => {
                write!(f, "radial-gradient(circle at {}% {}%, ", 
                    (self.center.0 * 100.0) as i32, 
                    (self.center.1 * 100.0) as i32
                )?;
                
                for (i, stop) in self.stops.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}%", 
                        stop.color.to_rgb_string(), 
                        (stop.position * 100.0) as i32
                    )?;
                }
                
                write!(f, ")")
            },
            GradientType::Conic => {
                let angle_deg = self.angle * 180.0 / std::f32::consts::PI;
                
                write!(f, "conic-gradient(from {}deg at {}% {}%, ", 
                    angle_deg as i32,
                    (self.center.0 * 100.0) as i32, 
                    (self.center.1 * 100.0) as i32
                )?;
                
                for (i, stop) in self.stops.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}%", 
                        stop.color.to_rgb_string(), 
                        (stop.position * 100.0) as i32
                    )?;
                }
                
                write!(f, ")")
            }
        }
    }
} 