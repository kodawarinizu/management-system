use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid Email!: {0}")]
    InvalidEmail(String),
}