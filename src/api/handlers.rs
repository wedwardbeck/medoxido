use crate::api::ApiContext;

use axum::Router;
use axum::routing::{delete, get, post, put};
use tower_http::trace::TraceLayer;
mod medication;

pub(crate) fn medication_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/medication/:id", post(medication::create))
    .route("/medication", post(medication::create))
    .route("/medication/:id", get(medication::read))
    .route("/medication", get(medication::read_body))
    .route("/medication/:id", put(medication::update))
    .route("/medication/:id", delete(medication::delete))
    .route("/medications", get(medication::list))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}
