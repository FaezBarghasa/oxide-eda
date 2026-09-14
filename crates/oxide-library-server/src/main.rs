use std::net::SocketAddr;

use actix_web::web::JsonConfig;
use actix_web::{App, HttpServer, web};
use oxide_library_server::{
    API_TOKEN_ENV, AppState, BearerAuth, DATABASE_URL_ENV, MAX_REQUEST_BODY_BYTES,
    configure_liveness, configure_protected, default_cors, start_lock_sweeper,
};

/// Default bind address — loopback only. Previously `0.0.0.0:3535`.
const DEFAULT_BIND: &str = "127.0.0.1:3535";
const BIND_ENV: &str = "OXIDE_LIBRARY_BIND";

fn is_loopback_bind(bind: &str) -> bool {
    bind.parse::<SocketAddr>()
        .map(|sa| sa.ip().is_loopback())
        .unwrap_or(false)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let bind = std::env::var(BIND_ENV).unwrap_or_else(|_| DEFAULT_BIND.to_string());

    let token_opt = std::env::var(API_TOKEN_ENV).ok().filter(|t| !t.is_empty());
    if !is_loopback_bind(&bind) && token_opt.is_none() {
        anyhow::bail!(
            "refusing to bind to non-loopback address `{bind}` without `{API_TOKEN_ENV}` set; \
             set the env var or bind to 127.0.0.1 / [::1]"
        );
    }

    let db_url = std::env::var(DATABASE_URL_ENV)
        .ok()
        .filter(|s| !s.is_empty());
    if !is_loopback_bind(&bind) && db_url.is_none() {
        anyhow::bail!(
            "refusing to bind to non-loopback address `{bind}` with ephemeral in-memory storage; \
             set `{DATABASE_URL_ENV}` to a persistent database (postgres:// or sqlite://<file>)"
        );
    }

    let state = match &db_url {
        Some(url) => {
            let s = AppState::connect(url).await?;
            s.migrate().await?;
            tracing::info!("persistent database backend connected");
            s
        }
        None => {
            tracing::warn!(
                "no {DATABASE_URL_ENV} set — using EPHEMERAL in-memory SQLite (loopback dev mode); \
                 ALL DATA IS LOST ON RESTART"
            );
            let s = AppState::new_sqlite_memory().await?;
            s.migrate().await?;
            s
        }
    };

    start_lock_sweeper(&state);

    if token_opt.is_none() {
        tracing::warn!(
            env = API_TOKEN_ENV,
            "server unauthenticated — set {API_TOKEN_ENV} for production (loopback only)",
        );
    }

    let server_state = state.clone();
    let auth_token = token_opt.clone();

    tracing::info!("oxide-library-server listening on http://{}", bind);

    #[cfg(feature = "http3")]
    {
        tracing::info!("HTTP/3 (QUIC) capability enabled via feature `http3`");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_state.clone()))
            .app_data(JsonConfig::default().limit(MAX_REQUEST_BODY_BYTES))
            .wrap(default_cors())
            .wrap(BearerAuth::new(auth_token.clone()))
            .configure(configure_liveness)
            .configure(configure_protected)
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}
