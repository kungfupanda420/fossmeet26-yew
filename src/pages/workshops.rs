use yew::prelude::*;
use crate::components::section::Section;
use crate::components::button::Button;

#[function_component(WorkshopsPage)]
pub fn workshops_page() -> Html {
    let instagram_url = "https://www.instagram.com/fosscellnitc?igsh=MWoyaHdpODEwcmR6Nw==".to_string();

    html! {
        <div class="page-workshops">
            // --- Page Header ---
            <section class="page-header">
                <div class="container">
                    <div class="page-header-content">
                        <span class="page-tag">{"// workshops"}</span>
                        <h1 class="page-title">
                            {"Hands-on "}
                            <span class="outline text-glow">{"Learning"}</span>
                        </h1>
                        <p class="page-subtitle">
                            {"Level up your skills with intensive, hands-on sessions led by community experts. 
                            From beginner-friendly introductions to advanced systems hacking."}
                        </p>
                        
                        <div class="page-terminal">
                            <code>
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"./status --check workshops"}</span>
                                <br />
                                <span class="terminal-output">{"[WAIT] "}</span>
                                <span class="terminal-highlight">{"Proposals are currently being reviewed."}</span>
                                <br />
                                <span class="terminal-output">{"Estimated release: February 2026"}</span>
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
            
            // --- Workshop Info Banner (Kept for educational value) ---
            <section class="workshop-info-banner">
                <div class="container">
                    <div class="info-cards">
                        <div class="info-card">
                            <span class="info-icon">{"{"}</span>
                            <div class="info-content">
                                <h4>{"Hands-on Learning"}</h4>
                                <p>{"Practice while you learn with real-world open source projects"}</p>
                            </div>
                        </div>
                        <div class="info-card">
                            <span class="info-icon">{"["}</span>
                            <div class="info-content">
                                <h4>{"Limited Seats"}</h4>
                                <p>{"Small batches ensure personalized attention from mentors"}</p>
                            </div>
                        </div>
                        <div class="info-card">
                            <span class="info-icon">{"<"}</span>
                            <div class="info-content">
                                <h4>{"Take Home Projects"}</h4>
                                <p>{"Build something you can showcase in your GitHub portfolio"}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // --- Coming Soon Placeholder ---
            <Section class="workshops-grid-section">
                <div style="text-align: center; padding: 4rem 1rem;">
                    <div class="no-results">
                        <code style="font-size: 1.2rem; color: var(--color-primary);">
                            {"// CURATING THE CURRICULUM..."}
                        </code>
                        <p style="margin-top: 2rem; opacity: 0.7; max-width: 600px; margin-left: auto; margin-right: auto;">
                            {"We are currently selecting the most impactful workshops for FOSSMeet'26. 
                            The full list of sessions, prerequisites, and registration links will be live in February."}
                        </p>
                        <div style="margin-top: 2rem;">
                             <Button variant="secondary" href={instagram_url} external={true}>
                                {"Get Notified on Instagram"}
                            </Button>
                        </div>
                    </div>
                </div>
            </Section>
            
            // --- Workshop Guidelines (Kept as helpful info) ---
            <section class="workshop-guidelines">
                <div class="container">
                    <div class="guidelines-content">
                        <h3>{"// Workshop Guidelines"}</h3>
                        <div class="guidelines-grid">
                            <div class="guideline-item">
                                <span class="guideline-num">{"01"}</span>
                                <div>
                                    <h4>{"Bring Your Laptop"}</h4>
                                    <p>{"Most sessions require a laptop with FOSS tools pre-installed."}</p>
                                </div>
                            </div>
                            <div class="guideline-item">
                                <span class="guideline-num">{"02"}</span>
                                <div>
                                    <h4>{"Check Prerequisites"}</h4>
                                    <p>{"Review required skills before the event to get the most out of your session."}</p>
                                </div>
                            </div>
                            <div class="guideline-item">
                                <span class="guideline-num">{"03"}</span>
                                <div>
                                    <h4>{"Join the Community"}</h4>
                                    <p>{"Workshops are collaborative. Be ready to hack and learn with your peers."}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // --- CTA Section ---
            <section class="workshops-cta">
                <div class="container">
                    <div class="cta-box" style="border: 1px dashed var(--color-primary);">
                        <h3>{"Have a skill to share?"}</h3>
                        <p>{"FOSSMeet is built by the community. If you want to conduct a workshop on your favorite tech, we'd love to hear from you."}</p>
                        <a href="mailto:fosscell@nitc.ac.in?subject=Workshop Proposal FOSSMeet'26" class="btn btn-primary">
                            {"Submit Workshop Proposal"}
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}