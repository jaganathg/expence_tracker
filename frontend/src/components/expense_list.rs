use dioxus::prelude::*;

#[component]
pub fn ExpenseList() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto",
            h2 {
                class: "text-2xl font-bold mb-6 text-gray-800",
                "All Expenses"
            }
            p {
                "List implementation coming soon..."
            }
        }
    }
}