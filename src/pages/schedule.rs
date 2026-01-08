use yew::prelude::*;
use crate::components::section::Section;
use crate::components::button::Button;

#[function_component(SchedulePage)]
pub fn schedule_page() -> Html {
    let instagram_url = "https://www.instagram.com/fosscellnitc?igsh=MWoyaHdpODEwcmR6Nw==".to_string();

    html! {
        <div class="page-schedule">
            // --- Page Header ---
            <section class="page-header">
                <div class="container">
                    <div class="page-header-content">
                        <span class="page-tag">{"// schedule"}</span>
                        <h1 class="page-title">
                            {"Event "}
                            <span class="outline text-glow">{"Timeline"}</span>
                        </h1>
                        <p class="page-subtitle">
                            {"Three days of deep-dives, hacking, and community building. 
                            The official FOSSMeet'26 schedule is currently under construction."}
                        </p>
                        
                        <div class="page-terminal">
                            <code>
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"date -d \"2026-02-01\""}</span>
                                <br />
                                <span class="terminal-output">{"Sun Feb  1 00:00:00 IST 2026"}</span>
                                <br />
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"check --release-status"}</span>
                                <br />
                                <span class="terminal-output">{"Status: "}</span>
                                <span class="terminal-highlight">{"STAY_TUNED"}</span>
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
            
            // --- Save the Date Section ---
            <Section class="schedule-section">
                <div style="text-align: center; padding: 4rem 1rem;">
                    <h2 style="font-size: 2.5rem; color: var(--color-primary); margin-bottom: 1rem;">
                        {"February 2026"}
                    </h2>
                    <p style="font-size: 1.2rem; opacity: 0.8; margin-bottom: 2rem;">
                        {"NIT Calicut is preparing for the 21st edition of FOSSMeet."}
                    </p>
                    <div class="day-tabs" style="justify-content: center; gap: 2rem;">
                        <div class="day-tab active" style="cursor: default;">
                            <span class="day-tab-label">{"Day 01"}</span>
                            <span class="day-tab-date">{"Keynotes"}</span>
                        </div>
                        <div class="day-tab active" style="cursor: default;">
                            <span class="day-tab-label">{"Day 02"}</span>
                            <span class="day-tab-date">{"Workshops"}</span>
                        </div>
                        <div class="day-tab active" style="cursor: default;">
                            <span class="day-tab-label">{"Day 03"}</span>
                            <span class="day-tab-date">{"Hackathon"}</span>
                        </div>
                    </div>
                </div>
            </Section>
            
            // --- Venue Info (Kept as it adds real context) ---
            <section class="venue-info" style="background: var(--color-surface-soft); padding: 4rem 0;">
                <div class="container">
                    <h3 style="margin-bottom: 2rem;">{"// Venue Overview"}</h3>
                    <div class="venue-grid">
                        <div class="venue-card">
                            <h4>{"Main Auditorium"}</h4>
                            <p>{"Keynotes and curated technical talks."}</p>
                            <span class="venue-capacity">{"Capacity: 500+"}</span>
                        </div>
                        <div class="venue-card">
                            <h4>{"Workshop Labs"}</h4>
                            <p>{"Dedicated environments for hands-on sessions."}</p>
                            <span class="venue-capacity">{"High-speed FOSS Mirror"}</span>
                        </div>
                        <div class="venue-card">
                            <h4>{"Community Zone"}</h4>
                            <p>{"Networking, project showcases, and FOSS birds of a feather (BoF) sessions."}</p>
                        </div>
                    </div>
                </div>
            </section>

            // --- CTA Section ---
            <section class="schedule-download" style="padding: 5rem 0; text-align: center;">
                <div class="container">
                    <div class="download-box" style="border: 1px solid var(--color-primary-dim);">
                        <h3>{"Don't miss the reveal."}</h3>
                        <p style="margin: 1.5rem 0;">{"We will release the detailed hour-by-hour schedule on our social handles first."}</p>
                        <div style="display: flex; justify-content: center; gap: 1rem; flex-wrap: wrap;">
                            <Button variant="primary" href={instagram_url} external={true}>
                                {"Follow for Schedule Release"}
                            </Button>
                            <Button variant="secondary" href={"mailto:fosscell@nitc.ac.in"}>
                                {"Contact Organizers"}
                            </Button>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}