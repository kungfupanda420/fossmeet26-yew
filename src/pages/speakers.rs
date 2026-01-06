use yew::prelude::*;
use crate::components::{
    section::Section,
    speaker_card::SpeakerCard,
};
use crate::types::{Speaker, SocialLink};

#[function_component(SpeakersPage)]
pub fn speakers_page() -> Html {
    let speakers = use_state(|| get_all_speakers());
    let selected_speaker = use_state(|| None::<Speaker>);
    let filter = use_state(|| "all".to_string());
    
    let on_speaker_click = {
        let selected = selected_speaker.clone();
        Callback::from(move |speaker: Speaker| {
            selected.set(Some(speaker));
        })
    };
    
    let close_modal = {
        let selected = selected_speaker.clone();
        Callback::from(move |_| {
            selected.set(None);
        })
    };
    
    html! {
        <div class="page-speakers">
            // Page Header
            <section class="page-header">
                <div class="container">
                    <div class="page-header-content">
                        <span class="page-tag">{"// speakers"}</span>
                        <h1 class="page-title">
                            {"Our "}
                            <span class="outline text-glow">{"Speakers"}</span>
                        </h1>
                        <p class="page-subtitle">
                            {"Industry experts, open source maintainers, and thought leaders sharing their knowledge and experience."}
                        </p>
                        
                        // Terminal-style intro
                        <div class="page-terminal">
                            <code>
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"ls -la /speakers/"}</span>
                                <br />
                                <span class="terminal-output">{"total "}</span>
                                <span class="terminal-highlight">{speakers.len()}</span>
                                <span class="terminal-output">{" experts ready to share"}</span>
                            </code>
                        </div>
                    </div>
                </div>
                
                // Decorative elements
                <div class="page-header-decoration">
                    <div class="decoration-line"></div>
                    <div class="decoration-dots">
                        <span></span><span></span><span></span>
                    </div>
                </div>
            </section>
            
            // Speakers Grid
            <Section class="speakers-grid-section">
                <div class="speakers-filter">
                    <span class="filter-label">{"// Filter by expertise:"}</span>
                    <div class="filter-buttons">
                        <button 
                            class={classes!("filter-btn", (*filter == "all").then_some("active"))}
                            onclick={let f = filter.clone(); Callback::from(move |_| f.set("all".to_string()))}
                        >
                            {"All"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*filter == "systems").then_some("active"))}
                            onclick={let f = filter.clone(); Callback::from(move |_| f.set("systems".to_string()))}
                        >
                            {"Systems"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*filter == "web").then_some("active"))}
                            onclick={let f = filter.clone(); Callback::from(move |_| f.set("web".to_string()))}
                        >
                            {"Web"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*filter == "security").then_some("active"))}
                            onclick={let f = filter.clone(); Callback::from(move |_| f.set("security".to_string()))}
                        >
                            {"Security"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*filter == "cloud").then_some("active"))}
                            onclick={let f = filter.clone(); Callback::from(move |_| f.set("cloud".to_string()))}
                        >
                            {"Cloud"}
                        </button>
                    </div>
                </div>
                
                <div class="cards-grid speakers-grid">
                    {for speakers.iter().map(|speaker| {
                        let onclick = on_speaker_click.clone();
                        html! {
                            <SpeakerCard 
                                speaker={speaker.clone()} 
                                onclick={Some(onclick)}
                            />
                        }
                    })}
                </div>
            </Section>
            
            // Speaker Modal
            if let Some(speaker) = (*selected_speaker).clone() {
                <div class="modal-overlay" onclick={close_modal.clone()}>
                    <div class="modal speaker-modal" onclick={|e: MouseEvent| e.stop_propagation()}>
                        <button class="modal-close" onclick={close_modal}>
                            <svg viewBox="0 0 24 24" width="24" height="24">
                                <path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                            </svg>
                        </button>
                        
                        <div class="modal-header">
                            <div class="modal-image">
                                <img src={speaker.image_url.clone()} alt={speaker.name.clone()} />
                            </div>
                            <div class="modal-info">
                                <h2 class="modal-name">{&speaker.name}</h2>
                                <p class="modal-talk">{format!("\"{}\"", &speaker.talk_title)}</p>
                            </div>
                        </div>
                        
                        <div class="modal-body">
                            <div class="modal-section">
                                <h3>{"// About"}</h3>
                                <p>{&speaker.bio}</p>
                            </div>
                            
                            <div class="modal-section">
                                <h3>{"// Connect"}</h3>
                                <div class="modal-socials">
                                    {for speaker.socials.iter().map(|social| {
                                        html! {
                                            <a 
                                                href={social.url.clone()} 
                                                target="_blank" 
                                                rel="noopener noreferrer"
                                                class="modal-social-link"
                                            >
                                                {format!("> {}", social.platform)}
                                            </a>
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                        
                        <div class="modal-footer">
                            <code class="modal-code">
                                {"speaker.connect() // Don't be shy!"}
                            </code>
                        </div>
                    </div>
                </div>
            }
            
            // Call to Action
            <section class="speakers-cta">
                <div class="container">
                    <div class="cta-box">
                        <h3>{"Interested in speaking at FOSSMeet'26?"}</h3>
                        <p>{"We're always looking for passionate speakers to share their knowledge with our community."}</p>
                        <a href="mailto:fosscell@nitc.ac.in?subject=CFP FOSSMeet'26" class="btn btn-primary">
                            {"Submit a Talk"}
                            <svg viewBox="0 0 24 24" width="18" height="18">
                                <path fill="currentColor" d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn get_all_speakers() -> Vec<Speaker> {
    vec![
        Speaker {
            id: "1".to_string(),
            name: "Alice Rustacean".to_string(),
            talk_title: "Building the Future with WebAssembly".to_string(),
            bio: "Core contributor to the Rust compiler and WebAssembly advocate. Alice has been working on making Rust the go-to language for safe, high-performance applications. She's passionate about memory safety and bringing systems programming to the web.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com".to_string() },
                SocialLink { platform: "Website".to_string(), url: "https://example.com".to_string() },
            ],
        },
        Speaker {
            id: "2".to_string(),
            name: "Bob Kernel".to_string(),
            talk_title: "Inside the Linux Kernel: A Deep Dive".to_string(),
            bio: "Linux kernel maintainer with 15 years of experience in systems programming. Bob has contributed to various subsystems and is known for his work on improving kernel performance and security.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "LinkedIn".to_string(), url: "https://linkedin.com".to_string() },
            ],
        },
        Speaker {
            id: "3".to_string(),
            name: "Carol Security".to_string(),
            talk_title: "Zero Trust Architecture in Open Source".to_string(),
            bio: "Security researcher and privacy advocate, focusing on open source security tools. Carol has discovered critical vulnerabilities in major open source projects and works on making software more secure by default.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com".to_string() },
                SocialLink { platform: "LinkedIn".to_string(), url: "https://linkedin.com".to_string() },
            ],
        },
        Speaker {
            id: "4".to_string(),
            name: "David Cloud".to_string(),
            talk_title: "Kubernetes: Cloud Native the FOSS Way".to_string(),
            bio: "Cloud architect and CNCF ambassador, passionate about cloud native technologies. David helps organizations adopt Kubernetes and microservices while maintaining their commitment to open source values.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com".to_string() },
            ],
        },
        Speaker {
            id: "5".to_string(),
            name: "Eve Distributed".to_string(),
            talk_title: "Building Decentralized Systems with IPFS".to_string(),
            bio: "Protocol Labs engineer working on IPFS and libp2p. Eve is building the foundation for a more decentralized web, one protocol at a time.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "Website".to_string(), url: "https://protocol.ai".to_string() },
            ],
        },
        Speaker {
            id: "6".to_string(),
            name: "Frank GNU".to_string(),
            talk_title: "The Philosophy of Free Software".to_string(),
            bio: "Free software advocate and educator. Frank has been teaching about software freedom for over two decades and continues to inspire new generations of developers to care about user rights.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "Website".to_string(), url: "https://fsf.org".to_string() },
            ],
        },
        Speaker {
            id: "7".to_string(),
            name: "Grace ML".to_string(),
            talk_title: "Open Source AI: Democratizing Machine Learning".to_string(),
            bio: "Machine learning researcher focused on making AI accessible to everyone. Grace contributes to popular open source ML frameworks and advocates for transparent, ethical AI development.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com".to_string() },
            ],
        },
        Speaker {
            id: "8".to_string(),
            name: "Henry DevOps".to_string(),
            talk_title: "GitOps: Infrastructure as Code Done Right".to_string(),
            bio: "DevOps engineer and Argo maintainer. Henry is passionate about automating everything and making deployment processes reproducible and reliable through open source tooling.".to_string(),
            image_url: "/assets/speakers/placeholder.jpg".to_string(),
            socials: vec![
                SocialLink { platform: "GitHub".to_string(), url: "https://github.com".to_string() },
                SocialLink { platform: "LinkedIn".to_string(), url: "https://linkedin.com".to_string() },
            ],
        },
    ]
}
