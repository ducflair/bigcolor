use csscolorparser::{BigColor, Color};

#[test]
fn test_parse() {
    let color = BigColor::parse("#ff0000").unwrap();
    assert_eq!(color.to_color().to_rgba8(), [255, 0, 0, 255]);
    
    let color = BigColor::parse("rgb(0, 255, 0)").unwrap();
    assert_eq!(color.to_color().to_rgba8(), [0, 255, 0, 255]);
    
    let color = BigColor::parse("blue").unwrap();
    assert_eq!(color.to_color().to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn test_to_peniko_color() {
    let color = BigColor::parse("#ff0000").unwrap();
    let peniko_color = color.to_peniko_color();
    assert_eq!(peniko_color.r(), 255);
    assert_eq!(peniko_color.g(), 0);
    assert_eq!(peniko_color.b(), 0);
    assert_eq!(peniko_color.a(), 255);
}

#[test]
fn test_get_contrast() {
    // Test with a light color
    let light_color = BigColor::parse("#ffffff").unwrap();
    
    // Extreme contrast should be black
    let contrast_extreme = light_color.get_contrast(true);
    assert_eq!(contrast_extreme.to_color().to_rgba8(), [0, 0, 0, 255]);
    
    // Soft contrast should be dark gray
    let contrast_soft = light_color.get_contrast(false);
    assert_eq!(contrast_soft.to_color().to_rgba8(), [74, 74, 74, 255]);
    
    // Test with a dark color
    let dark_color = BigColor::parse("#000000").unwrap();
    
    // Extreme contrast should be white
    let contrast_extreme = dark_color.get_contrast(true);
    assert_eq!(contrast_extreme.to_color().to_rgba8(), [255, 255, 255, 255]);
    
    // Soft contrast should be light gray
    let contrast_soft = dark_color.get_contrast(false);
    assert_eq!(contrast_soft.to_color().to_rgba8(), [201, 201, 201, 255]);
}

#[test]
fn test_color_manipulation() {
    let color = BigColor::parse("#ff0000").unwrap(); // Red
    
    // Test lighten
    let lighter = color.lighten(20);
    assert!(lighter.to_color().to_rgba8()[0] == 255);
    assert!(lighter.to_color().to_rgba8()[1] > 0);
    assert!(lighter.to_color().to_rgba8()[2] > 0);
    
    // Test darken
    let darker = color.darken(20);
    assert!(darker.to_color().to_rgba8()[0] < 255);
    assert_eq!(darker.to_color().to_rgba8()[1], 0);
    assert_eq!(darker.to_color().to_rgba8()[2], 0);
    
    // Test saturate/desaturate - red is already saturated
    let desaturated = color.desaturate(50);
    assert!(desaturated.to_color().to_rgba8()[1] > 0);
    
    // Test grayscale
    let gray = color.grayscale();
    assert_eq!(gray.to_color().to_rgba8()[0], gray.to_color().to_rgba8()[1]);
    assert_eq!(gray.to_color().to_rgba8()[1], gray.to_color().to_rgba8()[2]);
}

#[test]
fn test_color_combinations() {
    let color = BigColor::parse("#ff0000").unwrap(); // Red
    
    // Test complement
    let complement = color.complement();
    assert_eq!(complement.to_color().to_rgba8()[0], 0);
    assert!(complement.to_color().to_rgba8()[1] > 0 || complement.to_color().to_rgba8()[2] > 0);
    
    // Test triad
    let triad = color.triad();
    assert_eq!(triad.len(), 3);
    
    // Test tetrad
    let tetrad = color.tetrad();
    assert_eq!(tetrad.len(), 4);
    
    // Test analogous
    let analogous = color.analogous(3, 30);
    assert_eq!(analogous.len(), 3);
    
    // Test monochromatic
    let mono = color.monochromatic(5);
    assert_eq!(mono.len(), 5);
    assert_eq!(mono[0].to_color().to_rgba8()[0], 255);
    assert_eq!(mono[0].to_color().to_rgba8()[1], 0);
    assert_eq!(mono[0].to_color().to_rgba8()[2], 0);
}

#[test]
fn test_readability() {
    let black = BigColor::parse("#000000").unwrap();
    let white = BigColor::parse("#ffffff").unwrap();
    
    // Black on white should have high contrast
    assert!(black.is_readable_on(&white, None));
    
    // Test with custom options
    let options = csscolorparser::ReadableOptions {
        level: "AAA".to_string(),
        large: false,
    };
    assert!(black.is_readable_on(&white, Some(options)));
}

#[test]
fn test_conversions() {
    // Test convert from csscolorparser::Color
    let original = Color::from_rgba8(255, 0, 0, 255);
    let big_color = BigColor::from(original.clone());
    assert_eq!(big_color.to_color().to_rgba8(), original.to_rgba8());
    
    // Test convert from peniko::Color
    let peniko_color = vello::peniko::Color::from_rgba8(0, 255, 0, 255);
    let big_color_from_peniko = BigColor::from(peniko_color);
    assert_eq!(big_color_from_peniko.to_color().to_rgba8(), [0, 255, 0, 255]);
    
    // Test convert to peniko::Color
    let big_color = BigColor::parse("#0000ff").unwrap();
    let peniko_color: vello::peniko::Color = big_color.into();
    assert_eq!(peniko_color.r(), 0);
    assert_eq!(peniko_color.g(), 0);
    assert_eq!(peniko_color.b(), 255);
    assert_eq!(peniko_color.a(), 255);
} 