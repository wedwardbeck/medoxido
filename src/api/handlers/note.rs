
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const NOTE: &str = "note";

//TODO: Need to research better options for the note_thing field, as the current structure \
// does not allow for proper retrieval of relations
#[derive(Serialize, Deserialize)]
pub struct Note {
    id: Thing,
    note_thing: Thing,
    content: String,
    created: Datetime,
    updated: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNote {
    note_thing: String,
    content: String,
}

pub(crate) async fn create_note(
    ctx: State<ApiContext>,
    Json(note): Json<CreateNote>,
) -> Result<Json<Option<Note>>, Error> {
    let query =
        format!("CREATE note SET note_thing = '{}', content = '{}';", &note.note_thing, &note.content);
    let mut sql = ctx.db.query(query).await?;
    let note: Option<Note> = sql.take(0)?;
    Ok(Json(note))
}

pub(crate) async fn read_note(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.select((NOTE, &*id)).await?;
    Ok(Json(note))
}

pub(crate) async fn update_note(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(note): Json<Note>,
) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.update((NOTE, &*id)).content(note).await?;
    Ok(Json(note))
}

pub(crate) async fn delete_note(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Note>>, Error> {
    let note = ctx.db.delete((NOTE, &*id)).await?;
    Ok(Json(note))
}

pub(crate) async fn list_notes(ctx: State<ApiContext>,) -> Result<Json<Vec<Note>>, Error> {
    let notes = ctx.db.select(NOTE).await?;
    Ok(Json(notes))
}

