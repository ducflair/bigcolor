# BigColor Demo

A live demonstration of the BigColor Rust library for color manipulation.

## Features Demonstrated

- Color parsing (RGB, HEX, HSL, CMYK, etc.)
- Color manipulation (lighten, darken, saturate)
- Color theory (complementary colors)
- Gradient creation and manipulation
- Various color formats and conversions

## Running the Demo

1. Make sure you have Rust and Trunk installed:

```bash
cargo install trunk
```

2. Clone the repository and navigate to the demo directory:

```bash
git clone https://github.com/your-username/bigcolor.git
cd bigcolor/demo
```

3. Start the development server:

```bash
trunk serve
```

4. Open your browser and navigate to [http://localhost:8080](http://localhost:8080)

## Deploying to GitHub Pages

1. Build the project for release:

```bash
trunk build --release --public-url bigcolor
```

2. The built files will be in the `dist` directory. Copy these to the `gh-pages` branch or deploy them to your GitHub Pages host.

## How It Works

This demo uses [Yew](https://yew.rs/), a modern Rust framework for building web applications. It demonstrates the capabilities of the BigColor library by providing an interactive interface to:

- Parse and display colors
- Manipulate colors (HSL, RGB, CMYK)
- Create gradients
- Show different color formats

## Contributing

If you'd like to improve this demo or the BigColor library, feel free to submit a pull request!
