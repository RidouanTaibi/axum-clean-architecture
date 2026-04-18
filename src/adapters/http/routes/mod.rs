pub mod tenant;
pub mod user;
pub mod page;

use axum::Router;

use crate::adapters::http::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::router())
        .nest("/page", page::router())
}