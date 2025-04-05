use bigcolor::{BigColor, BlendMode, InterpolationSpace};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BigColor Feature Demo\n");

    println!("1. CMYK Color Support");
    println!("-----------------------");
    
    // Create a color from CMYK values
    let cmyk_color = BigColor::from_cmyk(1.0, 0.0, 0.0, 0.0); // Cyan
    println!("CMYK color: {}", cmyk_color.to_cmyk_string());
    println!("RGB equivalent: {}", cmyk_color.to_rgb_string());
    println!("HEX equivalent: {}", cmyk_color.to_hex_string());
    
    // Convert an RGB color to CMYK
    let red = BigColor::parse("#ff0000")?;
    let cmyk_values = red.to_cmyk();
    println!("Red color in CMYK: {}", red.to_cmyk_string());
    println!("CMYK values: C={}%, M={}%, Y={}%, K={}%", 
        (cmyk_values[0] * 100.0) as i32,
        (cmyk_values[1] * 100.0) as i32, 
        (cmyk_values[2] * 100.0) as i32, 
        (cmyk_values[3] * 100.0) as i32
    );
    
    println!("\n2. Blending Modes");
    println!("-----------------------");
    
    let red = BigColor::parse("#ff0000")?;
    let blue = BigColor::parse("#0000ff")?;
    
    println!("Base color: {}", red.to_hex_string());
    println!("Blend color: {}", blue.to_hex_string());
    
    // Test different blending modes
    println!("Multiply: {}", red.blend(&blue, BlendMode::Multiply, 100).to_hex_string());
    println!("Screen: {}", red.blend(&blue, BlendMode::Screen, 100).to_hex_string());
    println!("Overlay: {}", red.blend(&blue, BlendMode::Overlay, 100).to_hex_string());
    println!("Darken: {}", red.blend(&blue, BlendMode::Darken, 100).to_hex_string());
    println!("Lighten: {}", red.blend(&blue, BlendMode::Lighten, 100).to_hex_string());
    println!("Difference: {}", red.blend(&blue, BlendMode::Difference, 100).to_hex_string());
    println!("Exclusion: {}", red.blend(&blue, BlendMode::Exclusion, 100).to_hex_string());
    
    // Test blend with amount
    println!("Normal (50%): {}", red.blend(&blue, BlendMode::Normal, 50).to_hex_string());
    
    println!("\n3. CSS4 Color Function");
    println!("-----------------------");
    
    // Test different color functions
    let srgb_color = BigColor::parse("color(srgb 1 0 0)")?;
    let p3_color = BigColor::parse("color(display-p3 1 0 0)")?;
    let xyz_color = BigColor::parse("color(xyz 0.4124 0.2126 0.0193)")?;
    
    println!("sRGB red: {}", srgb_color.to_hex_string());
    println!("Display-P3 red: {}", p3_color.to_hex_string());
    println!("XYZ color: {}", xyz_color.to_hex_string());
    
    println!("\n4. Color Interpolation");
    println!("-----------------------");
    
    let yellow = BigColor::parse("#ffff00")?;
    let blue = BigColor::parse("#0000ff")?;
    
    println!("Start color: {}", yellow.to_hex_string());
    println!("End color: {}", blue.to_hex_string());
    
    // Interpolate in different color spaces
    println!("RGB interpolation (50%): {}", 
        yellow.interpolate(&blue, 0.5, InterpolationSpace::RGB).to_hex_string());
    println!("HSL interpolation (50%): {}", 
        yellow.interpolate(&blue, 0.5, InterpolationSpace::HSL).to_hex_string());
    println!("HSV interpolation (50%): {}", 
        yellow.interpolate(&blue, 0.5, InterpolationSpace::HSV).to_hex_string());
    
    // Show a sequence of interpolation steps
    println!("\nRGB interpolation sequence:");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let color = yellow.interpolate(&blue, t, InterpolationSpace::RGB);
        println!("  Step {}: {}", i, color.to_hex_string());
    }
    
    println!("\nHSL interpolation sequence:");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let color = yellow.interpolate(&blue, t, InterpolationSpace::HSL);
        println!("  Step {}: {}", i, color.to_hex_string());
    }
    
    #[cfg(feature = "lab")]
    {
        println!("\n5. LAB and LCH Colors");
        println!("-----------------------");
        
        // Create colors from LAB and LCH values
        let lab_color = BigColor::from_laba(50.0, 50.0, 0.0, 1.0);
        let lch_color = BigColor::from_lcha(50.0, 50.0, 60.0, 1.0);
        
        println!("LAB color: {}", lab_color.to_lab_string());
        println!("RGB equivalent: {}", lab_color.to_rgb_string());
        println!("HEX equivalent: {}", lab_color.to_hex_string());
        
        println!("LCH color: {}", lch_color.to_lch_string());
        println!("RGB equivalent: {}", lch_color.to_rgb_string());
        println!("HEX equivalent: {}", lch_color.to_hex_string());
        
        // Interpolate in LAB and LCH color spaces
        println!("\nLAB interpolation (50%): {}", 
            yellow.interpolate(&blue, 0.5, InterpolationSpace::LAB).to_hex_string());
        println!("LCH interpolation (50%): {}", 
            yellow.interpolate(&blue, 0.5, InterpolationSpace::LCH).to_hex_string());
    }
    
    Ok(())
} 