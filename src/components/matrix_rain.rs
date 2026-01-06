use yew::prelude::*;
use web_sys::window;

/// Matrix-style falling characters background effect
#[function_component(MatrixRain)]
pub fn matrix_rain() -> Html {
    let columns = use_state(|| 0);
    let drops = use_state(Vec::new);
    
    // Characters to use in the rain (mix of symbols and characters)
    let chars: Vec<char> = "01フォスミート自由ソフトウェア{}[]<>/*-+=#@$%^&"
        .chars()
        .collect();
    
    {
        let columns = columns.clone();
        let drops = drops.clone();
        
        use_effect_with((), move |_| {
            // Get window width
            if let Some(win) = window() {
                let width = win.inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1920.0) as usize;
                let col_count = width / 20; // Each column is ~20px wide
                columns.set(col_count);
                
                // Initialize drops at random positions
                let initial_drops: Vec<i32> = (0..col_count)
                    .map(|_| (js_sys::Math::random() * -100.0) as i32)
                    .collect();
                drops.set(initial_drops);
            }
            
            || ()
        });
    }
    
    html! {
        <div class="matrix-bg" aria-hidden="true">
            <svg 
                width="100%" 
                height="100%" 
                style="position: absolute; top: 0; left: 0;"
            >
                <defs>
                    <linearGradient id="matrix-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" style="stop-color:#FF6B00;stop-opacity:0.8" />
                        <stop offset="50%" style="stop-color:#FF6B00;stop-opacity:0.3" />
                        <stop offset="100%" style="stop-color:#FF6B00;stop-opacity:0" />
                    </linearGradient>
                </defs>
                
                {for (0..*columns).map(|i| {
                    let x = i * 20 + 10;
                    let delay = (js_sys::Math::random() * 5.0) as f32;
                    let duration = 8.0 + (js_sys::Math::random() * 12.0) as f32;
                    
                    // Pick a random character
                    let char_idx = ((js_sys::Math::random() * chars.len() as f64) as usize) % chars.len();
                    let char = chars[char_idx];
                    
                    html! {
                        <text
                            x={x.to_string()}
                            y="0"
                            fill="url(#matrix-gradient)"
                            font-family="JetBrains Mono, monospace"
                            font-size="16"
                            style={format!(
                                "animation: matrix-fall {}s linear {}s infinite;",
                                duration, delay
                            )}
                        >
                            {char}
                        </text>
                    }
                })}
            </svg>
            
            <style>
                {r#"
                @keyframes matrix-fall {
                    0% {
                        transform: translateY(-100vh);
                        opacity: 0;
                    }
                    10% {
                        opacity: 1;
                    }
                    90% {
                        opacity: 1;
                    }
                    100% {
                        transform: translateY(100vh);
                        opacity: 0;
                    }
                }
                "#}
            </style>
        </div>
    }
}
