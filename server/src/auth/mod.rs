use axum::{http::StatusCode, middleware::from_fn_with_state, routing::{delete, get, patch, post, put}, Router};

use crate::{auth::service::auth_middleware, AppState};

pub mod template;
pub mod service;
pub mod endpoint;
pub mod models;
pub mod dto;

async fn ok() -> StatusCode {
    StatusCode::OK
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/verify-token", get(ok).layer(from_fn_with_state(state.clone(), auth_middleware)))
        .route("/login", get(endpoint::get_login_page))
        .route("/login", post(endpoint::login))
        .route("/totp/verify", post(endpoint::totp_verify))
        .route("/logout", delete(endpoint::logout))
        .route("/users", get(endpoint::get_users).layer(from_fn_with_state(state, auth_middleware)))     
        .route("/users", post(endpoint::create_user))     
        .route("/users/{user_id}/permissions", get(endpoint::get_permissions))
        .route("/users/{user_id}/permissions", patch(endpoint::set_permission))
        .route("/users/{user_id}", put(endpoint::update_user_data))
        .route("/users/{user_id}", delete(endpoint::delete_user))
}