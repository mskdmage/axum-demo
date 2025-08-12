use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

impl Answer {
    pub fn new(question_uuid: &str, content: &str) -> Self {
        Self {
            question_uuid: question_uuid.to_owned(),
            content: content.to_owned(),
        }        
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerId {
    pub answer_uuid: String,
}

impl AnswerId {
    pub fn new(answer_uuid: &str) -> Self {
        Self {
            answer_uuid: answer_uuid.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

impl AnswerDetail {
    pub fn new(answer_uuid: &str, question_uuid: &str, content: &str, created_at: &str) -> Self {
        Self {
            answer_uuid: answer_uuid.to_owned(),
            question_uuid: question_uuid.to_owned(),
            content: content.to_owned(),
            created_at: created_at.to_owned(),
        }
    }
}

impl Default for AnswerDetail {
    fn default() -> Self {
        Self {
            answer_uuid: "0000".to_owned(),
            question_uuid: "0000".to_owned(),
            content: "No descr".to_owned(),
            created_at: "None".to_owned(),
        }
    }
}