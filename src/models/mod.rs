use sqlx::SqlitePool;
use std::sync::Arc;

pub mod account;
pub mod account_session;

pub type DynStore = Arc<dyn StoreTrait + Send + Sync>;

#[derive(Clone)]
pub struct Store {
    pub pool: SqlitePool,
}

pub trait StoreTrait {
    fn account(&self) -> account::DynAccountCtrl;
    fn account_session(&self) -> account_session::DynAccountSessionCtrl;
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl StoreTrait for Store {
    fn account(&self) -> account::DynAccountCtrl {
        Arc::new(account::AccountController::new(
            self.pool.clone(),
            self.account_session().clone(),
        )) as account::DynAccountCtrl
    }

    fn account_session(&self) -> account_session::DynAccountSessionCtrl {
        Arc::new(account_session::AccountSessionController::new(
            self.pool.clone(),
        )) as account_session::DynAccountSessionCtrl
    }
}
