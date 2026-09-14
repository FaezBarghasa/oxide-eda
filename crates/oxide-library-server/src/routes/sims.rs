//! `/sims` routes — primitive CRUD mirror of `routes::symbols`.

use actix_web::{HttpResponse, web};
use oxide_library::primitive::SimModel;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::{AppState, PrimitiveSummary};
use crate::routes::error::ApiError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/sims")
            .route(web::get().to(list_sims))
            .route(web::post().to(create_sim)),
    )
    .service(
        web::resource("/sims/{uuid}")
            .route(web::get().to(get_sim)),
    );
}

#[derive(Debug, Deserialize, Default)]
pub struct ListQuery {
    pub library_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBody {
    pub library_id: Uuid,
    #[serde(flatten)]
    pub sim: SimModel,
}

async fn create_sim(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();
    state.insert_sim(body.library_id, &body.sim).await?;
    Ok(HttpResponse::Created().json(body.sim))
}

async fn get_sim(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let library_id = q
        .library_id
        .ok_or_else(|| ApiError::bad_request("missing library_id query parameter"))?;
    let uuid = Uuid::parse_str(&uuid).map_err(|e| ApiError::bad_request(e.to_string()))?;
    let sim = state
        .fetch_sim(library_id, uuid)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("sim {library_id}/{uuid}")))?;
    Ok(HttpResponse::Ok().json(sim))
}

async fn list_sims(
    state: web::Data<AppState>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let summaries: Vec<PrimitiveSummary> = state.list_sims(q.library_id).await?;
    Ok(HttpResponse::Ok().json(summaries))
}
