use std::sync::Arc;

use axum::extract::FromRef;

use crate::{infra::config::AppConfig, core::user::UserService, core::page::PageService, core::tenant::TenantService};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserService>,
    pub page_use_cases: Arc<PageService>,
    pub tenant_use_cases: Arc<TenantService>,
}

impl FromRef<AppState> for Arc<TenantService> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.tenant_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<PageService> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.page_use_cases.clone()
    }
}