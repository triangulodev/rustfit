use clap::Parser;
use rustfit::config::Config;
use rustfit::http;
use sqlx::sqlite::SqlitePoolOptions;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let db = SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await?;

    info!("connected to db");
    sqlx::migrate!().run(&db).await?;
    info!("ran migrations");

    http::serve(config, db).await?;
    info!("serving application");

    Ok(())
}
