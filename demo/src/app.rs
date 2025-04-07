use yew::prelude::*;
use bigcolor::{BigColor, ColorFormat};
use web_sys::{HtmlInputElement, HtmlTextAreaElement, HtmlSelectElement, window};
use wasm_bindgen::JsCast;
use gloo_timers::callback::Timeout;
use regex::Regex;

/// Helper function to copy text to clipboard
fn copy_to_clipboard(text: &str) {
    if let Some(window) = window() {
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = js_sys::Promise::from(clipboard.write_text(text));
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormatProps {
    pub format_name: String,
    pub format_value: String,
}

#[function_component(FormatBox)]
fn format_box(props: &FormatProps) -> Html {
    let copied = use_state(|| false);
    let copied_clone = copied.clone();
    
    let onclick = {
        let format_value = props.format_value.clone();
        Callback::from(move |_: MouseEvent| {
            copy_to_clipboard(&format_value);
            
            // Show "Copied!" indicator
            copied_clone.set(true);
            
            // Reset after 2 seconds
            let copied_clone_inner = copied_clone.clone();
            let timeout = Timeout::new(2000, move || {
                copied_clone_inner.set(false);
            });
            timeout.forget();
        })
    };
    
    html! {
        <div class={classes!("format-box", (*copied).then_some("copied"))} onclick={onclick}>
            <div class="format-name">{ &props.format_name }</div>
            <div class="format-value">
                <code>{ &props.format_value }</code>
            </div>
            {
                if *copied {
                    html! { <div class="copy-badge">{"Copied!"}</div> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

// Helper function to get CSS-compatible color string for background-color
fn get_css_compatible_color(color: &BigColor) -> String {
    // CSS doesn't support HSV, HSB, or CMYK directly, so convert to RGB for these formats
    match color.get_format() {
        ColorFormat::HSV | ColorFormat::HSB | ColorFormat::CMYK => color.to_rgb_string(),
        _ => color.to_string(None)
    }
}

// Function to get all available color formats for the dropdown
fn get_color_format_options() -> Vec<(String, ColorFormat)> {
    vec![
        ("HEX".to_string(), ColorFormat::HEX),
        ("HEX8".to_string(), ColorFormat::HEX8),
        ("RGB".to_string(), ColorFormat::RGB),
        ("Percentage RGB".to_string(), ColorFormat::PRGB),
        ("HSL".to_string(), ColorFormat::HSL),
        ("HSV".to_string(), ColorFormat::HSV),
        ("HSB".to_string(), ColorFormat::HSB),
        ("CMYK".to_string(), ColorFormat::CMYK),
        ("LAB".to_string(), ColorFormat::LAB),
        ("LCH".to_string(), ColorFormat::LCH),
        ("OKLAB".to_string(), ColorFormat::OKLAB),
        ("OKLCH".to_string(), ColorFormat::OKLCH),
    ]
}

// Function to detect and convert colors in text
fn convert_colors_in_text(text: &str, target_format: ColorFormat) -> String {
    // Create patterns for various color formats
    let color_patterns = vec![
        // Hex colors
        r"#([0-9a-fA-F]{3})\b",
        r"#([0-9a-fA-F]{6})\b",
        r"#([0-9a-fA-F]{8})\b",
        // RGB colors
        r"rgb\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)",
        r"rgb\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)",
        r"rgba\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*([01]?\.?\d*)\s*\)",
        r"rgba\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*([01]?\.?\d*)\s*\)",
        // HSL colors
        r"hsl\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)",
        r"hsla\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*([01]?\.?\d*)\s*\)",
        // Space-separated HSL (like in CSS variables)
        r"(\b0\b|\b[1-9]\d*\b)\s+(\b0\b|\b[1-9]\d*\b)%\s+(\b0\b|\b[1-9]\d*\b)%",
        r"\s(\d+)\s+(\d+)%\s+(\d+)%\s",
        // HSV/HSB colors
        r"hsv\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*\)",
        r"hsva\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*,\s*([01]?\.?\d*)\s*\)",
        r"hsb\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*\)",
        r"hsba\s*\(\s*(\d+)\s*,\s*(\d+)%\s*,\s*(\d+)%\s*,\s*([01]?\.?\d*)\s*\)",
        // CMYK colors
        r"cmyk\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*,\s*(\d+(?:\.\d+)?)%\s*\)",
        // LAB colors
        r"lab\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)\s*\)",
        // LCH colors
        r"lch\s*\(\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)\s*\)",
        // OKLAB colors
        r"oklab\s*\(\s*(\d+(?:\.\d+)?)%\s*,\s*(-?\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)\s*\)",
        // OKLCH colors
        r"oklch\s*\(\s*(\d+(?:\.\d+)?)%\s*,?\s*(\d+(?:\.\d+)?)\s*,?\s*(\d+(?:\.\d+)?)\s*\)",
        r"oklch\s*\(\s*(\d*\.?\d+)\s+(\d*\.?\d+)\s+(\d+(?:\.\d+)?)\s*\)",
    ];
    
    let mut result = text.to_string();
    
    for pattern in color_patterns {
        let regex = Regex::new(pattern).unwrap();
        let mut offset = 0;
        
        while let Some(color_match) = regex.find(&result[offset..]) {
            let start = offset + color_match.start();
            let end = offset + color_match.end();
            let color_str = &result[start..end];
            
            // Special handling for space-separated HSL
            let color = if pattern.contains("\\b0\\b|\\b[1-9]") || pattern.contains("\\s(\\d+)") {
                // Convert space-separated HSL to standard HSL format
                let caps = regex.captures(color_str).unwrap();
                let h = caps.get(1).map_or("0", |m| m.as_str());
                let s = caps.get(2).map_or("0", |m| m.as_str());
                let l = caps.get(3).map_or("0", |m| m.as_str());
                let hsl_str = format!("hsl({}, {}%, {}%)", h, s, l);
                BigColor::new(&hsl_str)
            } else {
                BigColor::new(color_str)
            };
            
            if color.is_valid() {
                let converted = color.to(target_format);
                result.replace_range(start..end, &converted);
                offset = start + converted.len();
            } else {
                offset = end;
            }
        }
    }
    
    result
}

// Add after the ColorPreview struct
#[derive(Properties, PartialEq)]
struct ContrastPreviewProps {
    color: BigColor,
    intensity: f32,
}

#[function_component(ContrastPreview)]
fn contrast_preview(props: &ContrastPreviewProps) -> Html {
    let contrast_color = props.color.get_contrast_color(props.intensity);
    let contrast_ratio = props.color.get_contrast_ratio(&contrast_color);
    let wcag_pass = if contrast_ratio >= 4.5 { "AA" } else if contrast_ratio >= 3.0 { "AA Large" } else { "Fail" };
    
    let grid_style = format!("display: grid; grid-template-columns: repeat(auto-fill, minmax(2rem, 1fr)); grid-gap: 2px; padding: 10px; background-color: {};", get_css_compatible_color(&props.color));
    
    // Background color copied state
    let bg_copied = use_state(|| false);
    let bg_copied_clone = bg_copied.clone();
    
    let on_bg_copy = {
        let color_value = props.color.to_string(None);
        Callback::from(move |_: MouseEvent| {
            copy_to_clipboard(&color_value);
            bg_copied_clone.set(true);
            
            let bg_copied_inner = bg_copied_clone.clone();
            let timeout = Timeout::new(2000, move || {
                bg_copied_inner.set(false);
            });
            timeout.forget();
        })
    };
    
    // Contrast color copied state
    let contrast_copied = use_state(|| false);
    let contrast_copied_clone = contrast_copied.clone();
    
    let on_contrast_copy = {
        let color_value = contrast_color.to_string(None);
        Callback::from(move |_: MouseEvent| {
            copy_to_clipboard(&color_value);
            contrast_copied_clone.set(true);
            
            let contrast_copied_inner = contrast_copied_clone.clone();
            let timeout = Timeout::new(2000, move || {
                contrast_copied_inner.set(false);
            });
            timeout.forget();
        })
    };
    
    html! {
        <div class="contrast-preview">
            <div class="info-section">
                <div class="info-column">
                    <span class="info-label">{"Background Color"}</span>
                    <span 
                        class={classes!("info-value", "clickable", (*bg_copied).then_some("copied"))}
                        onclick={on_bg_copy}
                        title={"Click to copy"}
                    >
                        <span 
                            class="color-swatch" 
                            style={format!("background-color: {};", get_css_compatible_color(&props.color))}
                        ></span>
                        {props.color.to_string(None)}
                        {
                            if *bg_copied {
                                html! { <div class="copy-badge small">{"Copied!"}</div> }
                            } else {
                                html! {}
                            }
                        }
                    </span>
                </div>
                
                <div class="info-column">
                    <span class="info-label">{"Contrast Color"}</span>
                    <span 
                        class={classes!("info-value", "clickable", (*contrast_copied).then_some("copied"))}
                        onclick={on_contrast_copy}
                        title={"Click to copy"}
                    >
                        <span 
                            class="color-swatch" 
                            style={format!("background-color: {};", get_css_compatible_color(&contrast_color))}
                        ></span>
                        {contrast_color.to_string(None)}
                        {
                            if *contrast_copied {
                                html! { <div class="copy-badge small">{"Copied!"}</div> }
                            } else {
                                html! {}
                            }
                        }
                    </span>
                </div>
                
                <div class="info-column">
                    <span class="info-label">{"Contrast Ratio"}</span>
                    <span class="info-value">
                        {format!("{:.2}:1", contrast_ratio)}
                        <span class="contrast-badge">
                            {wcag_pass}
                        </span>
                    </span>
                </div>
            </div>
            
            <div class="grid-container" style={grid_style}>
                {
                    (0..48).map(|i| {
                        let cell_style = format!("background-color: {}; width: 2rem; height: 2rem; display: flex; justify-content: center; align-items: center; color: {}; border: 1px solid rgba(0,0,0,0.1);", 
                            get_css_compatible_color(&props.color), 
                            get_css_compatible_color(&contrast_color)
                        );
                        html! {
                            <div 
                                class="grid-cell" 
                                style={cell_style}
                                onclick={
                                    let color_value = contrast_color.to_string(None);
                                    Callback::from(move |_: MouseEvent| {
                                        copy_to_clipboard(&color_value);
                                    })
                                }
                                title={"Click to copy contrast color"}
                            >
                                {(i+1).to_string()}
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            
            <div class="text-sample">
                <div 
                    style={format!("background-color: {}; padding: 15px; color: {}; cursor: pointer;", 
                        get_css_compatible_color(&props.color), 
                        get_css_compatible_color(&contrast_color)
                    )}
                    onclick={
                        let color_value = contrast_color.to_string(None);
                        Callback::from(move |_: MouseEvent| {
                            copy_to_clipboard(&color_value);
                        })
                    }
                    title={"Click to copy contrast color"}
                >
                    <h3>{"Sample Text with Contrast Color"}</h3>
                    <p>{"This is an example of text using the contrast color. You can adjust the intensity to see how it affects readability. The contrast ratio is measured according to WCAG guidelines. Click anywhere to copy the contrast color."}</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct OperationProps {
    pub operation_name: String,
    pub color_value: String,
}

#[function_component(OperationBox)]
fn operation_box(props: &OperationProps) -> Html {
    let copied = use_state(|| false);
    let copied_clone = copied.clone();
    
    let onclick = {
        let color_value = props.color_value.clone();
        Callback::from(move |_: MouseEvent| {
            copy_to_clipboard(&color_value);
            
            // Show "Copied!" indicator
            copied_clone.set(true);
            
            // Reset after 2 seconds
            let copied_clone_inner = copied_clone.clone();
            let timeout = Timeout::new(2000, move || {
                copied_clone_inner.set(false);
            });
            timeout.forget();
        })
    };
    
    let bg_style = format!("background-color: {}", props.color_value);
    
    html! {
        <div class={classes!("operation-box", (*copied).then_some("copied"))} onclick={onclick}>
            <div class="operation-name">{ &props.operation_name }</div>
            <div class="operation-result" style={bg_style}></div>
            {
                if *copied {
                    html! { <div class="copy-badge">{"Copied!"}</div> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

// Add a SchemeNameBox component for the schemes section
#[derive(Clone, PartialEq, Properties)]
pub struct SchemeNameProps {
    pub scheme_name: String,
    pub colors: Vec<BigColor>,
}

#[function_component(SchemeNameBox)]
fn scheme_name_box(props: &SchemeNameProps) -> Html {
    let copied = use_state(|| false);
    let copied_clone = copied.clone();
    
    let onclick = {
        let colors = props.colors.iter()
            .map(|c| c.to_string(None))
            .collect::<Vec<String>>();
        let json = format!("[{}]", colors.join(", "));
        
        Callback::from(move |_: MouseEvent| {
            copy_to_clipboard(&json);
            
            // Show "Copied!" indicator
            copied_clone.set(true);
            
            // Reset after 2 seconds
            let copied_clone_inner = copied_clone.clone();
            let timeout = Timeout::new(2000, move || {
                copied_clone_inner.set(false);
            });
            timeout.forget();
        })
    };
    
    html! {
        <div 
            class={classes!("scheme-name", (*copied).then_some("copied"))} 
            onclick={onclick}
            title={"Click to copy all colors as JSON"}
        >
            { &props.scheme_name }
            {
                if *copied {
                    html! { <div class="copy-badge">{"Copied!"}</div> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let color_input = use_state(|| String::from("#1a6ef5"));
    let color = use_state(|| BigColor::new("#1a6ef5"));
    let show_error = use_state(|| false);
    
    let oninput = {
        let color_input = color_input.clone();
        let color = color.clone();
        let show_error = show_error.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value();
                color_input.set(value.clone());
                
                let new_color = BigColor::new(&value);
                if new_color.is_valid() {
                    color.set(new_color);
                    show_error.set(false);
                } else {
                    show_error.set(true);
                }
            }
        })
    };
    
    // Create color variants
    let format_variants = vec![
        ("HEX", color.to_hex_string(false)),
        ("HEX8", color.to_hex8_string(false)),
        ("RGB", color.to_rgb_string()),
        ("Percentage RGB", color.to_percentage_rgb_string()),
        ("HSL", color.to_hsl_string()),
        ("HSV", color.to_hsv_string()),
        ("HSB", color.to_hsb_string()),
        ("CMYK", color.to_cmyk_string()),
        ("LAB", color.to_lab_string()),
        ("LCH", color.to_lch_string()),
        ("OKLAB", color.to_oklab_string()),
        ("OKLCH", color.to_oklch_string()),
    ];
    
    // Create transformed color variants
    let lighten = {
        let mut c = color.clone_color();
        c.lighten(Some(20.0));
        ("Lighten 20%", c.to_hex_string(false))
    };
    
    let darken = {
        let mut c = color.clone_color();
        c.darken(Some(20.0));
        ("Darken 20%", c.to_hex_string(false))
    };
    
    let saturate = {
        let mut c = color.clone_color();
        c.saturate(Some(20.0));
        ("Saturate 20%", c.to_hex_string(false))
    };
    
    let desaturate = {
        let mut c = color.clone_color();
        c.desaturate(Some(20.0));
        ("Desaturate 20%", c.to_hex_string(false))
    };
    
    let greyscale = {
        let mut c = color.clone_color();
        c.greyscale();
        ("Greyscale", c.to_hex_string(false))
    };
    
    let operations = vec![lighten, darken, saturate, desaturate, greyscale];
    
    // New states for bulk color converter
    let input_text = use_state(|| String::from(""));
    let output_text = use_state(|| String::from(""));
    let target_format = use_state(|| ColorFormat::OKLCH);
    
    // Handler for input text change
    let on_input_text_change = {
        let input_text = input_text.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let textarea: HtmlTextAreaElement = target.dyn_into().unwrap();
                input_text.set(textarea.value());
            }
        })
    };
    
    // Handler for format selection change
    let on_format_change = {
        let target_format = target_format.clone();
        
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                let select: HtmlSelectElement = target.dyn_into().unwrap();
                let format_index = select.selected_index() as usize;
                let formats = get_color_format_options();
                if format_index < formats.len() {
                    target_format.set(formats[format_index].1);
                }
            }
        })
    };
    
    // Handler for convert button click
    let on_convert_click = {
        let input_text = input_text.clone();
        let output_text = output_text.clone();
        let target_format = target_format.clone();
        
        Callback::from(move |_: MouseEvent| {
            let converted = convert_colors_in_text(&input_text, *target_format);
            output_text.set(converted);
        })
    };
    
    // Copy output text
    let on_copy_output = {
        let output_text = output_text.clone();
        
        Callback::from(move |_: MouseEvent| {
            copy_to_clipboard(&output_text);
        })
    };
    
    // Add these new state variables
    let contrast_input_color = use_state(|| String::from("#1a6ef5"));
    let contrast_intensity = use_state(|| 0.5);
    let contrast_color = use_state(|| BigColor::new("#1a6ef5"));
    let contrast_error = use_state(|| false);
    
    // Add new callbacks for contrast section
    let on_contrast_color_change = {
        let contrast_input_color = contrast_input_color.clone();
        let contrast_color = contrast_color.clone();
        let contrast_error = contrast_error.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value();
                contrast_input_color.set(value.clone());
                
                let new_color = BigColor::new(&value);
                if new_color.is_valid() {
                    contrast_color.set(new_color);
                    contrast_error.set(false);
                } else {
                    contrast_error.set(true);
                }
            }
        })
    };
    
    let on_intensity_change = {
        let contrast_intensity = contrast_intensity.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value().parse::<f32>().unwrap_or(0.5);
                contrast_intensity.set(value);
            }
        })
    };
    
    html! {
        <div class="container">
            <div class="header">
                <h1>{ "bigcolor" }</h1>
            </div>
            
            <div class="color-input">
                <input 
                    type="text" 
                    placeholder="Enter a color (e.g. #1a6ef5, rgb(255,0,0), hsl(120, 100%, 50%))" 
                    value={color_input.to_string()}
                    oninput={oninput}
                />
            </div>
            
            {
                if *show_error {
                    html! {
                        <div class="error-message">
                            { format!("'{}' is not a valid color format", *color_input) }
                        </div>
                    }
                } else {
                    html! {
                        <>
                            <div class="color-preview" style={format!("background-color: {}", get_css_compatible_color(&color))}></div>
                            
                            <div class="color-info">
                                <div class="color-property">
                                    <span>{ "Format" }</span>
                                    <code>{ format!("{:?}", color.get_format()) }</code>
                                </div>
                                <div class="color-property">
                                    <span>{ "Original Input" }</span>
                                    <code>{ color.get_original_input() }</code>
                                </div>
                                <div class="color-property">
                                    <span>{ "Is Dark" }</span>
                                    <code>{ color.is_dark().to_string() }</code>
                                </div>
                                <div class="color-property">
                                    <span>{ "Alpha" }</span>
                                    <code>{ color.get_alpha().to_string() }</code>
                                </div>
                                <div class="color-property">
                                    <span>{ "Brightness" }</span>
                                    <code>{ color.get_brightness().to_string() }</code>
                                </div>
                                <div class="color-property">
                                    <span>{ "Luminance" }</span>
                                    <code>{ color.get_luminance().to_string() }</code>
                                </div>
                            </div>
                            
                            <h2 class="section-title">{ "Color Formats" }</h2>
                            <div class="format-grid">
                                {
                                    format_variants.into_iter().map(|(name, value)| {
                                        html! {
                                            <FormatBox format_name={name.to_string()} format_value={value} />
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                            
                            <h2 class="section-title">{ "Color Operations" }</h2>
                            <div class="operations-grid">
                                {
                                    operations.into_iter().map(|(name, value)| {
                                        let name_string = name.to_string(); 
                                        let value_string = value.clone();
                                        html! {
                                            <OperationBox operation_name={name_string} color_value={value_string} />
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                            
                            <h2 class="section-title">{ "Color Schemes" }</h2>
                            <div class="schemes-section">
                                <div class="scheme-box">
                                    <div 
                                        class="scheme-name"
                                        onclick={
                                            let colors = color.analogous(Some(5), Some(30))
                                                .iter()
                                                .map(|c| c.to_string(None))
                                                .collect::<Vec<String>>();
                                            let json = format!("[{}]", colors.join(", "));
                                            
                                            Callback::from(move |_: MouseEvent| {
                                                copy_to_clipboard(&json);
                                            })
                                        }
                                        title={"Click to copy all colors as JSON"}
                                    >{ "Analogous" }</div>
                                    <div class="scheme-colors">
                                        {
                                            color.analogous(Some(5), Some(30)).into_iter().map(|c| {
                                                let bg_style = format!("background-color: {}", get_css_compatible_color(&c));
                                                let color_value = c.to_string(None);
                                                html! {
                                                    <div 
                                                        class="scheme-color" 
                                                        style={bg_style}
                                                        onclick={
                                                            let color_value = color_value.clone();
                                                            Callback::from(move |_: MouseEvent| {
                                                                copy_to_clipboard(&color_value);
                                                            })
                                                        }
                                                        title={"Click to copy this color"}
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div 
                                        class="scheme-name"
                                        onclick={
                                            let colors = color.monochromatic(Some(5))
                                                .iter()
                                                .map(|c| c.to_string(None))
                                                .collect::<Vec<String>>();
                                            let json = format!("[{}]", colors.join(", "));
                                            Callback::from(move |_: MouseEvent| {
                                                copy_to_clipboard(&json);
                                            })
                                        }
                                        title={"Click to copy all colors as JSON"}
                                    >{ "Monochromatic" }</div>
                                    <div class="scheme-colors">
                                        {
                                            color.monochromatic(Some(5)).into_iter().map(|c| {
                                                let bg_style = format!("background-color: {}", get_css_compatible_color(&c));
                                                let color_value = c.to_string(None);
                                                html! {
                                                    <div 
                                                        class="scheme-color" 
                                                        style={bg_style}
                                                        onclick={
                                                            let color_value = color_value.clone();
                                                            Callback::from(move |_: MouseEvent| {
                                                                copy_to_clipboard(&color_value);
                                                            })
                                                        }
                                                        title={"Click to copy this color"}
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div 
                                        class="scheme-name"
                                        onclick={
                                            let colors = color.triad()
                                                .iter()
                                                .map(|c| c.to_string(None))
                                                .collect::<Vec<String>>();
                                            let json = format!("[{}]", colors.join(", "));
                                            Callback::from(move |_: MouseEvent| {
                                                copy_to_clipboard(&json);
                                            })
                                        }
                                        title={"Click to copy all colors as JSON"}
                                    >{ "Triad" }</div>
                                    <div class="scheme-colors">
                                        {
                                            color.triad().into_iter().map(|c| {
                                                let bg_style = format!("background-color: {}", get_css_compatible_color(&c));
                                                let color_value = c.to_string(None);
                                                html! {
                                                    <div 
                                                        class="scheme-color" 
                                                        style={bg_style}
                                                        onclick={
                                                            let color_value = color_value.clone();
                                                            Callback::from(move |_: MouseEvent| {
                                                                copy_to_clipboard(&color_value);
                                                            })
                                                        }
                                                        title={"Click to copy this color"}
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div 
                                        class="scheme-name"
                                        onclick={
                                            let colors = color.tetrad()
                                                .iter()
                                                .map(|c| c.to_string(None))
                                                .collect::<Vec<String>>();
                                            let json = format!("[{}]", colors.join(", "));
                                            Callback::from(move |_: MouseEvent| {
                                                copy_to_clipboard(&json);
                                            })
                                        }
                                        title={"Click to copy all colors as JSON"}
                                    >{ "Tetrad" }</div>
                                    <div class="scheme-colors">
                                        {
                                            color.tetrad().into_iter().map(|c| {
                                                let bg_style = format!("background-color: {}", get_css_compatible_color(&c));
                                                let color_value = c.to_string(None);
                                                html! {
                                                    <div 
                                                        class="scheme-color" 
                                                        style={bg_style}
                                                        onclick={
                                                            let color_value = color_value.clone();
                                                            Callback::from(move |_: MouseEvent| {
                                                                copy_to_clipboard(&color_value);
                                                            })
                                                        }
                                                        title={"Click to copy this color"}
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div 
                                        class="scheme-name"
                                        onclick={
                                            let colors = color.split_complement()
                                                .iter()
                                                .map(|c| c.to_string(None))
                                                .collect::<Vec<String>>();
                                            let json = format!("[{}]", colors.join(", "));
                                            Callback::from(move |_: MouseEvent| {
                                                copy_to_clipboard(&json);
                                            })
                                        }
                                        title={"Click to copy all colors as JSON"}
                                    >{ "Split Complement" }</div>
                                    <div class="scheme-colors">
                                        {
                                            color.split_complement().into_iter().map(|c| {
                                                let bg_style = format!("background-color: {}", get_css_compatible_color(&c));
                                                let color_value = c.to_string(None);
                                                html! {
                                                    <div 
                                                        class="scheme-color" 
                                                        style={bg_style}
                                                        onclick={
                                                            let color_value = color_value.clone();
                                                            Callback::from(move |_: MouseEvent| {
                                                                copy_to_clipboard(&color_value);
                                                            })
                                                        }
                                                        title={"Click to copy this color"}
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                            </div>
                            
                            <h2 class="section-title">{ "Bulk Color Converter" }</h2>
                            <div class="converter-section">
                                <div class="converter-description">
                                    { "Convert all colors in a text to your preferred format. Paste CSS, variables, or any text with color values." }
                                </div>
                                
                                <div class="converter-inputs">
                                    <textarea 
                                        class="converter-textarea"
                                        placeholder="Paste text with color values (e.g. CSS, variables)"
                                        value={(*input_text).clone()}
                                        oninput={on_input_text_change}
                                    />
                                    
                                    <div class="converter-controls">
                                        <div class="format-selector">
                                            <label for="format-select">{ "Convert to:" }</label>
                                            <select id="format-select" onchange={on_format_change}>
                                                {
                                                    get_color_format_options().into_iter().map(|(name, _)| {
                                                        html! { <option value={name.clone()}>{ name }</option> }
                                                    }).collect::<Html>()
                                                }
                                            </select>
                                        </div>
                                        <button class="convert-button" onclick={on_convert_click}>{ "Convert Colors" }</button>
                                    </div>
                                    
                                    <div class="output-container">
                                        <textarea 
                                            class="converter-textarea"
                                            readonly=true
                                            value={(*output_text).clone()}
                                        />
                                        {
                                            if !output_text.is_empty() {
                                                html! {
                                                    <button class="copy-button" onclick={on_copy_output}>
                                                        { "Copy Result" }
                                                    </button>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>
                            </div>
                            
                            <section class="contrast-section">
                                <h2 class="section-title">{"Contrast Color Explorer"}</h2>
                                <p class="section-description">
                                    {"Test contrast colors with different intensity levels. Find readable text colors for any background."}
                                </p>
                                
                                <div class="contrast-controls">
                                    <div class="input-group">
                                        <label for="contrast-color-input">{"Base Color:"}</label>
                                        <input
                                            id="contrast-color-input"
                                            type="text"
                                            placeholder="Enter a color (e.g. #1a6ef5, rgb(255,0,0))"
                                            value={(*contrast_input_color).clone()}
                                            oninput={on_contrast_color_change}
                                        />
                                        {
                                            if *contrast_error {
                                                html! {
                                                    <div class="error-message">
                                                        { format!("'{}' is not a valid color format", *contrast_input_color) }
                                                    </div>
                                                }
                                            } else {
                                                html! {
                                                    <div 
                                                        class="color-sample" 
                                                        style={format!("background-color: {}", (*contrast_color).to_rgb_string())}
                                                    ></div>
                                                }
                                            }
                                        }
                                    </div>
                                    
                                    <div class="input-group">
                                        <label for="intensity-slider">{"Contrast Intensity: "}{format!("{:.2}", *contrast_intensity)}</label>
                                        <input
                                            id="intensity-slider"
                                            type="range"
                                            min="0"
                                            max="1"
                                            step="0.01"
                                            value={(*contrast_intensity).to_string()}
                                            oninput={on_intensity_change}
                                        />
                                        <span class="intensity-labels">
                                            <span>{"Low"}</span>
                                            <span>{"High"}</span>
                                        </span>
                                    </div>
                                </div>
                                
                                {
                                    if !*contrast_error {
                                        html! {
                                            <ContrastPreview
                                                color={(*contrast_color).clone()}
                                                intensity={*contrast_intensity}
                                            />
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </section>
                        </>
                    }
                }
            }
            
            <footer>
                <p>{"BigColor - A Rust Color Manipulation Library"}</p>
                <div class="footer-links">
                    <p><a href="https://ducflair.com" target="_blank">{"Ducflair"}</a></p>
                    <p>{"|"}</p>
                    <p><a href="https://github.com/ducflair/bigcolor" target="_blank">{"GitHub"}</a></p>
                </div>
            </footer>
        </div>
    }
}
