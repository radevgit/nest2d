
use dioxus::prelude::*;

#[component]
pub fn Nesting() -> Element {
    rsx! {
        div {  
            p { "This is the nesting page!" }
        }
    }
}
