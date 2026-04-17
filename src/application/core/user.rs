use std::sync::Arc;

use async_trait::async_trait;
use secrcy::{SecretString, exposeSecret};
use tracing::{info, instrument};

use crate::app_error::AppResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        hashed_password: &str,
    ) -> AppResult<()>;
}

pub trait UserCredentialHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
}

#[derive(Clone)]
pub struct UserService {
    hasher: Arc<dyn UserCredentialHasher>,
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(hasher: Arc<dyn UserCredentialHasher>, repository: Arc<dyn UserRepository>) -> Self {
        Self { hasher, repository }
    }

    #[instrument(skip(self))]
    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &SecretString,
    ) -> AppResult<()> {
        info!("Registering user: {}", username);

        let hashed_password = self.hasher.hash_password(exposeSecret(&password))?;
        self.repository
            .create_user(username, email, &hashed_password)
            .await?;
        info!("User registered successfully");
        Ok(())
    }
}
