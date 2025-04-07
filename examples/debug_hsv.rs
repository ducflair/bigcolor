fn main() {
    let hsv_string = "hsv(0, 100%, 100%)";
    println!("Input string: {}", hsv_string);
    
    let color = bigcolor::BigColor::new(hsv_string);
    
    println!("Parsed color:");
    println!("RGB: {:?}", color.to_rgb());
    println!("HSV: {:?}", color.to_hsv());
    println!("HSV String: {}", color.to_hsv_string());
    println!("Original input: {}", color.get_original_input());
    println!("Format: {:?}", color.get_format());
    println!("Is valid: {}", color.is_valid());
    
    // Try creating from HSV values directly
    let from_values = bigcolor::BigColor::from_hsv(0.0, 1.0, 1.0, 1.0);
    println!("\nCreated from values (h=0, s=1.0, v=1.0):");
    println!("RGB: {:?}", from_values.to_rgb());
    println!("HSV: {:?}", from_values.to_hsv());
    println!("HSV String: {}", from_values.to_hsv_string());
    println!("Format: {:?}", from_values.get_format());
} 