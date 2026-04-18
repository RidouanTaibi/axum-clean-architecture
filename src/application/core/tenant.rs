use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::app_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
}

#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create_tenant(&self, name: &str) -> AppResult<()>;

    async fn list_tenants(&self) -> AppResult<Vec<Tenant>>;
}

#[derive(Clone)]
pub struct TenantService {
    repository: Arc<dyn TenantRepository>,
}

impl TenantService {
    pub fn new(repository: Arc<dyn TenantRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn create_tenant(&self, name: &str) -> AppResult<()> {
        info!("Creating tenant: {}", name);
        self.repository.create_tenant(name).await?;
        info!("Tenant created successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_tenants(&self) -> AppResult<Vec<Tenant>> {
        info!("Listing tenants");
        self.repository.list_tenants().await
    }
}
