use dioxus::prelude::*;
use crate::components::Layout;
use crate::models::Expense;
use crate::services::ExpenseService;

#[component]
pub fn Home() -> Element {
    let mut highest_expense = use_signal(|| None::<Expense>);
    let mut is_loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            match ExpenseService::get_highest_expense().await {
                Ok(expense) => {
                    highest_expense.set(expense);
                    is_loading.set(false);
                }
                Err(_) => {
                    is_loading.set(false);
                }
            }
        });
    });

    rsx! {
        Layout {
            div {
                class: "text-center py-12",
                h1 {
                    class: "text-4xl font-bold text-gray-900 mb-4",
                    "Welcome to Expense Tracker"
                }
                p {
                    class: "text-xl text-gray-600 mb-8",
                    "Track your expenses easily and efficiently"
                }
                
                if is_loading() {
                    div {
                        class: "text-gray-500",
                        "Loading..."
                    }
                } else if let Some(expense) = highest_expense() {
                    div {
                        class: "max-w-md mx-auto bg-white rounded-lg shadow-md p-6",
                        h2 {
                            class: "text-xl font-semibold text-gray-800 mb-4",
                            "Highest Expense"
                        }
                        div {
                            class: "text-center",
                            p {
                                class: "text-3xl font-bold text-red-600",
                                "${expense.amount:.2}"
                            }
                            p {
                                class: "text-sm text-gray-500",
                                "{expense.category} • {expense.date.format(\"%Y-%m-%d\")}"
                            }
                        }
                    }
                } else {
                    div {
                        class: "text-gray-500",
                        "No expenses found. Add your first expense!"
                    }
                }
            }
        }
    }
}