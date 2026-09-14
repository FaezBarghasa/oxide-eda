//! `/tables/:name/rows` routes — per-row CRUD over `ComponentRow`.
//!
//! All routes carry `?library_id=<uuid>` to scope into one library
//! inside the shared `component_rows` table. JSON body shape is
//! `ComponentRow` directly — no envelope wrapper.
//!
//! ```text
//! POST   /tables/:name/rows             insert row, body=ComponentRow
//! GET    /tables/:name/rows/:row_id     fetch one row
//! PUT    /tables/:name/rows/:row_id     replace, body=ComponentRow
//! DELETE /tables/:name/rows/:row_id     delete, 204 on success
//! ```
//!
//! `:row_id` is parsed as a [`RowId`] — a UUIDv7 newtype.

use actix_web::{HttpResponse, web};
use oxide_library::component::ComponentRow;
use oxide_library::identity::RowId;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::AppState;
use crate::routes::error::ApiError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tables/{name}/rows")
            .route(web::post().to(create_row)),
    )
    .service(
        web::resource("/tables/{name}/rows/{row_id}")
            .route(web::get().to(get_row))
            .route(web::put().to(update_row))
            .route(web::delete().to(delete_row)),
    );
}

#[derive(Debug, Deserialize)]
pub struct LibraryQuery {
    pub library_id: Uuid,
}

async fn create_row(
    state: web::Data<AppState>,
    name: web::Path<String>,
    q: web::Query<LibraryQuery>,
    row: web::Json<ComponentRow>,
) -> Result<HttpResponse, ApiError> {
    let row = row.into_inner();
    let inserted = state.insert_row(q.library_id, &name, &row).await?;
    if !inserted {
        return Err(ApiError::conflict(format!(
            "row {} already exists in table {name}",
            row.row_id
        )));
    }
    Ok(HttpResponse::Created().json(row))
}

async fn get_row(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError> {
    let (name, row_id_str) = path.into_inner();
    let row_id = parse_row_id(&row_id_str)?;
    let row = state
        .fetch_row(q.library_id, &name, row_id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("row {row_id} in table {name}")))?;
    Ok(HttpResponse::Ok().json(row))
}

async fn update_row(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    q: web::Query<LibraryQuery>,
    row: web::Json<ComponentRow>,
) -> Result<HttpResponse, ApiError> {
    let (name, row_id_str) = path.into_inner();
    let url_row_id = parse_row_id(&row_id_str)?;
    let row = row.into_inner();
    if row.row_id != url_row_id.as_uuid() {
        return Err(ApiError::bad_request(format!(
            "row_id mismatch: url={url_row_id}, body={}",
            row.row_id
        )));
    }
    let updated = state.update_row(q.library_id, &name, &row).await?;
    if !updated {
        return Err(ApiError::not_found(format!(
            "row {url_row_id} in table {name}"
        )));
    }
    Ok(HttpResponse::Ok().json(row))
}

async fn delete_row(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError> {
    let (name, row_id_str) = path.into_inner();
    let row_id = parse_row_id(&row_id_str)?;
    let deleted = state.delete_row(q.library_id, &name, row_id).await?;
    if !deleted {
        return Err(ApiError::not_found(format!("row {row_id} in table {name}")));
    }
    Ok(HttpResponse::NoContent().finish())
}

fn parse_row_id(raw: &str) -> Result<RowId, ApiError> {
    raw.parse()
        .map_err(|e: uuid::Error| ApiError::bad_request(format!("row_id: {e}")))
}
