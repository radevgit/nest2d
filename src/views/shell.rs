use dioxus::prelude::*;
use icons::nest2d_icon;

use crate::{components::Navbar, Route};


/// The Shell component that will be rendered on all pages of our app since every page is under the layout.
#[component]
pub fn Shell() -> Element {
    rsx! {    
        div {
            div { class: "md:fixed md:inset-y-0 md:z-50 md:flex md:w-64 md:flex-col bg-white",
                div { class: "flex grow flex-col gap-y-5 overflow-y-auto border-r border-gray-200 bg-white px-6",
                    div { class: "flex h-16 shrink-0 items-center",
                        nest2d_icon {}
                    }
                    Navbar {  }
                }
            }
            main { class: "py-10 md:pl-72",
                div { class: "px-4 sm:px-6 md:px-8",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

mod icons {
    use super::*;

    pub(crate) fn nest2d_icon() -> Element {
        rsx! {        
            svg {
                class: "size-7 shrink-0 text-indigo-600",
                "data-slot": "icon",
                fill: "#4f46e5",
                stroke: "white",
                stroke_width: "1.5",
                view_box: "0 0 32 32",
                path {
                    d: "M 4,0 V 26 C 10.711765,16.585641 18.74611,8.6233489 28,2 V 0 H 4 Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M 4,30 C 13.254441,23.376351 21.288558,15.413785 28,6 V 32 H 4 Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}
