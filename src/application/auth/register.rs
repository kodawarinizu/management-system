use std::sync::Arc;
use crate::{application::employee::{self, create_employee::CreateEmployeeInput}, domain::{entities::employee::Employee, errors::DomainError, ports::employee_repository::EmployeeRepository, value_objects::{email::Email, password_hash::HashedPassword}}};


pub struct RegisterUsecase {
    repository: Arc<dyn EmployeeRepository>
}

impl RegisterUsecase {
    pub fn new (repo: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository: repo }
    }

    pub async fn execute (&self, input: CreateEmployeeInput) -> Result<Employee, DomainError> {
        let email = Email::new(&input.email)?;
        let password = HashedPassword::new(&input.password_hash)?;

        let repo = self.repository
        .find_by_email(&email.value())
        .await?
        .is_some();

        if repo {
            return Err(DomainError::DuplicateEmail);
        }
    
        let employee = Employee::new(
            input.name, 
            input.departament, 
            email, 
            password, 
            input.salary
        );

        self.repository.save(&employee).await?;

        Ok(employee)
    }
}

