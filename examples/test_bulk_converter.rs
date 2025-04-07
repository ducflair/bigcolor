use bigcolor::{BigColor, ColorFormat};
use regex::Regex;

// Simplified version of the converter function to test
fn convert_colors_in_text(text: &str, target_format: ColorFormat) -> String {
    // Create patterns for various color formats
    let color_patterns = vec![
        // Hex colors
        r"#([0-9a-fA-F]{3})\b",
        r"#([0-9a-fA-F]{6})\b",
        r"#([0-9a-fA-F]{8})\b",
        // RGB colors
        r"rgb\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)",
        r"rgb\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)",
        // HSL colors
        r"hsl\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)",
        // Space-separated HSL
        r"(\d+(?:\.\d+)?)\s+(\d+(?:\.\d+)?)%\s+(\d+(?:\.\d+)?)%\b",
        r"\b(\d+(?:\.\d+)?)\s+(\d+(?:\.\d+)?)%\s+(\d+(?:\.\d+)?)%\b",
    ];
    
    let mut result = text.to_string();
    
    for pattern in color_patterns {
        let regex = Regex::new(pattern).unwrap();
        let mut offset = 0;
        
        while let Some(color_match) = regex.find(&result[offset..]) {
            let start = offset + color_match.start();
            let end = offset + color_match.end();
            let color_str = &result[start..end];
            
            // Special handling for space-separated HSL
            let color = if pattern.contains("\\b(\\d+") && !pattern.contains("rgb") && !pattern.contains("hsl") {
                // Convert space-separated HSL to standard HSL format
                let caps = regex.captures(color_str).unwrap();
                let h = caps.get(1).map_or("0", |m| m.as_str());
                let s = caps.get(2).map_or("0", |m| m.as_str());
                let l = caps.get(3).map_or("0", |m| m.as_str());
                let hsl_str = format!("hsl({}, {}%, {}%)", h, s, l);
                println!("Converting space-separated HSL '{}' to '{}'", color_str, hsl_str);
                BigColor::new(&hsl_str)
            } else {
                BigColor::new(color_str)
            };
            
            if color.is_valid() {
                let converted = color.to(target_format);
                println!("Found color '{}', converted to '{}'", color_str, converted);
                result.replace_range(start..end, &converted);
                offset = start + converted.len();
            } else {
                println!("Invalid color format: '{}'", color_str);
                offset = end;
            }
        }
    }
    
    result
}

fn main() {
    // Test with various color formats including space-separated HSL
    let test_text = r#"
    Here are some colors:
    #f00 and #00ff00 and #0000ff88
    rgb(255, 0, 0) and rgb(6%, 43%, 96%)
    hsl(120, 100%, 50%)
    0 0% 12% - this is space-separated HSL
    "#;
    
    println!("Original text:\n{}", test_text);
    println!("\nConverting to HEX format:");
    let hex_result = convert_colors_in_text(test_text, ColorFormat::HEX);
    println!("\nResult:\n{}", hex_result);
    
    println!("\nConverting to RGB format:");
    let rgb_result = convert_colors_in_text(test_text, ColorFormat::RGB);
    println!("\nResult:\n{}", rgb_result);
    
    println!("\nConverting to HSL format:");
    let hsl_result = convert_colors_in_text(test_text, ColorFormat::HSL);
    println!("\nResult:\n{}", hsl_result);
} 