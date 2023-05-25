
use axum::Router;
use axum::routing::{delete, get, post, put};
use tower_http::trace::TraceLayer;
pub(crate) mod dose;
pub(crate) mod medication;

use crate::api::ApiContext;

pub(crate) fn dose_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/dose", post(dose::create_dose))
    .route("/dose/:id", get(dose::read_dose))
    .route("/dose/:id", put(dose::update_dose))
    .route("/dose/:id", delete(dose::delete_dose))
    .route("/doses", get(dose::list_doses))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

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
