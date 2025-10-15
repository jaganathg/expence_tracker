use crate::models::expense::{CreateExpenseRequest, Expense};
use anyhow::Result;
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct ExpenseService {
    pool: SqlitePool,
}

impl ExpenseService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_expense(&self, request: CreateExpenseRequest) -> Result<Expense> {
        let expense = Expense::new(request.amount, request.category);

        sqlx::query("INSERT INTO expenses (id, amount, category, date) VALUES (?, ?, ?, ?)")
            .bind(expense.id.to_string())
            .bind(expense.amount)
            .bind(&expense.category)
            .bind(expense.date)
            .execute(&self.pool)
            .await?;

        Ok(expense)
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>> {
        let rows =
            sqlx::query("SELECT id, amount, category, date FROM expenses ORDER BY date DESC")
                .fetch_all(&self.pool)
                .await?;

        let expenses = rows
            .into_iter()
            .map(|row| Expense {
                id: uuid::Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                amount: row.get("amount"),
                category: row.get("category"),
                date: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("date"))
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
            .collect();

        Ok(expenses)
    }

    pub async fn get_highest_expense(&self) -> Result<Option<Expense>> {
        let rows = sqlx::query(
            "SELECT id, amount, category, date FROM expenses ORDER BY amount DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = rows {
            let expense = Expense {
                id: uuid::Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                amount: row.get("amount"),
                category: row.get("category"),
                date: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("date"))
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            };
            Ok(Some(expense))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::expense::CreateExpenseRequest;
    use sqlx::SqlitePool;

    async fn create_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        sqlx::query(
            r#"
                CREATE TABLE expenses (
                    id TEXT PRIMARY KEY,
                    amount REAL NOT NULL,
                    category TEXT NOT NULL,
                    date TEXT NOT NULL
                )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_add_expense() {
        let pool = create_test_pool().await;
        let service = ExpenseService::new(pool);

        let request = CreateExpenseRequest {
            amount: 25.50,
            category: "Groceries".to_string(),
        };

        let expense = service.add_expense(request).await.unwrap();

        assert_eq!(expense.amount, 25.50);
        assert_eq!(expense.category, "Groceries");
        assert!(!expense.id.to_string().is_empty())
    }

    #[tokio::test]
    async fn test_get_all_expenses_with_data() {
        let pool = create_test_pool().await;
        let service = ExpenseService::new(pool);

        let request1 = CreateExpenseRequest {
            amount: 15.50,
            category: "Transport".to_string(),
        };
        let request2 = CreateExpenseRequest {
            amount: 25.50,
            category: "Groceries".to_string(),
        };

        service.add_expense(request1).await.unwrap();
        service.add_expense(request2).await.unwrap();

        let expense = service.get_all_expenses().await.unwrap();

        assert_eq!(expense.len(), 2);
        assert!(expense.iter().any(|e| e.amount == 15.50));
        assert!(expense.iter().any(|e| e.amount == 25.50));
    }

    #[tokio::test]
    async fn test_get_highest_expense_empty() {
        let pool = create_test_pool().await;
        let service = ExpenseService::new(pool);

        let highest = service.get_highest_expense().await.unwrap();

        assert!(highest.is_none());
    }

    #[tokio::test]
    async fn test_get_highest_expense_with_data() {
        let pool = create_test_pool().await;
        let service = ExpenseService::new(pool);

        let request1 = CreateExpenseRequest {
            amount: 15.50,
            category: "Transport".to_string(),
        };
        let request2 = CreateExpenseRequest {
            amount: 25.50,
            category: "Groceries".to_string(),
        };
        let request3 = CreateExpenseRequest {
            amount: 5.50,
            category: "Entertainment".to_string(),
        };

        service.add_expense(request1).await.unwrap();
        service.add_expense(request2).await.unwrap();
        service.add_expense(request3).await.unwrap();

        let highest = service.get_highest_expense().await.unwrap();

        assert!(highest.is_some());
        assert_eq!(highest.as_ref().unwrap().amount, 25.50);
        assert_eq!(highest.as_ref().unwrap().category, "Groceries");
    }

    #[tokio::test]
    async fn test_get_highest_expense_single_item() {
        let pool = create_test_pool().await;
        let service = ExpenseService::new(pool);

        let request = CreateExpenseRequest {
            amount: 10.05,
            category: "Books".to_string(),
        };

        service.add_expense(request).await.unwrap();

        let highest = service.get_highest_expense().await.unwrap();

        assert!(highest.is_some());
        assert_eq!(highest.as_ref().unwrap().amount, 10.05);
        assert_eq!(highest.as_ref().unwrap().category, "Books");
    }
}
