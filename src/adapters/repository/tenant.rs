use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::repository::PostgresPersistence,
    app_error::{AppError, AppResult},
    core::tenant::TenantRepository,
    entities::tenant::Tenant,
};

// Tenant struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct TenantDb {
    pub id: Uuid,
    pub uuid: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<TenantDb> for Tenant {
    fn from(tenant_db: TenantDb) -> Self {
        Tenant {
            id: tenant_db.id,
            uuid: tenant_db.uuid,
            name: tenant_db.name,
            created_at: tenant_db.created_at.and_utc(),
        }
    }
}

#[async_trait]
impl TenantRepository for PostgresPersistence {
    async fn create_tenant(&self, name: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO tenants (id, uuid, name) VALUES ($1, $2, $3)",
            Uuid::new_v4(),
            uuid,
            name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn list_tenants(&self) -> AppResult<Vec<crate::core::tenant::Tenant>> {
        let tenants =
            sqlx::query_as::<_, TenantDb>("SELECT id, uuid, name, created_at FROM tenants")
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::from)?;

        Ok(tenants
            .into_iter()
            .map(|t| crate::core::tenant::Tenant {
                id: t.id,
                name: t.name,
            })
            .collect())
    }
}
