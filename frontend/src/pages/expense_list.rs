use crate::components::Layout;
use crate::models::Expense;
use crate::services::ExpenseService;
use dioxus::prelude::*;

#[component]
pub fn ExpenseList() -> Element {
    #[allow(clippy::redundant_closure)]
    let mut expenses = use_signal(|| Vec::<Expense>::new());
    let mut is_loading = use_signal(|| true);
    #[allow(clippy::redundant_closure)]
    let mut error = use_signal(|| String::new());

    use_effect(move || {
        spawn(async move {
            match ExpenseService::get_all_expenses().await {
                Ok(expense_list) => {
                    expenses.set(expense_list);
                    is_loading.set(false);
                }
                Err(e) => {
                    error.set(format!("Failed to load expenses: {}", e));
                    is_loading.set(false);
                }
            }
        });
    });

    rsx! {
        Layout {
            div {
                class: "max-w-4xl mx-auto",
                h2 {
                    class: "text-2xl font-bold mb-6 text-gray-800",
                    "All Expenses"
                }

                if is_loading() {
                    div {
                        class: "text-center py-8",
                        "Loading expenses..."
                    }
                } else if !error().is_empty() {
                    div {
                        class: "text-red-600 text-center py-8",
                        "{error}"
                    }
                } else if expenses().is_empty() {
                    div {
                        class: "text-center py-8 text-gray-500",
                        "No expenses found. Add your first expense!"
                    }
                } else {
                    div {
                        class: "space-y-4",
                        div {
                            class: "bg-white rounded-lg shadow-md p-4 border-l-4 border-blue-500",
                            div {
                                class: "flex justify-between items-start",
                                div {
                                    class: "flex-1",
                                    h3 {
                                        class: "text-lg font-semibold text-gray-800",
                                        "Total Expenses: {expenses().len()}"
                                    }
                                    p {
                                        class: "text-sm text-gray-600",
                                        "Click 'Add Expense' to add new expenses"
                                    }
                                }
                                div {
                                    class: "text-right",
                                    p {
                                        class: "text-xl font-bold text-green-600",
                                        "View All"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
