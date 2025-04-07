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

- **Extensive Color Support**: Parse and manipulate colors in RGB, HSL, HSV, HSB, CMYK, LAB, LCH, OKLAB, OKLCH formats
- **Flexible Input Parsing**: Accepts various input formats including hex, rgb(), rgba(), hsl(), hsla(), etc.
- **Color Modifications**: Lighten, darken, saturate, desaturate, greyscale, spin
- **Color Schemes**: Generate analogous, monochromatic, triad, tetrad, split complement, and complement colors
- **Contrast Calculation**: Determine contrast ratios according to WCAG standards
- **Peniko Integration**: Convert to/from the peniko Color library
- **Bulk Color Conversion**: Convert between formats in large text blocks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bigcolor = "0.1.0"
```

## Basic Usage

```rust
use bigcolor::BigColor;

fn main() {
    // Create colors from various formats
    let hex_color = BigColor::new("#1a6ef5");
    let rgb_color = BigColor::new("rgb(255, 0, 0)");
    let hsl_color = BigColor::new("hsl(120, 100%, 50%)");
    let rgba_color = BigColor::new("rgba(255, 0, 0, 0.5)");
    
    // Convert to different formats
    println!("As RGB: {}", hex_color.to_rgb_string());
    println!("As HSL: {}", rgb_color.to_hsl_string());
    println!("As HEX: {}", hsl_color.to_hex_string(false));
    println!("As CMYK: {}", rgba_color.to_cmyk_string());
}
```

## Color Modification

```rust
use bigcolor::BigColor;

fn main() {
    let mut color = BigColor::new("#1a6ef5");
    
    // Modify the color
    color.lighten(Some(20.0)); // Lighten by 20%
    println!("Lightened: {}", color.to_hex_string(false));
    
    color = BigColor::new("#1a6ef5"); // Reset
    color.darken(Some(20.0)); // Darken by 20%
    println!("Darkened: {}", color.to_hex_string(false));
    
    color = BigColor::new("#1a6ef5"); // Reset
    color.saturate(Some(20.0)); // Saturate by 20%
    println!("Saturated: {}", color.to_hex_string(false));
    
    color = BigColor::new("#1a6ef5"); // Reset
    color.desaturate(Some(20.0)); // Desaturate by 20%
    println!("Desaturated: {}", color.to_hex_string(false));
    
    color = BigColor::new("#1a6ef5"); // Reset
    color.greyscale(); // Convert to greyscale
    println!("Greyscale: {}", color.to_hex_string(false));
}
```

## Color Schemes

```rust
use bigcolor::BigColor;

fn main() {
    let color = BigColor::new("#1a6ef5");
    
    // Generate color schemes
    let analogous = color.analogous(Some(5), Some(30));
    let mono = color.monochromatic(Some(5));
    let triad = color.triad();
    let tetrad = color.tetrad();
    let split = color.split_complement();
    
    // Print the first color from each scheme
    println!("Analogous: {}", analogous[0].to_hex_string(false));
    println!("Monochromatic: {}", mono[0].to_hex_string(false));
    println!("Triad: {}", triad[0].to_hex_string(false));
    println!("Tetrad: {}", tetrad[0].to_hex_string(false));
    println!("Split complement: {}", split[0].to_hex_string(false));
}
```

## Contrast and Accessibility

```rust
use bigcolor::BigColor;

fn main() {
    let background = BigColor::new("#1a6ef5");
    
    // Generate contrast colors with different intensities
    let low_contrast = background.get_contrast_color(0.2);
    let medium_contrast = background.get_contrast_color(0.5);
    let high_contrast = background.get_contrast_color(1.0);
    
    // Check contrast ratios (WCAG)
    let ratio_low = background.get_contrast_ratio(&low_contrast);
    let ratio_medium = background.get_contrast_ratio(&medium_contrast);
    let ratio_high = background.get_contrast_ratio(&high_contrast);
    
    println!("Background: {}", background.to_hex_string(false));
    println!("Low contrast: {} (Ratio: {:.2}:1)", low_contrast.to_hex_string(false), ratio_low);
    println!("Medium contrast: {} (Ratio: {:.2}:1)", medium_contrast.to_hex_string(false), ratio_medium);
    println!("High contrast: {} (Ratio: {:.2}:1)", high_contrast.to_hex_string(false), ratio_high);
    
    // Check if background is light or dark
    println!("Is light color: {}", background.is_light());
}
```

## Peniko Integration

```rust
use bigcolor::{BigColor, conversion};
use peniko::Color as PenikoColor;

fn main() {
    // Convert from BigColor to peniko::Color
    let big_color = BigColor::new("#1a6ef5");
    let peniko_color = conversion::to_peniko_color(&big_color);
    
    // Convert from peniko::Color to BigColor
    let peniko_red = PenikoColor::from_rgb8(255, 0, 0);
    let big_red = conversion::from_peniko_color(&peniko_red);
    
    println!("Original: {}", big_color.to_hex_string(false));
    println!("Converted back and forth: {}", big_red.to_hex_string(false));
}
```

## Supported Input Formats

- **Hex**: `#RGB`, `#RRGGBB`, `#RRGGBBAA`
- **RGB**: `rgb(r, g, b)`, `rgba(r, g, b, a)`, `rgb(r%, g%, b%)`, `rgba(r%, g%, b%, a)`
- **HSL**: `hsl(h, s%, l%)`, `hsla(h, s%, l%, a)`, space-separated HSL values
- **HSV/HSB**: `hsv(h, s%, v%)`, `hsva(h, s%, v%, a)`
- **CMYK**: `cmyk(c%, m%, y%, k%)`
- **LAB**: `lab(l, a, b)`
- **LCH**: `lch(l, c, h)`
- **OKLAB**: `oklab(l%, a, b)`
- **OKLCH**: `oklch(l%, c, h)`

## Running the Demo

The project includes a web-based demo that showcases all of BigColor's capabilities:

```bash
cd demo
trunk serve
```

Then open your browser to http://localhost:8080

## License

MIT

## Contributing

This library is partially a port of [TinyColor](https://github.com/bgrins/TinyColor) by Brian Grinstead and inspired by [Color.js](https://github.com/color-js/color.js). 