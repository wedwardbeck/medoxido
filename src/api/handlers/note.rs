
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

//TODO: change create and update due to inclusion of user record, Optional for now.
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
    Json(note): Json<Note>,
) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.create(NOTE).content(note).await?;
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
    Json(note): Json<Note>,
) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.update((NOTE, &*id)).content(note).await?;
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

//TODO: Add function to list notes by tables and things (objects)
