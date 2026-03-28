use crate::domain::errors::DomainError;



#[derive(Debug)]
pub struct Email(String);
impl Email {
    pub fn new (value: &str) -> Result<Self, DomainError> {
        if value.contains('@') && value.len() > 5 {
            Ok(Self(value.to_lowercase()))
        }
        else {
            Err(DomainError::InvalidEmail(value.to_string()))
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

