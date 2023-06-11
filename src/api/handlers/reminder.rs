
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const REMINDER: &str = "reminder";
#[derive(Serialize, Deserialize)]
pub struct Reminder {
    id: Thing,
    medication: Thing,
    start: Datetime,
    end: String,
    days: String,
    times: Vec<String>,
    active: bool,
    user: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct CreateReminder {
    medication: String,
    start: Datetime,
    end: String,
    days: String,
    times: Vec<String>,
    user: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateReminder {
    id: Thing,
    medication: Thing,
    start: Datetime,
    end: String,
    days: String,
    times: Vec<String>,
}
pub(crate) async fn create_reminder(
    ctx: State<ApiContext>,
    Json(reminder): Json<CreateReminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    let query =
        format!("CREATE reminder SET medication = {}, start = {}, end = '{}',
        days = '{}', times = [{}], user = {};", &reminder.medication, &reminder.start,
        &reminder.end, &reminder.days,
        &reminder.times.into_iter().map(|time| format!("'{}'", time)).collect::<Vec<_>>().join(", "), &reminder.user);
    println!("query: {}", query);
    let mut sql = ctx.db.query(query).await?;
    let reminder: Option<Reminder> = sql.take(0)?;
    Ok(Json(reminder))
}

pub(crate) async fn read_reminder(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Reminder>>, Error> {
    let reminder = ctx.db.select((REMINDER, &*id)).await?;
    Ok(Json(reminder))
}

pub(crate) async fn update_reminder(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(reminder): Json<Reminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    let reminder = ctx.db.update((REMINDER, &*id)).content(reminder).await?;
    Ok(Json(reminder))
}

pub(crate) async fn delete_reminder(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Reminder>>, Error> {
    let reminder = ctx.db.delete((REMINDER, &*id)).await?;
    Ok(Json(reminder))
}

pub(crate) async fn list_reminders(ctx: State<ApiContext>,) -> Result<Json<Vec<Reminder>>, Error> {
    let reminders = ctx.db.select(REMINDER).await?;
    Ok(Json(reminders))
}

