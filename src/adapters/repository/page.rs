use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::repository::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::page::Page,
    core::page::PageRepository,
};

// Page struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct PageDb {
    pub id: Uuid,
    pub uuid: Uuid,
    pub title: String,
    pub type_code: String,
    pub template: String,
    pub created_at: NaiveDateTime,
}

impl From<PageDb> for Page {
    fn from(page_db: PageDb) -> Self {
        Page {
            id: page_db.id,
            uuid: page_db.uuid,
            title: page_db.title,
            type_code: page_db.type_code,
            template: page_db.template,
            created_at: page_db.created_at.and_utc(),
        }
    }
}

#[async_trait]
impl PageRepository for PostgresPersistence {
    async fn create_page(
        &self,
        title: &str,
        type_code: &str,
        template: &str,
    ) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO pages (id, uuid, title, type_code, template) VALUES ($1, $2, $3, $4, $5)",
            Uuid::new_v4(),
            uuid,
            title,
            type_code,
            template
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn list_pages(&self) -> AppResult<Vec<crate::core::page::Page>> {
        let pages = sqlx::query_as::<_, PageDb>("SELECT * FROM pages")
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(pages.into_iter().map(|p| crate::core::page::Page {
            id: p.id,
            title: p.title,
            type_code: p.type_code,
            template: p.template,
        }).collect())
    }
}