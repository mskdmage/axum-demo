use axum::{
    response::IntoResponse,
    Json,
};

use crate::models::{
    Answer,
    AnswerId,
    AnswerDetail,
};

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let payload = AnswerDetail::new(
        "0000",
        "0000",
        &answer.content,
        "2025-01-01",
    );

    Json(payload)
}

pub async fn read_answers() -> impl IntoResponse {
    let mut payload: Vec<AnswerDetail> = Vec::new();
    payload.push(AnswerDetail::default());

    Json(payload)
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) -> impl IntoResponse {
    
}