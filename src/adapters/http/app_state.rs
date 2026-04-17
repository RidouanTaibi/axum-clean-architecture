use std::sync::Arc;

use axum::extract::FromRef;

use crate::{infra::config::AppConfig, core::user::UserService};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserService>,
}

impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_cases.clone()
    }
}