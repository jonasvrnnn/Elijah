use lettre::{transport::smtp::authentication::{Credentials, Mechanism}, SmtpTransport};
use sqlx::SqliteConnection;
use uuid::Uuid;

#[derive(Debug)]
pub enum FormsInitError {
    UsernameNotFound,
    PasswordNotFound
}

pub fn init() -> Result<SmtpTransport, FormsInitError> {
    let username = std::env::var("SMTP_USERNAME").map_err(|_| FormsInitError::UsernameNotFound)?;
    let password = std::env::var("SMTP_PASSWORD").map_err(|_| FormsInitError::PasswordNotFound)?;
    
    let creds = Credentials::new(
        username,
        password,
    );

    let mailer = SmtpTransport::builder_dangerous("smtp-auth.mailprotect.be")
        .credentials(creds)
        .authentication(vec![Mechanism::Login])
        .build();

    Ok(mailer)
}

pub async fn add_entry_to_db(conn: &mut SqliteConnection, first_name: &str, last_name: &str, email: &str, phone: &str, message: &str, company: &str, sent_to: &Option<String>) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query!("INSERT INTO FORM_SUBMISSIONS(
        id,
        first_name,
        last_name,
        email,
        phone,
        message,
        company,
        sent_to
    ) VALUES (
        $1,$2,$3,$4,$5,$6,$7,$8
    )", id, first_name, last_name, email, phone, message, company, sent_to)
    .execute(conn)
    .await?;

    Ok(id)
}

pub async fn set_recaptcha_score(conn: &mut SqliteConnection, id: &str, recaptcha_score: u8) -> Result<(), sqlx::Error> {    
    sqlx::query!("UPDATE FORM_SUBMISSIONS SET recaptcha_score=$1 where id=$2", recaptcha_score, id)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn set_sent_to(conn: &mut SqliteConnection, id: &str, sent_to: &Option<String>) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE FORM_SUBMISSIONS SET sent_to=$1 where id=$2", sent_to, id)
    .execute(conn)
    .await
    .map(|_| ())
}
