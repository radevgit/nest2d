
use dioxus::prelude::*;

/// The Config page component that will be rendered when the current route is `[Route::Config]`
#[component]
pub fn Config() -> Element {
    rsx! {
        div {  
            p { "CONFIGURATION" }
        }
    }
}
