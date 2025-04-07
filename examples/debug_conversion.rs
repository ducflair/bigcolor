use bigcolor::BigColor;

fn main() {
    // Test with pure colors
    let colors = vec!["#FF0000", "#00FF00", "#0000FF"];
    
    for color_str in colors {
        let color = BigColor::new(color_str);
        println!("\nTesting color: {}", color_str);
        
        // Get the converted values
        let source_rgb = BigColor::from_rgb(
            u8::from_str_radix(&color_str[1..3], 16).unwrap(),
            u8::from_str_radix(&color_str[3..5], 16).unwrap(),
            u8::from_str_radix(&color_str[5..7], 16).unwrap(),
            1.0
        );
        
        println!("Source RGB: {:?}", source_rgb.to_rgb());
        println!("Source HSL: {}", source_rgb.to_hsl_string());
        
        // Check OKLCH conversion and round-trip
        let oklch = color.to_oklch();
        println!("OKLCH: l={:.4}, c={:.4}, h={:.2}", oklch.l, oklch.c, oklch.h);
        
        let round_trip = BigColor::from_oklch(oklch.l, oklch.c, oklch.h, oklch.alpha);
        println!("Round-trip RGB: {:?}", round_trip.to_rgb());
        println!("Round-trip HSL: {}", round_trip.to_hsl_string());
        println!("Round-trip HEX: {}", round_trip.to_hex_string(false));
    }
} 