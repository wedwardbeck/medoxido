
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const UNITOFMEASURE: &str = "unit_of_measure";

/// A unit of measure to be used by the client for medications and doses.
///
/// # Fields
///
/// * `id` - An optional unique identifier for the unit of measure.
/// * `name` - The name of the unit of measure.
/// * `abbreviation` - The abbreviation of the unit of measure.
/// * `created` - An optional timestamp indicating when the unit of measure was created.
/// * `updated` - An optional timestamp indicating when the unit of measure was last updated.
/// * `active` - An optional boolean indicating whether the unit of measure is currently active.
#[derive(Serialize, Deserialize)]
pub struct UnitOfMeasure {
    id: Option<Thing>,
    name: String,
    abbreviation: String,
    created: Option<Datetime>,
    updated: Option<Datetime>,
    active: Option<bool>,
}

/// Creates a new unit of measure and returns it as a JSON object
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`
/// * `Json(unitofmeasure)` - A JSON object containing the unit of measure to be created
///
/// # Returns
///
/// A `Json` object containing the newly created unit of measure wrapped in an `Option`. If the creation was successful, the `Option` will contain the unit of measure. If not, the `Option` will be `None`.
///
/// # Errors
///
/// Returns an `Error` if the creation of the unit of measure fails.
pub(crate) async fn create_uom(
    ctx: State<ApiContext>,
    Json(unitofmeasure): Json<UnitOfMeasure>,
) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let mut sql = ctx.db.query(
        "CREATE unit_of_measure set name = $name, abbreviation = $abbreviation;")
        .bind(("name", unitofmeasure.name))
        .bind(("abbreviation", unitofmeasure.abbreviation))
        .await?;
    let uom: Option<UnitOfMeasure> = sql.take(0)?;
    Ok(Json(uom))
}

/// Reads a unit of measure from the database given its ID
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the application context
/// * `id` - The ID of the unit of measure to read
///
/// # Returns
///
/// A `Json` object containing the unit of measure if it exists, otherwise `None`
///
/// # Errors
///
/// Returns an `Error` if there was an issue reading from the database.
pub(crate) async fn read_uom(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let unitofmeasure = ctx.db.select((UNITOFMEASURE, &*id)).await?;
    Ok(Json(unitofmeasure))
}

/// Updates a unit of measure with the given ID in the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`
/// * `id` - A `Path` object containing the ID of the unit of measure to update
/// * `Json(unitofmeasure)` - A `Json` object containing the updated unit of measure
///
/// # Returns
///
/// A `Json` object containing the updated unit of measure, wrapped in an `Option`, or an `Error` if the update fails.
pub(crate) async fn update_uom(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(unitofmeasure): Json<UnitOfMeasure>,
) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let unitofmeasure = ctx.db.update((UNITOFMEASURE, &*id)).content(unitofmeasure).await?;
    Ok(Json(unitofmeasure))
}

/// Deletes a unit of measure from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object that holds the `ApiContext` struct
/// * `id` - A `Path` object that holds the `id` of the unit of measure to be deleted
///
/// # Returns
///
/// * `Json<Option<UnitOfMeasure>>` - A JSON object that holds the deleted unit of measure, if it exists.
pub(crate) async fn delete_uom(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let unitofmeasure = ctx.db.delete((UNITOFMEASURE, &*id)).await?;
    Ok(Json(unitofmeasure))
}

/// Lists all the unit of measures from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` instance
///
/// # Returns
///
/// * A `Json` object containing a vector of `UnitOfMeasure` objects
///
/// # Errors
///
/// * Returns an `Error` if the database query fails.
pub(crate) async fn list_uoms(ctx: State<ApiContext>,) -> Result<Json<Vec<UnitOfMeasure>>, Error> {
    let unitofmeasures = ctx.db.select(UNITOFMEASURE).await?;
    Ok(Json(unitofmeasures))
}
