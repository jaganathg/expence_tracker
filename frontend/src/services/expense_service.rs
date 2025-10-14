use crate::models::{CreateExpenseRequest, Expense};
use thiserror::Error;

#[allow(dead_code)]
#[allow(unused_variables)]
const API_BASE_URL: &str = "http://localhost:3000";

#[derive(Error, Debug)]
pub enum ExpenseServiceError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[allow(dead_code)]
pub struct ExpenseService;

#[allow(dead_code)]
impl ExpenseService {
    pub async fn get_all_expenses() -> Result<Vec<Expense>, ExpenseServiceError> {
        let response = reqwest::get(&format!("{}/expenses", API_BASE_URL)).await?;

        if !response.status().is_success() {
            return Err(ExpenseServiceError::Validation(format!(
                "Failed to get expenses: {}",
                response.status()
            )));
        }

        let expenses: Vec<Expense> = response.json().await?;
        Ok(expenses)
    }

    pub async fn add_expense(request: CreateExpenseRequest) -> Result<(), ExpenseServiceError> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/expenses", API_BASE_URL))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ExpenseServiceError::Validation(format!(
                "Failed to add expense: {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn get_highest_expense() -> Result<Option<Expense>, ExpenseServiceError> {
        let response = reqwest::get(&format!("{}/expenses/highest", API_BASE_URL)).await?;

        if response.status().is_success() {
            let expense: Expense = response.json().await?;
            Ok(Some(expense))
        } else {
            Ok(None)
        }
    }
}
