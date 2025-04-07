use bigcolor::BigColor;

fn main() {
    println!("Testing custom color format parsing\n");
    
    // Test RGB percentage format
    let rgb_percent = "rgb(6%, 43%, 96%)";
    let color = BigColor::new(rgb_percent);
    println!("Original: {}", rgb_percent);
    println!("Is valid: {}", color.is_valid());
    println!("RGB: {:?}", color.to_rgb());
    println!("Format: {:?}", color.get_format());
    println!("----------------");
    
    // Test space-separated HSL format
    let hsl_space = "0 0% 12%";
    let color = BigColor::new(hsl_space);
    println!("Original: {}", hsl_space);
    println!("Is valid: {}", color.is_valid());
    println!("RGB: {:?}", color.to_rgb());
    println!("HSL: {}", color.to_hsl_string());
    println!("Format: {:?}", color.get_format());
    println!("----------------");
    
    // Test CSS compatibility
    println!("CSS compatibility for non-standard formats:");
    
    // HSV format - should be converted to RGB for CSS
    let hsv = BigColor::new("hsv(217, 89%, 96%)");
    println!("HSV: {}", hsv.to_hsv_string());
    println!("RGB for CSS: {}", hsv.to_rgb_string());
    println!("----------------");
    
    // HSB format - should be converted to RGB for CSS
    let hsb = BigColor::new("hsb(217, 89%, 96%)");
    println!("HSB: {}", hsb.to_hsb_string());
    println!("RGB for CSS: {}", hsb.to_rgb_string());
    println!("----------------");
    
    // CMYK format - should be converted to RGB for CSS
    let cmyk = BigColor::new("cmyk(100%, 50%, 0%, 20%)");
    println!("CMYK: {}", cmyk.to_cmyk_string());
    println!("RGB for CSS: {}", cmyk.to_rgb_string());
    println!("----------------");
} 