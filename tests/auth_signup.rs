use actix_web::{http::StatusCode, test, web, App};
use memo_app::app::auth::signup;
use memo_app::app::model::SignupInput;
use memo_app::repository::user::{MockRepoConflict, MockRepoSuccess, UserRepository};
use std::sync::Arc;

#[actix_web::test]
async fn signup_success_returns_201() {
    let mock_repo: Arc<dyn UserRepository> = Arc::new(MockRepoSuccess);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_repo))
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
    let mock_repo: Arc<dyn UserRepository> = Arc::new(MockRepoConflict);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_repo))
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


