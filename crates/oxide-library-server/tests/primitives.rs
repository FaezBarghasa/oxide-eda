//! Integration tests for the primitive routes
//! (`/symbols` / `/footprints` / `/sims`).
//!
//! Each test exercises a `POST` → `GET` round-trip via actix-web test harness
//! against the in-memory test harness, exactly mirroring the flow that the
//! `LibraryAdapter` will use in production. Auth is the same fixture bearer
//! token used by `tests/integration_db.rs`.

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use oxide_library::primitive::{Footprint, SimKind, SimModel, Symbol};
use oxide_library_server::db::{AppState, PrimitiveSummary};
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

fn bearer_header() -> String {
    format!("Bearer {TEST_BEARER}")
}

async fn fresh_state() -> AppState {
    ensure_test_token();
    let state = AppState::new_sqlite_memory()
        .await
        .expect("sqlite memory state");
    state.migrate().await.expect("migrations apply");
    state
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
async fn primitives_migration_creates_tables() {
    let state = fresh_state().await;
    for table in ["symbols", "footprints", "sims"] {
        let resp = state.db().query(format!("SELECT * FROM {table}")).await;
        assert!(resp.is_ok(), "table {table} should be queryable in surrealdb");
    }
}

#[actix_web::test]
async fn post_then_get_symbol_round_trip() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let mut sym = Symbol::empty("OPAMP-DUAL-8");
    sym.uuid = Uuid::now_v7();

    let body = serde_json::json!({
        "library_id": library_id,
        "uuid": sym.uuid,
        "name": sym.name,
        "anchor": sym.anchor,
        "pins": sym.pins,
        "graphics": sym.graphics,
        "schematic_params": sym.schematic_params,
        "created": sym.created,
        "updated": sym.updated,
    });

    let req = TestRequest::post()
        .uri("/symbols")
        .insert_header(("authorization", bearer_header()))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = TestRequest::get()
        .uri(&format!("/symbols/{}?library_id={}", sym.uuid, library_id))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: Symbol = test::read_body_json(resp).await;
    assert_eq!(got.uuid, sym.uuid);
    assert_eq!(got.name, sym.name);
    assert_eq!(got.pins.len(), sym.pins.len());
}

#[actix_web::test]
async fn list_symbols_filters_by_library_id() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let lib_a = Uuid::now_v7();
    let lib_b = Uuid::now_v7();

    for (lib, name) in [
        (lib_a, "RES-2T"),
        (lib_a, "CAP-2T"),
        (lib_b, "OPAMP-8"),
        (lib_b, "MCU-100"),
    ] {
        let mut sym = Symbol::empty(name);
        sym.uuid = Uuid::now_v7();
        let body = serde_json::json!({
            "library_id": lib,
            "uuid": sym.uuid,
            "name": sym.name,
            "anchor": sym.anchor,
            "pins": sym.pins,
            "graphics": sym.graphics,
            "schematic_params": sym.schematic_params,
            "created": sym.created,
            "updated": sym.updated,
        });
        let req = TestRequest::post()
            .uri("/symbols")
            .insert_header(("authorization", bearer_header()))
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    let req = TestRequest::get()
        .uri(&format!("/symbols?library_id={lib_a}"))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: Vec<PrimitiveSummary> = test::read_body_json(resp).await;
    assert_eq!(got.len(), 2);
    assert!(got.iter().all(|s| s.library_id == lib_a));
}

#[actix_web::test]
async fn post_then_get_footprint_round_trip() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let mut fp = Footprint::empty("SOIC-8");
    fp.uuid = Uuid::now_v7();

    let mut body = serde_json::to_value(&fp).unwrap();
    body.as_object_mut()
        .unwrap()
        .insert("library_id".into(), serde_json::json!(library_id));

    let req = TestRequest::post()
        .uri("/footprints")
        .insert_header(("authorization", bearer_header()))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = TestRequest::get()
        .uri(&format!("/footprints/{}?library_id={}", fp.uuid, library_id))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: Footprint = test::read_body_json(resp).await;
    assert_eq!(got.uuid, fp.uuid);
    assert_eq!(got.name, fp.name);
    assert_eq!(got.body_3d, fp.body_3d);
}

#[actix_web::test]
async fn post_then_get_sim_round_trip() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let mut sm = SimModel::empty("LM358", SimKind::Spice3);
    sm.uuid = Uuid::now_v7();
    sm.body = ".SUBCKT LM358 IN OUT VCC GND\n.ENDS".into();

    let mut body = serde_json::to_value(&sm).unwrap();
    body.as_object_mut()
        .unwrap()
        .insert("library_id".into(), serde_json::json!(library_id));

    let req = TestRequest::post()
        .uri("/sims")
        .insert_header(("authorization", bearer_header()))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = TestRequest::get()
        .uri(&format!("/sims/{}?library_id={}", sm.uuid, library_id))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: SimModel = test::read_body_json(resp).await;
    assert_eq!(got.uuid, sm.uuid);
    assert_eq!(got.name, sm.name);
    assert_eq!(got.body, sm.body);
    assert_eq!(got.kind, sm.kind);
}

#[actix_web::test]
async fn get_symbol_404_when_unknown() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let library_id = Uuid::now_v7();
    let unknown = Uuid::now_v7();

    let req = TestRequest::get()
        .uri(&format!("/symbols/{unknown}?library_id={library_id}"))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_symbol_400_without_library_id() {
    let state = fresh_state().await;
    let app = test_app!(state);

    let unknown = Uuid::now_v7();
    let req = TestRequest::get()
        .uri(&format!("/symbols/{unknown}"))
        .insert_header(("authorization", bearer_header()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
