
use dioxus::prelude::*;
use icons::grid_icon;

/// The PRojects page component that will be rendered when the current route is `[Route::Projects]`
#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            p { "PROJECTS" }
        }
    }
}



mod icons {
    use super::*;

    pub(crate) fn grid_icon() -> Element {
        rsx! {        
            svg {
                class: "size-6 shrink-0 text-gray-400 group-hover:text-indigo-600",
                "data-slot": "icon",
                fill: "9C92AC",
                fill_opacity: "0.4",
                stroke: "currentColor",
                stroke_width: "1.5",
                width: "100",
                height: "100",
                view_box: "0 0 100 100",
                fill_rule: "evenodd",
                path {
                    d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}