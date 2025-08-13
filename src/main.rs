use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};

mod error;
mod handlers;
mod models;
mod persistance;

use crate::persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let questions_dao = QuestionsDaoImpl::new(db_pool.clone());
    let answers_dao = AnswersDaoImpl::new(db_pool.clone());
    let app_state = AppState {
        questions_dao: Arc::new(questions_dao),
        answers_dao: Arc::new(answers_dao),
    };

    log::info!("********* Question Records *********");

    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&db_pool)
        .await
        .unwrap();

    log::info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
