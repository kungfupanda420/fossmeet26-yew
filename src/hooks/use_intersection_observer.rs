use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// Hook for intersection observer - useful for animations on scroll
#[hook]
pub fn use_intersection_observer(
    threshold: f64,
) -> (NodeRef, UseStateHandle<bool>) {
    let node_ref = use_node_ref();
    let is_visible = use_state(|| false);

    {
        let node_ref = node_ref.clone();
        let is_visible = is_visible.clone();
        
        use_effect_with(node_ref.clone(), move |node_ref| {
            let node_ref = node_ref.clone();
            let is_visible = is_visible.clone();
            
            let cleanup = if let Some(element) = node_ref.cast::<web_sys::Element>() {
                let callback = Closure::wrap(Box::new(move |entries: Vec<IntersectionObserverEntry>, _observer: IntersectionObserver| {
                    for entry in entries {
                        is_visible.set(entry.is_intersecting());
                    }
                }) as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);
                
                let options = IntersectionObserverInit::new();
                options.set_threshold(&JsValue::from_f64(threshold));
                
                if let Ok(observer) = IntersectionObserver::new_with_options(
                    callback.as_ref().unchecked_ref(),
                    &options,
                ) {
                    observer.observe(&element);
                    callback.forget(); // Prevent the closure from being dropped
                    Some(observer)
                } else {
                    None
                }
            } else {
                None
            };
            
            move || {
                if let Some(observer) = cleanup {
                    observer.disconnect();
                }
            }
        });
    }

    (node_ref, is_visible)
}
