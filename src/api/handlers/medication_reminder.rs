
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const MEDICATION_REMINDER: &str = "medicationreminder";
//TODO - need to fix times from Vec<String>
#[derive(Serialize, Deserialize)]
pub struct MedicationReminder {
    id: Thing,
    medication: Thing,
    start_date: Datetime,
    end_date: String,
    days: String,
    times: Vec<String>,
    active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMedicationReminder {
    medication: String,
    start_date: Datetime,
    end_date: String,
    days: String,
    times: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateMedicationReminder {
    id: Thing,
    medication: Thing,
    start_date: Datetime,
    end_date: String,
    days: String,
    times: Vec<String>,
}
// TODO - alter db chaning fields to snake case
pub(crate) async fn create_medication_reminder(
    ctx: State<ApiContext>,
    // Json(medication_reminder): Json<CreateMedicationReminder>,
    form: axum::extract::Form<CreateMedicationReminder>,
) -> Result<Json<Option<MedicationReminder>>, Error> {
    let medication_reminder: CreateMedicationReminder = form.0;
    // let calc_end_date = format!("'{}', '+{}');", &medication_reminder.duration_quantity, &medication_reminder.duration_unit);
    // let query =
    //     format!("CREATE medication_reminder SET medication = {}, startdate = '{}', enddate = '{}',
    //     days = '{}', times = '{}';", &medication_reminder.medication, &medication_reminder.start_date,
    //     calc_end_date, &medication_reminder.days, &medication_reminder.times);
    // println!("query: {}", query);
    // let mut sql = ctx.db.query(query).await?;
    // let medication_reminder: Option<MedicationReminder> = sql.take(0)?;
    let medication_reminder = ctx.db.create(MEDICATION_REMINDER).content(medication_reminder).await?;
    Ok(Json(medication_reminder))
}


// pub(crate) async fn create_medication_reminder_form(
//     ctx: State<ApiContext>,
//     // Json(medication_reminder): Json<CreateMedicationReminder>,
//     form: axum::extract::Form<CreateMedicationReminder>,
// ) -> Result<Json<Option<MedicationReminder>>, Error> {
//     let medication_reminder: CreateMedicationReminder = form.0;
//     let query =
//         format!("CREATE medication_reminder SET quantity = {}, unit = '{}', store = '{}';", &medication_reminder.quantity, &medication_reminder.unit, &medication_reminder.store);
//     println!("query: {}", query);
//     let mut sql = ctx.db.query(query).await?;
//     let medication_reminder: Option<MedicationReminder> = sql.take(0)?;
//     Ok(Json(medication_reminder))
// }


pub(crate) async fn read_medication_reminder(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<MedicationReminder>>, Error> {
    let medication_reminder = ctx.db.select((MEDICATION_REMINDER, &*id)).await?;
    Ok(Json(medication_reminder))
}

pub(crate) async fn update_medication_reminder(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(medication_reminder): Json<MedicationReminder>,
) -> Result<Json<Option<MedicationReminder>>, Error> {
    let medication_reminder = ctx.db.update((MEDICATION_REMINDER, &*id)).content(medication_reminder).await?;
    Ok(Json(medication_reminder))
}

pub(crate) async fn delete_medication_reminder(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<MedicationReminder>>, Error> {
    let medication_reminder = ctx.db.delete((MEDICATION_REMINDER, &*id)).await?;
    Ok(Json(medication_reminder))
}

pub(crate) async fn list_medication_reminders(ctx: State<ApiContext>,) -> Result<Json<Vec<MedicationReminder>>, Error> {
    let medication_reminders = ctx.db.select(MEDICATION_REMINDER).await?;
    Ok(Json(medication_reminders))
}

