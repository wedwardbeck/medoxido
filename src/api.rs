
use axum::Router;
use anyhow::Context;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{ Client };
use std::{
    net::{Ipv4Addr, SocketAddr},
    // sync::Arc,
};
use tower_http::trace::TraceLayer;

pub mod handlers;
pub mod error;
pub use error::{Error};
// pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct ApiContext {
    db: Surreal<Client>,
}

pub async fn serve(db: Surreal<Client>) -> anyhow::Result<()> {
    let api_context = ApiContext {
        db,
    };

    let app = api_router(api_context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router(api_context: ApiContext) -> Router {
    // This is the order that the modules were authored in.
    Router::new()
        .merge(handlers::medication_router(api_context.clone()))
        // .merge(profiles::router())
        // .merge(articles::router())
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
