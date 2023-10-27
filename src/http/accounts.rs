use crate::http::{ApiContext, Result};
use crate::models::account::Account;
use crate::models::account::NewAccount;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/api/accounts", post(create_account))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AccountBody<T> {
    account: T,
}

async fn create_account(
    ctx: State<ApiContext>,
    Json(req): Json<AccountBody<NewAccount>>,
) -> Result<Json<AccountBody<Account>>> {
    let account = ctx.store.account().create_account(req.account).await?;

    Ok(Json(AccountBody { account }))
}
