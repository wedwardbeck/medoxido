
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const STORE: &str = "store";

/// A struct representing a store of medication
///
/// # Fields
///
/// * `id` - The unique identifier of the store
/// * `medication` - The medication stored in the store
/// * `production_date` - The date the medication was produced
/// * `expiration_date` - The date the medication will expire
/// * `lot_number` - The lot number of the medication
/// * `quantity` - The quantity of medication stored
/// * `unit` - The unit of measurement for the quantity
/// * `created` - The date the store was created
/// * `updated` - The date the store was last updated
#[derive(Serialize, Deserialize)]
pub struct Store {
    id: Thing,
    user: Thing,
    medication: Thing,
    production_date: Datetime,
    expiration_date: Option<Datetime>,
    lot_number: String,
    quantity: f32,
    unit: String,
    created: Datetime,
    updated: Datetime,
    active: bool,
}

/// A struct representing the creation of a store with the following fields:
///
/// * `medication` - a `String` representing the name of the medication
/// * `production_date` - a `Datetime` representing the date of production
/// * `expiration_date` - a `Datetime` representing the date of expiration
/// * `lot_number` - a `String` representing the lot number of the medication
/// * `quantity` - a `f32` representing the quantity of the medication
/// * `unit` - a `String` representing the unit of measurement for the medication quantity.
#[derive(Serialize, Deserialize)]
pub struct CreateStore {
    user: String,
    medication: String,
    production_date: Datetime,
    expiration_date: Option<Datetime>,
    lot_number: String,
    quantity: f32,
    unit: String,
}

/// Creates a new store in the database with the given medication, production date,
/// expiration date, lot number, quantity, and unit. Returns the created store as a JSON object
/// wrapped in a Result. If the store creation is successful, the JSON object will contain the
/// created store. If not, it will be None.
pub(crate) async fn create_store(
    ctx: State<ApiContext>,
    Json(store): Json<CreateStore>,
) -> Result<Json<Option<Store>>, Error> {
    //TODO: Evaluate if the <decimal> function here on quantity is necessary
    let mut sql = ctx.db.query(
        "CREATE store SET  user = type::thing('user', $user), medication = type::thing('medication', $medication), production_date = $production_date,
        expiration_date = $expiration_date, lot_number = $lot_number , quantity = <decimal> $quantity, unit = $unit;")
        .bind(("user", store.user))
        .bind(("medication", store.medication))
        .bind(("production_date", store.production_date))
        .bind(("expiration_date", store.expiration_date))
        .bind(("lot_number", store.lot_number))
        .bind(("quantity", store.quantity))
        .bind(("unit", store.unit))
        .await?;
    let store: Option<Store> = sql.take(0)?;
    Ok(Json(store))
}

/// Reads the store from the database and returns it as a JSON object.
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`.
/// * `id` - A `Path` object containing the ID of the store to be read.
///
/// # Returns
///
/// Returns a `Json` object containing the store data if the store is found in the database, otherwise returns an `Error`.
pub(crate) async fn read_store(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Store>>, Error> {
    let store = ctx.db.select((STORE, &*id)).await?;
    Ok(Json(store))
}

/// Updates the store with the given id with the provided store information
///
/// # Arguments
///
/// * `ctx` - The API context
/// * `id` - The id of the store to update
/// * `Json(store)` - The store information to update
///
/// # Returns
///
/// Returns a JSON object containing the updated store information if successful, otherwise returns an error.
pub(crate) async fn update_store(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(store): Json<CreateStore>,
) -> Result<Json<Option<Store>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('store', $id) SET  user = type::thing('user', $user), medication = type::thing('medication', $medication), production_date = $production_date,
        expiration_date = $expiration_date, lot_number = $lot_number , quantity = $quantity, unit = $unit;")
        .bind(("id", &*id))
        .bind(("user", store.user))
        .bind(("medication", store.medication))
        .bind(("production_date", store.production_date))
        .bind(("expiration_date", store.expiration_date))
        .bind(("lot_number", store.lot_number))
        .bind(("quantity", store.quantity))
        .bind(("unit", store.unit))
        .await?;
    let store: Option<Store> = sql.take(0)?;
    Ok(Json(store))
}

pub(crate) async fn deactivate_store(
    ctx: State<ApiContext>,
    id: Path<String>,
    // Json(store): Json<Store>,
) -> Result<Json<Option<Store>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('store', $id) SET  active = false;")
        .bind(("id", &*id))
        .await?;
    let store: Option<Store> = sql.take(0)?;
    Ok(Json(store))
}

/// Deletes a store from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
/// * `id` - A `Path` object containing the `id` of the store to be deleted
///
/// # Returns
///
/// A `Json` object containing the deleted store, wrapped in an `Option` object. If the store was not found, returns `None`.
///
/// # Errors
///
/// Returns an `Error` if there was an issue deleting the store from the database.
pub(crate) async fn delete_store(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Store>>, Error> {
    let store = ctx.db.delete((STORE, &*id)).await?;
    Ok(Json(store))
}

/// Lists all stores in the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
///
/// # Returns
///
/// A `Json` object containing a vector of `Store` structs, or an `Error` if the database query fails.
pub(crate) async fn list_stores(ctx: State<ApiContext>,) -> Result<Json<Vec<Store>>, Error> {
    let stores = ctx.db.select(STORE).await?;
    Ok(Json(stores))
}

//TODO: Need to review and assess use of user for all queries to keep records isolated in case of multuiple users
#[derive(Serialize, Deserialize)]
pub struct StoreBool {
    active: Option<bool>,
    medication: String,
    user: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoreList {
    medication_id: Thing,
    medication_name: String,
    store_active: bool,
    store_created: Datetime,
    store_expiration_date: Option<Datetime>,
    store_id: Thing,
    store_lot_number: String,
    store_production_date: Datetime,
    store_start_quantity: f32,
    store_unit: String,
    store_updated: Datetime,
    user: Thing,
}

/// Lists all stores for a given medication and user based on the active status of the store
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`
/// * `Json(store_bool)` - A JSON object containing the medication ID, active status, and user ID
///
/// # Returns
///
/// A JSON object containing a list of stores that match the given medication ID and active status.
pub(crate) async fn list_stores_for_medication(
    ctx: State<ApiContext>,
    Json(store_bool): Json<StoreBool>,
) -> Result<Json<Vec<StoreList>>, Error> {
    let mut sql = ctx.db.query(
        "RETURN fn::list_stores_for_medication($id, $bool, $user);")
        .bind(("id", store_bool.medication))
        .bind(("bool", store_bool.active))
        .bind(("user", store_bool.user))
        .await?;
    let stores: Vec<StoreList> = sql.take(0)?;
    Ok(Json(stores))
}

/// Lists all stores for a given medication and user
///
/// # Arguments
///
/// * `ctx` - The API context
/// * `store_bool` - A JSON object containing the medication ID and user ID
///
/// # Returns
///
/// A JSON object containing a list of stores for the given medication and user
///
/// # Errors
///
/// Returns an error if the query fails.
pub(crate) async fn list_all_stores_for_medication(
    ctx: State<ApiContext>,
    Json(store_bool): Json<StoreBool>,
) -> Result<Json<Vec<StoreList>>, Error> {
    let mut sql = ctx.db.query(
        "RETURN fn::list_all_stores_for_medication($id, $user);")
        .bind(("id", store_bool.medication))
        .bind(("user", store_bool.user))
        .await?;
    let stores: Vec<StoreList> = sql.take(0)?;
    Ok(Json(stores))
}
