use std::fmt;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::domain::errors::DomainError;

#[derive(Clone)]
pub struct HashedPassword(String);
impl HashedPassword {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default().hash_password(value.as_bytes(), &salt);

        match hash {
            Ok(v) => Ok(Self(v.to_string())),
            Err(e) => Err(DomainError::HashError(e.to_string())),
        }
    }

    pub fn from_hash(hash: &str) -> Result<Self, DomainError> {
        let hash = PasswordHash::new(&hash).map_err(|_| DomainError::HashError("".to_string()))?;
        Ok(Self(hash.to_string()))
    }

    pub fn verify(&self, value: &str) -> Result<bool, DomainError> {
        let parsed_hash =
            PasswordHash::new(&self.0).map_err(|e| DomainError::HashError(e.to_string()))?;

        Ok(Argon2::default()
            .verify_password(value.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for HashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HashedPassword([REDACTED])") // oculto en producción
    }
}
