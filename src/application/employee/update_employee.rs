use std::sync::Arc;
use crate::domain::{
    entities::employee::{Departament, Employee},
    errors::DomainError,
    ports::employee_repository::EmployeeRepository, value_objects::email::Email,
};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct UpdateEmployeeInput {
    pub id: Uuid,
    pub name: String,
    pub departament: Departament,
    pub email: Email,
    pub salary: Decimal,
    pub active: bool,
}

pub struct UpdateEmployeeUseCase {
    repository: Arc<dyn EmployeeRepository>,
}

impl UpdateEmployeeUseCase {
    pub fn new(repo: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, input: UpdateEmployeeInput) -> Result<Employee, DomainError> {
        let mut employee = self.repository
            .find_by_id(&input.id)
            .await?
            .ok_or(DomainError::EmployeeNotFound(input.id.to_string()))?;

        employee.name = input.name;
        employee.departament = input.departament;
        employee.email = input.email;
        employee.salary = input.salary;
        employee.active = input.active;

        self.repository.update(&employee).await?;

        Ok(employee)
    }
}