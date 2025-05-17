
use dioxus::prelude::*;

/// The Parts page component that will be rendered when the current route is `[Route::Parts]`
#[component]
pub fn Parts() -> Element {
    rsx! {
        div {  
            p { "PARTS" }
        }
    }
}
