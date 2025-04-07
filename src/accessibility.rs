use crate::BigColor;

/// Determines whether a color is considered "light" (and should have dark text on it)
/// or "dark" (and should have light text on it)
pub fn is_light(color: &BigColor) -> bool {
    // Check if the color is light based on the OKLCH lightness
    // We need to use a direct check to avoid recursion with BigColor.is_light()
    color.to_oklch().l >= 0.5
}

/// Generates a contrast color for the given color with a specified intensity
/// 
/// # Arguments
/// 
/// * `color` - The base color
/// * `intensity` - A value between 0 and 1 where:
///    - 0: Minimum contrast (closer to original color)
///    - 1: Maximum contrast (black or white)
/// 
/// # Returns
/// 
/// A BigColor object representing the contrast color
pub fn get_contrast_color(color: &BigColor, intensity: f32) -> BigColor {
    let is_light_color = is_light(color);
    
    // Determine base colors for light and dark modes
    let (light_r, light_g, light_b) = (0, 0, 0); // Black for light backgrounds
    let (dark_r, dark_g, dark_b) = (255, 255, 255); // White for dark backgrounds
    
    // Interpolate between medium gray and extreme contrast based on intensity
    let intensity = intensity.max(0.0).min(1.0); // Clamp intensity to [0,1]

    if is_light_color {
        // For light backgrounds: interpolate between medium gray and black
        let r = ((128.0 * (1.0 - intensity)) + (light_r as f32 * intensity)) as u8;
        let g = ((128.0 * (1.0 - intensity)) + (light_g as f32 * intensity)) as u8;
        let b = ((128.0 * (1.0 - intensity)) + (light_b as f32 * intensity)) as u8;
        BigColor::from_rgb(r, g, b, 1.0)
    } else {
        // For dark backgrounds: interpolate between medium gray and white
        let r = ((128.0 * (1.0 - intensity)) + (dark_r as f32 * intensity)) as u8;
        let g = ((128.0 * (1.0 - intensity)) + (dark_g as f32 * intensity)) as u8;
        let b = ((128.0 * (1.0 - intensity)) + (dark_b as f32 * intensity)) as u8;
        BigColor::from_rgb(r, g, b, 1.0)
    }
}

/// Returns a contrast ratio between two colors according to WCAG standards
/// 
/// The ratio ranges from 1:1 (no contrast) to 21:1 (max contrast)
/// 
/// According to WCAG 2.1:
/// - 4.5:1 is the minimum for normal text (AA)
/// - 3:1 is the minimum for large text (AA)
/// - 7:1 is the enhanced minimum for normal text (AAA)
/// - 4.5:1 is the enhanced minimum for large text (AAA)
pub fn get_contrast_ratio(color1: &BigColor, color2: &BigColor) -> f32 {
    // Calculate relative luminance for both colors
    let l1 = color1.get_luminance();
    let l2 = color2.get_luminance();
    
    // Calculate contrast ratio
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Calculates luminance according to WCAG formula
/// This is a helper function used by get_contrast_ratio
fn calculate_luminance(color: &BigColor) -> f32 {
    color.get_luminance()
}

/// Converts an sRGB color component to linear RGB
/// This is a helper function for luminance calculations
fn to_linear(component: f32) -> f32 {
    if component <= 0.03928 {
        component / 12.92
    } else {
        ((component + 0.055) / 1.055).powf(2.4)
    }
} 