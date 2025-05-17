
use dioxus::prelude::*;

use crate::components::{Progress, Progressbar};

/// The Parts page component that will be rendered when the current route is `[Route::Parts]`
#[component]
pub fn Nesting() -> Element {
    rsx! {
        div {  
            Progressbar {  }
            Progress {}
        }
    }
}
