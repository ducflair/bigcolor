use peniko::Color;

pub fn get_contrast_color(color_str: &str, extreme: bool) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the input color string
    let color = Srgb::from_format(parse_hex_color(color_str)?);
    
    // Convert to HSL to determine lightness
    let hsl: Hsl = color.into_color();
    let is_light = hsl.lightness > 0.5;
    
    // Determine the contrast color based on the extreme flag
    let result = if extreme {
        // Extreme contrast: return black for light colors, white for dark colors
        if is_light { "#000000" } else { "#ffffff" }
    } else {
        // Soft contrast: return dark gray for light colors, light gray for dark colors
        if is_light { "#c9c9c9" } else { "#474747" }
    };
    
    Ok(result.to_string())
}

// Helper function to parse hex color
fn parse_hex_color(hex: &str) -> Result<Srgb<u8>, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        Ok(Srgb::new(r, g, b))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
        Ok(Srgb::new(r, g, b))
    } else {
        Err("Invalid hex color format".into())
    }
}

pub fn hex_to_color(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Invalid hex color format".into());
    }
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok(Color::from_rgba8(r, g, b, 255))
}