
use clap::Parser;
use surrealdb::engine::remote::ws::{ Ws };
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// mod api;
use medoxido::config::{Config};
use medoxido::api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize the logger.
    env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // let db = Surreal::new::<Ws>("localhost:8000").await?;
    let db_address = format!("{}:{}", &config.db_host, &config.db_port);

    let db = Surreal::new::<Ws>(db_address).await?;

    db.signin(Root {
        username: &config.db_user,
        password: &config.db_password,
    })
    .await?;

    db.use_ns(&config.db_namespace).use_db(&config.db_name).await?;

    api::serve(config, db).await?;

    Ok(())
}
