
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::Thing;

use crate::api::error::Error;
use crate::api::ApiContext;

const UNITOFMEASURE: &str = "uom";

/// Struct Uom for CRUD operations using the name field.
#[derive(Serialize, Deserialize)]
pub struct UnitOfMeasure {
    id: Thing,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUnitOfMeasure {
    name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UpdateUnitOfMeasure {
    id: String,
    name: String,
}

pub(crate) async fn create_uom(
    ctx: State<ApiContext>,
    Json(unitofmeasure): Json<CreateUnitOfMeasure>,
) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let unitofmeasure = ctx.db.create(UNITOFMEASURE).content(unitofmeasure).await?;
    // let slug = &unitofmeasure.name;
    // let query =
        // format!("CREATE type::thing('uom', string::slug('{}')) SET name = '{}';",  slug, &unitofmeasure.name);
    // println!("query: {}", query);
    // let mut sql = ctx.db.query(query).await?;
    // let unitofmeasure: Option<UnitOfMeasure> = sql.take(0)?;
    Ok(Json(unitofmeasure))
}

pub(crate) async fn read_uom(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let unitofmeasure = ctx.db.select((UNITOFMEASURE, &*id)).await?;
    Ok(Json(unitofmeasure))
}

// pub(crate) async fn read_body_uom(
//     ctx: State<ApiContext>,
//     Json(unitofmeasure_id): Json<UnitOfMeasureId>,
// ) -> Result<Json<Option<UnitOfMeasure>>, Error> {
//     let query =
//         format!("SELECT * from {};",  &unitofmeasure_id.id);
//     let mut sql = ctx.db.query(query).await?;
//     let unitofmeasure: Option<UnitOfMeasure> = sql.take(0)?;
//     Ok(Json(unitofmeasure))
// }

pub(crate) async fn update_uom(
    ctx: State<ApiContext>,
    form: axum::extract::Form<UpdateUnitOfMeasure>,
) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let uom: UpdateUnitOfMeasure = form.0;
    let id = uom.id.clone();
    let unitofmeasure = ctx.db.update((UNITOFMEASURE, id)).content(&uom).await?;
    Ok(Json(unitofmeasure))
}

pub(crate) async fn delete_uom(ctx: State<ApiContext>, id: Path<String>) -> Result<Json<Option<UnitOfMeasure>>, Error> {
    let unitofmeasure = ctx.db.delete((UNITOFMEASURE, &*id)).await?;
    Ok(Json(unitofmeasure))
}

pub(crate) async fn list_uoms(ctx: State<ApiContext>,) -> Result<Json<Vec<UnitOfMeasure>>, Error> {
    let unitofmeasures = ctx.db.select(UNITOFMEASURE).await?;
    Ok(Json(unitofmeasures))
}
