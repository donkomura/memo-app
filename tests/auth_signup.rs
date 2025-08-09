use actix_web::{http::StatusCode, test, web, App};
use memo_app::app::auth::signup;
use memo_app::app::model::SignupInput;
use memo_app::service::auth::{AuthService, MockAuthServiceConflict, MockAuthServiceSuccess};
use std::sync::Arc;

#[actix_web::test]
async fn signup_success_returns_201() {
    let mock_service: Arc<dyn AuthService> = Arc::new(MockAuthServiceSuccess);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .service(signup),
    )
    .await;

    let payload = SignupInput {
        email: "a@example.com".into(),
        password: "pass".into(),
    };
    let req = test::TestRequest::post()
        .uri("/auth/signup")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[actix_web::test]
async fn signup_conflict_returns_409() {
    let mock_service: Arc<dyn AuthService> = Arc::new(MockAuthServiceConflict);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .service(signup),
    )
    .await;

    let payload = SignupInput {
        email: "a@example.com".into(),
        password: "pass".into(),
    };
    let req = test::TestRequest::post()
        .uri("/auth/signup")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}


