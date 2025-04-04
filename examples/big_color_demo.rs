use bigcolor::BigColor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a color
    let color = BigColor::parse("#ff0000")?;
    println!("Original color: #{:02x}{:02x}{:02x}", 
        color.to_rgba8()[0], 
        color.to_rgba8()[1], 
        color.to_rgba8()[2]
    );
    
    // Get contrast color
    let contrast = color.get_contrast(1.0);
    println!("Contrast color: #{:02x}{:02x}{:02x}", 
        contrast.to_rgba8()[0], 
        contrast.to_rgba8()[1], 
        contrast.to_rgba8()[2]
    );
    
    // Manipulate colors
    let lighter = color.lighten(20);
    println!("Lighter color: #{:02x}{:02x}{:02x}", 
        lighter.to_rgba8()[0], 
        lighter.to_rgba8()[1], 
        lighter.to_rgba8()[2]
    );
    
    let darker = color.darken(20);
    println!("Darker color: #{:02x}{:02x}{:02x}", 
        darker.to_rgba8()[0], 
        darker.to_rgba8()[1], 
        darker.to_rgba8()[2]
    );
    
    let desaturated = color.desaturate(50);
    println!("Desaturated color: #{:02x}{:02x}{:02x}", 
        desaturated.to_rgba8()[0], 
        desaturated.to_rgba8()[1], 
        desaturated.to_rgba8()[2]
    );
    
    let gray = color.grayscale();
    println!("Grayscale color: #{:02x}{:02x}{:02x}", 
        gray.to_rgba8()[0], 
        gray.to_rgba8()[1], 
        gray.to_rgba8()[2]
    );
    
    // Generate color schemes
    let complement = color.complement();
    println!("Complement color: #{:02x}{:02x}{:02x}", 
        complement.to_rgba8()[0], 
        complement.to_rgba8()[1], 
        complement.to_rgba8()[2]
    );
    
    println!("\nTriad colors:");
    for (i, triad_color) in color.triad().iter().enumerate() {
        println!("  Color {}: #{:02x}{:02x}{:02x}", 
            i + 1,
            triad_color.to_rgba8()[0], 
            triad_color.to_rgba8()[1], 
            triad_color.to_rgba8()[2]
        );
    }
    
    println!("\nTetrad colors:");
    for (i, tetrad_color) in color.tetrad().iter().enumerate() {
        println!("  Color {}: #{:02x}{:02x}{:02x}", 
            i + 1,
            tetrad_color.to_rgba8()[0], 
            tetrad_color.to_rgba8()[1], 
            tetrad_color.to_rgba8()[2]
        );
    }
    
    println!("\nAnalogous colors:");
    for (i, analogous_color) in color.analogous(5, 30).iter().enumerate() {
        println!("  Color {}: #{:02x}{:02x}{:02x}", 
            i + 1,
            analogous_color.to_rgba8()[0], 
            analogous_color.to_rgba8()[1], 
            analogous_color.to_rgba8()[2]
        );
    }
    
    println!("\nMonochromatic colors:");
    for (i, mono_color) in color.monochromatic(5).iter().enumerate() {
        println!("  Color {}: #{:02x}{:02x}{:02x}", 
            i + 1,
            mono_color.to_rgba8()[0], 
            mono_color.to_rgba8()[1], 
            mono_color.to_rgba8()[2]
        );
    }

    // Split complement colors
    println!("\nSplit Complement colors:");
    for (i, split_color) in color.split_complement().iter().enumerate() {
        println!("  Color {}: #{:02x}{:02x}{:02x}", 
            i + 1,
            split_color.to_rgba8()[0], 
            split_color.to_rgba8()[1], 
            split_color.to_rgba8()[2]
        );
    }
    
    // Test readability
    let background = BigColor::parse("#ffffff")?;
    let is_readable = color.is_readable_on(&background, None);
    println!("\nIs red readable on white? {}", is_readable);
    
    // Convert to peniko::Color
    let peniko_color = color.to_peniko_color();
    println!("\nPeniko color: rgba({}, {}, {}, {})", 
        peniko_color.to_rgba8().r, 
        peniko_color.to_rgba8().g, 
        peniko_color.to_rgba8().b, 
        peniko_color.to_rgba8().a
    );

    // Demonstrate brighten method
    let brightened = color.brighten(30.0);
    println!("\nBrightened color: #{:02x}{:02x}{:02x}", 
        brightened.to_rgba8()[0], 
        brightened.to_rgba8()[1], 
        brightened.to_rgba8()[2]
    );

    // Demonstrate string formatting
    println!("\nRGB string: {}", color.to_rgb_string());
    println!("HSL string: {}", color.to_hsl_string());
    println!("HSV string: {}", color.to_hsv_string());
    println!("Percentage RGB string: {}", color.to_percentage_rgb_string());
    
    Ok(())
} 