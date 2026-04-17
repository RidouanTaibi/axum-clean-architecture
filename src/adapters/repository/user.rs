use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::repository::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::user::User,
    core::user::UserRepository,
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDb {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
            username: user_db.username,
            email: user_db.email,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at.and_utc(),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresPersistence {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            uuid,
            username,
            email,
            password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}