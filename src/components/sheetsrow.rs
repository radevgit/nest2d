use dioxus::prelude::*;

use crate::{engine::count_sheets, ServerFnError};

#[component]
pub fn SheetsRow(id: usize, name: String, swidth: f64, sheight: f64, quantity: usize,
    sheets: Resource<Result<Vec<(usize, String, f64, f64, usize)>, ServerFnError>>) -> Element {

    rsx! {
        tr {
            td { class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-6",
                "{name}"
            }
            td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{swidth}"
            }
            td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{sheight}"
            }

            td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                input {
                    class: "rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                    placeholder: "{quantity}",
                    type: "number",
                    min: "0",
                    max: "1000",
                    id: "quantity",
                    value: "{quantity}",
                    oninput: move |event| async move {
                        let value = event.value().parse::<usize>().unwrap_or(0);
                        if value != quantity {
                            let _ = crate::engine::update_sheet_quantity(id, value).await;
                        }
                    }
                }
            }
            td { class: "relative py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-6",
                button { 
                    class: "rounded-md bg-zinc-500 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-zinc-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    onclick: move |_| async move {
                        let _ = crate::engine::delete_sheet(id).await;
                        sheets.restart();
                    },
                    "Delete"
                    span { class: "sr-only", "" }
                }
            }
        }
    }
}