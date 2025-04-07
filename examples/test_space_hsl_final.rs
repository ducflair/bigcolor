use bigcolor::BigColor;
use regex::Regex;

fn main() {
    let test_pattern = r"(?<!\S)(\d+)\s+(\d+)%\s+(\d+)%(?!\S)";
    let test_color = "0 0% 12%";
    
    // Test if the pattern matches
    let regex = Regex::new(test_pattern).unwrap();
    if let Some(captures) = regex.captures(test_color) {
        println!("Success! Pattern matched the test color!");
        let h = captures.get(1).map_or("0", |m| m.as_str());
        let s = captures.get(2).map_or("0", |m| m.as_str());
        let l = captures.get(3).map_or("0", |m| m.as_str());
        
        println!("Hue: {}, Saturation: {}%, Lightness: {}%", h, s, l);
        
        // Create HSL color from captured values
        let hsl_str = format!("hsl({}, {}%, {}%)", h, s, l);
        println!("Constructed HSL string: {}", hsl_str);
        
        let color = BigColor::new(&hsl_str);
        println!("Is valid color: {}", color.is_valid());
        println!("RGB: {}", color.to_rgb_string());
        println!("HEX: {}", color.to_hex_string(false));
    } else {
        println!("Pattern did NOT match the test color!");
    }
    
    // Test with sample text
    let sample_text = "Background color: 0 0% 12%; Border color: #f00; Text color: rgb(255, 255, 255);";
    println!("\nSample text: {}", sample_text);
    
    let mut result = sample_text.to_string();
    let mut offset = 0;
    
    while let Some(color_match) = regex.find(&result[offset..]) {
        let start = offset + color_match.start();
        let end = offset + color_match.end();
        let color_str = &result[start..end];
        
        println!("Found color: '{}'", color_str);
        
        // Convert space-separated HSL to standard HSL format
        let caps = regex.captures(color_str).unwrap();
        let h = caps.get(1).map_or("0", |m| m.as_str());
        let s = caps.get(2).map_or("0", |m| m.as_str());
        let l = caps.get(3).map_or("0", |m| m.as_str());
        let hsl_str = format!("hsl({}, {}%, {}%)", h, s, l);
        
        println!("Converted to HSL: '{}'", hsl_str);
        
        let color = BigColor::new(&hsl_str);
        if color.is_valid() {
            let rgb = color.to_rgb_string();
            println!("Converted to RGB: '{}'", rgb);
            result.replace_range(start..end, &rgb);
            println!("Updated text: '{}'", result);
            offset = start + rgb.len();
        } else {
            offset = end;
        }
    }
    
    println!("\nFinal result: '{}'", result);
} 