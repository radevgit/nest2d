use dioxus::prelude::*;

#[component]
pub fn Progressbar() -> Element {
    rsx! {    
        div { class: "w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700",
            div { class: "bg-blue-600 h-2.5 rounded-full", style: "width: 45%" }
        }
    }
}