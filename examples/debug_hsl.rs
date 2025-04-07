use bigcolor::BigColor;

fn main() {
    let color_hex = "#FF0000";  // Pure red
    println!("Testing color: {}", color_hex);
    
    let color = BigColor::new(color_hex);
    
    // Get the raw RGB values
    let rgb = color.to_rgb();
    println!("RGB: r={}, g={}, b={}", rgb.r, rgb.g, rgb.b);
    
    // Test manually computed HSL directly from BigColor's source
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;
    
    println!("Normalized RGB: r={:.6}, g={:.6}, b={:.6}", r, g, b);
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let mut h = 0.0;
    let mut s = 0.0;
    let l = (max + min) / 2.0;
    
    println!("HSL calculation: max={:.6}, min={:.6}, l={:.6}", max, min, l);
    
    if max == min {
        println!("Achromatic case (max == min)");
        h = 0.0;
        s = 0.0;
    } else {
        let d = max - min;
        s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
        
        println!("Chromatic case: d={:.6}, s={:.6}", d, s);
        
        if max == r {
            println!("r is max");
            h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
        } else if max == g {
            println!("g is max");
            h = (b - r) / d + 2.0;
        } else if max == b {
            println!("b is max");
            h = (r - g) / d + 4.0;
        }
        
        h /= 6.0;
    }
    
    println!("Final HSL: h={:.6} ({:.1}°), s={:.6} ({:.1}%), l={:.6} ({:.1}%)", 
             h, h * 360.0, s, s * 100.0, l, l * 100.0);
    
    // Compare with the library result
    let hsl = color.to_hsl();
    println!("Library HSL: h={:.1}, s={:.1}%, l={:.1}%", hsl.h, hsl.s * 100.0, hsl.l * 100.0);
    println!("HSL String: {}", color.to_hsl_string());
} 