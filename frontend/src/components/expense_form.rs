use crate::models::CreateExpenseRequest;
use crate::services::ExpenseService;
use dioxus::prelude::*;

#[component]
pub fn ExpenseForm() -> Element {
    #[allow(clippy::redundant_closure)]
    let mut amount = use_signal(|| String::new());
    #[allow(clippy::redundant_closure)]
    let mut category = use_signal(|| String::new());
    #[allow(clippy::redundant_closure)]
    let mut message = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let submit_handler = move |_| {
        spawn(async move {
            is_loading.set(true);
            message.set(String::new());

            let amount_value: Result<f64, _> = amount().parse();
            match amount_value {
                Ok(amt) => {
                    let request = CreateExpenseRequest {
                        amount: amt,
                        category: category().clone(),
                    };

                    match ExpenseService::add_expense(request).await {
                        Ok(_) => {
                            message.set("Expense added successfully!".to_string());
                            amount.set(String::new());
                            category.set(String::new());
                        }
                        Err(e) => {
                            message.set(format!("Error: {}", e));
                        }
                    }
                }
                Err(_) => {
                    message.set("Invalid amount".to_string());
                }
            }
            is_loading.set(false);
        });
    };

    rsx! {
        div {
            class: "max-w-md mx-auto bg-white rounded-lg shadow-md p-6",
            h2 {
                class: "text-2xl font-bold mb-6 text-gray-800",
                "Add New Expense"
            }

            div {
                class: "space-y-4",

                // Amount field
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-2",
                        "Amount"
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "number",
                        step: "0.01",
                        placeholder: "0.00",
                        value: "{amount}",
                        oninput: move |e| amount.set(e.value())
                    }
                }

                // Category field
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-2",
                        "Category"
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "text",
                        placeholder: "e.g., Food, Transport, Entertainment",
                        value: "{category}",
                        oninput: move |e| category.set(e.value())
                    }
                }

                // Submit button
                button {
                    class: "w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50",
                    disabled: is_loading(),
                    onclick: submit_handler,
                    if is_loading() {
                        "Adding..."
                    } else {
                        "Add Expense"
                    }
                }

                // Message display
                if !message().is_empty() {
                    div {
                        class: if message().contains("Error") { "text-red-600" } else { "text-green-600" },
                        "{message}"
                    }
                }
            }
        }
    }
}
