
use dioxus::prelude::*;

#[component]
pub fn Configuration() -> Element {
    rsx! {
        div {  
            p { "This is the configuration page!" }
        }
    }
}
