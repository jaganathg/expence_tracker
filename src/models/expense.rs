use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: Uuid,
    pub amount: f64,
    pub category: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateExpenseRequest {
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: f64,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Category must be between 1 and 50 characters"
    ))]
    pub category: String,
}

impl Expense {
    pub fn new(amount: f64, category: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            amount,
            category,
            date: Utc::now(),
        }
    }
}
