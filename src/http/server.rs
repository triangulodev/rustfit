use crate::config::Config;
use crate::http::accounts;
use crate::http::health;
use crate::http::ApiContext;
use crate::models::DynStore;
use crate::models::Store;
use anyhow::Context;
use axum::Router;
use sqlx::SqlitePool;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub async fn serve(config: Config, db: SqlitePool) -> anyhow::Result<()> {
    let port = config.port;

    let api_context = ApiContext {
        config: Arc::new(config),
        store: Arc::new(Store::new(db.clone())) as DynStore,
    };
    let app = api_router(api_context);

    // Port is configured in .env
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    info!("addr {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router(api_context: ApiContext) -> Router {
    // This is the order that the modules were authored in.
    Router::new()
        .merge(accounts::router())
        .merge(health::router())
        .with_state(api_context)
}
