
use dioxus::prelude::*;

/// The Parts page component that will be rendered when the current route is `[Route::Sheets]`
#[component]
pub fn Sheets() -> Element {
    rsx! {
        div {  
            p { "SHEETS" }
        }
    }
}
