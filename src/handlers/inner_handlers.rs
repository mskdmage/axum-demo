use crate::{
    error::DBError,
    models::{Answer, AnswerDetail, AnswerId, Question, QuestionDetail, QuestionId},
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            HandlerError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

pub async fn create_question(
    question: Question,
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<QuestionDetail, HandlerError> {
    let question = questions_dao.create_question(question).await;

    match question {
        Ok(question) => Ok(question), // return question
        Err(err) => {
            log::error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn read_questions(
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<Vec<QuestionDetail>, HandlerError> {
    let questions = questions_dao.get_questions().await;

    match questions {
        Ok(questions) => Ok(questions),
        Err(err) => {
            log::error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn delete_question(
    question_uuid: QuestionId,
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<(), HandlerError> {
    let result = questions_dao
        .delete_question(question_uuid.question_uuid)
        .await;

    if result.is_err() {
        return Err(HandlerError::default_internal_error());
    }

    Ok(())
}

pub async fn create_answer(
    answer: Answer,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<AnswerDetail, HandlerError> {
    let answer = answers_dao.create_answer(answer).await;

    match answer {
        Ok(answer) => Ok(answer),
        Err(err) => {
            log::error!("{:?}", err);

            match err {
                DBError::InvalidUUID(s) => Err(HandlerError::BadRequest(s)),
                _ => Err(HandlerError::default_internal_error()),
            }
        }
    }
}

pub async fn read_answers(
    question_uuid: QuestionId,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<Vec<AnswerDetail>, HandlerError> {
    let answers = answers_dao.get_answers(question_uuid.question_uuid).await;

    match answers {
        Ok(answers) => Ok(answers),
        Err(e) => {
            log::error!("{:?}", e);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn delete_answer(
    answer_uuid: AnswerId,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<(), HandlerError> {
    let result = answers_dao.delete_answer(answer_uuid.answer_uuid).await;

    if result.is_err() {
        return Err(HandlerError::default_internal_error());
    }

    Ok(())
}
