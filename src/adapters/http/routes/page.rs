use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post, routing::get};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::http::app_state::AppState, app_error::AppResult, core::page::PageService,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_page))
        .route("/list", get(list_pages))
}

#[derive(Debug, Clone, Deserialize)]
struct CreatePagePayload {
    title: String,
    type_code: String,
    template: String,
}

#[derive(Debug, Clone, Serialize)]
struct CreatePageResponse {
    success: bool,
}

#[instrument(skip(page_use_cases))]
async fn create_page(
    State(page_use_cases): State<Arc<PageService>>,
    Json(payload): Json<CreatePagePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create page called");
    page_use_cases
        .create_page(&payload.title, &payload.type_code, &payload.template)
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(CreatePageResponse { success: true }),
    ))
}

#[instrument(skip(page_use_cases))]
async fn list_pages(
    State(page_use_cases): State<Arc<PageService>>,
) -> AppResult<impl IntoResponse> {
    info!("List pages called");
    let pages = page_use_cases.list_pages().await?;
    Ok(Json(pages))
}