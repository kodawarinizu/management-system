use argon2::{
     password_hash::{
         rand_core::OsRng,
         PasswordHash, PasswordHasher, PasswordVerifier, SaltString
     },
     Argon2
};

use crate::domain::errors::DomainError;
pub struct HashedPassword(String);
impl HashedPassword {
    pub fn new (value: &str) -> Result<Self, DomainError>{
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
        .hash_password(value.as_bytes(), &salt);
        
        match hash {
            Ok(v) => Ok(Self(v.to_string())), 
            Err(e) => Err(DomainError::HashError(e.to_string()))
        }
    }
    pub fn verify(&self, value: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(&self.0)
            .map_err(|e| DomainError::HashError(e.to_string()))?;

        Ok(Argon2::default()
            .verify_password(value.as_bytes(), &parsed_hash).is_ok())


    }

    pub fn value(&self) -> &str {
        &self.0
    }
}