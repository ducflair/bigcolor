use regex::Regex;

fn main() {
    let test_color = "0 0% 12%";
    let text_with_space_hsl = "Test with space-separated HSL: 0 0% 12% is a dark gray.";
    
    // Test various patterns
    let patterns = vec![
        r"(\b0\b|\b[1-9]\d*\b)\s+(\b0\b|\b[1-9]\d*\b)%\s+(\b0\b|\b[1-9]\d*\b)%",
        r"\s(\d+)\s+(\d+)%\s+(\d+)%\s"
    ];
    
    for (i, pattern) in patterns.iter().enumerate() {
        println!("Testing pattern {}: {}", i+1, pattern);
        
        match Regex::new(pattern) {
            Ok(regex) => {
                // Test direct match
                if let Some(captures) = regex.captures(test_color) {
                    println!("  Direct match successful!");
                    
                    // Extract and print captures
                    for j in 1..=3 {
                        if let Some(cap) = captures.get(j) {
                            println!("    Capture {}: {}", j, cap.as_str());
                        }
                    }
                } else {
                    println!("  Direct match failed");
                }
                
                // Test in-text match
                if let Some(captures) = regex.captures(text_with_space_hsl) {
                    println!("  In-text match successful!");
                    
                    // Extract and print captures
                    for j in 1..=3 {
                        if let Some(cap) = captures.get(j) {
                            println!("    Capture {}: {}", j, cap.as_str());
                        }
                    }
                    
                    // Print the full match
                    if let Some(full_match) = captures.get(0) {
                        println!("    Full match: '{}'", full_match.as_str());
                    }
                } else {
                    println!("  In-text match failed");
                }
                
                // Try finding all matches
                println!("  Searching for all matches in text:");
                for color_match in regex.find_iter(text_with_space_hsl) {
                    println!("    Match at {}-{}: '{}'", 
                        color_match.start(), 
                        color_match.end(), 
                        &text_with_space_hsl[color_match.start()..color_match.end()]);
                }
            },
            Err(e) => println!("  Pattern error: {}", e)
        }
        
        println!();
    }
} 