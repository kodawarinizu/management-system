use uuid::Uuid;
use async_trait::async_trait;
use crate::domain::entities::employee::Employee;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait EmployeeRepository {
    async fn save(&self, employee: Employee) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employee>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Employee>, DomainError>;
    async fn update(&self, employee: Employee) -> Result<(), DomainError>;
    async fn delete(&self , id: Uuid) -> Result<(), DomainError>;
}