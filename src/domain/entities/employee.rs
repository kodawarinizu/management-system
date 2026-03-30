use std::fmt::{self, write};
use std::str::FromStr;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub enum Departament {
    Engineering,
    Sale,
    RRHH,
    Finance,
    Operations,
}

impl fmt::Display for Departament {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Departament::Engineering => write!(f, "Engineering"),
            Departament::Sale => write!(f, "Sale"),
            Departament::RRHH => write!(f, "RRHH"),
            Departament::Finance => write!(f, "Finance"),
            Departament::Operations => write!(f, "Operations"),
        }
    }
}

impl fmt::Debug for Departament {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Departament::Engineering => write!(f, "Engineering"),
            Departament::Sale => write!(f, "Sale"),
            Departament::RRHH => write!(f, "RRHH"),
            Departament::Finance => write!(f, "Finance"),
            Departament::Operations => write!(f, "Operations"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub name:  String,
    pub departament: Departament,
    pub email: String,
    pub password_hash: String,
    pub salary: Decimal,
    pub active: bool,
}