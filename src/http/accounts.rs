use crate::http::{ApiContext, Result};
use crate::models::account::AccountDTO;
use crate::models::account::AccountWithTokenDTO;
use crate::models::account::LoginCredentials;
use crate::models::account::NewAccount;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/accounts", post(create_account))
        .route("/api/login", post(login_account))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AccountBody<T> {
    account: T,
}

async fn create_account(
    ctx: State<ApiContext>,
    Json(req): Json<AccountBody<NewAccount>>,
) -> Result<Json<AccountBody<AccountDTO>>> {
    let account = ctx.store.account().create_account(req.account).await?;

    Ok(Json(AccountBody { account }))
}

async fn login_account(
    ctx: State<ApiContext>,
    Json(credentials): Json<LoginCredentials>,
) -> Result<Json<AccountBody<AccountWithTokenDTO>>> {
    let account = ctx.store.account().login_account(credentials).await?;

    Ok(Json(AccountBody { account }))
}
