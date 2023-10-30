use std::sync::Arc;

use crate::http::{Error, Result};
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

#[derive(serde::Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct Account {
    id: Uuid,
    email: String,
    name: String,
    password_hash: String,
    inserted_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AccountDTO {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub inserted_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AccountWithTokenDTO {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub token: String,
    pub inserted_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AccountWithPasswordHashDTO {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
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
    async fn create_account(&self, new_account: NewAccount) -> Result<AccountDTO>;
    async fn login_account(&self, credentials: LoginCredentials) -> Result<AccountWithTokenDTO>;

    async fn get_account_by_email(&self, email: String) -> Result<AccountWithPasswordHashDTO>;
}

#[async_trait]
impl AccountCtrlTrait for AccountController {
    async fn create_account(&self, new_account: NewAccount) -> Result<AccountDTO> {
        let id = uuid::Uuid::new_v4();
        let password_hash = Account::hash_password(new_account.password.clone()).await?;
        let inserted_at = time::OffsetDateTime::now_utc();

        let account = sqlx::query_as!(
            AccountDTO,
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

    async fn login_account(&self, login_account: LoginCredentials) -> Result<AccountWithTokenDTO> {
        let account = &self.get_account_by_email(login_account.email).await?;

        Account::verify_password(login_account.password, account.password_hash.clone()).await?;

        // FIXME: token is not supposed to be a random string, should actually be safe and attached
        // to a session.
        let token = rand::random::<u32>().to_string();
        // create token
        // merge token to account

        Ok(AccountWithTokenDTO {
            id: account.id,
            name: account.name.clone(),
            email: account.email.clone(),
            token,
            inserted_at: account.inserted_at,
            updated_at: account.updated_at,
        })
    }

    async fn get_account_by_email(&self, email: String) -> Result<AccountWithPasswordHashDTO> {
        let account = sqlx::query_as!(
            AccountWithPasswordHashDTO,
            r#"select
                id as "id: Uuid", name, email, password_hash,
                inserted_at as "inserted_at: OffsetDateTime", updated_at as "updated_at: OffsetDateTime"
            from accounts
            where email = $1"#,
                  email
            ).fetch_one(&self.pool).await?;

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

    pub(crate) async fn verify_password(password: String, password_hash: String) -> Result<()> {
        tokio::task::spawn_blocking(move || -> Result<()> {
            let hash = PasswordHash::new(&password_hash)
                .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

            hash.verify_password(&[&Argon2::default()], password)
                .map_err(|e| match e {
                    argon2::password_hash::Error::Password => Error::Unauthorized,
                    _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
                })
        })
        .await
        .context("panic in verifying password hash")?
    }
}
