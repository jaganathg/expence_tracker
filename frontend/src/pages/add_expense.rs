use dioxus::prelude::*;
use crate::components::{Layout, ExpenseForm};

#[component]
pub fn AddExpense() -> Element {
    rsx! {
        Layout {
            ExpenseForm {}
        }
    }
}