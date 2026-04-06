use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{errors::DomainError, ports::employee_repository::EmployeeRepository};

pub struct DeleteEmployeeUsecase {
    repository: Arc<dyn EmployeeRepository>,
}

impl DeleteEmployeeUsecase {
    pub fn new(repo: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        if self.repository.find_by_id(&id).await?.is_some() {
            return Ok(self.repository.delete(id).await?);
        }

        Err(DomainError::DatabaseError(
            "employee ID does exists".to_string(),
        ))
    }
}
