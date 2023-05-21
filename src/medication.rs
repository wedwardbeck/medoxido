use crate::error::Error;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
// use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

const MEDICATION: &str = "medication";

// type Db = State<Surreal<Client>>;
type DB = State<Surreal<Db>>;

#[derive(Serialize, Deserialize)]
pub struct Medication {
	name: String,
}

pub async fn create(
	db: DB,
	id: Path<String>,
	Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
	let medication = db.create((MEDICATION, &*id)).content(medication).await?;
	Ok(Json(medication))
}

pub async fn read(db: DB, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
	let medication = db.select((MEDICATION, &*id)).await?;
	Ok(Json(medication))
}

pub async fn update(
	db: DB,
	id: Path<String>,
	Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
	let medication = db.update((MEDICATION, &*id)).content(medication).await?;
	Ok(Json(medication))
}

pub async fn delete(db: DB, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
	let medication = db.delete((MEDICATION, &*id)).await?;
	Ok(Json(medication))
}

pub async fn list(db: DB) -> Result<Json<Vec<Medication>>, Error> {
	let medications = db.select(MEDICATION).await?;
	Ok(Json(medications))
}
