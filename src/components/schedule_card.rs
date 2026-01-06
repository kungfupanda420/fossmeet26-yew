use yew::prelude::*;
use crate::types::{ScheduleEvent, EventType};

#[derive(Properties, PartialEq)]
pub struct ScheduleCardProps {
    pub event: ScheduleEvent,
}

#[function_component(ScheduleCard)]
pub fn schedule_card(props: &ScheduleCardProps) -> Html {
    let event = &props.event;
    
    let type_class = match event.event_type {
        EventType::Keynote => "event-keynote",
        EventType::Talk => "event-talk",
        EventType::Workshop => "event-workshop",
        EventType::Break => "event-break",
        EventType::Panel => "event-panel",
        EventType::Networking => "event-networking",
    };
    
    html! {
        <div class={classes!("schedule-event", type_class)}>
            <div class="schedule-event-time">
                <span class="time-value">{&event.time}</span>
                <span class="time-duration">{format!("({})", &event.duration)}</span>
            </div>
            
            <div class="schedule-event-content">
                <h4 class="schedule-event-title">{&event.title}</h4>
                
                if let Some(ref speaker) = event.speaker {
                    <p class="schedule-event-speaker">
                        <span class="speaker-prefix">{"by "}</span>
                        {&speaker.name}
                    </p>
                }
                
                if let Some(ref location) = event.location {
                    <p class="schedule-event-location">
                        <svg viewBox="0 0 24 24" width="14" height="14">
                            <path fill="currentColor" d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/>
                        </svg>
                        {location}
                    </p>
                }
                
                <span class="schedule-event-type">{event.event_type.as_str()}</span>
            </div>
            
            <div class="schedule-event-indicator">
                <div class="indicator-dot"></div>
                <div class="indicator-line"></div>
            </div>
        </div>
    }
}
