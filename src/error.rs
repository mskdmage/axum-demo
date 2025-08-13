use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error ocurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
