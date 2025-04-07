use bigcolor::BigColor;

fn main() {
    println!("Testing CMYK color format conversion\n");
    
    // Test with a few different colors
    let test_colors = vec![
        "#FF0000", // Red
        "#00FF00", // Green
        "#0000FF", // Blue
        "#FFFF00", // Yellow
        "#00FFFF", // Cyan
        "#FF00FF", // Magenta
        "#000000", // Black
        "#FFFFFF", // White
    ];
    
    for color_str in test_colors {
        let color = BigColor::new(color_str);
        let cmyk = color.to_cmyk();
        println!("Original: {}", color_str);
        println!("RGB: {:?}", color.to_rgb());
        println!("CMYK: c={}%, m={}%, y={}%, k={}%", 
            cmyk.c.round() as i32,
            cmyk.m.round() as i32, 
            cmyk.y.round() as i32, 
            cmyk.k.round() as i32
        );
        println!("CMYK string: {}", color.to_cmyk_string());
        
        // Test round-trip conversion
        let cmyk_color = BigColor::from_cmyk(cmyk.c, cmyk.m, cmyk.y, cmyk.k, 1.0);
        let rgb_roundtrip = cmyk_color.to_rgb();
        println!("RGB from CMYK: r={}, g={}, b={}", rgb_roundtrip.r, rgb_roundtrip.g, rgb_roundtrip.b);
        println!("----------------");
    }
    
    // Test creating a color directly from CMYK
    println!("\nCreating a color directly from CMYK values:");
    let direct_cmyk = BigColor::from_cmyk(0.0, 100.0, 100.0, 0.0, 1.0); // Red
    println!("CMYK: c=0%, m=100%, y=100%, k=0%");
    println!("RGB: {:?}", direct_cmyk.to_rgb());
    println!("HEX: {}", direct_cmyk.to_hex_string(false));
    
    // Test CMYK parsing
    println!("\nParsing a CMYK string:");
    let cmyk_string = "cmyk(0%, 100%, 100%, 0%)"; // Red
    let parsed_color = BigColor::new(cmyk_string);
    println!("CMYK string: {}", cmyk_string);
    println!("RGB: {:?}", parsed_color.to_rgb());
    println!("HEX: {}", parsed_color.to_hex_string(false));
    println!("Format: {:?}", parsed_color.get_format());
} 