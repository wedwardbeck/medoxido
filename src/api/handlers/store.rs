
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const STORE: &str = "store";

#[derive(Serialize, Deserialize)]
pub struct Store {
    id: Thing,
    medication: Thing,
    production_date: Datetime,
    expiration_date: Datetime,
    lot_number: String,
    quantity: f32,
    unit: String,
    created: Datetime,
    updated: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateStore {
    medication: String,
    production_date: Datetime,
    expiration_date: Datetime,
    lot_number: String,
    quantity: f32,
    unit: String,
}

pub(crate) async fn create_store(
    ctx: State<ApiContext>,
    Json(store): Json<CreateStore>,
) -> Result<Json<Option<Store>>, Error> {
    let query =
        format!("CREATE store SET medication = '{}', production_date = {}, expiration_date = {},
        lot_number = '{}', quantity = {}, unit = '{}';", &store.medication, &store.production_date,
        &store.expiration_date, &store.lot_number, &store.quantity, &store.unit);
    // println!("query: {}", query);
    let mut sql = ctx.db.query(query).await?;
    let store: Option<Store> = sql.take(0)?;
    Ok(Json(store))
}

pub(crate) async fn read_store(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Store>>, Error> {
    let store = ctx.db.select((STORE, &*id)).await?;
    Ok(Json(store))
}

pub(crate) async fn update_store(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(store): Json<Store>,
) -> Result<Json<Option<Store>>, Error> {
    let store = ctx.db.update((STORE, &*id)).content(store).await?;
    Ok(Json(store))
}

pub(crate) async fn delete_store(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Store>>, Error> {
    let store = ctx.db.delete((STORE, &*id)).await?;
    Ok(Json(store))
}

pub(crate) async fn list_stores(ctx: State<ApiContext>,) -> Result<Json<Vec<Store>>, Error> {
    let stores = ctx.db.select(STORE).await?;
    Ok(Json(stores))
}

