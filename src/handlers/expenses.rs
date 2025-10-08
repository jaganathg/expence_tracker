use crate::error::AppError;
use crate::models::expense::{CreateExpenseRequest, Expense};
use crate::services::expense_service::ExpenseService;
use anyhow::Result;
use axum::{extract::State, response::Json};
use validator::Validate;

pub async fn add_expense(
    State(service): State<ExpenseService>,
    Json(request): Json<CreateExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    println!("Validating request: amount= {}, category= {}", request.amount, request.category);
    
    request.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let expense = service.add_expense(request).await?;
    Ok(Json(expense))
}

pub async fn get_all_expenses(
    State(service): State<ExpenseService>,
) -> Result<Json<Vec<Expense>>, AppError> {
    let expenses = service.get_all_expenses().await?;
    Ok(Json(expenses))
}

pub async fn get_highest_expense(
    State(service): State<ExpenseService>,
) -> Result<Json<Expense>, AppError> {
    match service.get_highest_expense().await? {
        Some(expense) => Ok(Json(expense)),
        None => Err(AppError::NotFound),
    }
}
