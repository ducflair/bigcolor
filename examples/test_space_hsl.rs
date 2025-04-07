use bigcolor::BigColor;

fn main() {
    // Test space-separated HSL format
    let test_color = "0 0% 12%";
    let color = BigColor::new(&format!("hsl({}, {}%, {}%)", 0, 0, 12));
    
    println!("Testing space-separated HSL format: '{}'", test_color);
    println!("Converted to HSL string: {}", color.to_hsl_string());
    println!("RGB values: {}", color.to_rgb_string());
    println!("HEX: {}", color.to_hex_string(false));
    
    // Try directly with hsl
    let direct_hsl = BigColor::new("hsl(0, 0%, 12%)");
    println!("\nDirect HSL input 'hsl(0, 0%, 12%)':");
    println!("RGB values: {}", direct_hsl.to_rgb_string());
    println!("HEX: {}", direct_hsl.to_hex_string(false));
} 