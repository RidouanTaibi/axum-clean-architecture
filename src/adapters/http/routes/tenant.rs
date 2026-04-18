use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post, routing::get};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::http::app_state::AppState, app_error::AppResult, core::tenant::TenantService,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_tenant))
        .route("/list", get(list_tenants))
}  

#[derive(Debug, Clone, Deserialize)]
struct CreateTenantPayload {
    name: String,
}

#[derive(Debug, Clone, Serialize)]
struct CreateTenantResponse {
    success: bool,
}

#[instrument(skip(tenant_use_cases))]
async fn create_tenant(
    State(tenant_use_cases): State<Arc<TenantService>>,
    Json(payload): Json<CreateTenantPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create tenant called");
    tenant_use_cases.create_tenant(&payload.name).await?;
    Ok((    StatusCode::CREATED,
        Json(CreateTenantResponse { success: true }),
    ))
}

#[instrument(skip(tenant_use_cases))]
async fn list_tenants(
    State(tenant_use_cases): State<Arc<TenantService>>,
) -> AppResult<impl IntoResponse> {
    info!("List tenants called");
    let tenants = tenant_use_cases.list_tenants().await?;
    Ok(Json(tenants))
}