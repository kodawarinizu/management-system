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
    pub fn verify(&self) -> bool {
        true
    }
}