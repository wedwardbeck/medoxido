//! Medication CRUD
//!
//! Using SurrealDB to create a CRUD API for Medication Tracking

use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::Thing;

use crate::api::error::Error;
use crate::api::ApiContext;

const MEDICATION: &str = "medication";

/// Struct Medication for CRUD operations using the name field.
#[derive(Serialize, Deserialize)]
pub struct Medication {
    id: Option<Thing>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMedication {
    name: String,
}

/// Struct Medication for CRUD operations using the id.
/// Used for the read_body function, instead of a path parameter.
#[derive(Serialize, Deserialize, Default)]
pub struct MedicationId {
    id: String,
}

impl MedicationId {
    fn get_string(&self) -> String {
        self.id.to_string()
    }
}

pub(crate) async fn create_med(
    ctx: State<ApiContext>,
    Json(medication): Json<CreateMedication>,
) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.create(MEDICATION).content(medication).await?;
    Ok(Json(medication))
}

/// Reads a medication from the database with the given ID and returns it as JSON
///
/// # Arguments
///
/// * `ctx` - The API context containing the database connection
/// * `id` - The ID of the medication to read
///
/// # Returns
///
/// A `Json` object containing the medication with the given ID, or `None` if it does not exist in the database.
/// If an error occurs while accessing the database, an `Error` is returned.
pub(crate) async fn read_med(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.select((MEDICATION, &*id)).await?;
    Ok(Json(medication))
}

/// Function to read id from body.  Was Complex ID, changed to simplify using
/// SurrealQL select function.
pub(crate) async fn read_body(
    ctx: State<ApiContext>,
    Json(medication_id): Json<MedicationId>,
) -> Result<Json<Option<Medication>>, Error> {
    let id = medication_id.get_string();
    let medication = ctx.db.select((MEDICATION, id)).await?;
    Ok(Json(medication))
}

pub(crate) async fn update_med(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.update((MEDICATION, &*id)).content(medication).await?;
    Ok(Json(medication))
}

pub(crate) async fn delete_med(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.delete((MEDICATION, &*id)).await?;
    Ok(Json(medication))
}

pub(crate) async fn list_meds(ctx: State<ApiContext>,) -> Result<Json<Vec<Medication>>, Error> {
    let medications = ctx.db.select(MEDICATION).await?;
    Ok(Json(medications))
}
