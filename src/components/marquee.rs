use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarqueeProps {
    pub items: Vec<String>,
    #[prop_or(30)]
    pub speed: u32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Marquee)]
pub fn marquee(props: &MarqueeProps) -> Html {
    // Double the items for seamless loop
    let items: Vec<_> = props.items.iter()
        .chain(props.items.iter())
        .cloned()
        .collect();
    
    let animation_style = format!("animation-duration: {}s;", props.speed);
    
    html! {
        <div class={classes!("marquee", props.class.clone())}>
            <div class="marquee-content" style={animation_style}>
                {for items.iter().map(|item| {
                    html! {
                        <span class="marquee-item">{item}</span>
                    }
                })}
            </div>
        </div>
    }
}
