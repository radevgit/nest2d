
use dioxus::prelude::*;

use super::navbar::NavbarProps;

#[component]
pub fn NavbarRow(props: NavbarProps) -> Element {
    if props.active {
        rsx! {
            li {
                Link {
                    class: "group flex gap-x-3 rounded-md bg-gray-50 p-2 text-sm/6 font-semibold text-indigo-600",
                    to: props.route,
                    {props.icon}
                    "\n                {props.name}\n                "
                    span {
                        aria_hidden: "true",
                        class: "ml-auto w-9 min-w-max rounded-full bg-white px-2.5 py-0.5 text-center text-xs/5 font-medium whitespace-nowrap text-gray-600 ring-1 ring-gray-200 ring-inset",
                        {props.count.unwrap_or_else(|| "".to_string())}
                    }
                }
            }
        }
    } else {
        rsx! {
            li {
                Link {
                    class: "group flex gap-x-3 rounded-md p-2 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600",
                    to: props.route,
                    {props.icon}
                    "\n                {props.name}\n              "
                    span {
                        aria_hidden: "true",
                        class: "ml-auto w-9 min-w-max rounded-full bg-white px-2.5 py-0.5 text-center text-xs/5 font-medium whitespace-nowrap text-gray-600 ring-1 ring-gray-200 ring-inset",
                        {props.count.unwrap_or_else(|| "".to_string())}
                    }
                }
            }
        }
    }
}

