use dioxus::prelude::*;

use crate::{components::{SheetsRow, SheetsTable}, engine::create_sheet, views::CURRENT_PROJECT};


#[component]
pub fn Sheets() -> Element {
    let mut name = use_signal(|| "Sheet Name".to_string());
    let mut swidth = use_signal(|| 2000.0);
    let mut sheight = use_signal(|| 1000.0);
    let mut quantity = use_signal(|| 1);
    let mut sheets = use_resource(move || crate::engine::list_sheets(*CURRENT_PROJECT.read()));

    rsx! {
        div { class: "px-4 sm:px-6 lg:px-8",
            div { class: "sm:flex sm:items-center",
                div { class: "sm:flex-auto",
                    h2 { class: "text-base font-semibold text-gray-900",
                        "Add sheets to project: "
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
                                            "Width"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Height"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Qty"
                                        }
                                        th {
                                            class: "relative py-3.5 pr-4 pl-3 sm:pr-6",
                                            scope: "col",
                                            span { class: "sr-only", "Delete" }
                                        }
                                    }
                                }
                                tbody { class: "divide-y divide-gray-200 bg-white",
                                    tr {
                                        td { class: "py-4 pr-0 pl-2 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-3",
                                            input {
                                                class: "block w-min rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                                                placeholder: "{name}",
                                                type: "text",
                                                id: "name",
                                                value: "{name}",
                                                oninput: move |event| name.set(event.value())
                                            }
                                        }
                                        td { class: "px-3 py-4 pl-0 text-sm whitespace-nowrap text-gray-500 sm:pl-0",
                                            input {
                                                class: "block w-min rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                                                placeholder: "{swidth}",
                                                type: "number",
                                                min: "0.0",
                                                max: "100000000.0",
                                                id: "swidth",
                                                value: "{swidth}",
                                                oninput: move |event| swidth.set(event.value().parse::<f64>().unwrap_or(0.0))
                                            }
                                        }
                                        td { class: "px-3 py-4 pl-0 text-sm whitespace-nowrap text-gray-500 sm:pl-0",
                                            input {
                                                class: "block w-min rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                                                placeholder: "{sheight}",
                                                type: "number",
                                                min: "0.0",
                                                max: "100000000.0",
                                                id: "sheight",
                                                value: "{sheight}",
                                                oninput: move |event| sheight.set(event.value().parse::<f64>().unwrap_or(0.0))
                                            }
                                        }
                            
                                        td { class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                                            input {
                                                class: "block w-min rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6",
                                                placeholder: "{quantity}",
                                                type: "number",
                                                min: "1",
                                                max: "1000",
                                                id: "quantity",
                                                value: "{quantity}",
                                                oninput: move |event| quantity.set(event.value().parse::<usize>().unwrap_or(1))
                                            }
                                        }
                                        td { class: "relative py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-6",
                                            button {
                                                class: "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                                id: "create",
                                                onclick: move |_| async move {
                                                    let name = name();
                                                    let swidth = swidth();
                                                    let sheight = sheight();
                                                    let quantity = quantity();
                                                    _ = create_sheet(*CURRENT_PROJECT.read(), name, swidth, sheight, quantity).await;
                                                    _ = sheets.restart();
                                                },
                                                "Create"
                                            }
                                        }
                                    }
                                    
                                        for (id , name, swidth, sheight, quantity) in sheets.suspend()?.cloned().unwrap() {
                                                SheetsRow { id, name, swidth, sheight, quantity, sheets: sheets.clone() }
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



