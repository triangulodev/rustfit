use sqlx::SqlitePool;
use std::sync::Arc;

pub mod account;

pub type DynStore = Arc<dyn StoreTrait + Send + Sync>;

#[derive(Clone)]
pub struct Store {
    pub pool: SqlitePool,
}

pub trait StoreTrait {
    fn account(&self) -> account::DynAccountCtrl;
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl StoreTrait for Store {
    fn account(&self) -> account::DynAccountCtrl {
        Arc::new(account::AccountController::new(self.pool.clone())) as account::DynAccountCtrl
    }
}
