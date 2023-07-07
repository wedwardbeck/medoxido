
use axum::extract::{ State, Path, Query };
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
    id: Thing,
    user: Thing,
    name: String,
    created: Option<Datetime>,
    updated: Option<Datetime>,
    active: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMedication {
    user: String,
    name: String,
    created: Option<Datetime>,
    updated: Option<Datetime>,
    active: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct MedicationBool {
    active: Option<bool>,
    id: Option<String>,
    user: String,
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

pub(crate) async fn create_med(
    ctx: State<ApiContext>,
    Json(medication): Json<CreateMedication>,
) -> Result<Json<Option<Medication>>, Error> {
    let mut sql = ctx.db.query(
        "CREATE medication SET user = type::thing('user', $user), name = $name;")
        .bind(("user", medication.user))
        .bind(("name", medication.name))
        .await?;
    let medication: Option<Medication> = sql.take(0)?;
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
    Json(medication): Json<CreateMedication>,
) -> Result<Json<Option<Medication>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('medication', $id) SET user = type::thing('user', $user), name = $name;")
        .bind(("id", &*id))
        .bind(("user", medication.user))
        .bind(("name", medication.name))
        .await?;
    let medication: Option<Medication> = sql.take(0)?;
    Ok(Json(medication))
}

pub(crate) async fn deactivate_med(
    ctx: State<ApiContext>,
    // id: Path<String>,
    Json(medication): Json<MedicationBool>,
) -> Result<Json<Option<Medication>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('medication', $id) SET active = false WHERE user = type::thing('user', $user);")
        .bind(("id", medication.id))
        .bind(("user", medication.user))
        .await?;
    let medication: Option<Medication> = sql.take(0)?;
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
pub(crate) async fn list_all_meds(
    ctx: State<ApiContext>,
    // user: Path<String>,
    query: Query<MedicationBool>,
) -> Result<Json<Vec<Medication>>, Error> {
    let mut sql = ctx.db.query(
        "RETURN fn::list_user_medications($user);")
        .bind(("user", &*query.user))
        .await?;
    let medications: Vec<Medication> = sql.take(0)?;
    Ok(Json(medications))
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
pub(crate) async fn list_user_meds_by_status(
    ctx: State<ApiContext>,
    query: Query<MedicationBool>,
) -> Result<Json<Vec<Medication>>, Error> {
    let mut sql = ctx.db.query(
        "RETURN fn::list_user_medications_by_status($active, $user);")
        .bind(("user", &*query.user))
        .bind(("active", query.active))
        .await?;
    let medications: Vec<Medication> = sql.take(0)?;
    Ok(Json(medications))
}
