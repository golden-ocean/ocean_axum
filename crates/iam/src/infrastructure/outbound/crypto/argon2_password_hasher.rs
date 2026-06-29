use argon2::{
    Argon2,
    password_hash::{
        PasswordHash as ParsedHash, PasswordHasher as _, PasswordVerifier, SaltString,
        rand_core::OsRng,
    },
};

use crate::{
    application::ports::outbound::crypto::{PasswordHashError, PasswordHasher},
    domain::value_object::user::{Password, PasswordHash},
};

#[derive(Default)]
pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);

        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| PasswordHashError::HashFailed)?
            .to_string();

        Ok(PasswordHash::new(hash))
    }

    fn verify(&self, password: &Password, hash: &str) -> Result<(), PasswordHashError> {
        let parsed = ParsedHash::new(hash).map_err(|_| PasswordHashError::VerifyFailed)?;

        let ok = self
            .argon2
            .verify_password(password.as_bytes(), &parsed)
            .is_ok();

        if ok {
            Ok(())
        } else {
            Err(PasswordHashError::VerifyFailed)
        }
    }
}
