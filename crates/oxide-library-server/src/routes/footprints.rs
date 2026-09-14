//! `/footprints` routes — primitive CRUD mirror of `routes::symbols`.

use actix_web::{HttpResponse, web};
use oxide_library::primitive::Footprint;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::{AppState, PrimitiveSummary};
use crate::routes::error::ApiError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/footprints")
            .route(web::get().to(list_footprints))
            .route(web::post().to(create_footprint)),
    )
    .service(
        web::resource("/footprints/{uuid}")
            .route(web::get().to(get_footprint)),
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
    pub footprint: Footprint,
}

async fn create_footprint(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();
    state
        .insert_footprint(body.library_id, &body.footprint)
        .await?;
    Ok(HttpResponse::Created().json(body.footprint))
}

async fn get_footprint(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let library_id = q
        .library_id
        .ok_or_else(|| ApiError::bad_request("missing library_id query parameter"))?;
    let uuid = Uuid::parse_str(&uuid).map_err(|e| ApiError::bad_request(e.to_string()))?;
    let footprint = state
        .fetch_footprint(library_id, uuid)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("footprint {library_id}/{uuid}")))?;
    Ok(HttpResponse::Ok().json(footprint))
}

async fn list_footprints(
    state: web::Data<AppState>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let summaries: Vec<PrimitiveSummary> = state.list_footprints(q.library_id).await?;
    Ok(HttpResponse::Ok().json(summaries))
}
