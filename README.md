# Management System — Rust

Personal learning project. Not intended for production or commercial use.

Exploring Rust by applying hexagonal architecture (Ports & Adapters) in an employee management system with CRUD, authentication, and external API consumption.

## Stack

- **Rust** + Tokio (async runtime)
- **SQLx** + PostgreSQL 18
- **Argon2** for password hashing
- **Reqwest** for HTTP client (external APIs)
- **Serde** + serde_json for serialization / deserialization
- **thiserror** + anyhow for error handling
- **UUID** for typed entity IDs
- **dotenvy** for environment variables
- **tracing** + tracing-subscriber for logging

## Structure

```
domain/          → entities, value objects, traits
application/     → use cases
infrastructure/  → adapters (PostgreSQL, external API, CLI)
```

## Run

```bash
cp .env.example .env   # set DATABASE_URL
cargo run
cargo test
```

---

*Work in progress — learning Rust.*
