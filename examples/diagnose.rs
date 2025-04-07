use bigcolor::BigColor;

fn main() {
    // Test with three basic colors
    let colors = vec!["#FF0000", "#00FF00", "#0000FF"];
    
    for color_str in colors {
        let color = BigColor::new(color_str);
        println!("\nTesting color: {}", color_str);
        
        // Get the converted values
        let rgb = color.to_rgb();
        let hsl = color.to_hsl();
        let oklch = color.to_oklch();
        
        println!("OKLCH: l={:.2}, c={:.2}, h={:.1}", oklch.l, oklch.c, oklch.h);
        println!("RGB: r={}, g={}, b={}", rgb.r, rgb.g, rgb.b);
        println!("HSL: h={:.1}, s={:.1}%, l={:.1}%", hsl.h, hsl.s * 100.0, hsl.l * 100.0);
        
        // Test string representations
        println!("RGB string: {}", color.to_rgb_string());
        println!("HSL string: {}", color.to_hsl_string());
        println!("HEX string: {}", color.to_hex_string(false));
    }
} 