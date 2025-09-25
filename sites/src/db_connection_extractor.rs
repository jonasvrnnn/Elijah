use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};
use reqwest::StatusCode;
use sqlx::{pool::PoolConnection, Sqlite};

use crate::AppState;

pub struct SqliteConnectionExtractor(pub PoolConnection<Sqlite>);

impl<S> FromRequestParts<S> for SqliteConnectionExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        Ok(state.db_pool.acquire().await
        .map(|conn| SqliteConnectionExtractor(conn))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not connect to the database"))?)
    }
}