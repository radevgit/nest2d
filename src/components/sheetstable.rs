use dioxus::{html::canvas::{height, width}, prelude::*};

use crate::components::SheetsRow;


#[component]
pub fn SheetsTable(sheets: Resource<Result<Vec<(usize, String, f64, f64, usize)>, ServerFnError>>) -> Element {

    rsx! {

    }
}