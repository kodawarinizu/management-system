use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Departament {
    Engineering,
    Sale,
    RRHH,
    Finance,
    Operations,
}

pub struct Employee {
    pub id: Uuid,
    pub name:  String,
    pub departament: Departament,
    pub email: String,
    pub password_hash: String,
    pub salary: Decimal,
    pub active: bool,
}