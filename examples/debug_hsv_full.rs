use bigcolor::{BigColor, ColorFormat};
use bigcolor::color_space::{rgb_to_hsv, hsv_to_rgb};

fn main() {
    // Test raw rgb_to_hsv function with red
    let red_rgb = (255, 0, 0);
    let hsv = rgb_to_hsv(red_rgb.0, red_rgb.1, red_rgb.2);
    println!("Direct RGB to HSV conversion:");
    println!("RGB (255, 0, 0) -> HSV: h={}, s={}, v={}", hsv.h, hsv.s, hsv.v);
    
    // Test hsv_to_rgb function with red
    let red_hsv_to_rgb = hsv_to_rgb(0.0, 1.0, 1.0);
    println!("\nDirect HSV to RGB conversion:");
    println!("HSV (0, 1, 1) -> RGB: r={}, g={}, b={}", red_hsv_to_rgb.r, red_hsv_to_rgb.g, red_hsv_to_rgb.b);
    
    // Test BigColor with string input
    let color_from_hsv_string = BigColor::new("hsv(0, 100%, 100%)");
    println!("\nBigColor from HSV string:");
    println!("Input: hsv(0, 100%, 100%)");
    println!("Format: {:?}", color_from_hsv_string.get_format());
    println!("RGB: {:?}", color_from_hsv_string.to_rgb());
    println!("HSV: {:?}", color_from_hsv_string.to_hsv());
    println!("HSV String: {}", color_from_hsv_string.to_hsv_string());
    
    // Test BigColor from direct HSV values
    let color_from_hsv_values = BigColor::from_hsv(0.0, 1.0, 1.0, 1.0);
    println!("\nBigColor::from_hsv(0.0, 1.0, 1.0, 1.0):");
    println!("Format: {:?}", color_from_hsv_values.get_format());
    println!("RGB: {:?}", color_from_hsv_values.to_rgb());
    println!("HSV: {:?}", color_from_hsv_values.to_hsv());
    println!("HSV String: {}", color_from_hsv_values.to_hsv_string());
    println!("Original input: {}", color_from_hsv_values.get_original_input());
    
    // Test BigColor from RGB and round-trip
    let color_from_rgb = BigColor::from_rgb(255, 0, 0, 1.0);
    println!("\nBigColor::from_rgb(255, 0, 0, 1.0):");
    println!("Format: {:?}", color_from_rgb.get_format());
    println!("RGB: {:?}", color_from_rgb.to_rgb());
    println!("HSV: {:?}", color_from_rgb.to_hsv());
    println!("HSV String: {}", color_from_rgb.to_hsv_string());
    
    // Test OKLCH values
    println!("\nOKLCH values for comparison:");
    println!("From HSV string: {:?}", color_from_hsv_string.to_oklch());
    println!("From HSV values: {:?}", color_from_hsv_values.to_oklch());
    println!("From RGB values: {:?}", color_from_rgb.to_oklch());
} 