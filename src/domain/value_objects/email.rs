use crate::domain::errors::DomainError;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Email(String);
impl Email {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

        match email_regex.is_match(value) {
            true => Ok(Self(value.to_string())),
            false => Err(DomainError::InvalidEmail(value.to_string())),
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
