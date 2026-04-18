use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::app_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub type_code: String,
    pub template: String,
}

#[async_trait]
pub trait PageRepository: Send + Sync {
    async fn create_page(
        &self,
        title: &str,
        type_code: &str,
        template: &str,
    ) -> AppResult<()>;

    async fn list_pages(&self) -> AppResult<Vec<Page>>;
}

#[derive(Clone)]
pub struct PageService {
    repository: Arc<dyn PageRepository>,
}

impl PageService {
    pub fn new(repository: Arc<dyn PageRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn create_page(
        &self,
        title: &str,
        type_code: &str,
        template: &str,
    ) -> AppResult<()> {
        info!("Creating page: {}", title);
        self.repository.create_page(title, type_code, template).await?;
        info!("Page created successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_pages(&self) -> AppResult<Vec<Page>> {
        info!("Listing pages");
        self.repository.list_pages().await
    }
}