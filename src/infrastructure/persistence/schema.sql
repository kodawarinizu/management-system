CREATE TABLE IF NOT EXISTS employees (
    id  UUID PRIMARY KEY,
    name TEXT NOT NULL,
    departament TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    salary DECIMAL NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);