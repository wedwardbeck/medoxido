use crate::error::Error;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
// use clap::Id;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

const MEDICATION: &str = "medication";

type Db = State<Surreal<Client>>;

#[derive(Serialize, Deserialize)]
pub struct Medication {
    name: String,
}
#[derive(Serialize, Deserialize, Default)]
pub struct MedicationId {
    id: String,
}

impl MedicationId {
    fn get_string(&self) -> String {
        self.id.to_string()
    }
}

pub async fn create(
    db: Db,
    // id: Path<String>,
    Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
    let medication = db.create(MEDICATION).content(medication).await?;
    // let medication = db.create((MEDICATION, &*id)).content(medication).await?;
    Ok(Json(medication))
}

pub async fn read(db: Db, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
    let medication = db.select((MEDICATION, &*id)).await?;
    Ok(Json(medication))
}

pub async fn read_body(
    db: Db,
    Json(medication_id): Json<MedicationId>,
) -> Result<Json<Option<Medication>>, Error> {
    let id = medication_id.get_string();
    println!("id: {}", id);
    let query = format!("SELECT * FROM {}", id);
    let mut sql = db.query(query).await?;
    let medication: Option<Medication> = sql.take(0)?;
    Ok(Json(medication))
}

pub async fn update(
    db: Db,
    id: Path<String>,
    Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
    let medication = db.update((MEDICATION, &*id)).content(medication).await?;
    Ok(Json(medication))
}

pub async fn delete(db: Db, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
    let medication = db.delete((MEDICATION, &*id)).await?;
    Ok(Json(medication))
}

pub async fn list(db: Db) -> Result<Json<Vec<Medication>>, Error> {
    let medications = db.select(MEDICATION).await?;
    Ok(Json(medications))
}
