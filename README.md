<p align="center">
  <br/>
  <a target="_blank"><img width="256px" src="https://github.com/ducflair/bigcolor/blob/main/public/logo.png?raw=true" /></a>
  <p align="center">Rust Color Parser & Manipulation Library</p>
  <p align="center" style="align: center;">
    <a href="https://crates.io/crates/bigcolor/"><img src="https://shields.io/badge/Crates-FFC933?logo=Rust&logoColor=646464&style=round-square" alt="Crates" /></a>
    <a href="https://github.com/ducflair/bigcolor/releases"><img src="https://img.shields.io/crates/v/bigcolor?style=round-square&label=latest%20stable" alt="Crates.io bigcolor@latest release" /></a>
    <a href="https://crates.io/crates/bigcolor"><img src="https://img.shields.io/crates/d/bigcolor?style=round-square&color=salmon" alt="Downloads" /></a>
    <img src="https://shields.io/badge/Rust-CE412B?logo=Rust&logoColor=fff&style=round-square" alt="Rust" />
  </p>
</p>

# bigcolor

A comprehensive Rust color manipulation library, inspired by [TinyColor](https://github.com/bgrins/TinyColor) and [Color.js](https://github.com/color-js/color.js). Designed to be intuitive, powerful, and fast for working with colors in various formats.

## Features

- **Extensive Format Support**
  - Parse colors from RGB, RGBA, HSL, HSLA, HSV, HSVA, HSB, HSBA, HEX, CMYK, LAB, LCH, OKLAB, OKLCH
  - Convert between any of these formats easily
  - Support for named colors (CSS/SVG color keywords)
  - Space-separated HSL format (e.g. `0 0% 12%`) used in CSS variables

- **Color Manipulation**
  - Lighten, darken, saturate, desaturate, spin (adjust hue)
  - Convert to greyscale, complement, invert
  - Easily get brightness, luminance, and alpha values

- **Color Schemes**
  - Generate color schemes: analogous, monochromatic, triad, tetrad, split complement
  - Create harmonious color palettes for design projects

- **Utilities**
  - Color mixing and blending
  - WCAG contrast checking
  - Color readability tools
  - Bulk text color format conversion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bigcolor = "0.1.0"
```

## Usage Examples

### Basic Color Creation and Conversion

```rust
use bigcolor::BigColor;

fn main() {
    // Create colors from different formats
    let red_hex = BigColor::new("#ff0000");
    let blue_name = BigColor::new("blue");
    let green_rgb = BigColor::new("rgb(0, 255, 0)");
    let purple_hsl = BigColor::new("hsl(270, 100%, 50%)");
    let cyan_hsb = BigColor::new("hsb(180, 100%, 100%)");
    let yellow_cmyk = BigColor::new("cmyk(0%, 0%, 100%, 0%)");
    
    // Convert between formats
    println!("Red as RGB: {}", red_hex.to_rgb_string());          // rgb(255, 0, 0)
    println!("Blue as HSL: {}", blue_name.to_hsl_string());       // hsl(240, 100%, 50%)
    println!("Green as HEX: {}", green_rgb.to_hex_string(false)); // #00ff00
    println!("Purple as HSB: {}", purple_hsl.to_hsb_string());    // hsb(270, 100%, 100%)
    println!("Cyan as CMYK: {}", cyan_hsb.to_cmyk_string());      // cmyk(100%, 0%, 0%, 0%)
    println!("Yellow as OKLCH: {}", yellow_cmyk.to_oklch_string()); // oklch(97% 0.2 85)
}
```

### Color Modification

```rust
use bigcolor::BigColor;

fn main() {
    let color = BigColor::new("#1a6ef5"); // A blue color
    
    // Create variations
    let mut lighter = color.clone_color();
    lighter.lighten(Some(20.0));
    
    let mut darker = color.clone_color();
    darker.darken(Some(20.0));
    
    let mut more_saturated = color.clone_color();
    more_saturated.saturate(Some(30.0));
    
    let mut complementary = color.clone_color();
    complementary.complement();
    
    println!("Original: {}", color.to_hex_string(false));
    println!("Lighter: {}", lighter.to_hex_string(false));
    println!("Darker: {}", darker.to_hex_string(false));
    println!("More Saturated: {}", more_saturated.to_hex_string(false));
    println!("Complementary: {}", complementary.to_hex_string(false));
}
```

### Color Schemes

```rust
use bigcolor::BigColor;

fn main() {
    let base_color = BigColor::new("#1a6ef5");
    
    // Generate color schemes
    let analogous = base_color.analogous(Some(5), Some(30));
    let monochromatic = base_color.monochromatic(Some(5));
    let triad = base_color.triad();
    let tetrad = base_color.tetrad();
    let split_complement = base_color.split_complement();
    
    // Print the hex values for the triad scheme
    println!("Triad color scheme:");
    for color in triad {
        println!("  {}", color.to_hex_string(false));
    }
}
```

### Bulk Color Conversion

```rust
use bigcolor::{BigColor, ColorFormat};
use regex::Regex;

fn convert_colors_in_text(text: &str, target_format: ColorFormat) -> String {
    // Get a BigColor instance from the library
    // This is a simplified example - see demo for full implementation
    let color = BigColor::new("#ff0000");
    let converted = color.to(target_format);
    text.replace("#ff0000", &converted)
}

fn main() {
    let css_code = r#"
    .header {
        background-color: #ff0000;
        color: rgb(255, 255, 255);
        border: 1px solid hsl(0, 0%, 80%);
    }
    "#;
    
    // Convert all colors to HSL format
    let result = convert_colors_in_text(css_code, ColorFormat::HSL);
    println!("Converted CSS:\n{}", result);
}
```

## Demo Application

A demo web application is available to explore the capabilities of the BigColor library:

- Color parsing and conversion
- Color manipulation and scheme generation
- Bulk color conversion tool
- Interactive color previews

To run the demo:

```bash
cd demo
trunk serve
```

Then open your browser to `http://localhost:8080`

## Color Format Support

### Input Formats
The library accepts the following color formats:

- **Named Colors**: `red`, `blue`, `rebeccapurple`, etc.
- **Hex**: `#f00`, `#ff0000`, `#ff0000ff`
- **RGB**: `rgb(255, 0, 0)`, `rgba(255, 0, 0, 0.5)`, `rgb(100%, 0%, 0%)`
- **HSL**: `hsl(0, 100%, 50%)`, `hsla(0, 100%, 50%, 0.5)`
- **HSV/HSB**: `hsv(0, 100%, 100%)`, `hsb(0, 100%, 100%)`
- **CMYK**: `cmyk(0%, 100%, 100%, 0%)`
- **LAB**: `lab(50, 80, 67)`
- **LCH**: `lch(50, 80, 20)`
- **OKLAB**: `oklab(60%, 0.1, 0.2)`
- **OKLCH**: `oklch(60%, 0.1, 30)`
- **Space-separated HSL**: `0 0% 12%` (common in CSS variables)

### Output Formats
Convert to any of these formats using the corresponding method:

```rust
let color = BigColor::new("#ff0000");

color.to_hex_string(false);    // "#ff0000"
color.to_rgb_string();         // "rgb(255, 0, 0)"
color.to_hsl_string();         // "hsl(0, 100%, 50%)"
color.to_hsv_string();         // "hsv(0, 100%, 100%)"
color.to_hsb_string();         // "hsb(0, 100%, 100%)"
color.to_cmyk_string();        // "cmyk(0%, 100%, 100%, 0%)"
color.to_oklch_string();       // "oklch(63% 0.26 29)"
```

## License

MIT License - see the LICENSE file for details.

## Credits

This library is partially a port of [TinyColor](https://github.com/bgrins/TinyColor) by Brian Grinstead and inspired by [Color.js](https://github.com/color-js/color.js). 