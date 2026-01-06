use yew::prelude::*;
use gloo::events::EventListener;
use web_sys::window;

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollState {
    pub x: f64,
    pub y: f64,
    pub direction: ScrollDirection,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
    None,
}

impl Default for ScrollState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            direction: ScrollDirection::None,
        }
    }
}

/// Hook to track scroll position and direction
#[hook]
pub fn use_scroll() -> ScrollState {
    let state = use_state(ScrollState::default);
    let prev_y = use_state(|| 0.0f64);

    {
        let state = state.clone();
        let prev_y = prev_y.clone();
        
        use_effect_with((), move |_| {
            let listener = if let Some(win) = window() {
                Some(EventListener::new(&win, "scroll", move |_| {
                    if let Some(win) = window() {
                        let x = win.scroll_x().unwrap_or(0.0);
                        let y = win.scroll_y().unwrap_or(0.0);
                        let prev = *prev_y;
                        
                        let direction = if y > prev {
                            ScrollDirection::Down
                        } else if y < prev {
                            ScrollDirection::Up
                        } else {
                            ScrollDirection::None
                        };
                        
                        prev_y.set(y);
                        state.set(ScrollState { x, y, direction });
                    }
                }))
            } else {
                None
            };
            
            move || drop(listener)
        });
    }

    (*state).clone()
}

/// Hook to check if page has scrolled past a certain point
#[hook]
pub fn use_is_scrolled(threshold: f64) -> bool {
    let scroll = use_scroll();
    scroll.y > threshold
}
