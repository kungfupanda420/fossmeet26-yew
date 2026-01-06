use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GlitchTextProps {
    pub text: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub active: bool,
}

/// Text component with glitch effect
#[function_component(GlitchText)]
pub fn glitch_text(props: &GlitchTextProps) -> Html {
    html! {
        <span 
            class={classes!("glitch", props.active.then_some("glitch-active"), props.class.clone())}
            data-text={props.text.clone()}
        >
            {&props.text}
        </span>
    }
}

/// ASCII art logo component
#[function_component(AsciiLogo)]
pub fn ascii_logo() -> Html {
    let logo = r#"
███████╗ ██████╗ ███████╗███████╗███╗   ███╗███████╗███████╗████████╗
██╔════╝██╔═══██╗██╔════╝██╔════╝████╗ ████║██╔════╝██╔════╝╚══██╔══╝
█████╗  ██║   ██║███████╗███████╗██╔████╔██║█████╗  █████╗     ██║   
██╔══╝  ██║   ██║╚════██║╚════██║██║╚██╔╝██║██╔══╝  ██╔══╝     ██║   
██║     ╚██████╔╝███████║███████║██║ ╚═╝ ██║███████╗███████╗   ██║   
╚═╝      ╚═════╝ ╚══════╝╚══════╝╚═╝     ╚═╝╚══════╝╚══════╝   ╚═╝   
                            ' 2 6
    "#;
    
    html! {
        <pre class="ascii-logo text-glow">
            {logo}
        </pre>
    }
}

/// Typing animation text component
#[derive(Properties, PartialEq)]
pub struct TypingTextProps {
    pub text: String,
    #[prop_or(50)]
    pub speed_ms: u32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TypingText)]
pub fn typing_text(props: &TypingTextProps) -> Html {
    let displayed = use_state(String::new);
    let index = use_state(|| 0usize);
    
    {
        let text = props.text.clone();
        let speed = props.speed_ms;
        let displayed = displayed.clone();
        let index = index.clone();
        
        use_effect_with(text.clone(), move |_| {
            use gloo_timers::callback::Interval;
            
            let interval = Interval::new(speed, move || {
                let current_idx = *index;
                if current_idx < text.len() {
                    displayed.set(text[..current_idx + 1].to_string());
                    index.set(current_idx + 1);
                }
            });
            
            move || drop(interval)
        });
    }
    
    html! {
        <span class={classes!("typing-text", props.class.clone())}>
            {(*displayed).clone()}
            <span class="typing-cursor blink">{"_"}</span>
        </span>
    }
}
