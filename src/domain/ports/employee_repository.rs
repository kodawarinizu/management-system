use uuid::Uuid;
use crate::domain::entities::employee::Employee;
use crate::domain::errors::DomainError;

pub trait EmployeeRepository {
    fn save(&self, employee: Employee) -> Result<(), DomainError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Employee>, DomainError>;
    fn find_by_email(&self, email: &str) -> Result<Option<Employee>, DomainError>;
    fn update(&self, employee: Employee) -> Result<(), DomainError>;
    fn delete(&self , id: Uuid) -> Result<(), DomainError>;
}