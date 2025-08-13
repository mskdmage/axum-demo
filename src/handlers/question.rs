use axum::{Json, extract::State, response::IntoResponse};

use crate::AppState;
use crate::handlers::inner_handlers;
use crate::models::{Question, QuestionId};

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    inner_handlers::create_question(question, questions_dao.as_ref())
        .await
        .map(Json)
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    inner_handlers::read_questions(questions_dao.as_ref())
        .await
        .map(Json)
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    inner_handlers::delete_question(question_uuid, questions_dao.as_ref()).await
}
