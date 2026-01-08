use yew::prelude::*;
use crate::components::{
    button::Button,
    section::Section,
    terminal::{Terminal, TerminalLine},
    marquee::Marquee,
    glitch_text::GlitchText,
};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let marquee_items = vec![
        "Free Software".to_string(),
        "Open Source".to_string(),
        "Linux".to_string(),
        "Rust".to_string(),
        "Privacy".to_string(),
        "NIT Calicut".to_string(),
        "February 2026".to_string(),
        "Coming Soon".to_string(),
    ];
    
    let instagram_url = "https://www.instagram.com/fosscellnitc?igsh=MWoyaHdpODEwcmR6Nw==".to_string();
    let email_url = "mailto:fosscell@nitc.ac.in".to_string();

    html! {
        <>
            // --- Hero Section ---
            <section class="hero">
                <div class="hero-bg-grid"></div>
                <div class="hero-content fade-in">
                    <span class="hero-badge">{"// NIT Calicut 2026"}</span>
                    
                    <h1 class="hero-title">
                        <span class="accent text-glow">{"FOSS"}</span>
                        {"Meet"}
                        <span class="hero-year">{"'26"}</span>
                    </h1>
                    
                    <p class="hero-subtitle">
                        {"The premier celebration of "}
                        <span style="color: var(--color-primary)">{"software freedom"}</span>
                        {" is returning to NIT Calicut."}
                    </p>
                    
                    <div class="hero-date">
                        <div class="hero-date-item">
                            <svg viewBox="0 0 24 24" width="20" height="20">
                                <path fill="currentColor" d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11z"/>
                            </svg>
                            {"February 2026"}
                        </div>
                        <div class="hero-date-item">
                            <svg viewBox="0 0 24 24" width="20" height="20">
                                <path fill="currentColor" d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7z"/>
                            </svg>
                            {"NIT Calicut"}
                        </div>
                    </div>
                    
                    <div class="hero-cta">
                        <Button variant="primary" href={instagram_url.clone()} external={true}>
                            {"Inquire for Sponsorship"}
                        </Button>
                        <Button variant="secondary" href={"#about".to_string()}>
                            {"Learn More"}
                        </Button>
                    </div>
                    
                    <div class="hero-terminal slide-up stagger-3">
                        <Terminal title="fosscell@nitc:~">
                            <TerminalLine 
                                prompt={"$ ".to_string()}
                                command={"fossmeet --status".to_string()}
                            />
                            <TerminalLine output={"> Event: FOSSMeet'26".to_string()} />
                            <TerminalLine output={"> Date: February 2026".to_string()} />
                            <TerminalLine output={"> Location: NIT Calicut".to_string()} />
                            <TerminalLine output={"> Status: [PRE-ALPHA] Gathering Contributors".to_string()} />
                            <TerminalLine prompt={"$ ".to_string()} typing={true} />
                        </Terminal>
                    </div>
                </div>
            </section>
            
            <Marquee items={marquee_items} speed={25} />
            
            // --- About Section ---
            <Section 
                id="about"
                tag="about"
                title="About [FOSSMeet]"
                subtitle="Celebrating freedom in software since 2005"
            >
                <div class="about-content">
                    <div class="about-text">
                        <p>
                            {"Hosted at NIT Calicut, FOSSMeet is an annual gathering that brings together 
                            developers and students to promote Free and Open Source Software."}
                        </p>
                        <br />
                        <p>
                            {"FOSSMeet'26 will feature a curated selection of talks and workshops. 
                            We are currently in the planning phase—stay tuned for the official schedule."}
                        </p>
                        <div class="about-features" style="margin-top: 2rem;">
                            <div class="feature-item">
                                <span class="feature-icon">{">"}</span>
                                {"Technical sessions on FOSS technologies"}
                            </div>
                            <div class="feature-item">
                                <span class="feature-icon">{">"}</span>
                                {"Hands-on community-led workshops"}
                            </div>
                        </div>
                    </div>
                    
                    <div class="about-stats">
                        <div class="about-stat">
                            <span class="about-stat-value">{"21"}</span>
                            <span class="about-stat-label">{"Years"}</span>
                        </div>
                        <div class="about-stat">
                            <span class="about-stat-value">{"Feb"}</span>
                            <span class="about-stat-label">{"2026"}</span>
                        </div>
                    </div>
                </div>
            </Section>

            // --- Simplified Coming Soon Section ---
            <Section 
                tag="updates"
                title="Stay [Updated]"
                subtitle="Join the FOSSCell community"
                class="section-dark"
            >
                <div style="text-align: center; padding: 5rem 0;">
                    <GlitchText text={"COMING FEBRUARY 2026".to_string()} active={true} />
                    <p style="margin-top: 1.5rem; opacity: 0.8; max-width: 500px; margin: 1.5rem auto;">
                        {"We are working hard behind the scenes. Follow us on Instagram for 
                        announcements regarding Speakers, Workshops, and Registration."}
                    </p>
                    <div style="margin-top: 2.5rem;">
                         <Button variant="primary" href={instagram_url.clone()} external={true}>
                            {"Follow @fosscellnitc"}
                        </Button>
                    </div>
                </div>
            </Section>
            
            // --- Footer CTA Section ---
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2 class="cta-title">{"Ready to "} <span class="text-gradient">{"connect"}</span>{"?"}</h2>
                        <p class="cta-subtitle">{"Reach out to us for collaborations or queries."}</p>
                        <div class="cta-buttons">
                            <Button variant="secondary" href={instagram_url} external={true}>
                                {"Instagram"}
                            </Button>
                            <Button variant="secondary" href={email_url}>
                                {"Email Us"}
                            </Button>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}