
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
    end: Datetime,
    days: String,
    times: Vec<String>,
    active: bool,
    user: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReminder {
    medication: String,
    // start: String,
    end: Datetime,
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
    let mut sql = ctx.db.query(
        "CREATE reminder SET medication = type::thing('medication', $medication), end = $end, days = $days, times = $times, user = $user;")
        .bind(("medication", reminder.medication))
        .bind(("end", reminder.end))
        .bind(("days", reminder.days))
        .bind(("times", reminder.times))
        .bind(("user", reminder.user))
        .await?;
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
    Json(reminder): Json<CreateReminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('reminder', $id) SET medication = type::thing('medication', $medication), end = $end, days = $days, times = $times, user = $user;")
        .bind(("id", &*id))
        .bind(("medication", reminder.medication))
        .bind(("end", reminder.end))
        .bind(("days", reminder.days))
        .bind(("times", reminder.times))
        .bind(("user", reminder.user))
        .await?;
    let reminder: Option<Reminder> = sql.take(0)?;
    Ok(Json(reminder))
}

pub(crate) async fn deactivate_reminder(
    ctx: State<ApiContext>,
    id: Path<String>,
) -> Result<Json<Option<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "UPDATE type::thing('reminder', $id) SET active = false;").bind(("id", &*id)).await?;
    let reminder: Option<Reminder> = sql.take(0)?;
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

pub(crate) async fn list_active_reminders(ctx: State<ApiContext>,) -> Result<Json<Vec<Reminder>>, Error> {
    let mut sql = ctx.db.query(
        "SELECT * from reminder where active == true;").await?;
    let reminders: Vec<Reminder> = sql.take(0)?;
    Ok(Json(reminders))
}

//TODO: Remove roast and toast eventually (used for testing)
const TOAST: &str = "toast";

#[derive(Serialize, Deserialize)]
pub struct Toast {
    id: Option<Thing>,
    name: String,
    times: Vec<String>,
    end: Datetime,
}

pub(crate) async fn create_toast(
    ctx: State<ApiContext>,
    Json(toast): Json<Toast>,
) -> Result<Json<Option<Toast>>, Error> {
    let toast = ctx.db.create(TOAST).content(toast).await?;
    Ok(Json(toast))
}
// const ROAST: &str = "roast";
#[derive(Serialize, Deserialize)]
pub struct Roast {
    id: Option<Thing>,
    name: String,
    person: String,
    dog: String,
}

pub(crate) async fn create_roast(
    ctx: State<ApiContext>,
    Json(roast): Json<Roast>,
) -> Result<Json<Option<Roast>>, Error> {
    // let roast = ctx.db.create(ROAST).content(roast).await?;
    let mut sql = ctx.db.query(
        "CREATE roast SET name = ($name), person = $person, dog = $dog;")
        // .bind(thing(&reminder.medication))
        .bind(("name", roast.name))
        // // .bind(date_end.unwrap().to_string())
        .bind(("person", roast.person))
        .bind(("dog", roast.dog))
        .await?;
    let roast: Option<Roast> = sql.take(0)?;
    Ok(Json(roast))
}
