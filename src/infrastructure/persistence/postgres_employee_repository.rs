use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{domain::{
    entities::employee::Employee, errors::DomainError, ports::employee_repository::EmployeeRepository
}};

pub struct PostgressEmployeeRepository {
    pool: PgPool
}

impl PostgressEmployeeRepository {
    pub async fn new (database_url: &str) -> Result<Self, DomainError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        sqlx::query(include_str!("schema.sql")).execute(&pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl EmployeeRepository for PostgressEmployeeRepository {
    async fn save(&self, employee: &Employee) -> Result<(), DomainError> {
        sqlx::query(r"
        INSERT INTO employees (
        id,
        name, 
        departament, 
        email, 
        password_hash, 
        salary, 
        active
        ) VALUES ($1,$2,$3,$4,$5,$6,$7)
        ")
        .bind(&employee.id)
        .bind(&employee.name)
        .bind(format!("{}", &employee.departament))
        .bind(&employee.email)
        .bind(&employee.password_hash)
        .bind(&employee.salary)
        .bind(&employee.active)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    
    }
    
    async fn find_by_id (&self, id: &Uuid) -> Result<Option<Employee>, DomainError> {
        let employee: Option<Employee> = sqlx::query_as(r"
            SELECT * FROM employees WHERE id = $1
        ")
        .bind(&id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(employee)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Employee>, DomainError> {
        let employee: Option<Employee> = sqlx::query_as(r"
            SELECT * FROM employees WHERE email = $1
        ")
        .bind(&email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(employee)
    }

    async fn update(&self, employee: &Employee) -> Result<(), DomainError> {
        sqlx::query(r"
            UPDATE employees SET
                name          = $1,
                departament   = $2,
                email         = $3,
                password_hash = $4,
                salary        = $5,
                active        = $6
            WHERE id = $7
        ")
        .bind(&employee.name)
        .bind(&employee.departament.to_string())
        .bind(&employee.email)
        .bind(&employee.password_hash)
        .bind(&employee.salary)
        .bind(&employee.active)
        .bind(&employee.id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM employees WHERE id = $1")
            .bind(id)  // Uuid es Copy
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Employee>, DomainError> {
        let employees: Vec<Employee> = sqlx::query_as(r"
            SELECT * FROM employees
        ")
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(employees)
    }
}
