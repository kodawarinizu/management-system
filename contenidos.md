# 📚 Qué estudiar para aplicar la Guía Rust — Por Unidad

> Cada sección indica exactamente qué conceptos necesitas dominar
> antes de escribir el código correspondiente en la guía principal.

---

## 🧱 Unidad 1 — Dominio: Entidades y Value Objects

### Archivos que vas a escribir
- `domain/entities/employee.rs`
- `domain/value_objects/email.rs`
- `domain/value_objects/password_hash.rs`
- `domain/errors.rs`

---

### Qué estudiar

**1. Structs y enums en Rust**
Rust no tiene clases. Una `struct` es el equivalente a una clase sin métodos, y `impl` es donde van los métodos. Un `enum` puede tener variantes con datos adentro, no solo constantes como en Python.

```rust
struct Employee { ... }   // datos
impl Employee { ... }     // métodos
```

**2. El patrón Newtype**
Es la técnica detrás de `Email(String)` — envolver un tipo primitivo en una struct para darle identidad y validación propia. Sin esto no entenderás por qué `Email` no es simplemente un `String`.

**3. Ownership y borrowing básico**
Para entender por qué en `value()` retornamos `&str` en vez de `String`, y por qué usamos `&self` vs `self`. Es el concepto más distinto de Rust respecto a Python.

Conceptos clave: `&T` (referencia), `String` vs `&str`, `clone()`, `to_string()`.

**4. Result<T, E> y el operador `?`**
Rust no tiene excepciones. Los errores se retornan como valores con `Result<Ok, Err>`. El operador `?` propaga el error automáticamente (equivalente a `raise` en Python pero explícito).

```rust
fn new(value: &str) -> Result<Self, DomainError> { ... }
```

**5. Derives y traits básicos**
Las anotaciones `#[derive(Debug, Clone, PartialEq)]` implementan automáticamente comportamientos. Necesitas saber qué significa cada uno y cuándo usarlos.

**6. `thiserror` para errores tipados**
La librería `thiserror` permite definir enums de error con mensajes legibles. Es la forma profesional de manejar errores de dominio en vez de usar `String` o `Box<dyn Error>`.

---

### Recursos sugeridos
- The Rust Book: Cap. 3 (tipos), Cap. 4 (ownership), Cap. 6 (enums), Cap. 9 (errores)
- Rustlings: ejercicios `variables`, `structs`, `enums`, `error_handling`

---

## ⚙️ Unidad 2 — Ports, Use Cases y Base de Datos

### Archivos que vas a escribir
- `domain/ports/employee_repository.rs`
- `application/employee/create_employee.rs`
- `infrastructure/persistence/sqlite_employee_repo.rs`

---

### Qué estudiar

**1. Traits**
Son el reemplazo de las interfaces y la herencia de Python. Un `trait` define un contrato (qué métodos debe tener algo), sin decir cómo. `impl MiTrait for MiStruct` es donde implementas ese contrato.

```rust
trait EmployeeRepository {
    fn save(&self, e: &Employee) -> Result<(), DomainError>;
}
```

Esto es exactamente lo que hace `ABC` + `abstractmethod` en Python, pero con chequeo en tiempo de compilación.

**2. Trait objects y `dyn`**
Para poder usar el trait como tipo (polimorfismo), se usa `dyn Trait`. Y para compartirlo entre partes del código sin saber en tiempo de compilación qué implementación es, se usa `Arc<dyn Trait>`.

```rust
repository: Arc<dyn EmployeeRepository>  // cualquier cosa que implemente el trait
```

**3. `Arc` y `Rc` — punteros inteligentes**
`Arc<T>` permite compartir un valor entre múltiples dueños de forma segura en contexto async. Es el equivalente a pasar un objeto por referencia en Python, pero controlado por el compilador.

**4. Programación async/await**
Tokio es el runtime async de Rust. Necesitas entender `async fn`, `.await`, y por qué las operaciones de DB y HTTP son async. El macro `#[tokio::main]` convierte `main` en async.

```rust
async fn execute(&self, ...) -> Result<Employee, DomainError> {
    self.repository.save(&employee).await?;
}
```

**5. `async_trait`**
Los traits con métodos async requieren la librería `async-trait` (por limitaciones del compilador actual). El macro `#[async_trait]` resuelve esto automáticamente.

**6. `sqlx` — queries asíncronos**
`sqlx` permite ejecutar SQL con tipos verificados en compilación. Lo básico que necesitas: `SqlitePool::connect()`, `sqlx::query().bind().execute()`, y `sqlx::query_as!()` para mapear resultados a structs.

**7. Inyección de dependencias manual**
En Rust no hay un framework de DI como en Python. La DI se hace pasando el repositorio como argumento al constructor del use case. Entender este patrón es clave para toda la arquitectura.

---

### Recursos sugeridos
- The Rust Book: Cap. 10 (traits), Cap. 15 (smart pointers), Cap. 16 (async básico)
- Documentación de `sqlx`: https://docs.rs/sqlx
- Rustlings: ejercicios `traits`, `smart_pointers`

---

## 🌐 Unidad 3 — Autenticación, API Externa y Deserialización

### Archivos que vas a escribir
- `domain/value_objects/password_hash.rs` (completar)
- `infrastructure/external_api/country_api_adapter.rs`
- `application/auth/login.rs`

---

### Qué estudiar

**1. `serde` — serialización y deserialización**
`serde` es el equivalente a `json` + Pydantic en Python. Con `#[derive(Serialize, Deserialize)]` una struct puede convertirse a/desde JSON automáticamente. Para tipos personalizados se puede implementar manualmente.

```rust
#[derive(Serialize, Deserialize)]
struct CountryInfo {
    name: String,
    capital: Vec<String>,
}
```

**2. `serde_json::Value` — JSON dinámico**
Cuando la estructura del JSON externo no se conoce o varía, se usa `serde_json::Value` (equivalente a `dict` en Python). Permite navegar el JSON con indexación.

**3. `reqwest` — cliente HTTP**
Es el equivalente a `httpx` o `requests` en Python pero async. Lo básico: crear un `Client`, hacer `get(url).send().await`, y parsear la respuesta con `.json::<T>().await`.

**4. `argon2` — hash seguro**
Argon2 es el algoritmo recomendado actualmente para hashear contraseñas (más seguro que bcrypt). Necesitas entender: `SaltString` (sal aleatoria), `hash_password()` y `verify_password()`. Nunca guardar texto plano, siempre comparar hash.

**5. Variables de entorno con `dotenvy`**
Equivalente a `python-dotenv`. Carga el archivo `.env` y permite acceder a variables con `std::env::var("NOMBRE")`. Importante para no hardcodear URLs de DB, claves API, etc.

**6. Manejo de errores con `anyhow`**
`anyhow` se usa en el nivel de aplicación/infraestructura cuando no necesitas errores tipados específicos. Permite usar `?` con cualquier tipo de error y agregar contexto con `.context("mensaje")`.

La diferencia con `thiserror`: `thiserror` para errores de dominio (precisos), `anyhow` para infraestructura (flexible).

**7. `map_err()` — transformar errores**
Como los errores de `reqwest`, `sqlx` y `argon2` son tipos distintos al `DomainError`, necesitas convertirlos. `map_err(|e| DomainError::AlgunVariante)` transforma un error en otro.

---

### Recursos sugeridos
- Documentación de `serde`: https://serde.rs
- Documentación de `reqwest`: https://docs.rs/reqwest
- The Rust Book: Cap. 9 (errores avanzado)
- Rustlings: ejercicios `conversions`

---

## 🚀 Unidad 4 — Composición, Tests y Sistema Integrado

### Archivos que vas a escribir
- `main.rs` (composición final)
- `tests/employee_use_cases_test.rs`
- `infrastructure/cli/menu.rs`

---

### Qué estudiar

**1. El sistema de módulos de Rust**
`mod nombre;` declara un módulo desde un archivo. `use crate::dominio::...` importa desde la raíz del proyecto. Es diferente a Python donde `import` busca archivos directamente. Necesitas entender `mod.rs`, `pub`, `pub(crate)`, y el árbol de módulos.

**2. Tests unitarios e integración en Rust**
Rust tiene testing integrado sin librerías externas. Los tests unitarios van en el mismo archivo con `#[cfg(test)]`. Los tests de integración van en la carpeta `tests/`. Para tests async se usa `#[tokio::test]`.

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn mi_test() { ... }
}
```

**3. Mocks manuales con traits**
Como los repositorios son traits, puedes crear implementaciones falsas (mocks) solo para tests sin tocar la DB real. Es la ventaja directa de la arquitectura hexagonal — el dominio no sabe si habla con SQLite o con un `Vec` en memoria.

**4. `#[tokio::main]` y configuración del runtime**
El macro convierte `main` en async y levanta el runtime de Tokio. Necesitas entender que todo el código async debe ejecutarse dentro de este contexto.

**5. Lifetimes básicos**
En algunos puntos del código pueden aparecer lifetime annotations (`'a`). No necesitas dominarlos completamente, pero sí entender que existen y qué problema resuelven (garantizar que referencias no vivan más que los datos que apuntan).

**6. `cargo test`, `cargo clippy`, `cargo fmt`**
El toolchain de Rust incluye: `clippy` (linter que sugiere mejoras), `fmt` (formateador automático), `test` (runner de tests). Usarlos es práctica profesional estándar.

**7. Logging con `tracing`**
`tracing` es el estándar de logging async en Rust. `tracing_subscriber::fmt::init()` activa los logs. Se usan macros como `tracing::info!()`, `tracing::error!()`, `tracing::debug!()`.

---

### Recursos sugeridos
- The Rust Book: Cap. 7 (módulos), Cap. 11 (tests), Cap. 19 (lifetimes avanzado)
- Rustlings: ejercicios `modules`, `lifetimes`
- Documentación de `tracing`: https://docs.rs/tracing

---

## 🗺️ Resumen visual — Qué estudiar por archivo

```
domain/value_objects/email.rs
    → Structs, Newtype pattern, Result<T,E>, &str vs String

domain/value_objects/password_hash.rs
    → argon2, SaltString, map_err()

domain/entities/employee.rs
    → Structs, impl blocks, enums, derive macros, ownership

domain/errors.rs
    → thiserror, enums de error

domain/ports/employee_repository.rs
    → Traits, async_trait, dyn Trait

application/employee/create_employee.rs
    → Arc<dyn Trait>, async/await, inyección de dependencias

infrastructure/persistence/sqlite_employee_repo.rs
    → sqlx, async, impl Trait for Struct

infrastructure/external_api/country_api_adapter.rs
    → reqwest, serde_json::Value, async

application/auth/login.rs
    → Combinación de todo lo anterior

main.rs
    → Módulos, tokio::main, composición DI

tests/
    → #[cfg(test)], #[tokio::test], mocks con traits
```

---

## 📖 Orden de estudio recomendado

1. **Rust básico** — variables, tipos, funciones, control de flujo
2. **Ownership** — el concepto más importante y único de Rust
3. **Structs + impl + enums** — base de U1
4. **Result y manejo de errores** — base de toda la app
5. **Traits** — equivalente a interfaces, base de la arquitectura hexagonal
6. **Async/await + Tokio** — necesario para DB y HTTP
7. **serde** — serialización para U3
8. **sqlx** — base de datos para U2
9. **reqwest + argon2** — HTTP y seguridad para U3
10. **Módulos y tests** — organización final para U4

> 💡 The Rust Book (https://doc.rust-lang.org/book/) es gratuito, oficial y cubre todo esto en orden. Rustlings (https://github.com/rust-lang/rustlings) son ejercicios prácticos que complementan cada capítulo.
```