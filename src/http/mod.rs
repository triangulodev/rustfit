mod error;

pub mod accounts;

pub mod server;
pub use server::serve;

pub mod api_context;
pub use api_context::ApiContext;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;
