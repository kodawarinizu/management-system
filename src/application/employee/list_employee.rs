use std::sync::Arc;

use crate::domain::{entities::employee::Employee, errors::DomainError, ports::employee_repository::EmployeeRepository};

pub struct ListEmployeeUsecase {
    repository: Arc<dyn EmployeeRepository>
}

impl ListEmployeeUsecase {
    pub fn new (repo: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository: repo }
    }

    pub async fn execute (&self) -> Result<Vec<Employee>, DomainError> {
       self.repository.find_all().await
    }

}