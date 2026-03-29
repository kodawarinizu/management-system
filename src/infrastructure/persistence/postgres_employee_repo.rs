use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{
    entities::employee::{Employee, Departament},
    ports::employee_repository::EmployeeRepository,
    errors::DomainError,
};

pub struct PostgressEmployeeRepository {
    pool: PgPool
}

impl PostgressEmployeeRepository {
    pub async fn new (database_url: &str) -> Result<Self, DomainError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        sqlx::query(include_str!("schema.sql"))
    }
}