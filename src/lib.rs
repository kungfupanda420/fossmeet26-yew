use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod pages;
mod types;
mod hooks;

use components::{navbar::Navbar, footer::Footer, matrix_rain::MatrixRain};
use pages::{home::HomePage, speakers::SpeakersPage, workshops::WorkshopsPage, schedule::SchedulePage, not_found::NotFoundPage};

/// Application routes
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/speakers")]
    Speakers,
    #[at("/workshops")]
    Workshops,
    #[at("/schedule")]
    Schedule,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Speakers => html! { <SpeakersPage /> },
        Route::Workshops => html! { <WorkshopsPage /> },
        Route::Schedule => html! { <SchedulePage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

/// Main App component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container grid-pattern">
                <MatrixRain />
                <Navbar />
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}

/// Entry point for WASM
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("FOSSMeet'26 WASM application starting...");
    
    // Remove loading screen
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(loading) = document.get_element_by_id("loading-screen") {
                let _ = loading.remove();
            }
        }
    }
    
    // Mount the application
    yew::Renderer::<App>::new().render();
}
