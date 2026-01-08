use yew::prelude::*;
use crate::components::section::Section;
use crate::components::button::Button;

#[function_component(SpeakersPage)]
pub fn speakers_page() -> Html {
    let instagram_url = "https://www.instagram.com/fosscellnitc?igsh=MWoyaHdpODEwcmR6Nw==".to_string();

    html! {
        <div class="page-speakers">
            // --- Page Header ---
            <section class="page-header">
                <div class="container">
                    <div class="page-header-content">
                        <span class="page-tag">{"// speakers"}</span>
                        <h1 class="page-title">
                            {"Our "}
                            <span class="outline text-glow">{"Speakers"}</span>
                        </h1>
                        <p class="page-subtitle">
                            {"FOSSMeet brings together industry experts and open source maintainers from across the globe."}
                        </p>
                        
                        <div class="page-terminal">
                            <code>
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"grep -r \"lineup\" /events/2026"}</span>
                                <br />
                                <span class="terminal-output">{"Searching... "}</span>
                                <span class="terminal-highlight">{"[RELEASE_PENDING]"}</span>
                                <br />
                                <span class="terminal-output">{"Lineup announcement scheduled for early 2026."}</span>
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
            
            // --- CFP / Coming Soon Section ---
            <Section class="speakers-grid-section">
                <div style="text-align: center; padding: 4rem 1rem;">
                    <h2 style="font-size: 2rem; margin-bottom: 1.5rem;">{"The lineup is currently [empty]"}</h2>
                    <p style="max-width: 600px; margin: 0 auto 2rem auto; opacity: 0.8; line-height: 1.6;">
                        {"We are currently in the process of reaching out to pioneers in the FOSS community. 
                        Official speaker announcements will begin in January 2026."}
                    </p>
                    
                    <div class="speakers-filter" style="justify-content: center; border: none;">
                         <Button variant="primary" href={instagram_url} external={true}>
                            {"Follow for Announcements"}
                        </Button>
                    </div>
                </div>
            </Section>
            
            // --- Focused Call to Action ---
            <section class="speakers-cta">
                <div class="container">
                    <div class="cta-box" style="background: var(--color-surface-soft); border: 1px solid var(--color-primary-dim);">
                        <h3 style="color: var(--color-primary);">{"Want to share your story at FOSSMeet?"}</h3>
                        <p>{"Our Call for Proposals (CFP) will open soon. We welcome talks on Linux, Rust, Privacy, Open Hardware, and more."}</p>
                        <div style="margin-top: 2rem;">
                            <a href="mailto:fosscell@nitc.ac.in?subject=Speaker Inquiry FOSSMeet'26" class="btn btn-primary">
                                {"Send an Inquiry"}
                                <svg viewBox="0 0 24 24" width="18" height="18" style="margin-left: 10px;">
                                    <path fill="currentColor" d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}