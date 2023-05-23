//! Medication CRUD
//!
//! Using SurrealDB to create a CRUD API for Medication Tracking

use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
// use chrono::Utc;

use crate::api::error::Error;
use crate::api::ApiContext;

const MEDICATION: &str = "medication";

/// Struct Medication for CRUD operations using the name field.
#[derive(Serialize, Deserialize)]
pub struct Medication {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MedicationRecord {
    id: String,
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
    // db: Db,
    // id: Path<String>,
    Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {

    // let current_utc_time = Utc::now();
    // let formatted_timestamp = format!("{}", current_utc_time.format("%FT%TZ"));
    // let comnplex_id  = format!("medication:['{}', '{}']", &medication.name, &formatted_timestamp);
    let medication = ctx.db.create(MEDICATION).content(medication).await?;
    // println!("formatted_timestamp: {}", &formatted_timestamp);
    // println!("query: {}", &query);
    // let mut sql = ctx.db.query(query).await?;
    // let medication: Option<Medication> = sql.take(0)?;

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
    // db: Db,
    ctx: State<ApiContext>,
    Json(medication_id): Json<MedicationId>,
) -> Result<Json<Option<Medication>>, Error> {
    let id = medication_id.get_string();
    println!("id: {}", id);
    let query = format!("SELECT * FROM {};", id);
    println!("query: {}", &query);
    let mut sql = ctx.db.query(query).await?;
    println!("sql: {:?}", &sql);
    let medication: Option<Medication> = sql.take(0)?;
    Ok(Json(medication))
}

pub(crate) async fn update_med(
    // db: Db,
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
