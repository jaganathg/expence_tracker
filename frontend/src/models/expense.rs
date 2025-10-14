use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub amount: f64,
    pub category: String,
    pub date: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateExpenseRequest {
    #[validate(range(min = 0.01))]
    pub amount: f64,
    #[validate(length(min = 1, max = 50))]
    pub category: String,
}
