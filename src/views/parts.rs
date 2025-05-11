
use dioxus::prelude::*;

#[component]
pub fn Parts() -> Element {
    rsx! {
        div {  
            p { "This is the parts page!" }
        }
    }
}
