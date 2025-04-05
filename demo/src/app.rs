use bigcolor::{BigColor, SolidColor, Gradient, GradientType};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::prelude::*;
use gloo::console::log;

#[derive(Clone, PartialEq)]
struct ColorState {
    input: String,
    color: Option<SolidColor>,
    error: Option<String>,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for ColorState {
    fn default() -> Self {
        Self {
            input: "#ff0000".to_string(),
            color: Some(SolidColor::new(1.0, 0.0, 0.0, 1.0)),
            error: None,
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

#[derive(Clone, PartialEq)]
struct GradientState {
    gradient_type: GradientType,
    stops: Vec<GradientStop>,
    angle: f32,
    css_string: String,
}

#[derive(Clone, PartialEq)]
struct GradientStop {
    color: String,
    position: f32,
}

impl Default for GradientState {
    fn default() -> Self {
        Self {
            gradient_type: GradientType::Linear,
            stops: vec![
                GradientStop { color: "#ff0000".to_string(), position: 0.0 },
                GradientStop { color: "#0000ff".to_string(), position: 1.0 },
            ],
            angle: 90.0,
            css_string: "linear-gradient(to right, red, blue)".to_string(),
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let color_state = use_state(ColorState::default);
    let gradient_state = use_state(GradientState::default);
    
    let update_color = {
        let color_state = color_state.clone();
        Callback::from(move |input: String| {
            let mut new_state = (*color_state).clone();
            new_state.input = input.clone();
            
            // Try to handle hex values without "#"
            let input_to_parse = if !input.starts_with('#') && input.chars().all(|c| c.is_ascii_hexdigit()) {
                format!("#{}", input)
            } else {
                input
            };
            
            match BigColor::parse(&input_to_parse) {
                Ok(BigColor::Solid(color)) => {
                    let rgba = color.to_rgba8();
                    new_state.r = rgba[0];
                    new_state.g = rgba[1];
                    new_state.b = rgba[2];
                    new_state.a = rgba[3];
                    new_state.color = Some(color);
                    new_state.error = None;
                },
                Ok(_) => {
                    new_state.error = Some("Parsed as gradient, not solid color".to_string());
                    new_state.color = None;
                },
                Err(e) => {
                    new_state.error = Some(format!("Error: {:?}", e));
                    new_state.color = None;
                }
            }
            
            color_state.set(new_state);
        })
    };
    
    let update_from_rgba = {
        let color_state = color_state.clone();
        Callback::from(move |(r, g, b, a): (u8, u8, u8, u8)| {
            let solid_color = SolidColor::from_rgba8(r, g, b, a);
            let mut new_state = (*color_state).clone();
            new_state.r = r;
            new_state.g = g;
            new_state.b = b;
            new_state.a = a;
            new_state.color = Some(solid_color.clone());
            new_state.input = solid_color.to_hex_string();
            new_state.error = None;
            color_state.set(new_state);
        })
    };
    
    let on_r_change = {
        let update_from_rgba = update_from_rgba.clone();
        let color_state = color_state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value().parse::<u8>().unwrap_or(0);
                let state = &*color_state;
                update_from_rgba.emit((value, state.g, state.b, state.a));
            }
        })
    };
    
    let on_g_change = {
        let update_from_rgba = update_from_rgba.clone();
        let color_state = color_state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value().parse::<u8>().unwrap_or(0);
                let state = &*color_state;
                update_from_rgba.emit((state.r, value, state.b, state.a));
            }
        })
    };
    
    let on_b_change = {
        let update_from_rgba = update_from_rgba.clone();
        let color_state = color_state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value().parse::<u8>().unwrap_or(0);
                let state = &*color_state;
                update_from_rgba.emit((state.r, state.g, value, state.a));
            }
        })
    };
    
    let on_a_change = {
        let update_from_rgba = update_from_rgba.clone();
        let color_state = color_state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let value = input.value().parse::<u8>().unwrap_or(255);
                let state = &*color_state;
                update_from_rgba.emit((state.r, state.g, state.b, value));
            }
        })
    };
    
    let on_input_change = {
        let update_color = update_color.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                update_color.emit(input.value());
            }
        })
    };
    
    let on_parse_gradient = {
        let gradient_state = gradient_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let state = &*gradient_state;
            
            match Gradient::from_css_string(&state.css_string) {
                Ok(_gradient) => {
                    log!("Parsed gradient successfully");
                    
                    // Update the gradient preview with parsed result
                    let new_state = (*gradient_state).clone();
                    
                    // Set new gradient state
                    gradient_state.set(new_state);
                },
                Err(_) => {
                    log!("Error parsing gradient");
                }
            }
        })
    };
    
    let on_gradient_css_change = {
        let gradient_state = gradient_state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input: HtmlInputElement = target.dyn_into().unwrap();
                let mut new_state = (*gradient_state).clone();
                new_state.css_string = input.value();
                
                // Try to parse the gradient automatically
                let css_string = new_state.css_string.clone(); // Clone before moving
                gradient_state.set(new_state);
                
                if let Ok(_gradient) = Gradient::from_css_string(&css_string) {
                    log!("Auto-parsed gradient successfully");
                    // Success already handled by setting the state above
                }
            }
        })
    };
    
    let format_copy = {
        Callback::from(move |text: String| {
            // Just log that user clicked on the format
            log!("Format clicked: ", text);
        })
    };
    
    let current_color_style = if let Some(color) = &color_state.color {
        let rgba = color.to_rgba8();
        format!("background-color: rgba({}, {}, {}, {})", rgba[0], rgba[1], rgba[2], rgba[3] as f32 / 255.0)
    } else {
        "background-color: #cccccc".to_string()
    };
    
    let complementary_color = color_state.color.as_ref().map(|color| {
        let [h, s, l, a] = color.to_hsla();
        let new_h = (h + 180.0) % 360.0;
        SolidColor::from_hsla(new_h, s, l, a)
    });
    
    let complementary_style = if let Some(color) = &complementary_color {
        let rgba = color.to_rgba8();
        format!("background-color: rgba({}, {}, {}, {})", rgba[0], rgba[1], rgba[2], rgba[3] as f32 / 255.0)
    } else {
        "background-color: #cccccc".to_string()
    };
    
    let hsl_text = color_state.color.as_ref().map(|color| color.to_hsl_string()).unwrap_or_default();
    let rgb_text = color_state.color.as_ref().map(|color| color.to_rgb_string()).unwrap_or_default();
    let hex_text = color_state.color.as_ref().map(|color| color.to_hex_string()).unwrap_or_default();
    let hsv_text = color_state.color.as_ref().map(|color| color.to_hsv_string()).unwrap_or_default();
    let cmyk_text = color_state.color.as_ref().map(|color| color.to_cmyk_string()).unwrap_or_default();
    
    let gradient_preview_style = if !gradient_state.css_string.is_empty() && Gradient::from_css_string(&gradient_state.css_string).is_ok() {
        format!("background: {}", gradient_state.css_string)
    } else {
        match gradient_state.gradient_type {
            GradientType::Linear => format!("background: linear-gradient({}deg, {}, {})",
                gradient_state.angle,
                gradient_state.stops[0].color,
                gradient_state.stops[1].color
            ),
            GradientType::Radial => format!("background: radial-gradient(circle, {}, {})",
                gradient_state.stops[0].color,
                gradient_state.stops[1].color
            ),
            GradientType::Conic => format!("background: conic-gradient(from {}deg, {}, {})",
                gradient_state.angle,
                gradient_state.stops[0].color,
                gradient_state.stops[1].color
            ),
        }
    };
    
    let input_style = if color_state.error.is_some() {
        "color-input error-input"
    } else {
        "color-input"
    };
    
    html! {
        <div class="container">
            <header>
                <h1>{"bigcolor"}</h1>
            </header>
            
            <div class="card">
                <h2>{"color parser"}</h2>
                <div class="color-preview" style={current_color_style}></div>
                
                <div class="color-input-row">
                    <input 
                        type="text"
                        class={input_style}
                        value={color_state.input.clone()}
                        oninput={on_input_change.clone()}
                    />
                    <button onclick={Callback::from(move |_| update_color.emit("#ff0000".to_string()))}>
                        {"reset"}
                    </button>
                </div>
                
                {
                    if let Some(error) = &color_state.error {
                        html! { <p style="color: #fc8181;">{error}</p> }
                    } else {
                        html! {}
                    }
                }
                
                <div class="color-slider red">
                    <label for="r-slider">{"red: "}{color_state.r}</label>
                    <input 
                        id="r-slider"
                        type="range" 
                        min="0" 
                        max="255" 
                        value={color_state.r.to_string()} 
                        oninput={on_r_change}
                    />
                </div>
                
                <div class="color-slider green">
                    <label for="g-slider">{"green: "}{color_state.g}</label>
                    <input 
                        id="g-slider"
                        type="range" 
                        min="0" 
                        max="255" 
                        value={color_state.g.to_string()} 
                        oninput={on_g_change}
                    />
                </div>
                
                <div class="color-slider blue">
                    <label for="b-slider">{"blue: "}{color_state.b}</label>
                    <input 
                        id="b-slider"
                        type="range" 
                        min="0" 
                        max="255" 
                        value={color_state.b.to_string()} 
                        oninput={on_b_change}
                    />
                </div>
                
                <div class="color-slider alpha">
                    <label for="a-slider">{"alpha: "}{(color_state.a as f32 / 255.0 * 100.0).round()}{"% ("}{color_state.a}{")"}</label>
                    <input 
                        id="a-slider"
                        type="range" 
                        min="0" 
                        max="255" 
                        value={color_state.a.to_string()} 
                        oninput={on_a_change}
                    />
                </div>
                
                <h3>{"formats"}</h3>
                <div class="color-formats">
                    <div class="format" onclick={
                        let text = hex_text.clone();
                        format_copy.clone().reform(move |_| text.clone())
                    }>{hex_text}</div>
                    <div class="format" onclick={
                        let text = rgb_text.clone();
                        format_copy.clone().reform(move |_| text.clone())
                    }>{rgb_text}</div>
                    <div class="format" onclick={
                        let text = hsl_text.clone();
                        format_copy.clone().reform(move |_| text.clone())
                    }>{hsl_text}</div>
                    <div class="format" onclick={
                        let text = hsv_text.clone();
                        format_copy.clone().reform(move |_| text.clone())
                    }>{hsv_text}</div>
                    <div class="format" onclick={
                        let text = cmyk_text.clone();
                        format_copy.clone().reform(move |_| text.clone())
                    }>{cmyk_text}</div>
                </div>
            </div>
            
            <div class="card">
                <h2>{"color theory"}</h2>
                <div class="grid">
                    <div>
                        <h3>{"complementary"}</h3>
                        <div class="color-swatch" style={complementary_style}></div>
                        <div class="color-swatch-label">
                            {complementary_color.as_ref().map(|c| c.to_hex_string()).unwrap_or_default()}
                        </div>
                    </div>
                    
                    <div>
                        <h3>{"darker"}</h3>
                        <div class="color-swatch" style={
                            color_state.color.as_ref().map(|c| {
                                let [h, s, l, a] = c.to_hsla();
                                let darkened = SolidColor::from_hsla(h, s, (l * 0.7).clamp(0.0, 1.0), a);
                                let rgba = darkened.to_rgba8();
                                format!("background-color: rgba({}, {}, {}, {})", 
                                    rgba[0], rgba[1], rgba[2], rgba[3] as f32 / 255.0)
                            }).unwrap_or_default()
                        }></div>
                    </div>
                    
                    <div>
                        <h3>{"lighter"}</h3>
                        <div class="color-swatch" style={
                            color_state.color.as_ref().map(|c| {
                                let [h, s, l, a] = c.to_hsla();
                                let lightened = SolidColor::from_hsla(h, s, (l * 1.3).clamp(0.0, 1.0), a);
                                let rgba = lightened.to_rgba8();
                                format!("background-color: rgba({}, {}, {}, {})", 
                                    rgba[0], rgba[1], rgba[2], rgba[3] as f32 / 255.0)
                            }).unwrap_or_default()
                        }></div>
                    </div>
                    
                    <div>
                        <h3>{"saturated"}</h3>
                        <div class="color-swatch" style={
                            color_state.color.as_ref().map(|c| {
                                let [h, s, l, a] = c.to_hsla();
                                let saturated = SolidColor::from_hsla(h, (s * 1.5).clamp(0.0, 1.0), l, a);
                                let rgba = saturated.to_rgba8();
                                format!("background-color: rgba({}, {}, {}, {})", 
                                    rgba[0], rgba[1], rgba[2], rgba[3] as f32 / 255.0)
                            }).unwrap_or_default()
                        }></div>
                    </div>
                </div>
            </div>
            
            <div class="card">
                <h2>{"gradients"}</h2>
                <div class="gradient-preview" style={gradient_preview_style}></div>
                
                <div class="color-input-row">
                    <input 
                        type="text"
                        class="color-input"
                        value={gradient_state.css_string.clone()}
                        oninput={on_gradient_css_change}
                    />
                    <button onclick={on_parse_gradient}>{"parse"}</button>
                </div>
                
                <div class="grid">
                    <div>
                        <h3>{"linear"}</h3>
                        <div class="color-swatch" style="background: linear-gradient(to right, red, blue);"></div>
                    </div>
                    
                    <div>
                        <h3>{"radial"}</h3>
                        <div class="color-swatch" style="background: radial-gradient(circle, yellow, red);"></div>
                    </div>
                    
                    <div>
                        <h3>{"conic"}</h3>
                        <div class="color-swatch" style="background: conic-gradient(from 0deg, red, yellow, blue, red);"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
