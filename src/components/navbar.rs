use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let is_mobile_menu_open = use_state(|| false);
    let route = use_route::<Route>();
    
    let toggle_menu = {
        let is_open = is_mobile_menu_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };
    
    let close_menu = {
        let is_open = is_mobile_menu_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };
    
    let nav_items = vec![
        ("Home", Route::Home),
        ("Speakers", Route::Speakers),
        ("Workshops", Route::Workshops),
        ("Schedule", Route::Schedule),
    ];
    
    html! {
        <>
            <nav class={classes!("navbar", props.class.clone())}>
                <div class="navbar-inner">
                    // Logo
                    <Link<Route> to={Route::Home} classes="navbar-logo">
                        <span class="bracket">{"{"}</span>
                        {"FOSSMeet"}
                        <span class="bracket">{"'26}"}</span>
                    </Link<Route>>
                    
                    // Desktop Navigation Links
                    <div class="navbar-links">
                        {for nav_items.iter().map(|(label, nav_route)| {
                            let is_active = route.as_ref() == Some(nav_route);
                            html! {
                                <Link<Route>
                                    to={nav_route.clone()}
                                    classes={classes!("navbar-link", is_active.then_some("active"))}
                                >
                                    {label}
                                </Link<Route>>
                            }
                        })}
                        <a href="https://github.com/fosscell" 
                           target="_blank" 
                           rel="noopener noreferrer"
                           class="navbar-link">
                            {"GitHub"}
                        </a>
                    </div>
                    
                    // Mobile Menu Toggle
                    <button 
                        class={classes!("navbar-toggle", (*is_mobile_menu_open).then_some("open"))}
                        onclick={toggle_menu}
                        aria-label="Toggle navigation menu"
                    >
                        <span></span>
                        <span></span>
                        <span></span>
                    </button>
                </div>
            </nav>
            
            // Mobile Menu
            <div class={classes!("mobile-menu", (*is_mobile_menu_open).then_some("open"))}>
                {for nav_items.iter().map(|(label, nav_route)| {
                    let close = close_menu.clone();
                    html! {
                        <div onclick={close}>
                            <Link<Route>
                                to={nav_route.clone()}
                                classes="mobile-menu-link"
                            >
                                {format!("> {}", label)}
                            </Link<Route>>
                        </div>
                    }
                })}
                <a href="https://github.com/fosscell" 
                   target="_blank" 
                   rel="noopener noreferrer"
                   class="mobile-menu-link"
                   onclick={close_menu}>
                    {"> GitHub"}
                </a>
            </div>
        </>
    }
}
