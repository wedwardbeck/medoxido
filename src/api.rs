use crate::config::Config;
use anyhow::Context;
use axum::Router;
use surrealdb::{Surreal, engine::remote::ws::Client};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

// Utility modules.

/// Defines a common error type to use for all request handlers, compliant with the Realworld spec.
mod error;

/// Contains definitions for application-specific parameters to handler functions,
/// such as `AuthUser` which checks for the `Authorization: Token <token>` header in the request,
/// verifies `<token>` as a JWT and checks the signature,
/// then deserializes the information it contains.
// mod extractor;

// API modules.
pub mod v1;

pub use error::{Error};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

/// The core type through which handler functions can access common API state.
///

#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: Surreal<Client>,
}

pub async fn serve(config: Config, db: Surreal<Client>) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    // We use 8080 as our default HTTP server port.
    //
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router(api_context: ApiContext) -> Router {
    // This is the order that the modules were authored in.
    Router::new()
        // .merge(users::router())
        .merge(v1::router())
        .merge(articles::router())
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
