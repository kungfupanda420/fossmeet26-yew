use yew::prelude::*;
use crate::components::{
    section::Section,
    workshop_card::WorkshopCard,
};
use crate::types::Workshop;

#[function_component(WorkshopsPage)]
pub fn workshops_page() -> Html {
    let workshops = use_state(|| get_all_workshops());
    let selected_day = use_state(|| "all".to_string());
    
    let filtered_workshops: Vec<Workshop> = workshops.iter()
        .filter(|w| *selected_day == "all" || w.date.to_lowercase().contains(&selected_day.to_lowercase()))
        .cloned()
        .collect();
    
    html! {
        <div class="page-workshops">
            // Page Header
            <section class="page-header">
                <div class="container">
                    <div class="page-header-content">
                        <span class="page-tag">{"// workshops"}</span>
                        <h1 class="page-title">
                            {"Hands-on "}
                            <span class="outline text-glow">{"Workshops"}</span>
                        </h1>
                        <p class="page-subtitle">
                            {"Level up your skills with intensive, hands-on sessions led by industry experts. 
                            From beginner-friendly introductions to advanced deep dives."}
                        </p>
                        
                        // Terminal-style intro
                        <div class="page-terminal">
                            <code>
                                <span class="terminal-prompt">{"$ "}</span>
                                <span class="terminal-command">{"./register --workshop all"}</span>
                                <br />
                                <span class="terminal-output">{"Found "}</span>
                                <span class="terminal-highlight">{workshops.len()}</span>
                                <span class="terminal-output">{" workshops available for registration"}</span>
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
            
            // Workshop Info Banner
            <section class="workshop-info-banner">
                <div class="container">
                    <div class="info-cards">
                        <div class="info-card">
                            <span class="info-icon">{"{"}</span>
                            <div class="info-content">
                                <h4>{"Hands-on Learning"}</h4>
                                <p>{"Practice while you learn with real-world projects"}</p>
                            </div>
                        </div>
                        <div class="info-card">
                            <span class="info-icon">{"["}</span>
                            <div class="info-content">
                                <h4>{"Limited Seats"}</h4>
                                <p>{"Small batches ensure personalized attention"}</p>
                            </div>
                        </div>
                        <div class="info-card">
                            <span class="info-icon">{"<"}</span>
                            <div class="info-content">
                                <h4>{"Take Home Projects"}</h4>
                                <p>{"Build something you can showcase in your portfolio"}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Workshops Grid
            <Section class="workshops-grid-section">
                <div class="workshops-filter">
                    <span class="filter-label">{"// Filter by day:"}</span>
                    <div class="filter-buttons">
                        <button 
                            class={classes!("filter-btn", (*selected_day == "all").then_some("active"))}
                            onclick={let d = selected_day.clone(); Callback::from(move |_| d.set("all".to_string()))}
                        >
                            {"All Days"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*selected_day == "day 1").then_some("active"))}
                            onclick={let d = selected_day.clone(); Callback::from(move |_| d.set("day 1".to_string()))}
                        >
                            {"Day 1"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*selected_day == "day 2").then_some("active"))}
                            onclick={let d = selected_day.clone(); Callback::from(move |_| d.set("day 2".to_string()))}
                        >
                            {"Day 2"}
                        </button>
                        <button 
                            class={classes!("filter-btn", (*selected_day == "day 3").then_some("active"))}
                            onclick={let d = selected_day.clone(); Callback::from(move |_| d.set("day 3".to_string()))}
                        >
                            {"Day 3"}
                        </button>
                    </div>
                </div>
                
                if filtered_workshops.is_empty() {
                    <div class="no-results">
                        <code>
                            {"// No workshops found for this filter"}
                            <br />
                            {"// Try selecting a different day"}
                        </code>
                    </div>
                } else {
                    <div class="cards-grid workshops-grid">
                        {for filtered_workshops.iter().map(|workshop| {
                            html! {
                                <WorkshopCard workshop={workshop.clone()} />
                            }
                        })}
                    </div>
                }
            </Section>
            
            // Workshop Guidelines
            <section class="workshop-guidelines">
                <div class="container">
                    <div class="guidelines-content">
                        <h3>{"// Workshop Guidelines"}</h3>
                        <div class="guidelines-grid">
                            <div class="guideline-item">
                                <span class="guideline-num">{"01"}</span>
                                <div>
                                    <h4>{"Bring Your Laptop"}</h4>
                                    <p>{"All workshops require a laptop with the specified software pre-installed."}</p>
                                </div>
                            </div>
                            <div class="guideline-item">
                                <span class="guideline-num">{"02"}</span>
                                <div>
                                    <h4>{"Check Prerequisites"}</h4>
                                    <p>{"Review the prerequisites before registering to ensure you're prepared."}</p>
                                </div>
                            </div>
                            <div class="guideline-item">
                                <span class="guideline-num">{"03"}</span>
                                <div>
                                    <h4>{"Arrive Early"}</h4>
                                    <p>{"Reach 15 minutes before the workshop starts for setup and introductions."}</p>
                                </div>
                            </div>
                            <div class="guideline-item">
                                <span class="guideline-num">{"04"}</span>
                                <div>
                                    <h4>{"Ask Questions"}</h4>
                                    <p>{"Don't hesitate to ask questions. That's what the instructors are there for!"}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // CTA Section
            <section class="workshops-cta">
                <div class="container">
                    <div class="cta-box">
                        <h3>{"Want to conduct a workshop?"}</h3>
                        <p>{"Share your expertise with the community. We're looking for passionate instructors who can deliver hands-on learning experiences."}</p>
                        <a href="mailto:fosscell@nitc.ac.in?subject=Workshop Proposal FOSSMeet'26" class="btn btn-primary">
                            {"Submit Workshop Proposal"}
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn get_all_workshops() -> Vec<Workshop> {
    vec![
        Workshop {
            id: "1".to_string(),
            title: "Rust for Beginners".to_string(),
            description: "Learn the fundamentals of Rust programming, from ownership to async/await. Perfect for developers looking to write safe, concurrent code. We'll build a CLI tool from scratch.".to_string(),
            logo_url: "/assets/workshops/rust.svg".to_string(),
            date: "Day 1".to_string(),
            duration: "3".to_string(),
            speaker_name: "Alice Rustacean".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "Basic programming knowledge".to_string(), 
                "Laptop with Rust installed".to_string(),
                "Familiarity with command line".to_string(),
            ],
        },
        Workshop {
            id: "2".to_string(),
            title: "Linux Kernel Hacking 101".to_string(),
            description: "Get your hands dirty with kernel development. We'll write a simple kernel module, understand the Linux kernel architecture, and learn how to debug kernel code.".to_string(),
            logo_url: "/assets/workshops/linux.svg".to_string(),
            date: "Day 2".to_string(),
            duration: "4".to_string(),
            speaker_name: "Bob Kernel".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "C programming proficiency".to_string(), 
                "Linux basics".to_string(),
                "VM or spare machine recommended".to_string(),
            ],
        },
        Workshop {
            id: "3".to_string(),
            title: "Open Source Security Audit".to_string(),
            description: "Learn to identify and fix security vulnerabilities in open source projects. Hands-on with real CVEs, SAST/DAST tools, and responsible disclosure practices.".to_string(),
            logo_url: "/assets/workshops/security.svg".to_string(),
            date: "Day 2".to_string(),
            duration: "3".to_string(),
            speaker_name: "Carol Security".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "Programming experience".to_string(), 
                "Basic security concepts".to_string(),
            ],
        },
        Workshop {
            id: "4".to_string(),
            title: "WebAssembly Deep Dive".to_string(),
            description: "Explore the world of WebAssembly. Learn how to compile Rust to WASM, integrate with JavaScript, and build high-performance web applications.".to_string(),
            logo_url: "/assets/workshops/wasm.svg".to_string(),
            date: "Day 1".to_string(),
            duration: "3".to_string(),
            speaker_name: "Alice Rustacean".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "JavaScript knowledge".to_string(),
                "Basic Rust knowledge (helpful but not required)".to_string(),
            ],
        },
        Workshop {
            id: "5".to_string(),
            title: "Kubernetes from Scratch".to_string(),
            description: "Deploy your first Kubernetes cluster and applications. Learn about pods, deployments, services, and how to manage cloud-native applications at scale.".to_string(),
            logo_url: "/assets/workshops/kubernetes.svg".to_string(),
            date: "Day 3".to_string(),
            duration: "4".to_string(),
            speaker_name: "David Cloud".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "Docker basics".to_string(),
                "Command line proficiency".to_string(),
                "Understanding of containerization concepts".to_string(),
            ],
        },
        Workshop {
            id: "6".to_string(),
            title: "Contributing to Open Source".to_string(),
            description: "Your complete guide to making your first open source contribution. Learn Git workflows, how to find good first issues, write good PRs, and communicate with maintainers.".to_string(),
            logo_url: "/assets/workshops/git.svg".to_string(),
            date: "Day 1".to_string(),
            duration: "2".to_string(),
            speaker_name: "Frank GNU".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "Basic Git knowledge".to_string(),
                "GitHub account".to_string(),
            ],
        },
        Workshop {
            id: "7".to_string(),
            title: "Building AI with Open Source Tools".to_string(),
            description: "Train and deploy machine learning models using open source tools. We'll use PyTorch, Hugging Face, and MLflow to build an end-to-end ML pipeline.".to_string(),
            logo_url: "/assets/workshops/ml.svg".to_string(),
            date: "Day 3".to_string(),
            duration: "4".to_string(),
            speaker_name: "Grace ML".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "Python programming".to_string(),
                "Basic ML concepts".to_string(),
                "GPU access helpful but not required".to_string(),
            ],
        },
        Workshop {
            id: "8".to_string(),
            title: "GitOps with ArgoCD".to_string(),
            description: "Implement GitOps practices using ArgoCD. Learn declarative infrastructure management, continuous deployment, and how to manage Kubernetes applications the GitOps way.".to_string(),
            logo_url: "/assets/workshops/argo.svg".to_string(),
            date: "Day 3".to_string(),
            duration: "3".to_string(),
            speaker_name: "Henry DevOps".to_string(),
            speaker_image_url: "/assets/speakers/placeholder.jpg".to_string(),
            registration_link: Some("https://townscript.com".to_string()),
            prerequisites: vec![
                "Kubernetes basics".to_string(),
                "Git proficiency".to_string(),
                "YAML knowledge".to_string(),
            ],
        },
    ]
}
