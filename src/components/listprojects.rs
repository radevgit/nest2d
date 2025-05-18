use dioxus::prelude::*;

#[component]
pub fn ListProjects(projects: Resource<Result<Vec<(usize, String, String, String)>, ServerFnError>>) -> Element {
    //let mut projects = use_resource(move || crate::engine::list_projects());

    rsx! {
        div { id: "projects",
            div { id: "projects-container",
                for (id , name, description, created_at) in projects.suspend()?.cloned().unwrap() {
                    div { class: "projects", key: id,
                        p {  
                            "{name}, {description}, {created_at}"
                        }
                        button {
                            onclick: move |_| async move {
                                crate::engine::delete_project(id).await.unwrap();
                                projects.restart();
                            },
                            "❌"
                        }
                    }
                }
            }
        }
    }
}
