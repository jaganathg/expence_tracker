use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-100",
            header {
                class: "bg-blue-600 text-white p-4",
                div {
                    class: "container mx-auto flex justify-between items-center",
                    h1 {
                        class: "text-2xl font-bold",
                        "Expense Tracker"
                    }
                    nav {
                        class: "space-x-4",
                        Link {
                            to: Route::Home {},
                            class: "hover:text-blue-200 transition-colors",
                            "Home"
                        }
                        Link {
                            to: Route::AddExpense {},
                            class: "hover:text-blue-200 transition-colors",
                            "Add Expense"
                        }
                        Link {
                            to: Route::ExpenseList {},
                            class: "hover:text-blue-200 transition-colors",
                            "View Expenses"
                        }
                    }
                }
            }
            main {
                class: "container mx-auto p-4",
                {children}
            }
        }
    }
}