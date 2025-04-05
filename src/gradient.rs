use crate::parser::ParseColorError;
use crate::solid::SolidColor;
use std::fmt;

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
    pub fn from_css_string(css_string: &str) -> Result<Self, ParseColorError> {
        let css_string = css_string.trim();
        
        // Extract the gradient type and content
        if !css_string.contains("gradient(") || !css_string.ends_with(")") {
            return Err(ParseColorError::InvalidGradient);
        }
        
        let gradient_type;
        let content;
        
        if css_string.starts_with("linear-gradient(") {
            gradient_type = GradientType::Linear;
            content = &css_string["linear-gradient(".len()..css_string.len()-1];
        } else if css_string.starts_with("repeating-linear-gradient(") {
            gradient_type = GradientType::Linear;
            content = &css_string["repeating-linear-gradient(".len()..css_string.len()-1];
        } else if css_string.starts_with("radial-gradient(") {
            gradient_type = GradientType::Radial;
            content = &css_string["radial-gradient(".len()..css_string.len()-1];
        } else if css_string.starts_with("repeating-radial-gradient(") {
            gradient_type = GradientType::Radial;
            content = &css_string["repeating-radial-gradient(".len()..css_string.len()-1];
        } else if css_string.starts_with("conic-gradient(") {
            gradient_type = GradientType::Conic;
            content = &css_string["conic-gradient(".len()..css_string.len()-1];
        } else if css_string.starts_with("repeating-conic-gradient(") {
            gradient_type = GradientType::Conic;
            content = &css_string["repeating-conic-gradient(".len()..css_string.len()-1];
        } else {
            return Err(ParseColorError::InvalidGradient);
        }
        
        // Split the content into parts
        let parts: Vec<&str> = content.split(',').collect();
        if parts.is_empty() {
            return Err(ParseColorError::InvalidGradient);
        }
        
        // Parse gradient parameters
        let mut angle = 0.0;
        let mut center = (0.5, 0.5);
        let radius = 0.5;
        let mut start_point = (0.0, 0.0);
        let mut end_point = (1.0, 1.0);
        let mut stops = Vec::new();
        let extend = if css_string.contains("repeating-") {
            GradientExtend::Repeat
        } else {
            GradientExtend::Pad
        };
        
        // Process the first part to check for direction/angle
        let mut start_index = 0;
        if parts.len() > 1 {
            let first_part = parts[0].trim();
            
            // Handle linear gradient angle/direction
            if gradient_type == GradientType::Linear {
                if first_part.ends_with("deg") || first_part.contains("rad") || 
                   first_part.contains("grad") || first_part.contains("turn") {
                    // Parse angle
                    if let Some(angle_str) = first_part.split_whitespace().next() {
                        if angle_str.ends_with("deg") {
                            if let Ok(deg) = angle_str[..angle_str.len()-3].parse::<f32>() {
                                angle = deg;
                                // Calculate start and end points based on angle
                                let rad = angle * std::f32::consts::PI / 180.0;
                                start_point = (0.5, 0.5);
                                end_point = (
                                    0.5 + 0.5 * rad.sin(),
                                    0.5 - 0.5 * rad.cos()
                                );
                            }
                        }
                        // Handle other angle units (rad, grad, turn) similarly
                    }
                    start_index = 1;
                } else if first_part.starts_with("to ") {
                    // Handle "to top", "to bottom", etc.
                    match first_part.trim() {
                        "to top" => {
                            start_point = (0.5, 1.0);
                            end_point = (0.5, 0.0);
                            angle = 0.0;
                        },
                        "to right" => {
                            start_point = (0.0, 0.5);
                            end_point = (1.0, 0.5);
                            angle = 90.0;
                        },
                        "to bottom" => {
                            start_point = (0.5, 0.0);
                            end_point = (0.5, 1.0);
                            angle = 180.0;
                        },
                        "to left" => {
                            start_point = (1.0, 0.5);
                            end_point = (0.0, 0.5);
                            angle = 270.0;
                        },
                        "to top right" | "to right top" => {
                            start_point = (0.0, 1.0);
                            end_point = (1.0, 0.0);
                            angle = 45.0;
                        },
                        "to bottom right" | "to right bottom" => {
                            start_point = (0.0, 0.0);
                            end_point = (1.0, 1.0);
                            angle = 135.0;
                        },
                        "to bottom left" | "to left bottom" => {
                            start_point = (1.0, 0.0);
                            end_point = (0.0, 1.0);
                            angle = 225.0;
                        },
                        "to top left" | "to left top" => {
                            start_point = (1.0, 1.0);
                            end_point = (0.0, 0.0);
                            angle = 315.0;
                        },
                        _ => {}
                    }
                    start_index = 1;
                }
            }
            
            // Handle radial gradient parameters
            if gradient_type == GradientType::Radial && (
                first_part.contains("circle") || first_part.contains("ellipse") || first_part.contains("at")
            ) {
                if first_part.contains("at") {
                    let pos_parts: Vec<&str> = first_part.split("at").collect();
                    if pos_parts.len() > 1 {
                        let pos_str = pos_parts[1].trim();
                        if pos_str.contains('%') {
                            // Parse percentage values
                            let pos_values: Vec<&str> = pos_str.split_whitespace().collect();
                            if pos_values.len() >= 2 {
                                if let Ok(x) = pos_values[0].trim_end_matches('%').parse::<f32>() {
                                    if let Ok(y) = pos_values[1].trim_end_matches('%').parse::<f32>() {
                                        center = (x / 100.0, y / 100.0);
                                    }
                                }
                            }
                        } else if pos_str == "center" {
                            center = (0.5, 0.5);
                        } else if pos_str.contains("center") {
                            // Handle "center top", "left center", etc.
                            if pos_str.contains("top") {
                                center.1 = 0.0;
                            } else if pos_str.contains("bottom") {
                                center.1 = 1.0;
                            }
                            
                            if pos_str.contains("left") {
                                center.0 = 0.0;
                            } else if pos_str.contains("right") {
                                center.0 = 1.0;
                            }
                        }
                    }
                }
                start_index = 1;
            }
            
            // Handle conic gradient parameters
            if gradient_type == GradientType::Conic && (
                first_part.contains("from") || first_part.contains("at")
            ) {
                if first_part.contains("from") && first_part.contains("at") {
                    let from_part = first_part.split("from").collect::<Vec<&str>>();
                    let at_part = first_part.split("at").collect::<Vec<&str>>();
                    
                    if from_part.len() > 1 && at_part.len() > 1 {
                        // Parse angle
                        let angle_str = from_part[1].split("at").next().unwrap_or("0deg").trim();
                        if angle_str.ends_with("deg") {
                            if let Ok(deg) = angle_str[..angle_str.len()-3].parse::<f32>() {
                                angle = deg;
                            }
                        }
                        
                        // Parse center position
                        let pos_str = at_part[1].trim();
                        if pos_str.contains('%') {
                            let pos_values: Vec<&str> = pos_str.split_whitespace().collect();
                            if pos_values.len() >= 2 {
                                if let Ok(x) = pos_values[0].trim_end_matches('%').parse::<f32>() {
                                    if let Ok(y) = pos_values[1].trim_end_matches('%').parse::<f32>() {
                                        center = (x / 100.0, y / 100.0);
                                    }
                                }
                            }
                        } else if pos_str == "center" {
                            center = (0.5, 0.5);
                        }
                    }
                }
                start_index = 1;
            }
        }
        
        // Parse color stops
        for i in start_index..parts.len() {
            let part = parts[i].trim();
            
            // Find position of the color
            let mut position = -1.0;
            let mut color_str = part;
            
            if part.contains(' ') {
                let stop_parts: Vec<&str> = part.rsplitn(2, ' ').collect();
                if stop_parts.len() == 2 {
                    let pos_str = stop_parts[0].trim();
                    if pos_str.ends_with('%') {
                        if let Ok(pos) = pos_str[..pos_str.len()-1].parse::<f32>() {
                            position = pos / 100.0;
                            color_str = stop_parts[1].trim();
                        }
                    }
                }
            }
            
            // Parse the color
            if let Ok(color) = SolidColor::parse(color_str) {
                // If position wasn't specified, calculate based on stop index
                if position < 0.0 {
                    position = (i - start_index) as f32 / (parts.len() - start_index) as f32;
                }
                
                stops.push(ColorStop::new(color, position));
            }
        }
        
        // If no stops were parsed successfully, return an error
        if stops.is_empty() {
            return Err(ParseColorError::InvalidGradient);
        }
        
        // Create the appropriate gradient type
        match gradient_type {
            GradientType::Linear => {
                Ok(Self::linear(stops, start_point, end_point, extend))
            },
            GradientType::Radial => {
                Ok(Self::radial(stops, center, radius, extend))
            },
            GradientType::Conic => {
                Ok(Self::conic(stops, center, angle, extend))
            }
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
        // Skip rendering if there are no stops
        if self.stops.is_empty() {
            return write!(f, "linear-gradient(transparent, transparent)");
        }
        
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
                let prefix = if self.extend == GradientExtend::Repeat {
                    "repeating-radial-gradient("
                } else {
                    "radial-gradient("
                };
                
                write!(f, "{}circle at {}% {}%, ", 
                    prefix,
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
                let prefix = if self.extend == GradientExtend::Repeat {
                    "repeating-conic-gradient("
                } else {
                    "conic-gradient("
                };
                
                write!(f, "{}from {}deg at {}% {}%, ", 
                    prefix,
                    self.angle as i32,
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