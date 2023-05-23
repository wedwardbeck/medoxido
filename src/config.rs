/// The configuration parameters for the application.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the SurrealDB connection.
    #[clap(long, env)]
    pub db_host: String,

    /// The port for the SurrealDB connection.
    #[clap(long, env)]
    pub db_port: String,

    /// The user for the SurrealDB connection.
    #[clap(long, env)]
    pub db_user: String,

    /// The password for the SurrealDB connection.
    #[clap(long, env)]
    pub db_password: String,

    /// The namespace for the SurrealDB connection.
    #[clap(long, env)]
    pub db_namespace: String,

    /// The database name for the SurrealDB connection.
    #[clap(long, env)]
    pub db_name: String,

    /// The HMAC signing and verification key used for login tokens (JWTs).

    #[clap(long, env)]
    pub hmac_key: String,
}
