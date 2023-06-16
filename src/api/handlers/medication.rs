
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const MEDICATION: &str = "medication";

/// A struct representing a medication
///
/// # Fields
///
/// * `id` - An optional `Thing` representing the ID of the medication
/// * `user` - A `Thing` representing the ID of the user who created the medication
/// * `name` - A `String` representing the name of the medication
/// * `created` - An optional `Datetime` representing the date and time the medication was created
/// * `updated` - An optional `Datetime` representing the date and time the medication was last updated
/// * `active` - An optional `bool` representing whether the medication is currently active or not
#[derive(Serialize, Deserialize)]
pub struct Medication {
    id: Option<Thing>,
    user: Option<Thing>,
    name: String,
    created: Option<Datetime>,
    updated: Option<Datetime>,
    active: Option<bool>,
}

/// Creates a new medication record in the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
/// * `Json(medication)` - A JSON object containing the medication data
///
/// # Returns
///
/// A `Json` object containing the newly created medication record, wrapped in an `Option` object.
/// Creates a new medication and returns it as JSON
//TODO: change create and update due to inclusion of user record, Optional for now.
pub(crate) async fn create_med(
    ctx: State<ApiContext>,
    Json(medication): Json<Medication>,
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

/// Updates a medication with the given ID in the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`
/// * `id` - A `Path` object containing the ID of the medication to update
/// * `medication` - A `Json` object containing the updated medication information
///
/// # Returns
///
/// A `Json` object containing the updated medication information, wrapped in an `Option`, or an `Error` if the update fails.
pub(crate) async fn update_med(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(medication): Json<Medication>,
) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.update((MEDICATION, &*id)).content(medication).await?;
    Ok(Json(medication))
}

/// Deletes a medication from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object that holds the `ApiContext` struct
/// * `id` - A `Path` object that holds the ID of the medication to be deleted
///
/// # Returns
///
/// A `Json` object that holds an `Option` of the deleted medication or an `Error` if the operation fails.
pub(crate) async fn delete_med(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Medication>>, Error> {
    let medication = ctx.db.delete((MEDICATION, &*id)).await?;
    Ok(Json(medication))
}

/// Retrieves a list of all medications from the database and returns them as a JSON object
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
///
/// # Returns
///
/// A `Json` object containing a vector of `Medication` structs, or an `Error` if the database query fails.
pub(crate) async fn list_meds(ctx: State<ApiContext>,) -> Result<Json<Vec<Medication>>, Error> {
    let medications = ctx.db.select(MEDICATION).await?;
    Ok(Json(medications))
}
