# 🦀 Guía: Sistema de Gestión en Rust — Arquitectura Hexagonal

> Proyecto basado en TI3021 — Programación Orientada a Objeto Seguro  
> Implementado en Rust con prácticas y librerías profesionales

---

## 📐 Arquitectura Hexagonal (Ports & Adapters)

La arquitectura hexagonal separa el **núcleo de negocio** de los detalles externos (DB, APIs, UI).

```
┌─────────────────────────────────────────────┐
│              INFRAESTRUCTURA                │
│  ┌─────────────────────────────────────┐   │
│  │           APLICACIÓN                │   │
│  │  ┌───────────────────────────────┐  │   │
│  │  │       DOMINIO (núcleo)        │  │   │
│  │  │  - Entidades                  │  │   │
│  │  │  - Value Objects              │  │   │
│  │  │  - Reglas de negocio          │  │   │
│  │  └───────────────────────────────┘  │   │
│  │  - Use Cases / Services             │   │
│  │  - Ports (traits)                   │   │
│  └─────────────────────────────────────┘   │
│  - Adapters (DB, API, CLI, HTTP)            │
└─────────────────────────────────────────────┘
```

### Regla de dependencia
El dominio **nunca** importa infraestructura. Solo la infraestructura conoce el dominio.

---

## 🗂️ Estructura del Proyecto

```
sistema_gestion/
├── Cargo.toml
├── .env
├── src/
│   ├── main.rs
│   │
│   ├── domain/                      # Núcleo — cero dependencias externas
│   │   ├── mod.rs
│   │   ├── entities/
│   │   │   ├── employee.rs          # Entidad Empleado
│   │   │   └── travel_package.rs    # Entidad Paquete Turístico (U4)
│   │   ├── value_objects/
│   │   │   ├── email.rs             # Email validado
│   │   │   ├── password_hash.rs     # Hash seguro
│   │   │   └── employee_id.rs       # ID tipado
│   │   ├── ports/                   # Traits (interfaces)
│   │   │   ├── employee_repository.rs
│   │   │   └── external_api.rs
│   │   └── errors.rs                # Errores de dominio
│   │
│   ├── application/                 # Casos de uso
│   │   ├── mod.rs
│   │   ├── employee/
│   │   │   ├── create_employee.rs
│   │   │   ├── update_employee.rs
│   │   │   ├── delete_employee.rs
│   │   │   └── list_employees.rs
│   │   └── auth/
│   │       ├── login.rs
│   │       └── register.rs
│   │
│   └── infrastructure/              # Adaptadores externos
│       ├── mod.rs
│       ├── persistence/
│       │   ├── sqlite_employee_repo.rs
│       │   └── schema.sql
│       ├── external_api/
│       │   └── country_api_adapter.rs  # Consumo API externa (U3)
│       └── cli/
│           └── menu.rs                 # Interfaz de usuario CLI
│
└── tests/
    ├── employee_use_cases_test.rs
    └── auth_test.rs
```

---

## 📦 Cargo.toml — Librerías profesionales

```toml
[package]
name = "sistema_gestion"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialización / Deserialización (equivalente a Pydantic)
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Base de datos (ORM async)
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio", "macros"] }

# Hash seguro de contraseñas (bcrypt / argon2)
argon2 = "0.5"

# Cliente HTTP para consumo de APIs externas
reqwest = { version = "0.11", features = ["json"] }

# Variables de entorno
dotenvy = "0.15"

# Manejo de errores profesional
thiserror = "1"
anyhow = "1"

# UUIDs para IDs
uuid = { version = "1", features = ["v4"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"
```

---

## 🧱 Unidad 1 — Dominio: Entidades y Value Objects

### Value Object: Email validado

```rust
// src/domain/value_objects/email.rs
use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        if value.contains('@') && value.len() > 5 {
            Ok(Self(value.to_lowercase()))
        } else {
            Err(DomainError::InvalidEmail(value.to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

### Value Object: Hash de contraseña

```rust
// src/domain/value_objects/password_hash.rs
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_plain(plain: &str) -> Result<Self, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(plain.as_bytes(), &salt)
            .map_err(|_| DomainError::HashError)?
            .to_string();
        Ok(Self(hash))
    }

    pub fn verify(&self, plain: &str) -> bool {
        let parsed = PasswordHash::new(&self.0).unwrap();
        Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok()
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

### Entidad: Empleado

```rust
// src/domain/entities/employee.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::value_objects::{Email, HashedPassword};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub department: Department,
    pub email: String,         // almacenado como String para serde
    pub password_hash: String, // almacenado como String para serde
    pub salary: f64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Department {
    Engineering,
    Sales,
    HR,
    Finance,
    Operations,
}

impl Employee {
    pub fn new(
        name: String,
        department: Department,
        email: Email,
        password: HashedPassword,
        salary: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            department,
            email: email.value().to_string(),
            password_hash: password.value().to_string(),
            salary,
            active: true,
        }
    }

    // Regla de negocio en el dominio
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn update_salary(&mut self, new_salary: f64) -> Result<(), &'static str> {
        if new_salary <= 0.0 {
            return Err("El salario debe ser mayor a 0");
        }
        self.salary = new_salary;
        Ok(())
    }
}
```

### Errores de dominio

```rust
// src/domain/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Email inválido: {0}")]
    InvalidEmail(String),

    #[error("Error al generar hash de contraseña")]
    HashError,

    #[error("Empleado no encontrado: {0}")]
    EmployeeNotFound(String),

    #[error("Email ya registrado")]
    DuplicateEmail,

    #[error("Credenciales inválidas")]
    InvalidCredentials,
}
```

---

## 🔌 Ports (Interfaces / Traits)

```rust
// src/domain/ports/employee_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::Employee;
use crate::domain::errors::DomainError;

// Puerto de salida: el dominio define QUÉ necesita, sin saber CÓMO
#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn save(&self, employee: &Employee) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employee>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Employee>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Employee>, DomainError>;
    async fn update(&self, employee: &Employee) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// Puerto de salida: API externa
#[async_trait]
pub trait ExternalApiPort: Send + Sync {
    async fn fetch_country_info(&self, country: &str) -> Result<serde_json::Value, DomainError>;
}
```

---

## ⚙️ Unidad 2 — Application: Casos de Uso (CRUD)

```rust
// src/application/employee/create_employee.rs
use std::sync::Arc;
use crate::domain::{
    entities::{Employee, Department},
    value_objects::{Email, HashedPassword},
    ports::EmployeeRepository,
    errors::DomainError,
};

pub struct CreateEmployeeInput {
    pub name: String,
    pub department: Department,
    pub email: String,
    pub password: String,
    pub salary: f64,
}

pub struct CreateEmployeeUseCase {
    repository: Arc<dyn EmployeeRepository>,
}

impl CreateEmployeeUseCase {
    pub fn new(repository: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: CreateEmployeeInput) -> Result<Employee, DomainError> {
        // Validar email
        let email = Email::new(&input.email)?;

        // Verificar duplicado
        if self.repository.find_by_email(email.value()).await?.is_some() {
            return Err(DomainError::DuplicateEmail);
        }

        // Hash de contraseña
        let password = HashedPassword::from_plain(&input.password)?;

        // Crear entidad
        let employee = Employee::new(
            input.name,
            input.department,
            email,
            password,
            input.salary,
        );

        // Persistir
        self.repository.save(&employee).await?;

        Ok(employee)
    }
}
```

---

## 🗄️ Unidad 2 — Infraestructura: Adaptador SQLite

```rust
// src/infrastructure/persistence/sqlite_employee_repo.rs
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::domain::{
    entities::{Employee, Department},
    ports::EmployeeRepository,
    errors::DomainError,
};

pub struct SqliteEmployeeRepository {
    pool: SqlitePool,
}

impl SqliteEmployeeRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::query(include_str!("schema.sql"))
            .execute(&pool)
            .await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl EmployeeRepository for SqliteEmployeeRepository {
    async fn save(&self, employee: &Employee) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO employees (id, name, department, email, password_hash, salary, active)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(employee.id.to_string())
        .bind(&employee.name)
        .bind(format!("{:?}", employee.department))
        .bind(&employee.email)
        .bind(&employee.password_hash)
        .bind(employee.salary)
        .bind(employee.active)
        .execute(&self.pool)
        .await
        .map_err(|_| DomainError::DuplicateEmail)?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Employee>, DomainError> {
        // implementación completa con sqlx::query_as!
        todo!()
    }

    // ... resto de métodos
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Employee>, DomainError> { todo!() }
    async fn find_by_email(&self, _email: &str) -> Result<Option<Employee>, DomainError> { todo!() }
    async fn update(&self, _employee: &Employee) -> Result<(), DomainError> { todo!() }
    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> { todo!() }
}
```

---

## 🌐 Unidad 3 — Consumo de API Externa

```rust
// src/infrastructure/external_api/country_api_adapter.rs
use async_trait::async_trait;
use reqwest::Client;
use crate::domain::{ports::ExternalApiPort, errors::DomainError};

pub struct RestCountriesAdapter {
    client: Client,
    base_url: String,
}

impl RestCountriesAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://restcountries.com/v3.1".to_string(),
        }
    }
}

#[async_trait]
impl ExternalApiPort for RestCountriesAdapter {
    async fn fetch_country_info(&self, country: &str) -> Result<serde_json::Value, DomainError> {
        let url = format!("{}/name/{}", self.base_url, country);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|_| DomainError::InvalidEmail("Error de red".to_string()))?;

        let data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|_| DomainError::InvalidEmail("Error al deserializar".to_string()))?;

        Ok(data)
    }
}
```

---

## 🔒 Autenticación — Use Case Login

```rust
// src/application/auth/login.rs
use std::sync::Arc;
use crate::domain::{
    ports::EmployeeRepository,
    value_objects::HashedPassword,
    errors::DomainError,
    entities::Employee,
};

pub struct LoginUseCase {
    repository: Arc<dyn EmployeeRepository>,
}

impl LoginUseCase {
    pub fn new(repository: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, email: &str, plain_password: &str) -> Result<Employee, DomainError> {
        let employee = self.repository
            .find_by_email(email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // Verificar hash — nunca comparar texto plano
        let hashed = HashedPassword::from_stored(&employee.password_hash);
        if !hashed.verify(plain_password) {
            return Err(DomainError::InvalidCredentials);
        }

        Ok(employee)
    }
}
```

---

## 🚀 main.rs — Composición (Dependency Injection manual)

```rust
// src/main.rs
use std::sync::Arc;
use dotenvy::dotenv;

mod domain;
mod application;
mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or("sqlite:./sistema.db".to_string());

    // Infraestructura
    let repo = Arc::new(
        infrastructure::persistence::SqliteEmployeeRepository::new(&db_url).await?
    );

    // Casos de uso (inyección de dependencias)
    let create_employee = application::employee::CreateEmployeeUseCase::new(repo.clone());
    let login = application::auth::LoginUseCase::new(repo.clone());

    // CLI / UI
    infrastructure::cli::run(create_employee, login).await?;

    Ok(())
}
```

---

## 🧪 Tests

```rust
// tests/employee_use_cases_test.rs
use std::sync::Arc;
use tokio::test;

// Mock del repositorio para tests unitarios
struct MockEmployeeRepo {
    employees: std::sync::Mutex<Vec<Employee>>,
}

#[tokio::test]
async fn test_crear_empleado_con_email_invalido() {
    let repo = Arc::new(MockEmployeeRepo::new());
    let use_case = CreateEmployeeUseCase::new(repo);

    let result = use_case.execute(CreateEmployeeInput {
        name: "Juan".to_string(),
        email: "no-es-un-email".to_string(),
        password: "secure123".to_string(),
        ..Default::default()
    }).await;

    assert!(matches!(result, Err(DomainError::InvalidEmail(_))));
}

#[tokio::test]
async fn test_password_no_se_almacena_en_texto_plano() {
    let password = "mi_password_secreto";
    let hashed = HashedPassword::from_plain(password).unwrap();

    assert_ne!(hashed.value(), password);
    assert!(hashed.verify(password));
    assert!(!hashed.verify("password_incorrecto"));
}
```

---

## 🗺️ Mapa de Unidades → Implementación Rust

| Unidad | Requisito del curso | Implementación en Rust |
|--------|--------------------|-----------------------|
| U1 | Diagrama de clases UML | Entidades + Value Objects en `domain/` |
| U2 | CRUD + POO + DB | Use Cases + SQLx adapter |
| U3 | Auth hash + consumo API + deserialización | Argon2 + Reqwest + Serde |
| U4 | Sistema integrado + metodología ágil | Composición en `main.rs` + tests |

---

## 📚 Patrones utilizados

| Patrón | Dónde | Para qué |
|--------|-------|----------|
| **Repository** | `domain/ports/` | Abstrae la persistencia |
| **Use Case / Interactor** | `application/` | Encapsula lógica de negocio |
| **Value Object** | `domain/value_objects/` | Tipos con validación (Email, Hash) |
| **Adapter** | `infrastructure/` | Implementaciones concretas (SQLite, API) |
| **Dependency Injection** | `main.rs` | Composición sin acoplamiento |
| **Result / Error Handling** | Toda la app | Manejo explícito de errores con `thiserror` |

---

## ⚡ Comandos útiles

```bash
# Crear proyecto
cargo new sistema_gestion

# Compilar
cargo build

# Tests
cargo test

# Ejecutar con logs
RUST_LOG=debug cargo run

# Verificar sin compilar
cargo check

# Formatear código
cargo fmt

# Linter
cargo clippy
```

---

> 💡 **Tip**: En Rust no hay herencia clásica como en Python. Se reemplaza con **Traits** (equivalente a interfaces/protocolos) y **Composición**. La arquitectura hexagonal encaja perfectamente con este paradigma.