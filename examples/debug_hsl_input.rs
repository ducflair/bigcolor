fn main() {
    let hsl_string = "hsl(0, 100%, 50%)";
    println!("Input string: {}", hsl_string);
    
    let color = bigcolor::BigColor::new(hsl_string);
    
    println!("Parsed color:");
    println!("RGB: {:?}", color.to_rgb());
    println!("HSL: {}", color.to_hsl_string());
    println!("Original input: {}", color.get_original_input());
    println!("Format: {:?}", color.get_format());
    println!("Is valid: {}", color.is_valid());
    
    // Try creating from HSL values directly
    let from_values = bigcolor::BigColor::from_hsl(0.0, 1.0, 0.5, 1.0);
    println!("\nCreated from values (h=0, s=1.0, l=0.5):");
    println!("RGB: {:?}", from_values.to_rgb());
    println!("HSL: {}", from_values.to_hsl_string());
    println!("Format: {:?}", from_values.get_format());
} 