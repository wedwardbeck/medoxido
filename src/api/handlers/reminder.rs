
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
// use axum_extra::extract::Form;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const REMINDER: &str = "reminder";
//TODO - need to fix times from Vec<String>
#[derive(Serialize, Deserialize)]
pub struct Reminder {
    id: Thing,
    medication: Thing,
    start: Datetime,
    end: String,
    days: String,
    times: Vec<String>,
    active: bool,
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
// TODO - alter db changing fields to snake case
// stuck in a loop of trying to get the times to work with the Thing for medication
pub(crate) async fn create_reminder(
    ctx: State<ApiContext>,
    Json(reminder): Json<CreateReminder>,
    // form: axum::extract::Form<CreateReminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    // println!("reminder: {:?}", reminder.medication);
    // let reminder: CreateReminder = form.0;
    let times = &reminder.times.join(",");
    let query =
        format!("CREATE reminder SET medication = {}, start = {}, end = '{}',
        days = '{}', times = '{}', user = {};", &reminder.medication, &reminder.start,
        &reminder.end, &reminder.days, times, &reminder.user);
    println!("query: {}", query);
    let mut sql = ctx.db.query(query).await?;
    let reminder: Option<Reminder> = sql.take(0)?;
    // let reminder = ctx.db.create(REMINDER).content(reminder).await?;
    Ok(Json(reminder))
}

pub(crate) async fn create_reminder_form(
    ctx: State<ApiContext>,
    form: axum::extract::Form<CreateReminder>,
) -> Result<Json<Option<Reminder>>, Error> {
    let reminder: CreateReminder = form.0;
    // let calc_end_date = format!("'{}', '+{}');", &reminder.duration_quantity, &reminder.duration_unit);
    // let query =
    //     format!("CREATE reminder SET medication = {}, startdate = '{}', enddate = '{}',
    //     days = '{}', times = '{}';", &reminder.medication, &reminder.start_date,
    //     calc_end_date, &reminder.days, &reminder.times);
    // println!("query: {}", query);
    // let mut sql = ctx.db.query(query).await?;
    // let reminder: Option<Reminder> = sql.take(0)?;
    let reminder = ctx.db.create(REMINDER).content(reminder).await?;
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

