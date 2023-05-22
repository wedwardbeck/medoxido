use axum::extract::{Path, State};
use axum::routing::{get, post, put, delete};
use axum::{ Router};

use crate::api::ApiContext;

// use self::medication::create;
pub mod medication;


pub(crate) fn router() -> Router<ApiContext> {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
    .route("/medication", post(medication::create))
    .route("/medication/:id", get(medication::read))
    .route("/medication", get(medication::read_body))
    .route("/medication/:id", put(medication::update))
    .route("/medication/:id", delete(medication::delete))
    .route("/medications", get(medication::list))
    // .with_state(db);
}
