
use axum::Router;
use axum::routing::{delete, get, post, put};
use tower_http::trace::TraceLayer;
pub(crate) mod dose;
pub(crate) mod medication;
pub(crate) mod reminder;
pub(crate) mod note;
pub(crate) mod store;
pub(crate) mod uom;

use crate::api::ApiContext;

pub(crate) fn dose_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/dose", post(dose::create_dose))
    .route("/doseform", post(dose::create_dose_form))
    .route("/dose/:id", get(dose::read_dose))
    .route("/dose", put(dose::update_dose))
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

pub(crate) fn note_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/note", post(note::create_note))
    .route("/note/:id", get(note::read_note))
    .route("/note/:id", put(note::update_note))
    .route("/note/:id", delete(note::delete_note))
    .route("/note", get(note::list_notes))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

pub(crate) fn reminder_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/reminder", post(reminder::create_reminder))
    .route("/reminder_form", post(reminder::create_reminder_form))
    .route("/reminder/:id", get(reminder::read_reminder))
    .route("/reminder/:id", put(reminder::update_reminder))
    .route("/reminder/:id", delete(reminder::delete_reminder))
    .route("/reminders", get(reminder::list_reminders))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

pub(crate) fn store_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/store", post(store::create_store))
    .route("/store/:id", get(store::read_store))
    .route("/store/:id", put(store::update_store))
    .route("/store/:id", delete(store::delete_store))
    .route("/store", get(store::list_stores))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

pub(crate) fn uom_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/uom", post(uom::create_uom))
    .route("/uom/:id", get(uom::read_uom))
    // .route("/uom", get(uom::read_body_uom))
    .route("/uom", put(uom::update_uom))
    .route("/uom/:id", delete(uom::delete_uom))
    .route("/uoms", get(uom::list_uoms))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}
