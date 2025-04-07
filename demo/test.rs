fn main() { let color = bigcolor::BigColor::new("#FF0000"); println!("Original: #FF0000"); println!("RGB: {:?}", color.to_rgb()); println!("OKLCH: {:?}", color.to_oklch()); }
