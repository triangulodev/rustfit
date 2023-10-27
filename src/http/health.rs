use crate::http::{ApiContext, Result};
use axum::routing::get;
use axum::Router;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/health", get(get_health))
}

async fn get_health() -> Result<&'static str> {
    Ok("healthy")
}
