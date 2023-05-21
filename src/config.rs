/// The configuration parameters for the application.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Database connection.
    #[clap(long, env)]
    pub database_url: String,

    /// The HMAC signing and verification key used for login tokens (JWTs).

    #[clap(long, env)]
    pub hmac_key: String,
}
