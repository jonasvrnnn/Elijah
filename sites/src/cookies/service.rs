use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use sqlx::{query_as, SqliteConnection};

#[derive(Debug)]
pub enum UpdateCookieConsentError {
    TimeWentBackwards,
    UpdateError(sqlx::Error)
}

#[derive(Deserialize, Default)]
pub struct CookieData {
    #[serde(default)]
    pub analytics: bool
}

pub async fn update_cookie_consent(
    conn: &mut SqliteConnection,
    ip: &str,
    data: &CookieData
) -> Result<CookieData, UpdateCookieConsentError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|err| UpdateCookieConsentError::TimeWentBackwards)?.as_secs_f64();
    query_as!(CookieData, "INSERT INTO COOKIE_CONSENT(ip, date_modified, analytics) VALUES($1, $2, $3) ON CONFLICT(ip) DO UPDATE SET date_modified=excluded.date_modified, analytics=excluded.analytics RETURNING analytics", ip, now, data.analytics)
    .fetch_one(conn)
    .await
    .map_err(|err| UpdateCookieConsentError::UpdateError(err))
}

pub async fn get_cookie_consent(
    conn: &mut SqliteConnection,
    ip: &str
) -> Result<Option<CookieData>, sqlx::Error> {
    query_as!(CookieData, "SELECT analytics FROM COOKIE_CONSENT WHERE ip=$1", ip)
    .fetch_optional(conn)
    .await
}