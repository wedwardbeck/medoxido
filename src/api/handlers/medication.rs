//! Medication CRUD
//!
//! Using SurrealDB to create a CRUD API for Medication Tracking

use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::api::error::Error;
use crate::api::ApiContext;

const MEDICATION: &str = "medication";

/// Struct Medication for CRUD operations using the name field.
#[derive(Serialize, Deserialize)]
pub struct Medication {
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
    Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.create(MEDICATION).content(medication).await?;
    Ok(Json(medication))
}

pub(crate) async fn read_med(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.select((MEDICATION, &*id)).await?;
    Ok(Json(medication))
}

/// This function is meant to take a complex id to return one record.
/// The id is composed of the name and timestamp of the record creation.
/// Example: ['ibuprofen', '2021-01-01T00:00:00.000Z']
pub(crate) async fn read_body(
    ctx: State<ApiContext>,
    Json(medication_id): Json<MedicationId>,
) -> Result<Json<Option<Medication>>, Error> {
    let id = medication_id.get_string();
    let query = format!("SELECT * FROM {};", id);
    let mut sql = ctx.db.query(query).await?;
    let medication: Option<Medication> = sql.take(0)?;
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
