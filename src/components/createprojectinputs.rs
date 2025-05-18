use dioxus::prelude::*;

#[component]
pub fn CreateProjectInputs() -> Element {
    rsx! { 
        
           
        div {
            label {
                class: "block text-sm/6 font-medium text-gray-900",
                r#for: "query",
                "Create a new project"
            }
            
           

                div { class: "-mr-px grid grow grid-cols-1 focus-within:relative",
                    input {
                        class: "col-start-1 row-start-1 block w-full rounded-l-md bg-white py-1.5 pr-3 pl-10 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:pl-9 sm:text-sm/6",
                        id: "name",
                        name: "name",
                        placeholder: "Project Name",
                        r#type: "text",
                    }
                    icons::icon_0 {}
                }
        //         div { class: "-mr-px grid grow grid-cols-1 focus-within:relative",
        //             input {
        //                 class: "col-start-1 row-start-1 block w-full rounded-l-md bg-white py-1.5 pr-3 pl-10 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:pl-9 sm:text-sm/6",
        //                 id: "name",
        //                 name: "name",
        //                 placeholder: "Project Description",
        //                 r#type: "text",
        //             }
        //             icons::icon_0 {}
        //         }
        //         button {
        //             class: "flex shrink-0 items-center gap-x-1.5 rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 outline-1 -outline-offset-1 outline-gray-300 hover:bg-gray-50 focus:relative focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600",
        //             r#type: "button",
        //             icons::icon_1 {}
        //             "\n      Create\n    "
        //         }
            
        }
    }
}

mod icons {
    use super::*;

    pub(crate) fn icon_0() -> Element {
        rsx! {        
            svg {
                class: "pointer-events-none col-start-1 row-start-1 ml-3 size-5 self-center text-gray-400 sm:size-4",
                "data-slot": "icon",
                fill: "currentColor",
                view_box: "0 0 16 16",
                path { d: "M8.543 2.232a.75.75 0 0 0-1.085 0l-5.25 5.5A.75.75 0 0 0 2.75 9H4v4a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1a1 1 0 1 1 2 0v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1V9h1.25a.75.75 0 0 0 .543-1.268l-5.25-5.5Z" }
            }
        }
    }

    pub(crate) fn projects_icon(selected: bool) -> Element {
        rsx! {        
            svg {
                class: if selected {
                    "size-6 shrink-0 text-indigo-600"
                } else {
                    "size-6 shrink-0 text-gray-400 group-hover:text-indigo-600"
                },
                "data-slot": "icon",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                view_box: "0 0 24 24",
                path {
                    d: "m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(crate) fn icon_1() -> Element {
        rsx! {        
            svg {
                class: "-ml-0.5 size-4 text-gray-400",
                "data-slot": "icon",
                fill: "currentColor",
                view_box: "0 0 16 16",
                path {
                    clip_rule: "evenodd",
                    d: "M2 2.75A.75.75 0 0 1 2.75 2h9.5a.75.75 0 0 1 0 1.5h-9.5A.75.75 0 0 1 2 2.75ZM2 6.25a.75.75 0 0 1 .75-.75h5.5a.75.75 0 0 1 0 1.5h-5.5A.75.75 0 0 1 2 6.25Zm0 3.5A.75.75 0 0 1 2.75 9h3.5a.75.75 0 0 1 0 1.5h-3.5A.75.75 0 0 1 2 9.75ZM9.22 9.53a.75.75 0 0 1 0-1.06l2.25-2.25a.75.75 0 0 1 1.06 0l2.25 2.25a.75.75 0 0 1-1.06 1.06l-.97-.97v5.69a.75.75 0 0 1-1.5 0V8.56l-.97.97a.75.75 0 0 1-1.06 0Z",
                    fill_rule: "evenodd",
                }
            }
        }
    }
}
