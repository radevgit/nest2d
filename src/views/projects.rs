use dioxus::{html::u, prelude::*};

use crate::{components::CreateProjectInputs, engine::{create_project, CreateProject}, ProjectState};
use crate::{components::Navbar, components::ListProjects};


/// The PRojects page component that will be rendered when the current route is `[Route::Projects]`
#[component]
pub fn Projects() -> Element {
    let projectState = use_context::<ProjectState>();
    let mut name = use_signal(|| "Default".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut projects = use_resource(move || crate::engine::list_projects());

    rsx! {
        div {
            div {class: "space-y-12",
                div {class: "pb-8",
                    h2 { class: "text-base/7 font-semibold text-gray-900", "Create and select projects" }
                    div { class: "mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6",
                        div { class: "relative",
                            label { 
                                class: "absolute -top-2 left-2 inline-block rounded-lg bg-white px-1 text-xs font-medium text-gray-900",
                                r#for: "Name",
                                "Name"
                            }
                            input {
                                class: "block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                                placeholder: "{name}",
                                type: "text",
                                id: "name",
                                value: "{name}",
                                oninput: move |event| name.set(event.value())
                            }
                        }
                        div { class: "relative",
                            label { 
                                class: "absolute -top-2 left-2 inline-block rounded-lg bg-white px-1 text-xs font-medium text-gray-900",
                                r#for: "Description",
                                "Description"
                            }
                            input {
                                class: "block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                                placeholder: "{description}",
                                type: "text",
                                id: "description",
                                value: "{description}",
                                oninput: move |event| description.set(event.value())
                            }
                        }
                        div { class: "relative",
                            button {
                                class: "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                id: "create",
                                onclick: move |_| async move {
                                    let name = name();
                                    let description = description();
                                    _ = create_project(name, description).await;
                                    _ = projects.restart();
                                },
                                "Create"
                            }
                        }
                    
                    }   
                }
                
            }
        }
        div {
            ListProjects { projects }
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


