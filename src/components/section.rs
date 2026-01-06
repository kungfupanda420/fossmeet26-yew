use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub tag: Option<String>,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section 
            class={classes!("section", props.class.clone())} 
            id={props.id.clone()}
        >
            <div class="container">
                if props.title.is_some() || props.tag.is_some() {
                    <div class="section-header">
                        if let Some(ref tag) = props.tag {
                            <span class="section-tag">{tag}</span>
                        }
                        if let Some(ref title) = props.title {
                            <h2 class="section-title">
                                {render_title_with_outline(title)}
                            </h2>
                        }
                        if let Some(ref subtitle) = props.subtitle {
                            <p class="section-subtitle">{subtitle}</p>
                        }
                    </div>
                }
                {props.children.clone()}
            </div>
        </section>
    }
}

/// Renders title with outlined text for words in brackets
/// Example: "Meet our [Speakers]" -> "Meet our" + outlined "Speakers"
fn render_title_with_outline(title: &str) -> Html {
    let parts: Vec<&str> = title.split(['[', ']']).collect();
    
    html! {
        <>
            {for parts.iter().enumerate().map(|(i, part)| {
                if i % 2 == 1 {
                    // This is the part that was inside brackets
                    html! { <span class="outline">{*part}</span> }
                } else {
                    html! { <>{*part}</> }
                }
            })}
        </>
    }
}
