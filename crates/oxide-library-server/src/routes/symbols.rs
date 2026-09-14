//! `/symbols` routes — primitive CRUD for the v0.9 library refactor.

use actix_web::{HttpResponse, web};
use oxide_library::primitive::Symbol;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::{AppState, PrimitiveSummary};
use crate::routes::error::ApiError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/symbols")
            .route(web::get().to(list_symbols))
            .route(web::post().to(create_symbol)),
    )
    .service(
        web::resource("/symbols/{uuid}")
            .route(web::get().to(get_symbol)),
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
    pub symbol: Symbol,
}

async fn create_symbol(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();
    state.insert_symbol(body.library_id, &body.symbol).await?;
    Ok(HttpResponse::Created().json(body.symbol))
}

async fn get_symbol(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let library_id = q
        .library_id
        .ok_or_else(|| ApiError::bad_request("missing library_id query parameter"))?;
    let uuid = Uuid::parse_str(&uuid).map_err(|e| ApiError::bad_request(e.to_string()))?;
    let symbol = state
        .fetch_symbol(library_id, uuid)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("symbol {library_id}/{uuid}")))?;
    Ok(HttpResponse::Ok().json(symbol))
}

async fn list_symbols(
    state: web::Data<AppState>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let summaries: Vec<PrimitiveSummary> = state.list_symbols(q.library_id).await?;
    Ok(HttpResponse::Ok().json(summaries))
}
