use std::sync::Arc;

use crate::http::Result;
use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use async_trait::async_trait;

use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewAccount {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub inserted_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AccountBody {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub inserted_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Clone)]
pub struct AccountController {
    pool: SqlitePool,
}

impl AccountController {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

pub type DynAccountCtrl = Arc<dyn AccountCtrlTrait + Send + Sync>;
#[async_trait]
pub trait AccountCtrlTrait {
    async fn create_account(&self, new_account: NewAccount) -> Result<AccountBody>;
}

#[async_trait]
impl AccountCtrlTrait for AccountController {
    async fn create_account(&self, new_account: NewAccount) -> Result<AccountBody> {
        let id = uuid::Uuid::new_v4();
        let password_hash = Account::hash_password(new_account.password.clone()).await?;
        let inserted_at = time::OffsetDateTime::now_utc();

        let account = sqlx::query_as!(
            AccountBody,
            r#"insert into "accounts" (
                id, name, email, password_hash,
                inserted_at, updated_at
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6
            ) returning
                id as "id: Uuid", name, email,
                inserted_at as "inserted_at: OffsetDateTime", updated_at as "updated_at: OffsetDateTime""#,
            id,
            new_account.name,
            new_account.email,
            password_hash,
            inserted_at,
            inserted_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }
}

impl Account {
    pub(crate) async fn hash_password(password: String) -> Result<String> {
        // Argon2 hashing is designed to be computationally intensive,
        // so we need to do this on a blocking thread.
        tokio::task::spawn_blocking(move || -> Result<String> {
            let salt = SaltString::generate(rand::thread_rng());
            Ok(PasswordHash::generate(Argon2::default(), password, &salt)
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string())
        })
        .await
        .context("panic in generating password hash")?
    }
}
