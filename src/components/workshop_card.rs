use yew::prelude::*;
use crate::types::Workshop;

#[derive(Properties, PartialEq)]
pub struct WorkshopCardProps {
    pub workshop: Workshop,
    #[prop_or_default]
    pub onclick: Option<Callback<Workshop>>,
}

#[function_component(WorkshopCard)]
pub fn workshop_card(props: &WorkshopCardProps) -> Html {
    let workshop = props.workshop.clone();
    let onclick = props.onclick.clone();
    
    let handle_click = {
        let workshop = workshop.clone();
        Callback::from(move |_| {
            if let Some(ref cb) = onclick {
                cb.emit(workshop.clone());
            }
        })
    };
    
    html! {
        <div class="workshop-card" onclick={handle_click}>
            <div class="workshop-card-header">
                <div class="workshop-card-icon">
                    <img 
                        src={workshop.logo_url.clone()} 
                        alt={format!("{} logo", workshop.title)} 
                        loading="lazy"
                    />
                </div>
                <h3 class="workshop-card-title">{&workshop.title}</h3>
                <div class="workshop-card-meta">
                    <span class="workshop-card-tag">{&workshop.date}</span>
                    <span class="workshop-card-tag">{format!("{} hrs", &workshop.duration)}</span>
                </div>
            </div>
            
            <div class="workshop-card-body">
                <p class="workshop-card-description">{&workshop.description}</p>
                
                if !workshop.prerequisites.is_empty() {
                    <div class="workshop-card-prereqs">
                        <span class="prereqs-label">{"// Prerequisites:"}</span>
                        <ul class="prereqs-list">
                            {for workshop.prerequisites.iter().map(|prereq| {
                                html! { <li>{format!("- {}", prereq)}</li> }
                            })}
                        </ul>
                    </div>
                }
                
                <div class="workshop-card-instructor">
                    <img 
                        src={workshop.speaker_image_url.clone()} 
                        alt={format!("{} photo", workshop.speaker_name)} 
                        loading="lazy"
                    />
                    <div>
                        <span class="workshop-card-instructor-name">{&workshop.speaker_name}</span>
                        <span class="workshop-card-instructor-label">{"Instructor"}</span>
                    </div>
                </div>
            </div>
            
            if let Some(ref link) = workshop.registration_link {
                <div class="workshop-card-footer">
                    <a 
                        href={link.clone()} 
                        target="_blank" 
                        rel="noopener noreferrer"
                        class="btn btn-primary workshop-card-btn"
                        onclick={|e: MouseEvent| e.stop_propagation()}
                    >
                        {"Register"}
                        <svg viewBox="0 0 24 24" width="16" height="16">
                            <path fill="currentColor" d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
                        </svg>
                    </a>
                </div>
            }
        </div>
    }
}
