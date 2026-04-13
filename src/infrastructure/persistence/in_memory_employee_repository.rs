use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::{
    entities::employee::Employee, errors::DomainError,
    ports::employee_repository::EmployeeRepository,
};

pub struct InMemoryEmployeeRepository {
    store: Arc<Mutex<HashMap<Uuid, Employee>>>,
}

impl InMemoryEmployeeRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EmployeeRepository for InMemoryEmployeeRepository {
    async fn save(&self, employee: &Employee) -> Result<(), DomainError> {
        self.store.lock().await.insert(employee.id, employee.clone());
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Employee>, DomainError> {
        Ok(self.store.lock().await.values().cloned().collect())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Employee>, DomainError> {
        Ok(self.store.lock().await.get(id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Employee>, DomainError> {
        Ok(self
            .store
            .lock()
            .await
            .values()
            .find(|e| e.email.value() == email)
            .cloned())
    }

    async fn update(&self, employee: &Employee) -> Result<(), DomainError> {
        let mut store = self.store.lock().await;
        if store.contains_key(&employee.id) {
            store.insert(employee.id, employee.clone());
            Ok(())
        } else {
            Err(DomainError::EmployeeNotFound(employee.id.to_string()))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.store
            .lock()
            .await
            .remove(&id)
            .map(|_| ())
            .ok_or(DomainError::EmployeeNotFound(id.to_string()))
    }
}
