use axum::Router;
use anyhow::Context;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tower_http::trace::TraceLayer;

use crate::config::Config;
pub mod handlers;
pub mod error;
pub use error::Error;
// pub mod extractor;

pub type Result<T, E = Error> = std::result::Result<T, E>;

// TODO: Remove this when we implement auth and use config.
#[allow(unused)]
#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: Surreal<Client>,
}

/// Serves the API using the given configuration and database client
///
/// # Arguments
///
/// * `config` - A `Config` struct containing the configuration for the API
/// * `db` - A `Surreal<Client>` struct representing the database client
///
/// # Returns
///
/// Returns `Ok(())` if the server was successfully started, otherwise returns an `anyhow::Error`
///
// TODO: Alter the Surreal<Client> to Surreal<DB> for local file storage.
// TODO: adjust comments for change in ws to db local.
pub async fn serve(config: Config, db: Surreal<Client>) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

/// Creates a router for the API context and merges all the handlers for the different routes.
/// It also adds a trace layer for HTTP requests and sets the API context as the state of the router.
/// Returns the router.
fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .merge(handlers::dose_router(api_context.clone()))
        .merge(handlers::medication_router(api_context.clone()))
        .merge(handlers::reminder_router(api_context.clone()))
        .merge(handlers::note_router(api_context.clone()))
        .merge(handlers::store_router(api_context.clone()))
        .merge(handlers::uom_router(api_context.clone()))
        // .merge(handlers::user_router(api_context.clone()))
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
