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
    let target_format = use_state(|| ColorFormat::RGB);
    
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
                                        let bg_style = format!("background-color: {}", value);
                                        let color_value = value.clone();
                                        html! {
                                            <div class="operation-box">
                                                <div class="operation-name">{ name }</div>
                                                <div 
                                                    class="operation-result" 
                                                    style={bg_style}
                                                    onclick={
                                                        let color_value = color_value.clone();
                                                        Callback::from(move |_: MouseEvent| {
                                                            copy_to_clipboard(&color_value);
                                                        })
                                                    }
                                                ></div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                            
                            <h2 class="section-title">{ "Color Schemes" }</h2>
                            <div class="schemes-section">
                                <div class="scheme-box">
                                    <div class="scheme-name">{ "Analogous" }</div>
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
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div class="scheme-name">{ "Monochromatic" }</div>
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
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div class="scheme-name">{ "Triad" }</div>
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
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div class="scheme-name">{ "Tetrad" }</div>
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
                                                    ></div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <div class="scheme-box">
                                    <div class="scheme-name">{ "Split Complement" }</div>
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
                        </>
                    }
                }
            }
        </div>
    }
}
