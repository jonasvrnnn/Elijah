use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{auth::service::auth_middleware, forms::endpoint::{get_message, get_submissions}, AppState};

pub mod endpoint;
mod service;
mod template;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/submissions", get(get_submissions).layer(from_fn_with_state(state.clone(), auth_middleware)))
        .route("/submissions/{message_id}/message", get(get_message).layer(from_fn_with_state(state.clone(), auth_middleware)))
}