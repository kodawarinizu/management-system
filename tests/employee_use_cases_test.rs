use std::sync::{Arc, Mutex};
use sistema_gestion::domain::{
    entities::employee::Employee,
    errors::DomainError,
    ports::employee_repository::EmployeeRepository,
};
use uuid::Uuid;
use async_trait::async_trait;

struct MockEmployeeRepository {
    employees: Mutex<Vec<Employee>>,
}

impl MockEmployeeRepository {
    fn new() -> Self {
        Self { employees: Mutex::new(vec![]) }
    }
}

#[async_trait]
impl EmployeeRepository for MockEmployeeRepository {
    async fn save(&self, employee: &Employee) -> Result<(), DomainError> {
        self.employees.lock().unwrap().push(employee.clone());
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Employee>, DomainError> {
        Ok(self.employees.lock().unwrap()
            .iter()
            .find(|e| e.email.value() == email)
            .cloned())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Employee>, DomainError> {
        Ok(self.employees.lock().unwrap()
            .iter()
            .find(|e| e.id == *id)  // *id para desreferenciar
            .cloned())
}

    async fn find_all(&self) -> Result<Vec<Employee>, DomainError> {
        Ok(self.employees.lock().unwrap().clone())
    }

    async fn update(&self, employee: &Employee) -> Result<(), DomainError> {
        let mut employees = self.employees.lock().unwrap();
        if let Some(e) = employees.iter_mut().find(|e| e.id == employee.id) {
            *e = employee.clone();
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.employees.lock().unwrap().retain(|e| e.id != id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use sistema_gestion::application::employee::create_employee::{
        CreateEmployeeInput, CreateEmployeeUseCase
    };
    use sistema_gestion::application::auth::login::LoginUseCase;
    use sistema_gestion::domain::entities::employee::Departament;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_crear_empleado_exitoso() {
        let repo = Arc::new(MockEmployeeRepository::new());
        let use_case = CreateEmployeeUseCase::new(repo);

        let input = CreateEmployeeInput {
            name: "Juan Test".to_string(),
            departament: Departament::Engineering,
            email: "juan@test.com".to_string(),
            password_hash: "password123".to_string(),
            salary: dec!(50000.00),
        };

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_email_duplicado() {
        let repo = Arc::new(MockEmployeeRepository::new());
        let use_case = CreateEmployeeUseCase::new(repo);

        let input = CreateEmployeeInput {
            name: "Juan Test".to_string(),
            departament: Departament::Engineering,
            email: "juan@test.com".to_string(),
            password_hash: "password123".to_string(),
            salary: dec!(50000.00),
        };

        use_case.execute(input).await.unwrap();

        let input2 = CreateEmployeeInput {
            name: "Juan Test".to_string(),
            departament: Departament::Engineering,
            email: "juan@test.com".to_string(),
            password_hash: "password123".to_string(),
            salary: dec!(50000.00),
        };

        let result = use_case.execute(input2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_exitoso() {
        let repo = Arc::new(MockEmployeeRepository::new());
        let create = CreateEmployeeUseCase::new(repo.clone());
        let login = LoginUseCase::new(repo);

        let input = CreateEmployeeInput {
            name: "Juan Test".to_string(),
            departament: Departament::Engineering,
            email: "juan@test.com".to_string(),
            password_hash: "password123".to_string(),
            salary: dec!(50000.00),
        };

        create.execute(input).await.unwrap();

        let result = login.execute("juan@test.com", "password123").await;
        assert!(result.is_ok());
    }
}