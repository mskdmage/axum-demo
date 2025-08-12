use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub title: String,
    pub description: String,
}

impl Question {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_owned(),
            description: description.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionId {
    pub question_uuid: String,
}

impl QuestionId {
    pub fn new(uuid: &str) -> Self {
        Self {
            question_uuid: uuid.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

impl QuestionDetail {
    pub fn new(question_uuid: &str, title: &str, description: &str, created_at: &str) -> Self {
        Self {
            question_uuid: question_uuid.to_owned(),
            title: title.to_owned(),
            description: description.to_owned(),
            created_at: created_at.to_owned(),
        }
    }
}

impl Default for QuestionDetail {
    fn default() -> Self {
        Self {
            question_uuid: "0000".to_owned(),
            title: "No title".to_owned(),
            description: "No descr".to_owned(),
            created_at: "None".to_owned(),
        }
    }
}