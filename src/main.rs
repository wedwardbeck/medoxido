<<<<<<< Updated upstream
mod error;
mod medication;

use axum::routing::{delete, get, post, put};
use axum::{Router, Server};
use std::net::SocketAddr;
// use surrealdb::engine::any::File;
use surrealdb::engine::local::{File};

// use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;


=======
#![allow(unused)]
use clap::Parser;
use surrealdb::engine::remote::ws::{ Ws };
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::File;
use medoxido::config::{Config};
use medoxido::api;
>>>>>>> Stashed changes

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<File>("tempsDB").await?;
    db.use_ns("temps").use_db("temps").await?;



	// let db = Surreal::new::<Ws>("localhost:8000").await?;

<<<<<<< Updated upstream
	db.signin(Root {
		username: "root",
		password: "root",
	})
	.await?;

	db.use_ns("temps").use_db("temps").await?;
=======
    // let db = Surreal::new::<Ws>(db_address).await?;
    let db = Surreal::new::<File>("../temps.db").await?;

    // db.signin(Root {
    //     username: &config.db_user,
    //     password: &config.db_password,
    // })
    // .await?;
>>>>>>> Stashed changes

	let app = Router::new()
		.route("/medication/:id", post(medication::create))
		.route("/medication/:id", get(medication::read))
		.route("/medication/:id", put(medication::update))
		.route("/medication/:id", delete(medication::delete))
		.route("/medications", get(medication::list))
		.with_state(db);

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

	Server::bind(&addr).serve(app.into_make_service()).await?;

	Ok(())
}
