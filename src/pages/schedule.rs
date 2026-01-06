use yew::prelude::*;
use crate::components::{
    section::Section,
    schedule_card::ScheduleCard,
};
use crate::types::{ScheduleDay, ScheduleEvent, EventType, Speaker};

#[function_component(SchedulePage)]
pub fn schedule_page() -> Html {
    let schedule = get_schedule();
    let active_day = use_state(|| 0usize);
    
    html! {
        <div class="page-schedule">
            // Page Header
            <section class="page-header">
                <div class="container">
                    <div class="page-header-content">
                        <span class="page-tag">{"// schedule"}</span>
                        <h1 class="page-title">
                            {"Event "}
                            <span class="outline text-glow">{"Schedule"}</span>
                        </h1>
                        <p class="page-subtitle">
                            {"Three days of talks, workshops, and networking. Plan your FOSSMeet'26 experience."}
                        </p>
                        
                        // Terminal-style intro
                        <div class="page-terminal">
                            <code>
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"crontab -l | grep fossmeet"}</span>
                                <br />
                                <span class="terminal-output">{"0 9 14-16 3 * ~/attend_fossmeet26.sh"}</span>
                            </code>
                        </div>
                    </div>
                </div>
                
                <div class="page-header-decoration">
                    <div class="decoration-line"></div>
                    <div class="decoration-dots">
                        <span></span><span></span><span></span>
                    </div>
                </div>
            </section>
            
            // Day Selector
            <section class="schedule-selector">
                <div class="container">
                    <div class="day-tabs">
                        {for schedule.iter().enumerate().map(|(i, day)| {
                            let active = *active_day == i;
                            let onclick = {
                                let active_day = active_day.clone();
                                Callback::from(move |_| active_day.set(i))
                            };
                            html! {
                                <button 
                                    class={classes!("day-tab", active.then_some("active"))}
                                    onclick={onclick}
                                >
                                    <span class="day-tab-date">{&day.date}</span>
                                    <span class="day-tab-label">{&day.day_label}</span>
                                </button>
                            }
                        })}
                    </div>
                </div>
            </section>
            
            // Schedule Timeline
            <Section class="schedule-section">
                {if let Some(day) = schedule.get(*active_day) {
                    html! {
                        <div class="schedule-timeline">
                            <div class="timeline-header">
                                <h2>{&day.date}</h2>
                                <span class="timeline-label">{&day.day_label}</span>
                            </div>
                            
                            <div class="timeline-events">
                                {for day.events.iter().map(|event| {
                                    html! { <ScheduleCard event={event.clone()} /> }
                                })}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="no-schedule">
                            <code>{"// Schedule data not available"}</code>
                        </div>
                    }
                }}
            </Section>
            
            // Schedule Download
            <section class="schedule-download">
                <div class="container">
                    <div class="download-box">
                        <div class="download-info">
                            <h3>{"// Export Schedule"}</h3>
                            <p>{"Add FOSSMeet'26 to your calendar so you don't miss any sessions."}</p>
                        </div>
                        <div class="download-buttons">
                            <a href="/assets/fossmeet26.ics" class="btn btn-secondary" download="fossmeet26.ics">
                                <svg viewBox="0 0 24 24" width="18" height="18">
                                    <path fill="currentColor" d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM9 10H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2zm-8 4H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2z"/>
                                </svg>
                                {"Add to Calendar (.ics)"}
                            </a>
                            <a href="/assets/schedule.pdf" class="btn btn-secondary" download="fossmeet26-schedule.pdf">
                                <svg viewBox="0 0 24 24" width="18" height="18">
                                    <path fill="currentColor" d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
                                </svg>
                                {"Download PDF"}
                            </a>
                        </div>
                    </div>
                </div>
            </section>
            
            // Venue Info
            <section class="venue-info">
                <div class="container">
                    <h3>{"// Venue Information"}</h3>
                    <div class="venue-grid">
                        <div class="venue-card">
                            <h4>{"Main Auditorium"}</h4>
                            <p>{"Keynotes and main talks"}</p>
                            <span class="venue-capacity">{"Capacity: 500"}</span>
                        </div>
                        <div class="venue-card">
                            <h4>{"Workshop Hall A"}</h4>
                            <p>{"Technical workshops"}</p>
                            <span class="venue-capacity">{"Capacity: 50"}</span>
                        </div>
                        <div class="venue-card">
                            <h4>{"Workshop Hall B"}</h4>
                            <p>{"Technical workshops"}</p>
                            <span class="venue-capacity">{"Capacity: 50"}</span>
                        </div>
                        <div class="venue-card">
                            <h4>{"Open Area"}</h4>
                            <p>{"Networking & projects showcase"}</p>
                            <span class="venue-capacity">{"Open space"}</span>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn get_schedule() -> Vec<ScheduleDay> {
    vec![
        ScheduleDay {
            date: "March 14".to_string(),
            day_label: "Day 1 - Opening".to_string(),
            events: vec![
                ScheduleEvent {
                    id: "d1-1".to_string(),
                    title: "Registration & Check-in".to_string(),
                    time: "08:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Networking,
                    speaker: None,
                    location: Some("Main Entrance".to_string()),
                },
                ScheduleEvent {
                    id: "d1-2".to_string(),
                    title: "Opening Ceremony".to_string(),
                    time: "09:00".to_string(),
                    duration: "30m".to_string(),
                    event_type: EventType::Keynote,
                    speaker: None,
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d1-3".to_string(),
                    title: "Keynote: The Future of FOSS".to_string(),
                    time: "09:30".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Keynote,
                    speaker: Some(Speaker {
                        id: "k1".to_string(),
                        name: "Frank GNU".to_string(),
                        talk_title: "The Philosophy of Free Software".to_string(),
                        bio: "Free software advocate".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d1-4".to_string(),
                    title: "Coffee Break".to_string(),
                    time: "10:30".to_string(),
                    duration: "30m".to_string(),
                    event_type: EventType::Break,
                    speaker: None,
                    location: Some("Cafeteria".to_string()),
                },
                ScheduleEvent {
                    id: "d1-5".to_string(),
                    title: "Building the Future with WebAssembly".to_string(),
                    time: "11:00".to_string(),
                    duration: "45m".to_string(),
                    event_type: EventType::Talk,
                    speaker: Some(Speaker {
                        id: "s1".to_string(),
                        name: "Alice Rustacean".to_string(),
                        talk_title: "Building the Future with WebAssembly".to_string(),
                        bio: "Rust contributor".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d1-6".to_string(),
                    title: "Workshop: Rust for Beginners".to_string(),
                    time: "11:00".to_string(),
                    duration: "3h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s1".to_string(),
                        name: "Alice Rustacean".to_string(),
                        talk_title: "Rust Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall A".to_string()),
                },
                ScheduleEvent {
                    id: "d1-7".to_string(),
                    title: "Workshop: Contributing to Open Source".to_string(),
                    time: "11:00".to_string(),
                    duration: "2h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s6".to_string(),
                        name: "Frank GNU".to_string(),
                        talk_title: "Open Source Contributing".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall B".to_string()),
                },
                ScheduleEvent {
                    id: "d1-8".to_string(),
                    title: "Lunch Break".to_string(),
                    time: "13:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Break,
                    speaker: None,
                    location: Some("Cafeteria".to_string()),
                },
                ScheduleEvent {
                    id: "d1-9".to_string(),
                    title: "Building Decentralized Systems with IPFS".to_string(),
                    time: "14:00".to_string(),
                    duration: "45m".to_string(),
                    event_type: EventType::Talk,
                    speaker: Some(Speaker {
                        id: "s5".to_string(),
                        name: "Eve Distributed".to_string(),
                        talk_title: "Building Decentralized Systems with IPFS".to_string(),
                        bio: "Protocol Labs engineer".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d1-10".to_string(),
                    title: "Workshop: WebAssembly Deep Dive".to_string(),
                    time: "14:00".to_string(),
                    duration: "3h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s1".to_string(),
                        name: "Alice Rustacean".to_string(),
                        talk_title: "WebAssembly Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall A".to_string()),
                },
                ScheduleEvent {
                    id: "d1-11".to_string(),
                    title: "Project Showcase & Networking".to_string(),
                    time: "17:00".to_string(),
                    duration: "2h".to_string(),
                    event_type: EventType::Networking,
                    speaker: None,
                    location: Some("Open Area".to_string()),
                },
            ],
        },
        ScheduleDay {
            date: "March 15".to_string(),
            day_label: "Day 2 - Deep Dives".to_string(),
            events: vec![
                ScheduleEvent {
                    id: "d2-1".to_string(),
                    title: "Inside the Linux Kernel: A Deep Dive".to_string(),
                    time: "09:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Keynote,
                    speaker: Some(Speaker {
                        id: "s2".to_string(),
                        name: "Bob Kernel".to_string(),
                        talk_title: "Inside the Linux Kernel".to_string(),
                        bio: "Linux kernel maintainer".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d2-2".to_string(),
                    title: "Coffee Break".to_string(),
                    time: "10:00".to_string(),
                    duration: "30m".to_string(),
                    event_type: EventType::Break,
                    speaker: None,
                    location: Some("Cafeteria".to_string()),
                },
                ScheduleEvent {
                    id: "d2-3".to_string(),
                    title: "Zero Trust Architecture in Open Source".to_string(),
                    time: "10:30".to_string(),
                    duration: "45m".to_string(),
                    event_type: EventType::Talk,
                    speaker: Some(Speaker {
                        id: "s3".to_string(),
                        name: "Carol Security".to_string(),
                        talk_title: "Zero Trust Architecture".to_string(),
                        bio: "Security researcher".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d2-4".to_string(),
                    title: "Workshop: Linux Kernel Hacking 101".to_string(),
                    time: "10:30".to_string(),
                    duration: "4h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s2".to_string(),
                        name: "Bob Kernel".to_string(),
                        talk_title: "Kernel Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall A".to_string()),
                },
                ScheduleEvent {
                    id: "d2-5".to_string(),
                    title: "Workshop: Open Source Security Audit".to_string(),
                    time: "10:30".to_string(),
                    duration: "3h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s3".to_string(),
                        name: "Carol Security".to_string(),
                        talk_title: "Security Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall B".to_string()),
                },
                ScheduleEvent {
                    id: "d2-6".to_string(),
                    title: "Lunch Break".to_string(),
                    time: "13:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Break,
                    speaker: None,
                    location: Some("Cafeteria".to_string()),
                },
                ScheduleEvent {
                    id: "d2-7".to_string(),
                    title: "Panel: The State of Open Source in India".to_string(),
                    time: "14:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Panel,
                    speaker: None,
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d2-8".to_string(),
                    title: "Lightning Talks".to_string(),
                    time: "15:30".to_string(),
                    duration: "1.5h".to_string(),
                    event_type: EventType::Talk,
                    speaker: None,
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d2-9".to_string(),
                    title: "Hacking Night Begins".to_string(),
                    time: "19:00".to_string(),
                    duration: "12h".to_string(),
                    event_type: EventType::Networking,
                    speaker: None,
                    location: Some("All Venues".to_string()),
                },
            ],
        },
        ScheduleDay {
            date: "March 16".to_string(),
            day_label: "Day 3 - Closing".to_string(),
            events: vec![
                ScheduleEvent {
                    id: "d3-1".to_string(),
                    title: "Hacking Night Ends".to_string(),
                    time: "07:00".to_string(),
                    duration: "30m".to_string(),
                    event_type: EventType::Networking,
                    speaker: None,
                    location: Some("All Venues".to_string()),
                },
                ScheduleEvent {
                    id: "d3-2".to_string(),
                    title: "Breakfast".to_string(),
                    time: "07:30".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Break,
                    speaker: None,
                    location: Some("Cafeteria".to_string()),
                },
                ScheduleEvent {
                    id: "d3-3".to_string(),
                    title: "Kubernetes: Cloud Native the FOSS Way".to_string(),
                    time: "09:00".to_string(),
                    duration: "45m".to_string(),
                    event_type: EventType::Talk,
                    speaker: Some(Speaker {
                        id: "s4".to_string(),
                        name: "David Cloud".to_string(),
                        talk_title: "Kubernetes Talk".to_string(),
                        bio: "Cloud architect".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d3-4".to_string(),
                    title: "Open Source AI: Democratizing Machine Learning".to_string(),
                    time: "10:00".to_string(),
                    duration: "45m".to_string(),
                    event_type: EventType::Talk,
                    speaker: Some(Speaker {
                        id: "s7".to_string(),
                        name: "Grace ML".to_string(),
                        talk_title: "Open Source AI".to_string(),
                        bio: "ML researcher".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d3-5".to_string(),
                    title: "Workshop: Kubernetes from Scratch".to_string(),
                    time: "10:00".to_string(),
                    duration: "4h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s4".to_string(),
                        name: "David Cloud".to_string(),
                        talk_title: "K8s Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall A".to_string()),
                },
                ScheduleEvent {
                    id: "d3-6".to_string(),
                    title: "Workshop: Building AI with Open Source Tools".to_string(),
                    time: "10:00".to_string(),
                    duration: "4h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s7".to_string(),
                        name: "Grace ML".to_string(),
                        talk_title: "AI Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall B".to_string()),
                },
                ScheduleEvent {
                    id: "d3-7".to_string(),
                    title: "Lunch Break".to_string(),
                    time: "13:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Break,
                    speaker: None,
                    location: Some("Cafeteria".to_string()),
                },
                ScheduleEvent {
                    id: "d3-8".to_string(),
                    title: "Workshop: GitOps with ArgoCD".to_string(),
                    time: "14:00".to_string(),
                    duration: "3h".to_string(),
                    event_type: EventType::Workshop,
                    speaker: Some(Speaker {
                        id: "s8".to_string(),
                        name: "Henry DevOps".to_string(),
                        talk_title: "GitOps Workshop".to_string(),
                        bio: "".to_string(),
                        image_url: "/assets/speakers/placeholder.jpg".to_string(),
                        socials: vec![],
                    }),
                    location: Some("Workshop Hall A".to_string()),
                },
                ScheduleEvent {
                    id: "d3-9".to_string(),
                    title: "Hack Night Project Presentations".to_string(),
                    time: "15:00".to_string(),
                    duration: "1.5h".to_string(),
                    event_type: EventType::Talk,
                    speaker: None,
                    location: Some("Main Auditorium".to_string()),
                },
                ScheduleEvent {
                    id: "d3-10".to_string(),
                    title: "Closing Ceremony & Prize Distribution".to_string(),
                    time: "17:00".to_string(),
                    duration: "1h".to_string(),
                    event_type: EventType::Keynote,
                    speaker: None,
                    location: Some("Main Auditorium".to_string()),
                },
            ],
        },
    ]
}
