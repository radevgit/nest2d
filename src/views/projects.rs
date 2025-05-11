
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {  background_color: "#DFDBE5",
            p { "This is the projects page!" }
        }
    }
}
