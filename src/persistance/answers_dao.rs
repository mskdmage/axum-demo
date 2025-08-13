use async_trait::async_trait;
use sqlx::PgPool;

use crate::error::{DBError, postgres_error_codes};
use crate::models::{Answer, AnswerDetail};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .map_err(|_| DBError::InvalidUUID(format!("Invalid UUID: {}", answer.question_uuid)))?;

        let record = sqlx::query!(
            r#"
                INSERT INTO answers (question_uuid, content)
                VALUES ($1, $2)
                RETURNING *;
            "#,
            uuid,
            answer.content,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(sql_err) => {
                let error_code = sql_err.code();
                match error_code {
                    Some(code) => {
                        if code == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                            return DBError::InvalidUUID(format!("UUID error!"));
                        }
                        DBError::Other(Box::new(sql_err))
                    }
                    None => DBError::Other(Box::new(sql_err)),
                }
            }
            _ => DBError::Other(Box::new(e)),
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content.to_string(),
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        sqlx::query!(
            r#"
            DELETE FROM answers WHERE answer_uuid = $1
            "#,
            uuid
        )
        .execute(&self.db)
        .await
        .map_err(|e: sqlx::Error| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let records = sqlx::query!(r#"SELECT * FROM answers WHERE question_uuid = $1"#, uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.to_string(),
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
