use axum::{Json, extract::State, response::IntoResponse};

use crate::AppState;
use crate::handlers::inner_handlers;
use crate::models::{Answer, AnswerDetail, AnswerId, QuestionId};

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    inner_handlers::create_answer(answer, answers_dao.as_ref())
        .await
        .map(Json)
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    inner_handlers::read_answers(question_uuid, answers_dao.as_ref())
        .await
        .map(Json)
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> impl IntoResponse {
    inner_handlers::delete_answer(answer_uuid, answers_dao.as_ref()).await
}
