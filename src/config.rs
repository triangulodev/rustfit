#[derive(clap::Parser)]
#[cfg_attr(test, derive(Default))]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub port: u16,
}
