// mod error;
// mod medication;

use anyhow::Context;
use clap::Parser;

use medoxido::config::Config;
use axum::routing::{delete, get, post, put};
use axum::{Router, Server};
use std::net::SocketAddr;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> anyhow::Result<()>  {
// async fn mai÷n() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize the logger.
    env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // TODO: refactor this to use a config file and env variables.
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("temps").use_db("temps").await?;

    let app = Router::new()
        // .route("/medication/:id", post(medication::create))
        .route("/medication", post(medication::create))
        .route("/medication/:id", get(medication::read))
        .route("/medication", get(medication::read_body))
        .route("/medication/:id", put(medication::update))
        .route("/medication/:id", delete(medication::delete))
        .route("/medications", get(medication::list))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    Server::bind(&addr).serve(app.into_make_service()).await?;

    // Finally, we spin up our API.
    core::serve(config, db).await?;

    Ok(())
}
