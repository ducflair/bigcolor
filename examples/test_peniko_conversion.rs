use bigcolor::{BigColor, conversion};
use peniko::Color as PenikoColor;

fn main() {
    // Test conversion from BigColor to peniko::Color
    let big_color = BigColor::new("#1a6ef5");
    let peniko_color = conversion::to_peniko_color(&big_color);
    
    println!("Original BigColor: {}", big_color);
    println!("As peniko::Color: RGBA({:?})", peniko_color.to_rgba8());
    
    // Test conversion from peniko::Color to BigColor
    let peniko_red = PenikoColor::from_rgb8(255, 0, 0);
    let big_red = conversion::from_peniko_color(&peniko_red);
    
    println!("\nOriginal peniko::Color: RGBA({:?})", peniko_red.to_rgba8());
    println!("As BigColor: {}", big_red);
    
    // Test contrast colors with different intensities
    let base_color = BigColor::new("#1a6ef5"); // Blue
    
    println!("\nTesting contrast colors for: {}", base_color);
    println!("Is light: {}", base_color.is_light());
    
    let intensities = [0.0, 0.25, 0.5, 0.75, 1.0];
    
    for intensity in intensities {
        let contrast = base_color.get_contrast_color(intensity);
        let ratio = base_color.get_contrast_ratio(&contrast);
        let wcag_pass = if ratio >= 4.5 { "AA" } else if ratio >= 3.0 { "AA Large" } else { "Fail" };
        
        println!(
            "Intensity {:.2}: {} (Contrast ratio: {:.2}:1 - {})",
            intensity, contrast, ratio, wcag_pass
        );
    }
    
    // Test with a light color
    let light_color = BigColor::new("#f5f5f5"); // Light gray
    
    println!("\nTesting contrast colors for: {}", light_color);
    println!("Is light: {}", light_color.is_light());
    
    for intensity in intensities {
        let contrast = light_color.get_contrast_color(intensity);
        let ratio = light_color.get_contrast_ratio(&contrast);
        let wcag_pass = if ratio >= 4.5 { "AA" } else if ratio >= 3.0 { "AA Large" } else { "Fail" };
        
        println!(
            "Intensity {:.2}: {} (Contrast ratio: {:.2}:1 - {})",
            intensity, contrast, ratio, wcag_pass
        );
    }
} 