use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod database;
mod error;
mod handlers;
mod models;
mod services;

use handlers::expenses::{add_expense, get_all_expenses, get_highest_expense};
use services::expense_service::ExpenseService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "sqlite:./expenses.db";
    let pool = database::create_pool(database_url).await?;

    let expense_service = ExpenseService::new(pool);

    let app = Router::new()
        .route("/expenses", post(add_expense))
        .route("/expenses", get(get_all_expenses))
        .route("/expenses/highest", get(get_highest_expense))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .with_state(expense_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Expense Tracker API is running on  http://{}", addr);
    println!("Database: {}", database_url);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
