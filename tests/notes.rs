use std::sync::Arc;

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use memo_app::app::notes::{create_note, get_note, update_note};
use memo_app::app::model::{CreateNoteInput, UpdateNoteInput};
use memo_app::domain::model::Note;
use memo_app::middleware::auth::token::JwtTokenService;
use memo_app::repository::note::NoteRepository;
use memo_app::repository::user::RepoError;

// ---- Mocks ----

struct MockNoteRepoCreateOk;

#[async_trait]
impl NoteRepository for MockNoteRepoCreateOk {
    async fn create_note(&self, user_id: i64, title: &str, content: &str) -> Result<Note, RepoError> {
        Ok(Note {
            id: 1,
            author_id: user_id,
            title: title.to_string(),
            content: content.to_string(),
            created_at: 1,
            updated_at: 1,
        })
    }

    async fn find_by_id(&self, _note_id: i64) -> Result<Option<Note>, RepoError> {
        Ok(None)
    }

    async fn update_note(
        &self,
        _note_id: i64,
        _user_id: i64,
        _title: Option<&str>,
        _content: Option<&str>,
    ) -> Result<Option<Note>, RepoError> {
        Ok(None)
    }

    async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
        Ok(vec![])
    }
}

struct MockNoteRepoFindSome;

#[async_trait]
impl NoteRepository for MockNoteRepoFindSome {
    async fn create_note(&self, _user_id: i64, _title: &str, _content: &str) -> Result<Note, RepoError> {
        Err(RepoError::Internal)
    }

    async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError> {
        Ok(Some(Note {
            id: note_id,
            author_id: 7,
            title: "t".into(),
            content: "c".into(),
            created_at: 1,
            updated_at: 1,
        }))
    }

    async fn update_note(
        &self,
        _note_id: i64,
        _user_id: i64,
        _title: Option<&str>,
        _content: Option<&str>,
    ) -> Result<Option<Note>, RepoError> {
        Ok(None)
    }

    async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
        Ok(vec![])
    }
}

struct MockNoteRepoFindNone;

#[async_trait]
impl NoteRepository for MockNoteRepoFindNone {
    async fn create_note(&self, _user_id: i64, _title: &str, _content: &str) -> Result<Note, RepoError> {
        Err(RepoError::Internal)
    }

    async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError> {
        Ok(None)
    }

    async fn update_note(
        &self,
        _note_id: i64,
        _user_id: i64,
        _title: Option<&str>,
        _content: Option<&str>,
    ) -> Result<Option<Note>, RepoError> {
        Ok(None)
    }

    async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
        Ok(vec![])
    }
}

struct MockNoteRepoUpdateOk;

#[async_trait]
impl NoteRepository for MockNoteRepoUpdateOk {
    async fn create_note(&self, _user_id: i64, _title: &str, _content: &str) -> Result<Note, RepoError> {
        Err(RepoError::Internal)
    }

    async fn find_by_id(&self, _note_id: i64) -> Result<Option<Note>, RepoError> { Ok(None) }

    async fn update_note(
        &self,
        note_id: i64,
        user_id: i64,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, RepoError> {
        Ok(Some(Note {
            id: note_id,
            author_id: user_id,
            title: title.unwrap_or("orig").to_string(),
            content: content.unwrap_or("orig").to_string(),
            created_at: 1,
            updated_at: 2,
        }))
    }

    async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
        Ok(vec![])
    }
}

struct MockNoteRepoUpdateNone;

#[async_trait]
impl NoteRepository for MockNoteRepoUpdateNone {
    async fn create_note(&self, _user_id: i64, _title: &str, _content: &str) -> Result<Note, RepoError> { Err(RepoError::Internal) }
    async fn find_by_id(&self, _note_id: i64) -> Result<Option<Note>, RepoError> { Ok(None) }
    async fn update_note(
        &self,
        _note_id: i64,
        _user_id: i64,
        _title: Option<&str>,
        _content: Option<&str>,
    ) -> Result<Option<Note>, RepoError> { Ok(None) }

    async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
        Ok(vec![])
    }
}

fn jwt() -> JwtTokenService {
    JwtTokenService::from_secret(b"test-secret", 3600)
}

// ---- Tests ----

#[actix_web::test]
async fn create_note_returns_201() {
    let repo: Arc<dyn NoteRepository> = Arc::new(MockNoteRepoCreateOk);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo))
            .app_data(web::Data::new(jwt()))
            .service(create_note),
    )
    .await;

    let user_id = 10;
    let token = jwt().generate(user_id).unwrap();
    let payload = CreateNoteInput { title: "Hello".into(), content: "World".into() };

    let req = test::TestRequest::post()
        .uri("/notes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let created: Note = test::read_body_json(resp).await;
    assert_eq!(created.author_id, user_id);
    assert_eq!(created.title, "Hello");
    assert_eq!(created.content, "World");
}

#[actix_web::test]
async fn get_note_returns_200_public() {
    let repo: Arc<dyn NoteRepository> = Arc::new(MockNoteRepoFindSome);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo))
            .service(get_note),
    )
    .await;

    let req = test::TestRequest::get().uri("/notes/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_note_returns_404_when_absent() {
    let repo: Arc<dyn NoteRepository> = Arc::new(MockNoteRepoFindNone);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo))
            .service(get_note),
    )
    .await;

    let req = test::TestRequest::get().uri("/notes/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn update_note_returns_200_for_owner() {
    let repo: Arc<dyn NoteRepository> = Arc::new(MockNoteRepoUpdateOk);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo))
            .app_data(web::Data::new(jwt()))
            .service(update_note),
    )
    .await;

    let user_id = 42;
    let token = jwt().generate(user_id).unwrap();
    let payload = UpdateNoteInput { title: Some("New".into()), content: None };

    let req = test::TestRequest::put()
        .uri("/notes/1")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let updated: Note = test::read_body_json(resp).await;
    assert_eq!(updated.id, 1);
    assert_eq!(updated.author_id, user_id);
    assert_eq!(updated.title, "New");
}

#[actix_web::test]
async fn update_note_returns_404_when_not_owner_or_absent() {
    let repo: Arc<dyn NoteRepository> = Arc::new(MockNoteRepoUpdateNone);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo))
            .app_data(web::Data::new(jwt()))
            .service(update_note),
    )
    .await;

    let token = jwt().generate(99).unwrap();
    let payload = UpdateNoteInput { title: None, content: Some("C".into()) };

    let req = test::TestRequest::put()
        .uri("/notes/2")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}


