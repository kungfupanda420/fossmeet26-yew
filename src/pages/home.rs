use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;
use crate::components::{
    button::Button,
    section::Section,
    terminal::{Terminal, TerminalLine},
    marquee::Marquee,
    glitch_text::GlitchText,
    speaker_card::SpeakerCard,
    workshop_card::WorkshopCard,
};
use crate::types::{Speaker, Workshop, SocialLink, Sponsor, SponsorTier};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // Sample speakers data
    let speakers = get_sample_speakers();
    let workshops = get_sample_workshops();
    let sponsors = get_sample_sponsors();
    
    let marquee_items = vec![
        "Free Software".to_string(),
        "Open Source".to_string(),
        "Linux".to_string(),
        "Rust".to_string(),
        "WebAssembly".to_string(),
        "Privacy".to_string(),
        "Security".to_string(),
        "Community".to_string(),
        "Collaboration".to_string(),
        "Innovation".to_string(),
    ];
    
    html! {
        <>
            // Hero Section
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
                        {"Where "}
                        <GlitchText text={"hackers".to_string()} active={true} />
                        {", developers, and enthusiasts gather to celebrate "}
                        <span style="color: var(--color-primary)">{"freedom"}</span>
                        {" in software."}
                    </p>
                    
                    <div class="hero-date">
                        <div class="hero-date-item">
                            <svg viewBox="0 0 24 24">
                                <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM9 10H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2zm-8 4H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2z"/>
                            </svg>
                            {"March 2026"}
                        </div>
                        <div class="hero-date-item">
                            <svg viewBox="0 0 24 24">
                                <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/>
                            </svg>
                            {"NIT Calicut"}
                        </div>
                    </div>
                    
                    <div class="hero-cta">
                        <Button variant="primary" href={"#register".to_string()}>
                            {"Register Now"}
                            <svg viewBox="0 0 24 24" width="18" height="18">
                                <path fill="currentColor" d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
                            </svg>
                        </Button>
                        <Button variant="secondary" href={"#about".to_string()}>
                            {"Learn More"}
                        </Button>
                    </div>
                    
                    // Terminal preview
                    <div class="hero-terminal slide-up stagger-3">
                        <Terminal title="fossmeet@nitc:~">
                            <TerminalLine 
                                prompt={"$ ".to_string()}
                                command={"cat /etc/fossmeet/2026.conf".to_string()}
                            />
                            <TerminalLine 
                                output={"[event]".to_string()}
                            />
                            <TerminalLine 
                                output={"name = \"FOSSMeet'26\"".to_string()}
                            />
                            <TerminalLine 
                                output={"location = \"NIT Calicut\"".to_string()}
                            />
                            <TerminalLine 
                                output={"theme = \"Free as in Freedom\"".to_string()}
                            />
                            <TerminalLine 
                                prompt={"$ ".to_string()}
                                typing={true}
                            />
                        </Terminal>
                    </div>
                </div>
                
                // Scroll indicator
                <div class="hero-scroll-indicator">
                    <span>{"scroll"}</span>
                    <div class="scroll-arrow"></div>
                </div>
            </section>
            
            // Marquee
            <Marquee items={marquee_items} speed={25} />
            
            // About Section
            <Section 
                id="about"
                tag="about"
                title="About [FOSSMeet]"
                subtitle="Two decades of celebrating freedom in software"
            >
                <div class="about-content">
                    <div class="about-text">
                        <p>
                            {"Since its inception in 2005, FOSSMeet has been the premier platform for 
                            promoting Free and Open Source Software in India. Hosted at NIT Calicut, 
                            this annual gathering brings together brilliant minds from across the globe."}
                        </p>
                        <br />
                        <p>
                            {"FOSSMeet'26 continues this legacy with engaging talks, hands-on workshops, 
                            and discussions that inspire and empower the FOSS community. Whether you're 
                            a seasoned contributor or just starting your open source journey, there's 
                            something here for everyone."}
                        </p>
                        <br />
                        <div class="about-features">
                            <div class="feature-item">
                                <span class="feature-icon">{">"}</span>
                                {"Technical talks from industry experts"}
                            </div>
                            <div class="feature-item">
                                <span class="feature-icon">{">"}</span>
                                {"Hands-on workshops and hack sessions"}
                            </div>
                            <div class="feature-item">
                                <span class="feature-icon">{">"}</span>
                                {"Networking with the FOSS community"}
                            </div>
                            <div class="feature-item">
                                <span class="feature-icon">{">"}</span>
                                {"Project showcases and demos"}
                            </div>
                        </div>
                    </div>
                    
                    <div class="about-stats">
                        <div class="about-stat">
                            <span class="about-stat-value">{"21"}</span>
                            <span class="about-stat-label">{"Years"}</span>
                        </div>
                        <div class="about-stat">
                            <span class="about-stat-value">{"50+"}</span>
                            <span class="about-stat-label">{"Speakers"}</span>
                        </div>
                        <div class="about-stat">
                            <span class="about-stat-value">{"1000+"}</span>
                            <span class="about-stat-label">{"Attendees"}</span>
                        </div>
                        <div class="about-stat">
                            <span class="about-stat-value">{"20+"}</span>
                            <span class="about-stat-label">{"Workshops"}</span>
                        </div>
                    </div>
                </div>
            </Section>
            
            // Speakers Preview
            <Section 
                tag="speakers"
                title="Meet our [Speakers]"
                subtitle="Industry experts sharing their knowledge and experience"
                class="section-dark"
            >
                <div class="cards-grid">
                    {for speakers.iter().take(4).map(|speaker| {
                        html! { <SpeakerCard speaker={speaker.clone()} /> }
                    })}
                </div>
                <div class="section-cta">
                    <Link<Route> to={Route::Speakers} classes="btn btn-secondary">
                        {"View All Speakers"}
                        <svg viewBox="0 0 24 24" width="18" height="18">
                            <path fill="currentColor" d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
                        </svg>
                    </Link<Route>>
                </div>
            </Section>
            
            // Workshops Preview
            <Section 
                tag="workshops"
                title="Engaging [Workshops]"
                subtitle="Hands-on sessions to level up your skills"
            >
                <div class="cards-grid">
                    {for workshops.iter().take(3).map(|workshop| {
                        html! { <WorkshopCard workshop={workshop.clone()} /> }
                    })}
                </div>
                <div class="section-cta">
                    <Link<Route> to={Route::Workshops} classes="btn btn-secondary">
                        {"View All Workshops"}
                        <svg viewBox="0 0 24 24" width="18" height="18">
                            <path fill="currentColor" d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
                        </svg>
                    </Link<Route>>
                </div>
            </Section>
            
            // Sponsors Section
            <Section 
                tag="sponsors"
                title="Supported by [Community]"
                subtitle="Our sponsors and community partners"
                class="section-sponsors"
            >
                <div class="sponsors-container">
                    <h3 class="sponsors-tier-title">{"// Sponsors"}</h3>
                    <div class="sponsors-grid">
                        {for sponsors.iter().filter(|s| matches!(s.tier, SponsorTier::Platinum | SponsorTier::Gold)).map(|sponsor| {
                            html! {
                                <a 
                                    href={sponsor.website.clone()} 
                                    target="_blank" 
                                    rel="noopener noreferrer"
                                    class="sponsor-item"
                                >
                                    <img src={sponsor.logo_url.clone()} alt={sponsor.name.clone()} />
                                </a>
                            }
                        })}
                    </div>
                    
                    <h3 class="sponsors-tier-title">{"// Community Partners"}</h3>
                    <div class="sponsors-grid sponsors-community">
                        {for sponsors.iter().filter(|s| matches!(s.tier, SponsorTier::Community)).map(|sponsor| {
                            html! {
                                <a 
                                    href={sponsor.website.clone()} 
                                    target="_blank" 
                                    rel="noopener noreferrer"
                                    class="sponsor-item"
                                >
                                    <img src={sponsor.logo_url.clone()} alt={sponsor.name.clone()} />
                                </a>
                            }
                        })}
                    </div>
                </div>
            </Section>
            
            // CTA Section
            <section class="cta-section" id="register">
                <div class="container">
                    <div class="cta-content">
                        <h2 class="cta-title">
                            {"Ready to "}
                            <span class="text-gradient">{"join"}</span>
                            {" the movement?"}
                        </h2>
                        <p class="cta-subtitle">
                            {"Register now and be part of FOSSMeet'26. Limited seats available!"}
                        </p>
                        <div class="cta-buttons">
                            <Button variant="primary" href={"https://townscript.com/fossmeet26".to_string()} external={true}>
                                {"Register Now"}
                            </Button>
                            <Button variant="secondary" href={"mailto:fosscell@nitc.ac.in".to_string()}>
                                {"Contact Us"}
                            </Button>
                        </div>
                        <p class="cta-note">
                            {"// Event is free for students with valid ID"}
                        </p>
                    </div>
                </div>
            </section>
        </>
    }
}

fn get_sample_speakers() -> Vec<Speaker> {
    vec![
        Speaker {
            id: "1".to_string(),
            name: "Alice Rustacean".to_string(),
            talk_title: "Building the Future with WebAssembly".to_string(),
            bio: "Core contributor to the Rust compiler and WebAssembly advocate.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com".to_string() },
            ],
        },
        Speaker {
            id: "2".to_string(),
            name: "Bob Kernel".to_string(),
            talk_title: "Inside the Linux Kernel: A Deep Dive".to_string(),
            bio: "Linux kernel maintainer with 15 years of experience in systems programming.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
            ],
        },
        Speaker {
            id: "3".to_string(),
            name: "Carol Security".to_string(),
            talk_title: "Zero Trust Architecture in Open Source".to_string(),
            bio: "Security researcher and privacy advocate, focusing on open source security tools.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "LinkedIn".to_string(), url: "https://linkedin.com".to_string() },
            ],
        },
        Speaker {
            id: "4".to_string(),
            name: "David Cloud".to_string(),
            talk_title: "Kubernetes: Cloud Native the FOSS Way".to_string(),
            bio: "Cloud architect and CNCF ambassador, passionate about cloud native technologies.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com".to_string() },
            ],
        },
    ]
}

fn get_sample_workshops() -> Vec<Workshop> {
    vec![
        Workshop {
            id: "1".to_string(),
            title: "Rust for Beginners".to_string(),
            description: "Learn the fundamentals of Rust programming, from ownership to async/await. Perfect for developers looking to write safe, concurrent code.".to_string(),
            logo_url: "/assets/workshops/rust.svg".to_string(),
            date: "Day 1".to_string(),
            duration: "3".to_string(),
            speaker_name: "Alice Rustacean".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec!["Basic programming knowledge".to_string(), "Laptop with Rust installed".to_string()],
        },
        Workshop {
            id: "2".to_string(),
            title: "Linux Kernel Hacking 101".to_string(),
            description: "Get your hands dirty with kernel development. We'll write a simple kernel module and understand the Linux kernel architecture.".to_string(),
            logo_url: "/assets/workshops/linux.svg".to_string(),
            date: "Day 2".to_string(),
            duration: "4".to_string(),
            speaker_name: "Bob Kernel".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec!["C programming".to_string(), "Linux basics".to_string()],
        },
        Workshop {
            id: "3".to_string(),
            title: "Open Source Security Audit".to_string(),
            description: "Learn to identify and fix security vulnerabilities in open source projects. Hands-on with real CVEs and security tools.".to_string(),
            logo_url: "/assets/workshops/security.svg".to_string(),
            date: "Day 2".to_string(),
            duration: "3".to_string(),
            speaker_name: "Carol Security".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec!["Programming experience".to_string(), "Basic security concepts".to_string()],
        },
    ]
}

fn get_sample_sponsors() -> Vec<Sponsor> {
    vec![
        Sponsor {
            name: "FOSS United".to_string(),
            logo_url: "/assets/sponsors/fossunited.svg".to_string(),
            website: "https://fossunited.org".to_string(),
            tier: SponsorTier::Platinum,
        },
        Sponsor {
            name: "Nilenso".to_string(),
            logo_url: "/assets/sponsors/nilenso.svg".to_string(),
            website: "https://nilenso.com".to_string(),
            tier: SponsorTier::Gold,
        },
        Sponsor {
            name: "Saeloun".to_string(),
            logo_url: "/assets/sponsors/saeloun.svg".to_string(),
            website: "https://saeloun.com".to_string(),
            tier: SponsorTier::Gold,
        },
        Sponsor {
            name: "amFOSS".to_string(),
            logo_url: "/assets/sponsors/amfoss.png".to_string(),
            website: "https://amfoss.in".to_string(),
            tier: SponsorTier::Community,
        },
        Sponsor {
            name: "FOSSNSS".to_string(),
            logo_url: "/assets/sponsors/fossnss.png".to_string(),
            website: "https://fossnss.org".to_string(),
            tier: SponsorTier::Community,
        },
        Sponsor {
            name: "FOSS MEC".to_string(),
            logo_url: "/assets/sponsors/fossmec.png".to_string(),
            website: "https://fossmec.in".to_string(),
            tier: SponsorTier::Community,
        },
    ]
}
