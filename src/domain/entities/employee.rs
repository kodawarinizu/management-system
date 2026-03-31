use std::fmt;
use std::str::FromStr;
use rust_decimal::Decimal;
use sqlx::{FromRow, Row};
use uuid::Uuid;

use crate::domain::errors::DomainError;

#[derive(Clone, PartialEq)]
pub enum Departament {
    Engineering,
    Sale,
    RRHH,
    Finance,
    Operations,
}




// * Temporaly i don't want to use this trait
impl FromStr for Departament {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Engineering" => Ok(Departament::Engineering),
            "Sale" => Ok(Departament::Sale),
            "RRHH" => Ok(Departament::RRHH),
            "Finance" => Ok(Departament::Finance),
            "Operations" => Ok(Departament::Operations),
            _ => Err(DomainError::DepartamentError(format!("Enum does exist.'{}'", s))),
        }
    }
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


#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub id: Uuid,
    pub name:  String,
    pub departament: Departament,
    pub email: String,
    pub password_hash: String,
    pub salary: Decimal,
    pub active: bool,
}

//*! sqlx: Manual mapping for Employee entity 
impl FromRow<'_, sqlx::postgres::PgRow> for Employee {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let deps: String = row.try_get("departament")?;
        let departament: Departament = deps.parse::<Departament>()
        .map_err(|e| sqlx::Error::TypeNotFound { type_name: e.to_string() })?;

        Ok( Self { 
        id: row.try_get("id")?, 
        name: row.try_get("name")?, 
        departament: departament,
        email: row.try_get("email")?, 
        password_hash: row.try_get("password_hash")?, 
        salary: row.try_get("salary")?, 
        active: row.try_get("active")? 
        
        })
    }

}
