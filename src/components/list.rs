use dioxus::prelude::*;

#[component]
pub fn List() -> Element {
    rsx! {    
        div { class: "px-4 sm:px-6 lg:px-8",
            div { class: "sm:flex sm:items-center",
                div { class: "sm:flex-auto",
                    h1 { class: "text-base font-semibold text-gray-900", "Users" }
                    p { class: "mt-2 text-sm text-gray-700",
                        "A list of all the users in your account including their name, title, email and role."
                    }
                }
                div { class: "mt-4 sm:mt-0 sm:ml-16 sm:flex-none",
                    button {
                        class: "block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                        r#type: "button",
                        "Add"
                    }
                }
            }
            div { class: "mt-8 flow-root",
                div { class: "-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8",
                    div { class: "inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8",
                        div { class: "overflow-hidden shadow-sm ring-1 ring-black/5 sm:rounded-lg",
                            table { class: "min-w-full divide-y divide-gray-300",
                                thead { class: "bg-gray-50",
                                    tr {
                                        th {
                                            class: "py-3.5 pr-3 pl-4 text-left text-sm font-semibold text-gray-900 sm:pl-6",
                                            scope: "col",
                                            "Name"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Title"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Email"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Role"
                                        }
                                        th {
                                            class: "relative py-3.5 pr-4 pl-3 sm:pr-6",
                                            scope: "col",
                                            span { class: "sr-only", "Edit" }
                                        }
                                    }
                                }
                                tbody { class: "divide-y divide-gray-200 bg-white",
                                    tr {
                                        td { class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-6",
                                            "Lindsay Walton"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "Front-end Developer"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "lindsay.walton@example.com"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "Member"
                                        }
                                        td { class: "relative py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-6",
                                            a {
                                                class: "text-indigo-600 hover:text-indigo-900",
                                                href: "#",
                                                "Edit"
                                                span { class: "sr-only", ", Lindsay Walton" }
                                            }
                                        }
                                    }
                                    tr {
                                        td { class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-6",
                                            "Lindsay Walton"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "Front-end Developer"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "lindsay.walton@example.com"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "Member"
                                        }
                                        td { class: "relative py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-6",
                                            a {
                                                class: "text-indigo-600 hover:text-indigo-900",
                                                href: "#",
                                                "Edit"
                                                span { class: "sr-only", ", Lindsay Walton" }
                                            }
                                        }
                                    }
                                    tr {
                                        td { class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-6",
                                            "Lindsay Walton"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "Front-end Developer"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "lindsay.walton@example.com"
                                        }
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            "Member"
                                        }
                                        td { class: "relative py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-6",
                                            a {
                                                class: "text-indigo-600 hover:text-indigo-900",
                                                href: "#",
                                                "Edit"
                                                span { class: "sr-only", ", Lindsay Walton" }
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
    }
}