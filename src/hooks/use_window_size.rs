use yew::prelude::*;
use gloo::events::EventListener;
use web_sys::window;

#[derive(Clone, Debug, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}

/// Hook to get and track window size
#[hook]
pub fn use_window_size() -> WindowSize {
    let size = use_state(|| {
        if let Some(win) = window() {
            WindowSize {
                width: win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1920.0) as u32,
                height: win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(1080.0) as u32,
            }
        } else {
            WindowSize::default()
        }
    });

    {
        let size = size.clone();
        use_effect_with((), move |_| {
            let listener = if let Some(win) = window() {
                Some(EventListener::new(&win, "resize", move |_| {
                    if let Some(win) = window() {
                        size.set(WindowSize {
                            width: win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1920.0) as u32,
                            height: win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(1080.0) as u32,
                        });
                    }
                }))
            } else {
                None
            };
            
            move || drop(listener)
        });
    }

    (*size).clone()
}

/// Check if the screen is mobile-sized
#[hook]
pub fn use_is_mobile() -> bool {
    let size = use_window_size();
    size.width < 768
}

/// Check if the screen is tablet-sized
#[hook]
pub fn use_is_tablet() -> bool {
    let size = use_window_size();
    size.width >= 768 && size.width < 1024
}
