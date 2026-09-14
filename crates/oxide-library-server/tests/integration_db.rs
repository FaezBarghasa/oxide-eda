//! Integration tests covering DB schema migrations + the `/tables`
//! and `/rows` HTTP routes for the DBLib row model.
//!
//! Default backend: in-memory SQLite. Postgres path is gated behind
//! `OXIDE_TEST_PG_URL` env var so CI without Postgres still passes.

use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use chrono::Utc;
use oxide_library::adapter::FieldSet;
use oxide_library::component::{ComponentRow, DatasheetRef, PinPadOverride, PlmReserved};
use oxide_library::identity::{ComponentClass, InternalPn, RowId};
use oxide_library::lifecycle::LifecycleState;
use oxide_library::manufacturer::ManufacturerPart;
use oxide_library::param::ParamMap;
use oxide_library::primitive::PrimitiveRef;
use oxide_library_server::db::AppState;
use oxide_library_server::{
    API_TOKEN_ENV, BearerAuth, configure_protected, default_cors,
};
use uuid::Uuid;

const TEST_BEARER: &str = "test-bearer-token";

fn ensure_test_token() {
    unsafe {
        std::env::set_var(API_TOKEN_ENV, TEST_BEARER);
    }
}

fn fixture_row(internal_pn: &str) -> ComponentRow {
    let lib = Uuid::now_v7();
    ComponentRow {
        row_id: Uuid::now_v7(),
        internal_pn: InternalPn::new(internal_pn),
        class: ComponentClass::new("resistor"),
        datasheet: DatasheetRef::url("https://example.com/ds.pdf"),
        state: LifecycleState::Released,
        symbol_ref: PrimitiveRef::new(lib, Uuid::now_v7()),
        footprint_ref: Some(PrimitiveRef::new(lib, Uuid::now_v7())),
        sim_ref: None,
        pin_map_overrides: Vec::<PinPadOverride>::new(),
        primary_mpn: ManufacturerPart::draft("Acme", format!("MPN-{internal_pn}")),
        alternates: Vec::new(),
        supply: Vec::new(),
        parameters: ParamMap::new(),
        plm: PlmReserved::default(),
        version: "0.0.1".into(),
        released: false,
        symbol_version: String::new(),
        footprint_version: String::new(),
        sim_version: String::new(),
        created: Utc::now(),
        updated: Utc::now(),
        content_hash: [0u8; 32],
    }
}

async fn fresh_state() -> AppState {
    ensure_test_token();
    let state = AppState::new_sqlite_memory()
        .await
        .expect("sqlite memory state");
    state.migrate().await.expect("migrations apply");
    state
}

fn bearer_header() -> String {
    format!("Bearer {TEST_BEARER}")
}

macro_rules! test_app {
    ($state:expr) => {
        test::init_service(
            App::new()
                .app_data(web::Data::new($state))
                .wrap(default_cors())
                .wrap(BearerAuth::new(Some(TEST_BEARER.to_string())))
                .configure(configure_protected),
        )
        .await
    };
}

#[tokio::test]
async fn migrations_apply_cleanly() {
    let state = fresh_state().await;
    for table in ["component_rows", "symbols", "footprints", "sims"] {
        let resp = state.db().query(format!("SELECT * FROM {table}")).await;
        assert!(resp.is_ok(), "table {table} should be queryable in surrealdb");
    }
}

#[actix_web::test]
async fn route_tables_lists_empty() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let req = TestRequest::get()
        .uri(&format!("/tables?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let names: Vec<String> = test::read_body_json(resp).await;
    assert!(names.is_empty());
}

#[actix_web::test]
async fn route_post_row_then_get() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let row = fixture_row("R0805_10k");

    let req = TestRequest::post()
        .uri(&format!("/tables/resistors/rows?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .set_json(&row)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = TestRequest::get()
        .uri(&format!(
            "/tables/resistors/rows/{}?library_id={library_id}",
            row.row_id
        ))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: ComponentRow = test::read_body_json(resp).await;
    assert_eq!(got, row);

    let req = TestRequest::get()
        .uri(&format!("/tables/resistors?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let listed: Vec<ComponentRow> = test::read_body_json(resp).await;
    assert_eq!(listed, vec![row.clone()]);

    let req = TestRequest::get()
        .uri(&format!("/tables?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let names: Vec<String> = test::read_body_json(resp).await;
    assert_eq!(names, vec!["resistors".to_string()]);
}

#[actix_web::test]
async fn route_post_duplicate_row_conflicts_and_preserves_original() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let row1 = fixture_row("R0805_10k");
    let mut row2 = row1.clone();
    row2.internal_pn = InternalPn::new("R0805_CLOBBER");
    row2.version = "9.9.9".into();

    let req1 = TestRequest::post()
        .uri(&format!("/tables/resistors/rows?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .set_json(&row1)
        .to_request();
    let resp1 = test::call_service(&app, req1).await;
    assert_eq!(resp1.status(), StatusCode::CREATED);

    let req2 = TestRequest::post()
        .uri(&format!("/tables/resistors/rows?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .set_json(&row2)
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status(), StatusCode::CONFLICT);

    let req = TestRequest::get()
        .uri(&format!(
            "/tables/resistors/rows/{}?library_id={library_id}",
            row1.row_id
        ))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: ComponentRow = test::read_body_json(resp).await;
    assert_eq!(got, row1, "the original row must survive a conflicting POST");
}

#[actix_web::test]
async fn route_put_row_updates() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let row = fixture_row("R0805_10k");
    let row_id = row.row_id;

    let req = TestRequest::post()
        .uri(&format!("/tables/resistors/rows?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .set_json(&row)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let mut updated = row.clone();
    updated.internal_pn = InternalPn::new("R0805_10k_REV2");
    updated.state = LifecycleState::Deprecated;

    let req = TestRequest::put()
        .uri(&format!(
            "/tables/resistors/rows/{row_id}?library_id={library_id}"
        ))
        .insert_header(("authorization", bearer_header()))
        .set_json(&updated)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = TestRequest::get()
        .uri(&format!(
            "/tables/resistors/rows/{row_id}?library_id={library_id}"
        ))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: ComponentRow = test::read_body_json(resp).await;
    assert_eq!(got.internal_pn, InternalPn::new("R0805_10k_REV2"));
    assert_eq!(got.state, LifecycleState::Deprecated);
}

#[actix_web::test]
async fn route_delete_row() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let row = fixture_row("R0805_10k");
    let row_id = row.row_id;

    let req = TestRequest::post()
        .uri(&format!("/tables/resistors/rows?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .set_json(&row)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = TestRequest::delete()
        .uri(&format!(
            "/tables/resistors/rows/{row_id}?library_id={library_id}"
        ))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let req = TestRequest::get()
        .uri(&format!(
            "/tables/resistors/rows/{row_id}?library_id={library_id}"
        ))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn route_unauthenticated_returns_401() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let req = TestRequest::get()
        .uri(&format!("/tables?library_id={library_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn lock_contention_second_attempt_blocks_until_release() {
    let state = fresh_state().await;
    state.locks().set_idle_ttl(Duration::from_millis(200));

    let row_uuid = RowId::new().as_uuid();

    state
        .locks()
        .try_lock(row_uuid, FieldSet::Symbol, "alice")
        .expect("alice acquires");

    let err = state
        .locks()
        .try_lock(row_uuid, FieldSet::Symbol, "bob")
        .unwrap_err();
    assert!(matches!(
        err.kind,
        oxide_library_server::locks::LockErrorKind::Held { .. }
    ));

    state
        .locks()
        .release(row_uuid, FieldSet::Symbol, "alice")
        .unwrap();
    state
        .locks()
        .try_lock(row_uuid, FieldSet::Symbol, "bob")
        .expect("bob acquires after release");

    state
        .locks()
        .try_lock(row_uuid, FieldSet::Footprint, "alice")
        .expect("different field-set is independent");
}

#[tokio::test]
async fn lock_contention_ttl_expiry_allows_takeover() {
    let state = fresh_state().await;
    state.locks().set_idle_ttl(Duration::from_millis(50));

    let row_uuid = RowId::new().as_uuid();
    state
        .locks()
        .try_lock(row_uuid, FieldSet::Symbol, "alice")
        .unwrap();

    tokio::time::sleep(Duration::from_millis(100)).await;

    state
        .locks()
        .try_lock(row_uuid, FieldSet::Symbol, "bob")
        .expect("bob takes over after TTL");
}

#[actix_web::test]
async fn locks_endpoint_returns_409_when_held() {
    let state = fresh_state().await;
    state.locks().set_idle_ttl(Duration::from_secs(60));
    let app = test_app!(state);

    let row_id = RowId::new();

    let mk_req = |holder: &'static str| {
        TestRequest::post()
            .uri(&format!("/rows/{row_id}/locks"))
            .insert_header(("authorization", bearer_header()))
            .insert_header(("x-oxide-holder", holder))
            .set_json(&serde_json::json!({"field_set": "Symbol"}))
            .to_request()
    };

    let resp1 = test::call_service(&app, mk_req("alice")).await;
    assert_eq!(resp1.status(), StatusCode::CREATED);

    let resp2 = test::call_service(&app, mk_req("bob")).await;
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}

#[tokio::test]
#[ignore = "requires OXIDE_TEST_PG_URL"]
async fn postgres_migrations_apply_when_env_set() {
    let url = match std::env::var("OXIDE_TEST_PG_URL") {
        Ok(u) => u,
        Err(_) => return,
    };
    let state = AppState::connect(&url).await.expect("pg connect");
    state.migrate().await.expect("pg migrations apply");
}
