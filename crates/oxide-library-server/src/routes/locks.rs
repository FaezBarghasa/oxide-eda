//! `/rows/:row_id/locks` — advisory locking over the row tier.

use actix_web::{HttpRequest, HttpResponse, web};
use oxide_library::adapter::FieldSet;
use oxide_library::identity::RowId;
use serde::{Deserialize, Serialize};

use crate::db::AppState;
use crate::locks::LockErrorKind;
use crate::routes::error::ApiError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/rows/{row_id}/locks")
            .route(web::post().to(acquire_lock))
            .route(web::delete().to(release_lock)),
    );
}

#[derive(Debug, Deserialize)]
pub struct LockBody {
    pub field_set: FieldSetWire,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum FieldSetWire {
    Symbol,
    Footprint,
    Model3d,
    SharedParams,
    SharedSupplyChain,
    SharedSimulation,
    Lifecycle,
}

impl From<FieldSetWire> for FieldSet {
    fn from(value: FieldSetWire) -> Self {
        match value {
            FieldSetWire::Symbol => FieldSet::Symbol,
            FieldSetWire::Footprint => FieldSet::Footprint,
            FieldSetWire::Model3d => FieldSet::Model3d,
            FieldSetWire::SharedParams => FieldSet::SharedParams,
            FieldSetWire::SharedSupplyChain => FieldSet::SharedSupplyChain,
            FieldSetWire::SharedSimulation => FieldSet::SharedSimulation,
            FieldSetWire::Lifecycle => FieldSet::Lifecycle,
        }
    }
}

const MAX_HOLDER_LEN: usize = 256;

fn holder_from(req: &HttpRequest) -> Result<String, ApiError> {
    let raw = req
        .headers()
        .get("x-oxide-holder")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiError::bad_request("missing x-oxide-holder header"))?;
    if raw.is_empty() {
        return Err(ApiError::bad_request("x-oxide-holder is empty"));
    }
    if raw.len() > MAX_HOLDER_LEN {
        return Err(ApiError::bad_request(format!(
            "x-oxide-holder exceeds {MAX_HOLDER_LEN}-byte limit"
        )));
    }
    if raw.chars().any(|c| c.is_control()) {
        return Err(ApiError::bad_request(
            "x-oxide-holder contains control characters",
        ));
    }
    Ok(raw.to_string())
}

async fn acquire_lock(
    state: web::Data<AppState>,
    row_id: web::Path<String>,
    req: HttpRequest,
    body: web::Json<LockBody>,
) -> Result<HttpResponse, ApiError> {
    let row_id: RowId = row_id
        .parse()
        .map_err(|e: uuid::Error| ApiError::bad_request(e.to_string()))?;
    let holder = holder_from(&req)?;
    state
        .locks()
        .try_lock(row_id.as_uuid(), body.field_set.into(), &holder)
        .map_err(|e| match e.kind {
            LockErrorKind::Held { holder } => ApiError::conflict(format!("lock held by {holder}")),
            LockErrorKind::UnknownHolder => ApiError::bad_request("unknown holder"),
            LockErrorKind::Internal => ApiError::internal("lock manager internal error"),
        })?;
    Ok(HttpResponse::Created().finish())
}

async fn release_lock(
    state: web::Data<AppState>,
    row_id: web::Path<String>,
    req: HttpRequest,
    body: web::Json<LockBody>,
) -> Result<HttpResponse, ApiError> {
    let row_id: RowId = row_id
        .parse()
        .map_err(|e: uuid::Error| ApiError::bad_request(e.to_string()))?;
    let holder = holder_from(&req)?;
    state
        .locks()
        .release(row_id.as_uuid(), body.field_set.into(), &holder)
        .map_err(|e| match e.kind {
            LockErrorKind::Held { holder } => ApiError::conflict(format!("lock held by {holder}")),
            LockErrorKind::UnknownHolder => ApiError::bad_request("not lock holder"),
            LockErrorKind::Internal => ApiError::internal("lock manager internal error"),
        })?;
    Ok(HttpResponse::NoContent().finish())
}
