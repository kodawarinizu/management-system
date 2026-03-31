use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("[Employee Error]: {0}")]
    EmployeeError(String),
    #[error("[Invalid Salary]: {0}")]
    InvalidSalary(String),
    #[error("[Invalid Email!]: {0}")]
    InvalidEmail(String),
    #[error("[PasswordHash Error!]: {0}")]
    HashError(String),
    #[error("[DataBase Error!]: {0}")]
    DatabaseError(String),
    #[error("[Departament Error]: {0}")]
    DepartamentError(String),
    #[error("[Duplicate Email!]")]
    DuplicateEmail,
    #[error("[Employee Not Found!]: {0}")]
    EmployeeNotFound(String),
}