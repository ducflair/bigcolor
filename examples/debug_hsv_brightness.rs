use bigcolor::{BigColor, ColorFormat};

fn main() {
    println!("Testing HSV brightness with different colors:");
    
    // Test colors to check darkness issue
    let test_colors = vec![
        "red", "#ff0000", "rgb(255, 0, 0)", 
        "green", "#00ff00", "rgb(0, 255, 0)",
        "blue", "#0000ff", "rgb(0, 0, 255)",
        "yellow", "#ffff00", "rgb(255, 255, 0)",
        "purple", "#800080", "rgb(128, 0, 128)",
        "orange", "#ffa500", "rgb(255, 165, 0)",
        "deepskyblue", "#00bfff", "rgb(0, 191, 255)"
    ];
    
    for color_str in test_colors {
        let color = BigColor::new(color_str);
        let hsv = color.to_hsv();
        let rgb = color.to_rgb();
        
        // Create a new color directly from HSV and get its RGB
        let hsv_color = BigColor::from_hsv(hsv.h, hsv.s, hsv.v, 1.0);
        let hsv_rgb = hsv_color.to_rgb();
        
        println!("\nColor: {}", color_str);
        println!("Original RGB: r={}, g={}, b={}", rgb.r, rgb.g, rgb.b);
        println!("HSV: h={}, s={}%, v={}%", hsv.h, hsv.s * 100.0, hsv.v * 100.0);
        println!("Round-trip RGB from HSV: r={}, g={}, b={}", hsv_rgb.r, hsv_rgb.g, hsv_rgb.b);
        println!("Difference: r={}, g={}, b={}", 
            i16::abs(rgb.r as i16 - hsv_rgb.r as i16),
            i16::abs(rgb.g as i16 - hsv_rgb.g as i16),
            i16::abs(rgb.b as i16 - hsv_rgb.b as i16)
        );
    }
} 