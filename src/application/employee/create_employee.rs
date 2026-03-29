use std::sync::Arc;
use crate::domain::{
    entities::employee::{Employee, Departament},
    value_objects::{email::Email, password_hash::HashedPassword},
    ports::employee_repository::EmployeeRepository,
    errors::DomainError,
};

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