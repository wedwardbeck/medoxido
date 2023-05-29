
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime };

use crate::api::error::Error;
use crate::api::ApiContext;

const DOSE: &str = "dose";

#[derive(Serialize, Deserialize)]
pub struct Dose {
    id: Thing,
    medication: Thing,
    store: Thing,
    quantity: f32,
    unit: String,
    created: Datetime,
    updated: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDose {
    quantity: f32,
    unit: String,
    medication: String,
    store: String,
}

pub(crate) async fn create_dose(
    ctx: State<ApiContext>,
    Json(dose): Json<CreateDose>,
    // form: axum::extract::Form<CreateDose>,
) -> Result<Json<Option<Dose>>, Error> {
    // let dose: CreateDose = form.0;
    let query =
        format!("CREATE dose SET quantity = {}, unit = '{}', medication = '{}', store = '{}';", &dose.quantity, &dose.unit, &dose.medication, &dose.store);
    println!("query: {}", query);
    let mut sql = ctx.db.query(query).await?;
    let dose: Option<Dose> = sql.take(0)?;
    Ok(Json(dose))
}

pub(crate) async fn read_dose(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Dose>>, Error> {
    let dose = ctx.db.select((DOSE, &*id)).await?;
    Ok(Json(dose))
}

pub(crate) async fn update_dose(
    ctx: State<ApiContext>,
    id: Path<String>,
    Json(dose): Json<Dose>,
) -> Result<Json<Option<Dose>>, Error> {
    let dose = ctx.db.update((DOSE, &*id)).content(dose).await?;
    Ok(Json(dose))
}

pub(crate) async fn delete_dose(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<Dose>>, Error> {
    let dose = ctx.db.delete((DOSE, &*id)).await?;
    Ok(Json(dose))
}

pub(crate) async fn list_doses(ctx: State<ApiContext>,) -> Result<Json<Vec<Dose>>, Error> {
    let doses = ctx.db.select(DOSE).await?;
    Ok(Json(doses))
}

