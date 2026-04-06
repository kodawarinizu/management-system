use std::sync::Arc;

use crate::domain::{
    entities::employee::Employee, errors::DomainError,
    ports::employee_repository::EmployeeRepository, value_objects::password_hash::HashedPassword,
};

pub struct LoginUseCase {
    repository: Arc<dyn EmployeeRepository>,
}

impl LoginUseCase {
    pub fn new(repo: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<Employee, DomainError> {
        let employee =
            self.repository
                .find_by_email(email)
                .await?
                .ok_or(DomainError::InvalidCredentials(
                    "Email or Password incorrect".to_string(),
                ))?;

        match employee.password_hash.verify(password)? {
            true => Ok(employee),
            false => Err(DomainError::InvalidCredentials(
                "Email or Password incorrect".to_string(),
            )),
        }
    }
}
