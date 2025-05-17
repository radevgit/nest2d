
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {  
            p { "This is the home page!" }
        }
    }
}