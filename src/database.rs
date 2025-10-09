use anyhow::Result;
use sqlx::SqlitePool;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS expenses (
            id TEXT PRIMARY KEY,
            amount REAL NOT NULL,
            category TEXT NOT NULL,
            date TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    println!("Database created successfully");
    Ok(pool)
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_create_pool_in_memory() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        let result = sqlx::query("SELECT 1 as test")
            .fetch_one(&pool)
            .await
            .unwrap();

        let test_value: i32 = result.get("test");
        assert_eq!(test_value, 1);
    }

    #[tokio::test]
    async fn test_table_creation() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND  name='expenses'")
            .fetch_optional(&pool)
            .await
            .unwrap();

        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_table_structure() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        let test_id = "test_uuid";
        let test_amount = 10.0;
        let test_category = "Test";
        let test_date = "2025-01-01T00:00:00Z";

        sqlx::query("INSERT INTO expenses (id, amount, category, date) VALUES (?, ?, ?, ?)")
            .bind(test_id)
            .bind(test_amount)
            .bind(test_category)
            .bind(test_date)
            .execute(&pool)
            .await
            .unwrap();

        let result = sqlx::query("SELECT * FROM expenses WHERE id = ?")
            .bind(test_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(result.get::<String, _>("id"), test_id);
        assert_eq!(result.get::<f64, _>("amount"), test_amount);
        assert_eq!(result.get::<String, _>("category"), test_category);
            
    }
}
