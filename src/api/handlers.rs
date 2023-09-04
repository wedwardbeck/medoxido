
use axum::Router;
use axum::routing::{delete, get, patch, post, put};
use tower_http::trace::TraceLayer;
pub(crate) mod dose;
pub(crate) mod medication;
pub(crate) mod reminder;
pub(crate) mod note;
pub(crate) mod store;
pub(crate) mod uom;
// pub(crate) mod user;

use crate::api::ApiContext;

/// Creates a router for the Dose API with the following routes:
/// - POST /dose - creates a new dose
/// - GET /dose/:id - reads a dose with the given ID
/// - PUT /dose/:id - updates a dose with the given ID
/// - DELETE /dose/:id - deletes a dose with the given ID
/// - GET /doses - lists all doses
/// The router is also layered with a TraceLayer for HTTP tracing and is initialized with the given ApiContext state.
pub(crate) fn dose_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/doses", post(dose::create_dose))
    .route("/doses/:id", get(dose::read_dose))
    .route("/doses/:id", put(dose::update_dose))
    .route("/doses/:id", delete(dose::delete_dose))
    .route("/doses", get(dose::list_doses_for_user))
    .route("/doses/medications", get(dose::list_doses_for_medication))
    .route("/doses/stores", get(dose::list_doses_for_store))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

/// Returns a router for medication API endpoints with the specified `api_context`
///
/// # Arguments
///
/// * `api_context` - An instance of `ApiContext` containing the necessary context for the API
///
/// # Returns
///
/// A `Router` instance with the following routes:
///
/// * POST `/medications` - Creates a new medication
/// * GET `/medications/:id` - Retrieves a medication by ID
/// * PUT `/medications/:id` - Updates a medication by ID
/// * DELETE `/medications/:id` - Deletes a medication by ID
/// * GET `/medications` - Retrieves a list of all medications
///
/// The router is also
pub(crate) fn medication_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/medications", post(medication::create_med))
    .route("/medications/:id", get(medication::read_med))
    .route("/medications/:id", put(medication::update_med))
    .route("/medications/deactivate", patch(medication::deactivate_med))
    .route("/medications/:id", delete(medication::delete_med))
    .route("/medications", get(medication::list_all_meds))
    .route("/medications/status", get(medication::list_user_meds_by_status))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

/// Returns a router for handling note-related HTTP requests
///
/// # Arguments
///
/// * `api_context` - An instance of `ApiContext` containing the necessary state for handling requests
///
/// # Returns
///
/// A `Router` instance with routes for creating, reading, updating, and deleting notes, as well as listing all notes.
/// The router is also layered with `TraceLayer` for logging HTTP requests and responses.
pub(crate) fn note_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/notes", post(note::create_note))
    .route("/notes/:id", get(note::read_note))
    .route("/notes/:id", put(note::update_note))
    .route("/notes/:id", delete(note::delete_note))
    .route("/notes", get(note::list_notes))
    .route("/notes/dose", get(note::list_all_dose_notes))
    .route("/notes/dose/:id", get(note::list_notes_for_dose))
    .route("/notes/meds", get(note::list_all_medication_notes))
    .route("/notes/med/:id", get(note::list_notes_for_medication))
    .route("/notes/store", get(note::list_all_store_notes))
    .route("/notes/store/:id", get(note::list_notes_for_store))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

/// Creates a router for the reminder API endpoints and applies a trace layer for HTTP requests
///
/// # Arguments
///
/// * `api_context` - An instance of `ApiContext` containing the necessary context for the API
///
/// # Returns
///
/// A `Router` instance with the following routes:
///
/// * POST /reminder - Create a new reminder
/// * GET /reminder/:id - Read a reminder by ID
/// * PUT /reminder/:id - Update a reminder by ID
/// * PATCH /reminder/:id - Deactivate a reminder by ID
/// * DELETE /reminder/:id - Delete a reminder by ID
/// * GET /reminders - List all reminders
/// *
pub(crate) fn reminder_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/reminders", post(reminder::create_reminder))
    .route("/reminders/:id", get(reminder::read_reminder))
    .route("/reminders/:id", put(reminder::update_reminder))
    .route("/reminders/:id", patch(reminder::deactivate_reminder))
    .route("/reminders/:id", delete(reminder::delete_reminder))
    .route("/reminders/", get(reminder::list_reminders))
    .route("/activereminders/", get(reminder::list_active_reminders))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

/// Creates a new router for the store API endpoints and adds the necessary routes and middleware
///
/// # Arguments
///
/// * `api_context` - The context object containing the necessary dependencies for the API
///
/// # Returns
///
/// The router object with the store API endpoints and middleware added
pub(crate) fn store_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/stores", post(store::create_store))
    .route("/stores/:id", get(store::read_store))
    .route("/stores/:id", put(store::update_store))
    .route("/stores/:id", patch(store::deactivate_store))
    .route("/stores/:id", delete(store::delete_store))
    .route("/stores", get(store::list_stores))
    .route("/stores/med", get(store::list_stores_for_medication))
    .route("/stores/all", get(store::list_all_stores_for_medication))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

/// Returns a router for the UOM API with the following routes:
/// - POST /uom - creates a new UOM
/// - GET /uom/:id - reads a UOM by ID
/// - PUT /uom/:id - updates a UOM by ID
/// - DELETE /uom/:id - deletes a UOM by ID
/// - GET /uoms - lists all UOMs
/// The router is also layered with a TraceLayer for HTTP tracing and is initialized with the provided `api_context`.
pub(crate) fn uom_router(api_context: ApiContext) -> Router<ApiContext> {
    Router::new()
    .route("/uoms", post(uom::create_uom))
    .route("/uoms/:id", get(uom::read_uom))
    .route("/uoms/:id", put(uom::update_uom))
    .route("/uoms/:id", delete(uom::delete_uom))
    .route("/uoms", get(uom::list_uoms))
    .layer(TraceLayer::new_for_http())
    .with_state(api_context)
}

// pub(crate) fn user_router(api_context: ApiContext) -> Router<ApiContext> {
//     Router::new()
//     .route("/users", post(user::create_user))
//     .route("/users/login", post(user::login_user))
//     .route("/user", get(user::get_current_user).put(user::update_user))
//     .layer(TraceLayer::new_for_http())
//     .with_state(api_context)
// }

//TODO: Deal with any table index constraints in the client side ahead of time.
//TODO: Revisit this approach of prechecks when we switch to embedded since it should have better error responses
