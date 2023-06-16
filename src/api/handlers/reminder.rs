
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };
use crate::api::error::Error;
use crate::api::ApiContext;

const REMINDER: &str = "reminder";

/// A struct representing a reminder for taking a medication with the following fields:
/// * `id`: A unique identifier for the reminder
/// * `medication`: The medication for which the reminder is set
/// * `start`: The start date and time of the reminder
/// * `end`: The end date and time of the reminder
/// * `days`: A string representing the days on which the reminder should be active
/// * `times`: A vector of strings representing the times at which the reminder should be active
/// * `active`: A boolean indicating whether the reminder is currently active
/// * `user`: An optional string representing the user who created the reminder
/// * `created`: The date and time when the reminder was created
#[derive(Serialize, Deserialize)]
pub struct Reminder {
    id: Thing,
    user: Option<String>,
    medication: Thing,
    start: Datetime,
    end: Datetime,
    days: String,
    times: Vec<String>,
    active: bool,
    created: Datetime,
    updated: Datetime,
}
/// Creates a new note in the database with the provided content
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`
/// * `Json(note)` - A `Json` object containing the content of the note to be created
///
/// # Returns
///
/// A `Json` object containing the newly created note, wrapped in an `Option`. If the note was not created successfully, returns an `Error`.

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReminder {
    user: String,
    medication: String,
    // start: String,
    end: Datetime,
    days: String,
    times: Vec<String>,
}

// /// A struct representing a reminder for updating medication
// ///
// /// # Fields
// ///
// /// * `id` - The unique identifier for the reminder
// /// * `medication` - The medication to be updated
// /// * `start` - The date and time when the reminder starts
// /// * `end` - The date and time when the reminder ends
// /// * `days` - The days of the week when the reminder is active
// /// * `times` - The times of day when the reminder is active
// #[derive(Serialize, Deserialize)]
// pub struct UpdateReminder {
//     id: Thing,
//     medication: Thing,
//     start: Datetime,
//     end: String,
//     days: String,
//     times: Vec<String>,
// }
/// Creates a new reminder in the database with the given parameters
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext`
/// * `Json(reminder)` - A `Json` object containing the parameters for the new reminder
///
/// # Returns
///
/// A `Json` object containing the newly created reminder, or `None` if the creation failed.
pub(crate) async fn create_reminder(
    ctx: State<ApiContext>,
    Json(reminder): Json<CreateReminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "CREATE reminder SET user = type::thing('user', $user), medication = type::thing('medication', $medication), end = $end, days = $days, times = $times;")
        .bind(("user", reminder.user))
        .bind(("medication", reminder.medication))
        .bind(("end", reminder.end))
        .bind(("days", reminder.days))
        .bind(("times", reminder.times))
        .await?;
    let reminder: Option<Reminder> = sql.take(0)?;
    Ok(Json(reminder))
}

/// Reads a reminder from the database with the given ID and returns it as JSON.
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct.
/// * `id` - A `Path` object containing the ID of the reminder to be read.
///
/// # Returns
///
/// Returns a `Json` object containing the reminder with the given ID, or `None` if no such reminder exists.
///
/// # Errors
///
/// Returns an `Error` if there is an issue with the database connection or query.
pub(crate) async fn read_reminder(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Reminder>>, Error> {
    let reminder = ctx.db.select((REMINDER, &*id)).await?;
    Ok(Json(reminder))
}

/// Updates the reminder with the given id with the provided information
///
/// # Arguments
///
/// * `ctx` - The API context
/// * `id` - The id of the reminder to update
/// * `reminder` - The new information to update the reminder with
///
/// # Returns
///
/// * `Json<Option<Reminder>>` - The updated reminder wrapped in an `Option` and then wrapped in a `Json` object
/// * `Error` - An error that occurred while updating the reminder, if any.
pub(crate) async fn update_reminder(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(reminder): Json<CreateReminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('reminder', $id) SET user = type::thing('user', $user), medication = type::thing('medication', $medication), end = $end, days = $days, times = $times;")
        .bind(("id", &*id))
        .bind(("user", reminder.user))
        .bind(("medication", reminder.medication))
        .bind(("end", reminder.end))
        .bind(("days", reminder.days))
        .bind(("times", reminder.times))
        .await?;
    let reminder: Option<Reminder> = sql.take(0)?;
    Ok(Json(reminder))
}

/// Deactivates a reminder with the given ID by setting its `active` field to `false` in the database.
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct.
/// * `id` - A `Path` object containing the ID of the reminder to be deactivated.
///
/// # Returns
///
/// Returns a `Json` object containing an `Option` of the deactivated `Reminder` object, or an `Error` if the operation fails.
pub(crate) async fn deactivate_reminder(
    ctx: State<ApiContext>,
    id: Path<String>,
) -> Result<Json<Option<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('reminder', $id) SET active = false;").bind(("id", &*id)).await?;
    let reminder: Option<Reminder> = sql.take(0)?;
    Ok(Json(reminder))
}

/// Deletes a reminder with the given ID from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
/// * `id` - A `Path` object containing the ID of the reminder to be deleted
///
/// # Returns
///
/// A `Json` object containing the deleted reminder, or `None` if the reminder was not found in the database.
pub(crate) async fn delete_reminder(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Reminder>>, Error> {
    let reminder = ctx.db.delete((REMINDER, &*id)).await?;
    Ok(Json(reminder))
}

/// Lists all reminders from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
///
/// # Returns
///
/// * A `Json` object containing a vector of `Reminder` structs
///
/// # Errors
///
/// * Returns an `Error` if there is an issue with the database query or connection.
pub(crate) async fn list_reminders(ctx: State<ApiContext>,) -> Result<Json<Vec<Reminder>>, Error> {
    let reminders = ctx.db.select(REMINDER).await?;
    Ok(Json(reminders))
}

/// Lists all active reminders from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
///
/// # Returns
///
/// A `Json` object containing a vector of `Reminder` structs that are active
///
/// # Errors
///
/// Returns an `Error` if there is an issue with the database query or if the query returns no results.
pub(crate) async fn list_active_reminders(ctx: State<ApiContext>,) -> Result<Json<Vec<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "SELECT * from reminder where active == true;").await?;
    let reminders: Vec<Reminder> = sql.take(0)?;
    Ok(Json(reminders))
}
