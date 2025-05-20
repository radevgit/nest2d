use dioxus::prelude::*;

use crate::{engine::count_projects, views::CURRENT_PROJECT};

#[component]
pub fn ProjectsRow(id: usize, name: String, description: String, created_at: String, 
    projects: Resource<Result<Vec<(usize, String, String, String)>, ServerFnError>>) -> Element {

    rsx! {
        tr {
            td { class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-0",
                    input {
                        class: "relative size-4 appearance-none rounded-full border border-gray-300 bg-white before:absolute before:inset-1 before:rounded-full before:bg-white not-checked:before:hidden checked:border-indigo-600 checked:bg-indigo-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:before:bg-gray-400 forced-colors:appearance-auto forced-colors:before:hidden",
                        onclick: move |_| async move {
                            *CURRENT_PROJECT.write() = id;
                        },
                        id: "{id}",
                        name: "project",
                        r#type: "radio",
                        checked: if *CURRENT_PROJECT.read() == id {
                            "true"
                        } else {
                            "false"
                        }
                    }
            }
            td { class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-0",
                "{name}"
            }
            td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{description}"
            }
            td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{created_at}"
            }
            td { class: "relative py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-0",
                div { 
                    class: "text-indigo-600 hover:text-indigo-900",
                    onclick: move |_| async move {
                        crate::engine::delete_project(id).await.unwrap();
                        if *CURRENT_PROJECT.read() == id {
                            *CURRENT_PROJECT.write() = std::usize::MAX;
                        }
                        projects.restart();
                    },
                    "Delete"
                    span { class: "sr-only", "" }
                }
            }
        }
    }
}