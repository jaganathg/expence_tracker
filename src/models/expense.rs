use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_validate_expense_cretion() {
        let expense = Expense::new(25.50, "Groceries".to_string());

        assert!(expense.amount > 0.0);
        assert!(!expense.category.is_empty());
        assert!(!expense.id.to_string().is_empty());
    }

    #[test]
    fn test_valid_create_expense_request() {
        let request = CreateExpenseRequest {
            amount: 25.50,
            category: "Groceries".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_invalid_amount_negative() {
        let request = CreateExpenseRequest {
            amount: -10.0,
            category: "Groceries".to_string(),
        };

        assert!(request.validate().is_err());
        let errors = request.validate().unwrap_err();
        assert!(errors.to_string().contains("Amount must be greater than 0"));
    }

    #[test]
    fn test_invalid_amount_zero() {
        let request = CreateExpenseRequest {
            amount: 0.0,
            category: "Groceries".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_invalid_category_empty() {
        let request = CreateExpenseRequest {
            amount: 25.50,
            category: "".to_string(),
        };

        assert!(request.validate().is_err());
        let errors = request.validate().unwrap_err();
        assert!(
            errors
                .to_string()
                .contains("Category must be between 1 and 50 characters")
        );
    }

    #[test]
    fn test_invalid_category_too_long() {
        let request = CreateExpenseRequest {
            amount: 25.50,
            category: "a".repeat(51),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_valid_category_boundary() {
        let request = CreateExpenseRequest {
            amount: 25.50,
            category: "a".repeat(50),
        };

        assert!(request.validate().is_ok());
    }
}
