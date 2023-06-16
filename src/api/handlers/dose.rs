
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const DOSE: &str = "dose";

/// A struct representing a dose of a certain medication
///
/// # Fields
///
/// * `id` - A `Thing` representing the unique identifier of the dose
/// * `store` - A `Thing` representing the medication that the dose is for
/// * `quantity` - A `f32` representing the amount of medication in the dose
/// * `unit` - A `String` representing the unit of measurement for the medication in the dose
/// * `created` - A `Datetime` representing the date and time the dose was created
/// * `updated` - A `Datetime` representing the date and time the dose was last updated
#[derive(Serialize, Deserialize)]
pub struct Dose {
    id: Thing,
    user: Thing,
    store: Thing,
    quantity: f32,
    unit: String,
    created: Datetime,
    updated: Datetime,
}

/// A struct representing a dose to be created
///
/// # Fields
///
/// * `id` - An optional `String` representing the ID of the dose
/// * `store` - A `String` representing the store where the dose is located
/// * `quantity` - A `f32` representing the quantity of the dose
/// * `unit` - A `String` representing the unit of the dose
#[derive(Serialize, Deserialize)]
pub struct CreateDose {
    id: Option<String>,
    user: String,
    store: String,
    quantity: f32,
    unit: String,
}

/// Creates a new dose with the given store, quantity, and unit and returns the created dose as a JSON object
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the API context
/// * `Json(dose)` - A `Json` object containing the dose information to be created
///
/// # Returns
///
/// A `Json` object containing the created dose, or `None` if the dose could not be created.
//TODO: Find fix for quantity f32 issue - temp changed all to f32, when decimal is implemented, change
pub(crate) async fn create_dose(
    ctx: State<ApiContext>,
    Json(dose): Json<CreateDose>,
) -> Result<Json<Option<Dose>>, Error> {
    let mut sql = ctx.db.query(
        "CREATE dose SET user = type::thing('user', $user), store = type::thing('store', $store), quantity = $quantity, unit = $unit;")
        .bind(("user", dose.user))
        .bind(("store", dose.store))
        .bind(("quantity", dose.quantity))
        .bind(("unit", dose.unit))
        .await?;
    let dose: Option<Dose> = sql.take(0)?;
    Ok(Json(dose))
}

/// Reads a dose from the database with the given ID and returns it as JSON
///
/// # Arguments
///
/// * `ctx` - The API context containing the database connection
/// * `id` - The ID of the dose to read
///
/// # Returns
///
/// Returns a `Json` object containing the dose with the given ID, or `None` if no dose was found. If an error occurs while reading from the database, an `Error` is returned.
pub(crate) async fn read_dose(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Dose>>, Error> {
    let dose = ctx.db.select((DOSE, &*id)).await?;
    Ok(Json(dose))
}

/// Updates the dose with the given id with the new quantity, unit, and store. Returns the updated dose if it exists, otherwise None.
pub(crate) async fn update_dose(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(dose): Json<CreateDose>,
) -> Result<Json<Option<Dose>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('dose', $id) SET quantity = $quantity, unit = $unit, store = type::thing('store', $store),
        user = type::thing('user', $user);")
        .bind(("id", &*id))
        .bind(("quantity", dose.quantity))
        .bind(("unit", dose.unit))
        .bind(("store", dose.store))
        .bind(("user", dose.user))
        .await?;
    let dose: Option<Dose> = sql.take(0)?;
    Ok(Json(dose))
}

/// Deletes a dose from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
/// * `id` - A `Path` object containing the ID of the dose to be deleted
///
/// # Returns
///
/// A `Json` object containing the deleted dose, or `None` if the dose was not found in the database.
pub(crate) async fn delete_dose(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Dose>>, Error> {
    let dose = ctx.db.delete((DOSE, &*id)).await?;
    Ok(Json(dose))
}

/// Retrieves a list of all doses from the database and returns them as a JSON object
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
///
/// # Returns
///
/// A `Json` object containing a `Vec` of `Dose` structs, representing all doses in the database. If there is an error retrieving the doses from the database, an `Error` is returned.
pub(crate) async fn list_doses(ctx: State<ApiContext>,) -> Result<Json<Vec<Dose>>, Error> {
    let doses = ctx.db.select(DOSE).await?;
    Ok(Json(doses))
}

//TODO: Add tests for dose handlers
//TODO: add function to get all doses for a given medication
//TODO: add function to get summary data on doses - stats for graphing, ot other reports
//TODO: add function to get summary data on dose timings and other patterns

