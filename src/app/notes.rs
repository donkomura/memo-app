use actix_web::{get, post, put, web, HttpResponse, Responder};
use std::sync::Arc;

use crate::middleware::auth::extractor::AuthenticatedUser;
use crate::app::model::CreateNoteInput;
use crate::repository::note::NoteRepository;
use crate::app::model::UpdateNoteInput;

#[get("/notes/{id}")]
pub async fn get_note(
    note_repo: web::Data<Arc<dyn NoteRepository>>,
    path: web::Path<i64>,
) -> impl Responder {
    let note_id = path.into_inner();
    match note_repo.find_by_id(note_id).await {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/notes")]
pub async fn create_note(
    user: AuthenticatedUser,
    note_repo: web::Data<Arc<dyn NoteRepository>>,
    payload: web::Json<CreateNoteInput>,
) -> impl Responder {
    match note_repo.create_note(user.0.sub, &payload.title, &payload.content).await {
        Ok(note) => HttpResponse::Created().json(note),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/notes/{id}")]
pub async fn update_note(
    user: AuthenticatedUser,
    note_repo: web::Data<Arc<dyn NoteRepository>>,
    path: web::Path<i64>,
    payload: web::Json<UpdateNoteInput>,
) -> impl Responder {
    let note_id = path.into_inner();
    let user_id = user.0.sub;
    let note = note_repo.find_by_id(note_id).await.unwrap().unwrap();
    if !note.is_owner(user_id) {
        return HttpResponse::Forbidden().finish();
    }
    match note_repo
        .update_note(
            note_id,
            user_id,
            payload.title.as_deref(),
            payload.content.as_deref(),
        )
        .await
    {
        Ok(Some(note)) => { 
            HttpResponse::Ok().json(note)
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

