use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
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

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
}

#[derive(Clone)]
pub struct UserService {
    hasher: Arc<dyn UserCredentialsHasher>,
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(hasher: Arc<dyn UserCredentialsHasher>, repository: Arc<dyn UserRepository>) -> Self {
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

        let hash = &self.hasher.hash_password(password.expose_secret())?;
        self.repository.create_user(username, email, hash).await?;

        // let hashed_password = self.hasher.hash_password(exposeSecret(&password))?;
        // self.repository
        //     .create_user(username, email, &hashed_password)
        //     .await?;
        info!("User registered successfully");
        Ok(())
    }
}
