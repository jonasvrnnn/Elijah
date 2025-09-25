use ::chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, query_as, query_scalar, types::chrono, SqliteConnection};

#[derive(FromRow)]
pub struct FormSubmission {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub sent_to: Option<String>,
    pub recaptcha_score: Option<i64>,
    pub email: String,
    phone: String,
    message: String,
    pub company: String,
    pub datetime: NaiveDateTime
}

pub async fn get_submissions(conn: &mut SqliteConnection) -> Result<Vec<FormSubmission>, sqlx::Error> {
    query_as(r#"SELECT * from form_submissions"#)
    .fetch_all(&mut *conn)
    .await
}

pub async fn get_message(conn: &mut SqliteConnection, id: &String) -> Result<String, sqlx::Error> {
    query_scalar!("SELECT message from form_submissions WHERE id=$1", id)
    .fetch_one(&mut *conn)
    .await
}