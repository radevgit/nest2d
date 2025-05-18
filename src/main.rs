
use dioxus::prelude::*;
use views::{Config, Nesting, Parts, Projects, Sheets, Shell};



/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

mod engine;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Shell)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Projects {},

        #[route("/sheets/")]
        Sheets {},

        #[route("/parts/")]
        Parts {},

        #[route("/nest/")]
        Nesting {},

        #[route("/config")]
        Config {},


        // // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        // #[route("/blog/:id")]
        // // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // // an `id` prop of type `i32`.
        // Blog { id: i32 },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
//const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
//const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
    // let window = WindowBuilder::new()
    //     .with_title("Nest2D")
    //     .with_resizable(true)
    //     .with_inner_size(LogicalSize::new(850.0, 600.0))
    //     .with_min_inner_size(LogicalSize::new(850.0, 600.0));

    // dioxus::LaunchBuilder::new()
    //     .with_cfg(Config::new().with_window(window.clone()))
    //     .launch(App);
}

// Create a new wrapper type
#[derive(Clone)]
struct ProjectState(String);

#[component]
fn App() -> Element {
    use_context_provider(|| ProjectState("DefaultProject".to_string()));
    rsx! {
        div {
            document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0",}
            document::Meta { name: "Charset", content: "utf-8",}
            //document::Link { rel: "icon", href: FAVICON }
            //document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }

            Router::<Route> {}
        }
    }
}


