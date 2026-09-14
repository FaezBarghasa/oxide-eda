//! Oxide library DB-flavour server with Actix-Web and optional QUIC/HTTP/3.
//!
//! Exposes a JSON HTTP API over a shared `AppState` (DB pool + lock manager).
//! Liveness checks (`/health`, `/version`) stay anonymous so process
//! supervisors don't need credentials. Every other route — the
//! `/tables` + `/rows` row tier, the primitive
//! (`/symbols` / `/footprints` / `/sims`) routes, and the advisory
//! `/rows/:row_id/locks` endpoint — is gated behind a bearer-token
//! check sourced from the `OXIDE_API_TOKEN` env var.

use std::sync::Arc;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpResponse, Responder, web};
use futures_util::future::{LocalBoxFuture, Ready, ok, ready};
use serde_json::json;

pub mod db;
pub mod locks;
pub mod routes;

pub use db::AppState;

/// Env var that holds the bearer token for the protected routes.
/// Unset → unauthenticated mode (with a startup warning).
pub const API_TOKEN_ENV: &str = "OXIDE_API_TOKEN";

/// Env var holding the persistent database URL (`postgres://…` or
/// `sqlite://<file>`). Unset → an ephemeral in-memory SQLite that
/// loses every row on restart; the binary only allows that on a
/// loopback bind and logs a prominent warning.
pub const DATABASE_URL_ENV: &str = "OXIDE_DATABASE_URL";

/// Maximum request body in bytes accepted on protected mutation routes.
pub const MAX_REQUEST_BODY_BYTES: usize = 1 << 20;

/// How often to drop expired entries from the in-memory `LockManager`.
const LOCK_SWEEP_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Anonymous liveness endpoints
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

pub async fn version() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": "oxide-library-server",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub fn configure_liveness(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/version", web::get().to(version));
}

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    routes::tables::configure(cfg);
    routes::rows::configure(cfg);
    routes::locks::configure(cfg);
    routes::symbols::configure(cfg);
    routes::footprints::configure(cfg);
    routes::sims::configure(cfg);
}

/// Spawns the background task to clean expired advisory locks.
pub fn start_lock_sweeper(state: &AppState) {
    let locks_handle: Arc<crate::locks::LockManager> = state.locks_arc();
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(LOCK_SWEEP_INTERVAL);
        tick.tick().await;
        loop {
            tick.tick().await;
            locks_handle.sweep_expired();
        }
    });
}

/// Bearer token validation middleware
pub struct BearerAuth {
    token: Option<String>,
}

impl BearerAuth {
    pub fn new(token: Option<String>) -> Self {
        Self { token }
    }
}

impl<S, B> Transform<S, ServiceRequest> for BearerAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = BearerAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BearerAuthMiddleware {
            service,
            expected_token: self.token.clone(),
        })
    }
}

pub struct BearerAuthMiddleware<S> {
    service: S,
    expected_token: Option<String>,
}

impl<S, B> Service<ServiceRequest> for BearerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip auth for liveness routes
        let path = req.path();
        if path == "/health" || path == "/version" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_boxed_body())
            });
        }

        if let Some(ref expected) = self.expected_token {
            let auth_header = req.headers().get(AUTHORIZATION);
            let authorized = auth_header
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(|token| token == expected)
                .unwrap_or(false);

            if !authorized {
                let response = HttpResponse::Unauthorized()
                    .json(json!({ "error": "unauthorized" }));
                return Box::pin(ready(Ok(req.into_response(response).map_into_boxed_body())));
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_boxed_body())
        })
    }
}

/// Create default CORS policy
pub fn default_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://127.0.0.1:3535")
        .allowed_origin("http://localhost:3535")
        .allowed_origin("https://oxide.dev")
        .allowed_origin("https://www.oxide.dev")
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
