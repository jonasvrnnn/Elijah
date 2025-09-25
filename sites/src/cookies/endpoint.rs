use axum::{http::HeaderMap, Form};
use maud::Markup;
use reqwest::StatusCode;

use crate::{cookies::{service::CookieData, template}, db_connection_extractor::SqliteConnectionExtractor};

pub async fn update_cookie_consent(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    headers: HeaderMap,
    Form(body): Form<CookieData>,
) -> Result<Markup, StatusCode> {
    let ip = headers
        .get("X-Real-IP")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = super::service::update_cookie_consent(&mut *conn, ip, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let template = template::cookie_items(&data, "button");

    Ok(template)
}

pub async fn get_cookie_buttons(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    headers: HeaderMap
) -> Result<Markup, StatusCode> {
    let ip = headers
        .get("X-Real-IP")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = super::service::get_cookie_consent(&mut *conn, ip)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let next_state = if data.is_some() { "button" } else { "banner" };

    let data = data.unwrap_or_default();

    let template = template::cookie_items(&data, next_state);

    Ok(template)
}
