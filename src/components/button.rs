use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("button".to_string())]
    pub variant: String,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(false)]
    pub external: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_class = match props.variant.as_str() {
        "primary" => "btn-primary",
        "secondary" => "btn-secondary",
        "ghost" => "btn-ghost",
        _ => "btn-primary",
    };
    
    let classes = classes!(
        "btn",
        variant_class,
        props.class.clone(),
        props.disabled.then_some("btn-disabled")
    );
    
    if let Some(ref href) = props.href {
        if props.external {
            html! {
                <a 
                    class={classes}
                    href={href.clone()}
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    {props.children.clone()}
                    <svg viewBox="0 0 24 24" width="16" height="16" class="btn-icon-external">
                        <path fill="currentColor" d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
                    </svg>
                </a>
            }
        } else {
            html! {
                <a 
                    class={classes}
                    href={href.clone()}
                >
                    {props.children.clone()}
                </a>
            }
        }
    } else {
        html! {
            <button 
                class={classes}
                onclick={props.onclick.clone()}
                disabled={props.disabled}
            >
                {props.children.clone()}
            </button>
        }
    }
}
