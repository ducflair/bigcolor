fn main() {
    let colors = vec!["#FF0000", "#00FF00", "#0000FF", "rgb(255, 0, 0)", "hsl(0, 100%, 50%)", "hsv(0, 100%, 100%)", "hsb(0, 100%, 100%)", "cmyk(0%, 100%, 100%, 0%)", "oklch(70% 0.2 30)"];
    
    for color_str in colors {
        let color = bigcolor::BigColor::new(color_str);
        println!("Original: {}", color_str);
        println!("RGB: {:?}", color.to_rgb());
        println!("HEX: {}", color.to_hex_string(false));
        println!("HSL: {}", color.to_hsl_string());
        println!("HSV: {}", color.to_hsv_string());
        println!("HSB: {}", color.to_hsb_string());
        println!("CMYK: {}", color.to_cmyk_string());
        println!("OKLCH: {}", color.to_oklch_string());
        println!("----------------");
    }
} 