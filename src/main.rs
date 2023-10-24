use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct CreateAccount {
    email: String,
}

#[derive(Serialize)]
struct Account {
    id: u64,
    email: String,
}

async fn create_account(Json(payload): Json<CreateAccount>) -> impl IntoResponse {
    let account = Account {
        id: 1337,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(account))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/api/v1/accounts", post(create_account));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Rustfit"
}
