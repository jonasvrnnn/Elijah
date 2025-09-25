mod template;
mod properties;

use axum::{routing::{delete, get, post, put}, Router};

use crate::{blocks::config::get_properties, AppState};

pub mod config;
pub mod set_0;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/list", get(config::list))
        .route("/testpage", get(config::testpage))
        .route("/tree", get(config::testpage_tree))
        .route("/add-element", post(config::create_new_element))
        .route("/delete-element/{element_id}", delete(config::delete_element))
        .route("/{element_id}/properties", get(config::get_properties))
        .route("/{element_id}/properties", put(config::update_properties))
        .route("/{element_id}/move", put(config::move_item))
        .route("/pages", get(config::pages))
        .route("/pages", post(config::create_page))
}