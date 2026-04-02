use std::sync::Arc;
use crate::{application::employee::{self, create_employee}, domain::{
    entities::employee::{Departament, Employee}, errors::DomainError, ports::employee_repository::EmployeeRepository, value_objects::{email::Email, password_hash::HashedPassword}
}};

pub struct CreateEmployeeInput {
    pub name:  String,
    pub departament: Departament,
    pub email: String,
    pub password_hash: String,
    pub salary: rust_decimal::Decimal,
}

pub struct CreateEmployeeUseCase {
    repository: Arc<dyn EmployeeRepository>,
}

impl CreateEmployeeUseCase {
    pub fn new (repo: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository: repo }
    }
    async fn execute (&self, input: CreateEmployeeInput) -> Result<Employee, DomainError> {
        let email = Email::new(&input.email)?;
        if self.repository.find_by_email(&email.value()).await?.is_some() {
            return Err(DomainError::InvalidEmail(email.value().to_string()));
        }

        let passhash = HashedPassword::new(&input.password_hash)?;

        let employee = Employee::new(
            input.name, 
            input.departament, 
            email,
            passhash, 
            input.salary
        );

        self.repository.save(&employee).await?;

        Ok(employee)
    }
}