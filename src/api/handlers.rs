
use axum::Router;
use axum::routing::{delete, get, post, put};
use tower_http::trace::TraceLayer;
pub(crate) mod medication;

use crate::api::ApiContext;

pub(crate) fn medication_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    // .route("/medication/:id", post(medication::create_med))
    .route("/medication", post(medication::create_med))
    .route("/medication/:id", get(medication::read_med))
    .route("/medication", get(medication::read_body))
    .route("/medication/:id", put(medication::update_med))
    .route("/medication/:id", delete(medication::delete_med))
    .route("/medications", get(medication::list_meds))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}
