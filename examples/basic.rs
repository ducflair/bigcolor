use bigcolor::{BigColor, mix, equals, random};

fn main() {
    println!("BigColor Examples with OKLCH Foundation\n");

    // Create colors from different formats
    let red = BigColor::new("#FF0000");
    let green = BigColor::new("rgb(0, 255, 0)");
    let blue = BigColor::new("blue");
    let transparent_purple = BigColor::new("rgba(128, 0, 128, 0.5)");
    
    println!("=== Basic Color Formats ===");
    println!("Red as RGB: {}", red.to_rgb_string());
    println!("Red as HSL: {}", red.to_hsl_string());
    println!("Red as HSV: {}", red.to_hsv_string());
    println!("Red as HEX: {}", red.to_hex_string(false));
    
    // New color space representations
    println!("\n=== New Color Space Formats ===");
    println!("Red as OKLCH: {}", red.to_oklch_string());
    println!("Red as OKLab: {}", red.to_oklab_string());
    println!("Red as LCH: {}", red.to_lch_string());
    println!("Red as Lab: {}", red.to_lab_string());
    
    // Create colors directly using OKLCH
    println!("\n=== Creating Colors with OKLCH ===");
    let oklch_purple = BigColor::from_oklch(0.5, 0.2, 300.0, 1.0);
    println!("OKLCH Purple: {}", oklch_purple);
    println!("OKLCH Purple as RGB: {}", oklch_purple.to_rgb_string());
    
    // Create colors using LCH
    let lch_teal = BigColor::from_lch(70.0, 30.0, 180.0, 1.0);
    println!("LCH Teal: {}", lch_teal);
    println!("LCH Teal as RGB: {}", lch_teal.to_rgb_string());
    
    // Check properties
    println!("\n=== Color Properties ===");
    println!("Red is dark: {}", red.is_dark());
    println!("Green is light: {}", green.is_light());
    println!("Blue is valid: {}", blue.is_valid());
    
    // Color modifications - now directly manipulating OKLCH values
    println!("\n=== Color Modifications ===");
    let mut light_blue = blue.clone();
    light_blue.lighten(Some(20.0));
    println!("Lightened blue: {}", light_blue);
    
    let mut dark_green = green.clone();
    dark_green.darken(Some(20.0));
    println!("Darkened green: {}", dark_green);
    
    let mut desaturated_red = red.clone();
    desaturated_red.desaturate(Some(50.0));
    println!("Desaturated red: {}", desaturated_red);
    
    let mut bright_red = red.clone();
    bright_red.brighten(Some(20.0));
    println!("Brightened red: {}", bright_red);
    
    // Color combinations
    println!("\n=== Color Combinations ===");
    println!("Mix of red and blue: {}", mix(&red, &blue, Some(50.0)));
    println!("Red and red are equal: {}", equals(&red, &red));
    println!("Red and blue are equal: {}", equals(&red, &blue));
    
    // Color schemes
    println!("\n=== Color Schemes ===");
    println!("Complement of red: {}", red.complement());
    
    println!("\nTriad of red:");
    for color in red.triad() {
        println!("  {}", color);
    }
    
    println!("\nAnalogous of green:");
    for color in green.analogous(Some(3), Some(30)) {
        println!("  {}", color);
    }
    
    println!("\nSplit complement of blue:");
    for color in blue.split_complement() {
        println!("  {}", color);
    }
    
    println!("\nMonochromatic of purple:");
    for color in transparent_purple.monochromatic(Some(5)) {
        println!("  {}", color);
    }
    
    // Parsing examples with new color formats
    println!("\n=== Parsing New Color Formats ===");
    let oklch_from_string = BigColor::new("oklch(50% 0.2 240)");
    println!("OKLCH from string: {}", oklch_from_string);
    println!("As RGB: {}", oklch_from_string.to_rgb_string());
    
    let lab_from_string = BigColor::new("lab(70 20 -30)");
    println!("Lab from string: {}", lab_from_string);
    println!("As RGB: {}", lab_from_string.to_rgb_string());
    
    // Random color
    println!("\n=== Random Color ===");
    println!("Random color: {}", random());
    println!("Random color in OKLCH: {}", random().to_oklch_string());
} 