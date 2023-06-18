
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const NOTE: &str = "note";

/// A struct representing a note that can relate to other objects. Used to store notes on
/// medications, stores, and other objects. The `note_table` and `note_thing` fields are used to
/// identify the object the note relates to. The `content` field is used to store the note itself.
///
/// # Fields
///
/// * `id` - The unique identifier of the note
/// * `note_table` - The table the note relates to
/// * `note_thing` - The thing the note relates to
/// * `content` - The content of the note
/// * `created` - The date the note was created
/// * `updated` - The date the note was last updated
#[derive(Serialize, Deserialize)]
pub struct Note {
    id: Option<Thing>,
    user: Option<Thing>,
    note_table: String,
    note_thing: String,
    content: String,
    created: Option<Datetime>,
    updated: Option<Datetime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNote {
    user: String,
    note_table: String,
    note_thing: String,
    content: String,
    created: Option<Datetime>,
    updated: Option<Datetime>,
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
/// A `Json` object containing the newly created note, wrapped in an `Option`.
/// If the note was not created successfully, returns an `Error`.
pub(crate) async fn create_note(
    ctx: State<ApiContext>,
    Json(note): Json<CreateNote>,
) -> Result<Json<Option<Note>>, Error> {
    let mut sql = ctx.db.query(
        "CREATE note SET user = type::thing('user', $user), note_table = $note_table, note_thing = $note_thing, content = $content;")
        .bind(("user", note.user))
        .bind(("note_table", note.note_table))
        .bind(("note_thing", note.note_thing))
        .bind(("content", note.content))
        .await?;
    let note: Option<Note> = sql.take(0)?;
    Ok(Json(note))
}

/// Reads a note from the database with the given ID and returns it as JSON.
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct.
/// * `id` - A `Path` object containing the ID of the note to be read.
///
/// # Returns
///
/// Returns a `Json` object containing the note with the given ID, or `None` if no note was found.
///
/// # Errors
///
/// Returns an `Error` if there was an issue with the database query.
pub(crate) async fn read_note(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.select((NOTE, &*id)).await?;
    Ok(Json(note))
}

/// Updates the note with the given ID in the database with the new content provided in the request body.
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct.
/// * `id` - A `Path` object containing the ID of the note to be updated.
/// * `Json(note)` - A `Json` object containing the new content of the note.
///
/// # Returns
///
/// A `Json` object containing the updated note, or `None` if the note was not found in the database.
///
/// # Errors
///
/// Returns an `Error` if there was an issue updating the note in the database.
pub(crate) async fn update_note(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(note): Json<CreateNote>,
) -> Result<Json<Option<Note>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('note', $id) SET user = type::thing('user', $user), note_table = $note_table, note_thing = $note_thing, content = $content;")
        .bind(("id", &*id))
        .bind(("user", note.user))
        .bind(("note_table", note.note_table))
        .bind(("note_thing", note.note_thing))
        .bind(("content", note.content))
        .await?;
    let note: Option<Note> = sql.take(0)?;
    Ok(Json(note))
}

/// Deletes a note with the given ID from the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` struct
/// * `id` - A `Path` object containing the ID of the note to be deleted
///
/// # Returns
///
/// A `Json` object containing the deleted note, or `None` if the note was not found
/// in the database. If an error occurs during the deletion process, an `Error` object is returned.
pub(crate) async fn delete_note(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.delete((NOTE, &*id)).await?;
    Ok(Json(note))
}

/// Lists all notes in the database
///
/// # Arguments
///
/// * `ctx` - A `State` object containing the `ApiContext` instance
///
/// # Returns
///
/// * A `Json` object containing a vector of `Note` objects
///
/// # Errors
///
/// * Returns an `Error` if the database query fails.
pub(crate) async fn list_notes(ctx: State<ApiContext>,) -> Result<Json<Vec<Note>>, Error> {
    let notes = ctx.db.select(NOTE).await?;
    Ok(Json(notes))
}

#[derive(Serialize, Deserialize)]
pub struct DoseNote {
    // user: String,
    dose_id: String,
    dose_quantity: f32,
    unit: String,
    store_id: String,
    store_start_quantity: f32,
    store_production_date: Datetime,
    dose_created: Datetime,
    dose_updated: Datetime,
    medication_id: String,
    medication_name: String,
    note_table: String,
    note_thing: String,
    content: String,
    created: Datetime,
    updated: Datetime,
    id: Thing,
}

pub(crate) async fn list_all_dose_notes(ctx: State<ApiContext>,) -> Result<Json<Vec<DoseNote>>, Error> {
    let mut sql = ctx.db.query(
        "RETURN fn::list_all_dose_notes();")
        .await?;
    let notes: Vec<DoseNote> = sql.take(0)?;
    Ok(Json(notes))
}

pub(crate) async fn list_notes_for_dose(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Vec<DoseNote>>, Error> {
    let mut sql = ctx.db.query(
        "RETURN fn::list_notes_for_dose($id);")
        .bind(("id", &*id))
        .await?;
    let notes: Vec<DoseNote> = sql.take(0)?;
    Ok(Json(notes))
}

//TODO: Add function to list notes by tables and things (objects)
