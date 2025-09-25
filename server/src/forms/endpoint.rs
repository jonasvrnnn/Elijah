use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Result;
use maud::Markup;

use crate::admin::DBConnection;

use super::service;
use super::template;

pub async fn get_submissions(
    DBConnection(mut connection): DBConnection
) -> Result<Markup> {
    let submissions = service::get_submissions(&mut *connection).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::submissions_overview(&submissions))
}

pub async fn get_message(
    DBConnection(mut connection): DBConnection,
    Path(message_id): Path<String>
) -> Result<String> {
    let message = service::get_message(&mut *connection, &message_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(message)
}