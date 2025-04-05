use bigcolor::{BigColor, SolidColor, Gradient, ColorStop, GradientType, GradientExtend};
use std::str::FromStr;

fn main() {
    // Define some solid colors
    let red = SolidColor::new(1.0, 0.0, 0.0, 1.0);
    let blue = SolidColor::new(0.0, 0.0, 1.0, 1.0);
    let yellow = SolidColor::new(1.0, 1.0, 0.0, 1.0);

    println!("=== Basic Gradients ===");
    
    // Linear gradient from red to blue
    let mut stops = Vec::new();
    stops.push(ColorStop::new(red.clone(), 0.0));
    stops.push(ColorStop::new(blue.clone(), 1.0));
    
    let linear_gradient = Gradient::linear(
        stops,
        (0.0, 0.0),
        (1.0, 1.0),
        GradientExtend::Pad,
    );
    
    println!("Linear gradient:");
    println!("  Color at 0.0: {:?}", linear_gradient.color_at(0.0).to_hex_string());
    println!("  Color at 0.5: {:?}", linear_gradient.color_at(0.5).to_hex_string());
    println!("  Color at 1.0: {:?}", linear_gradient.color_at(1.0).to_hex_string());
    
    // Radial gradient from yellow at center to red at edge
    let mut stops = Vec::new();
    stops.push(ColorStop::new(yellow.clone(), 0.0));
    stops.push(ColorStop::new(red.clone(), 1.0));
    
    let radial_gradient = Gradient::radial(
        stops,
        (0.5, 0.5),
        0.5,
        GradientExtend::Pad,
    );
    
    println!("\nRadial gradient:");
    println!("  Color at 0.0: {:?}", radial_gradient.color_at(0.0).to_hex_string());
    println!("  Color at 0.5: {:?}", radial_gradient.color_at(0.5).to_hex_string());
    println!("  Color at 1.0: {:?}", radial_gradient.color_at(1.0).to_hex_string());
    
    // Conic gradient with red, yellow, blue
    let mut stops = Vec::new();
    stops.push(ColorStop::new(red.clone(), 0.0));
    stops.push(ColorStop::new(yellow.clone(), 0.33));
    stops.push(ColorStop::new(blue.clone(), 0.66));
    stops.push(ColorStop::new(red.clone(), 1.0));
    
    let conic_gradient = Gradient::conic(
        stops,
        (0.5, 0.5),
        0.0,
        GradientExtend::Pad,
    );
    
    println!("\nConic gradient:");
    println!("  Color at 0.0: {:?}", conic_gradient.color_at(0.0).to_hex_string());
    println!("  Color at 0.33: {:?}", conic_gradient.color_at(0.33).to_hex_string());
    println!("  Color at 0.66: {:?}", conic_gradient.color_at(0.66).to_hex_string());
    println!("  Color at 1.0: {:?}", conic_gradient.color_at(1.0).to_hex_string());
    
    // Test parsing CSS gradient string
    println!("\n=== CSS Gradient Parsing ===");
    let css_gradient = "linear-gradient(to right, red, blue)";
    match Gradient::from_css_string(css_gradient) {
        Ok(gradient) => {
            println!("Parsed CSS gradient: {}", css_gradient);
            println!("  Color at 0.0: {:?}", gradient.color_at(0.0).to_hex_string());
            println!("  Color at 0.5: {:?}", gradient.color_at(0.5).to_hex_string());
            println!("  Color at 1.0: {:?}", gradient.color_at(1.0).to_hex_string());
        },
        Err(e) => println!("Failed to parse CSS gradient: {:?}", e),
    }
    
    // Test complementary gradient
    println!("\n=== Complementary Gradient ===");
    let comp_gradient = linear_gradient.complementary();
    println!("Complementary gradient:");
    println!("  Color at 0.0: {:?}", comp_gradient.color_at(0.0).to_hex_string());
    println!("  Color at 0.5: {:?}", comp_gradient.color_at(0.5).to_hex_string());
    println!("  Color at 1.0: {:?}", comp_gradient.color_at(1.0).to_hex_string());
} 