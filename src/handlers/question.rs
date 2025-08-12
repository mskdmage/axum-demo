use axum::{
    response::IntoResponse,
    Json,
};

use crate::models::{
    Question,
    QuestionId,
    QuestionDetail,
};

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let payload = QuestionDetail::new(
        "0000",
        &question.title,
        &question.description,
        "2025-01-01"
    );
    
    Json(payload)
}

pub async fn read_questions() -> impl IntoResponse {
    let mut payload: Vec<QuestionDetail> = Vec::new();
    payload.push(QuestionDetail::default());
    
    Json(payload)
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    
}