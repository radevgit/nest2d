
use dioxus::prelude::*;

use crate::{components::PartsRow, views::CURRENT_PROJECT};
use rfd::FileDialog;


/// The Parts page component that will be rendered when the current route is `[Route::Parts]`
#[component]
pub fn Parts() -> Element {
    let mut name = use_signal(|| "Sheet Name".to_string());
    let mut swidth = use_signal(|| 2000.0);
    let mut sheight = use_signal(|| 1000.0);
    let mut quantity = use_signal(|| 0);
    let mut sheets = use_resource(move || crate::engine::list_sheets(*CURRENT_PROJECT.read()));
    let mut filenames = use_signal(|| Vec::<String>::new());
    let mut file_content = use_signal(|| String::new());


    rsx! {
        div { class: "px-4 sm:px-6 lg:px-8 bg-zinc-50",
            div { class: "sm:flex sm:items-center",
                div { class: "sm:flex-auto",
                    h2 { class: "text-base font-semibold text-gray-900",
                        "Add parts to project: "
                    }
                }
            }
            div { class: "mt-8 flow-root",
                div { class: "-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8",
                    div { class: "max-h-96 inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8",
                        div { class: "overflow-hidden shadow-sm ring-1 ring-black/5 sm:rounded-lg",
                            table { class: "min-w-full divide-y divide-gray-300 w-full h-96 overflow-y-auto",
                                thead { class: "bg-zinc-100",
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
                                            "Width"
                                        }
                                        th {
                                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                            scope: "col",
                                            "Quantity"
                                        }
                                        th {
                                            // input {
                                            //     class: "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                            //     id: "create",
                                            //     type: "file",
                                            //     accept: ".dxf",
                                            //     onchange: move |event| {
                                            //         let files = event.files();
                                            //         if let Some(file) = files.get(0) {
                                            //             let file_name = file.name();
                                            //             filenames.modify(|f| f.push(file_name));
                                            //             let reader = web_sys::FileReader::new().unwrap();
                                            //             reader.set_onload(Some(&Closure::once_into_js(move |event: web_sys::ProgressEvent| {
                                            //                 if let Ok(content) = event.target().unwrap().result() {
                                            //                     file_content.set(content.as_string().unwrap());
                                            //                 }
                                            //             })));
                                            //             reader.read_as_text(&file).unwrap();
                                            //         }
                                            //     },
                                            //     onclick: move |_| async move {
                                            //         let name = name();
                                            //         let swidth = swidth();
                                            //         let sheight = sheight();
                                            //         let quantity = quantity();
                                            //         //_ = create_sheet(*CURRENT_PROJECT.read(), name, swidth, sheight, quantity).await;
                                            //         //_ = sheets.restart();
                                            //     },
                                            //     "Add"
                                            // }

                                            input {
                                                class: "hidden",
                                                id: "file_input",
                                                onchange: move |event| {
                                                    async move {
                                                        if let Some(file_engine)  = event.files() {
                                                            let file_names = file_engine.files();
                                                            for f in &file_names {
                                                                filenames.write().push(f.clone());
                                                                if let Some(content) = file_engine.read_file_to_string(f).await {
                                                                    file_content.set(content);
                                                                }  

                                                            }
                                                        }
                                                    }
                                                },
                                                type: "file",
                                                accept: ".svg, .dxf",
                                                multiple: true,
                                                name: "Add Parts",
                                                placeholder: "Add Parts"
                                            }
                                            label {
                                                class: "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",  
                                                for: "file_input",
                                                "Add Parts"
                                            }
                                            // input {
                                            //     r#type: "file",
                                            //     accept: ".svg",
                                            //     onclick: move |_| {
                                            //         let path = FileDialog::new()
                                            //         .add_filter("CAD", &["svg", "dxf"])
                                            //         .set_directory("~")
                                            //         .set_title("Select SVG or DXF files")
                                            //         //.set_file_name("CAD files")
                                            //         .pick_files();
                                    
                                            //         println!("picked files: {:?}", path);
                                            //     },
                                            //     "Pick files"
                                            // }
                                        }
                                    }
                                }

                                for f in filenames.iter()  {
                                    h1 {
                                        "{f}"
                                    }
                                }

                                // tbody { class: "divide-y divide-gray-200 class: bg-slate-50",
                                    
                                //     for (id , name, swidth, sheight, quantity) in sheets.suspend()?.cloned().unwrap() {
                                //             PartsRow { id, name, swidth, sheight, quantity, sheets: sheets.clone() }
                                //     }
                                    
                                // }
                            }
                        }
                    }
                }
            }
            div { class: "sm:flex sm:items-center",
                div { class: "sm:flex-auto",
                    h2 { class: "text-base font-semibold text-gray-900 py-12",
                        "Add part to project: "
                    }
                    
                    
                }
            }
        }

    }
}
