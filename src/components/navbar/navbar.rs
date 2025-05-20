
use dioxus::prelude::*;
use icons::{cog_icon, projects_icon, nesting_icon, parts_icon, sheets_icon};

use crate::{components::navbar::NavbarRow, engine::count_projects, views::CURRENT_PROJECT, Route};


#[derive(PartialEq, Props, Clone)]
pub(crate) struct NavbarProps {
    pub name: String,
    #[props(default = false)]
    pub active: bool,
    pub route: Route,
    pub icon: Element,
    pub count: Option<String>,
}

#[component]
pub fn Navbar() -> Element {
    let mut current = use_signal(|| 0);
    let mut p_count = use_resource(move || crate::engine::count_projects());
    let mut s_count = use_resource(move || crate::engine::count_sheets(*CURRENT_PROJECT.read()));
    
    rsx! {
        nav {
            class: "flex flex-1 flex-col bg-zinc-50",
            onmouseenter: move |_| {
                p_count.restart();
                s_count.restart();
            },
            ul { class: "flex flex-1 flex-col gap-y-7", role: "list",
                li {
                    ul { class: "-mx-2 space-y-1", role: "list",
                        onclick: move |_| current.set(0),
                        NavbarRow {name: "Projects", active: is_active(0, current()), route: Route::Projects {  }, icon: projects_icon(is_active(0, current())), 
                        count: Some(p_count.suspend()?.cloned().unwrap_or(0).to_string())}
                    }
                    if *CURRENT_PROJECT.read() != std::usize::MAX {
                        ul { class: "-mx-2 space-y-1", role: "list",
                            onclick: move |_| current.set(1),
                            NavbarRow {name: "Sheets", active: is_active(1, current()), route: Route::Sheets {}, icon: sheets_icon(is_active(1, current())), 
                            count: Some(s_count.suspend()?.cloned().unwrap_or(0).to_string())}
                        }
                        ul { class: "-mx-2 space-y-1", role: "list",
                            onclick: move |_| current.set(2),
                            NavbarRow {name: "Parts", active: is_active(2, current()), route: Route::Parts {}, icon: parts_icon(is_active(2, current())), 
                            count: Some("2".to_string())}
                        }
                        ul { class: "-mx-2 space-y-1", role: "list",
                            onclick: move |_| current.set(3),
                            NavbarRow {name: "Nesting", active: is_active(3, current()), route: Route::Nesting {}, icon: nesting_icon(is_active(3, current())), 
                            count: Some("75%".to_string())}
                        }
                    }
                    ul { class: "-mx-2 space-y-1", role: "list",
                        onclick: move |_| current.set(4),
                        NavbarRow {name: "Config", active: is_active(4, current()), route: Route::Configuration {}, icon: cog_icon(is_active(4, current()))}
                    }
                }
            }
        }
    }
}


pub(crate) fn is_active(position: i64, current: i64) -> bool {
    position == current
}

mod icons {
    use super::*;



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

    pub(crate) fn sheets_icon(selected: bool) -> Element {
        rsx! {        
            svg {
                class: if selected {
                    "size-6 shrink-0 text-indigo-600"
                } else {
                    "size-6 shrink-0 text-gray-400 group-hover:text-indigo-600"
                },
                class: "size-6 shrink-0 text-gray-400 group-hover:text-indigo-600",
                "data-slot": "icon",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                view_box: "0 0 24 24",
                path {
                    d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(crate) fn sheets2_icon() -> Element {
        rsx! {        
            svg {
                class: "size-6 shrink-0 text-gray-400 group-hover:text-indigo-600",
                "data-slot": "icon",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                view_box: "0 0 24 24",
                path {
                    d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(crate) fn parts_icon(selected: bool) -> Element {
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
                    d: "M14.25 6.087c0-.355.186-.676.401-.959.221-.29.349-.634.349-1.003 0-1.036-1.007-1.875-2.25-1.875s-2.25.84-2.25 1.875c0 .369.128.713.349 1.003.215.283.401.604.401.959v0a.64.64 0 0 1-.657.643 48.39 48.39 0 0 1-4.163-.3c.186 1.613.293 3.25.315 4.907a.656.656 0 0 1-.658.663v0c-.355 0-.676-.186-.959-.401a1.647 1.647 0 0 0-1.003-.349c-1.036 0-1.875 1.007-1.875 2.25s.84 2.25 1.875 2.25c.369 0 .713-.128 1.003-.349.283-.215.604-.401.959-.401v0c.31 0 .555.26.532.57a48.039 48.039 0 0 1-.642 5.056c1.518.19 3.058.309 4.616.354a.64.64 0 0 0 .657-.643v0c0-.355-.186-.676-.401-.959a1.647 1.647 0 0 1-.349-1.003c0-1.035 1.008-1.875 2.25-1.875 1.243 0 2.25.84 2.25 1.875 0 .369-.128.713-.349 1.003-.215.283-.4.604-.4.959v0c0 .333.277.599.61.58a48.1 48.1 0 0 0 5.427-.63 48.05 48.05 0 0 0 .582-4.717.532.532 0 0 0-.533-.57v0c-.355 0-.676.186-.959.401-.29.221-.634.349-1.003.349-1.035 0-1.875-1.007-1.875-2.25s.84-2.25 1.875-2.25c.37 0 .713.128 1.003.349.283.215.604.401.96.401v0a.656.656 0 0 0 .658-.663 48.422 48.422 0 0 0-.37-5.36c-1.886.342-3.81.574-5.766.689a.578.578 0 0 1-.61-.58v0Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(crate) fn nesting_icon(selected: bool) -> Element {
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
                    d: "M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006 0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3-3 3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(crate) fn cog_icon(selected: bool) -> Element {
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
                    d: "M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 0 1 1.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.559.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.894.149c-.424.07-.764.383-.929.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 0 1-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.398.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 0 1-.12-1.45l.527-.737c.25-.35.272-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.108-1.204l-.526-.738a1.125 1.125 0 0 1 .12-1.45l.773-.773a1.125 1.125 0 0 1 1.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }



}
