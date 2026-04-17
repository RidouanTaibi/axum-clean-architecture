use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{
    app_error::{AppError, AppResult},
    core::user::UserCredentialHasher,
};

#[derive(Default)]
pub struct ArgonPasswordHasher {
    hasher: Argon2<'static>,
}

impl UserCredentialHasher for ArgonPasswordHasher {
    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::internal("Password hashing failed".into()))?
            .to_string();
        Ok(hash)
    }
}
