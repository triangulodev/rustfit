use std::sync::Arc;

use crate::http::{Error, Result};
use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use async_trait::async_trait;

use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct AccountSession {
    id: Uuid,
    account_id: Uuid,
    expires_at: OffsetDateTime,
    active: i64,
    inserted_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AccountSessionDTO {
    pub id: Uuid,
    pub account_id: Uuid,
    pub expires_at: OffsetDateTime,
    pub active: i64,
    pub inserted_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(serde::Deserialize)]
pub struct AccountSessionCreate {
    pub account_id: Uuid,
    pub expires_at: OffsetDateTime,
}

#[derive(serde::Deserialize)]
pub struct AccountSessionDelete {
    pub id: Uuid,
}

#[derive(Clone)]
pub struct AccountSessionController {
    pool: SqlitePool,
}

impl AccountSessionController {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

pub type DynAccountSessionCtrl = Arc<dyn AccountSessionCtrlTrait + Send + Sync>;
#[async_trait]
pub trait AccountSessionCtrlTrait {
    async fn create_account_session(
        &self,
        account_session_create: AccountSessionCreate,
    ) -> Result<AccountSessionDTO>;
}

#[async_trait]
impl AccountSessionCtrlTrait for AccountSessionController {
    async fn create_account_session(
        &self,
        account_session_create: AccountSessionCreate,
    ) -> Result<AccountSessionDTO> {
        let id = uuid::Uuid::new_v4();
        let expires_at = time::OffsetDateTime::now_utc();
        let inserted_at = time::OffsetDateTime::now_utc();

        let account_session = sqlx::query_as!(AccountSessionDTO, r#"
            insert into "account_sessions" (
                id, account_id,
                expires_at, active,
                inserted_at,
                updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) returning
                id as "id: Uuid", account_id as "account_id: Uuid",
                expires_at as "expires_at: OffsetDateTime", active,
                inserted_at as "inserted_at: OffsetDateTime",
                updated_at as "updated_at: OffsetDateTime""#,
            id,
            account_session_create.account_id,
            expires_at,
            1,
            inserted_at,
            inserted_at
            )
            .fetch_one(&self.pool)
            .await?;

        Ok(account_session)
    }
}
