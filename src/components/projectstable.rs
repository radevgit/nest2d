use dioxus::prelude::*;

use crate::{components::ProjectsRow, views::CURRENT_PROJECT};

#[component]
pub fn ProjectsTable(projects: Resource<Result<Vec<(usize, String, String, String)>, ServerFnError>>) -> Element {
    //let mut projects = use_resource(move || crate::engine::list_projects());

    rsx! {
        div { class: "px-4 sm:px-6 lg:px-8",
            div { class: "mt-8 flow-root",
                div { class: "-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8",
                    div { class: "inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8",
                        fieldset {
                            table { class: "min-w-min divide-y divide-gray-300",
                                thead {
                                    tr {
                                        th {
                                            class: "py-3.5 pr-3 pl-4 text-left text-sm font-semibold text-gray-900 sm:pl-0",
                                            scope: "col",
                                            input {
                                                class: "relative size-4 appearance-none rounded-full border border-gray-300 bg-white before:absolute before:inset-1 before:rounded-full before:bg-white not-checked:before:hidden checked:border-indigo-600 checked:bg-indigo-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:before:bg-gray-400 forced-colors:appearance-auto forced-colors:before:hidden",
                                                onclick: move |_| async move {
                                                    *CURRENT_PROJECT.write() = std::usize::MAX;
                                                },
                                                name: "project",
                                                r#type: "radio",
                                                checked: if *CURRENT_PROJECT.read() == std::usize::MAX  {
                                                    "true"
                                                } else {
                                                    "false"
                                                }
                                            }
                                        }
                                        th {
                                            class: "py-3.5 pr-3 pl-4 text-left text-sm font-semibold text-gray-900 sm:pl-0",
                                            scope: "col",
                                            "Name"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Description"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Created at"
                                        }
                                        th {
                                            class: "relative py-3.5 pr-4 pl-3 sm:pr-0",
                                            scope: "col",
                                            span { class: "sr-only", "Delete" }
                                        }
                                    }
                                }
                                tbody { class: "divide-y divide-gray-200",
                                for (id , name, description, created_at) in projects.suspend()?.cloned().unwrap() {
                                        ProjectsRow { id, name, description, created_at, projects: projects.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}