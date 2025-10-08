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
