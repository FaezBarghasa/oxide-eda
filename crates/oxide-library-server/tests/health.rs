use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::App;
use oxide_library_server::configure_liveness;

#[actix_web::test]
async fn health_returns_ok() {
    let app = test::init_service(App::new().configure(configure_liveness)).await;
    let req = TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn version_returns_crate_version() {
    let app = test::init_service(App::new().configure(configure_liveness)).await;
    let req = TestRequest::get().uri("/version").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "oxide-library-server");
}
