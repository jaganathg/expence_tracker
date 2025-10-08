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
