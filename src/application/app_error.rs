use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid credentials")]
    InvalidCredentials(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}
