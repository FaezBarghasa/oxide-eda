//! `/tables` routes — DBLib row model.
//!
//! Two endpoints under this prefix:
//!
//! * `GET  /tables                 ?library_id=<uuid>`
//!   → `[String]` — distinct table names with at least one row.
//! * `GET  /tables/:name           ?library_id=<uuid>`
//!   → `[ComponentRow]` — every row in `name`, ordered by `internal_pn`.
//!
//! `library_id` rides on the query string the same way it does for the
//! primitive routes (`/symbols` / `/footprints` / `/sims`). Once OIDC lands
//! in v0.9.x the value can be derived from the bearer token claims; until
//! then it stays explicit so the wire contract is symmetric across the
//! refactor.

use actix_web::{HttpResponse, web};
use oxide_library::component::ComponentRow;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::AppState;
use crate::routes::error::ApiError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tables")
            .route(web::get().to(list_tables)),
    )
    .service(
        web::resource("/tables/{name}")
            .route(web::get().to(list_rows_in_table)),
    );
}

#[derive(Debug, Deserialize)]
pub struct LibraryQuery {
    pub library_id: Uuid,
}

async fn list_tables(
    state: web::Data<AppState>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError> {
    let names = state.list_table_names(q.library_id).await?;
    Ok(HttpResponse::Ok().json(names))
}

async fn list_rows_in_table(
    state: web::Data<AppState>,
    name: web::Path<String>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError> {
    let rows: Vec<ComponentRow> = state.list_rows_in_table(q.library_id, &name).await?;
    Ok(HttpResponse::Ok().json(rows))
}
