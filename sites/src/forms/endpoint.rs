use axum::{Form, extract::State, http::HeaderMap, response::IntoResponse};
use axum_extra::extract::Host;
use lettre::{
    Message, Transport,
    message::{Mailbox, header::ContentType},
};
use maud::html;
use reqwest::StatusCode;

use crate::{
    AppState,
    db_connection_extractor::SqliteConnectionExtractor,
    forms::{recaptcha, service, structs::FormBody, template},
};

pub async fn form(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    State(state): State<AppState>,
    headers: HeaderMap,
    Host(host): Host,
    Form(body): Form<FormBody>,
) -> impl IntoResponse {
    let company = match state.company_data.domain_to_company(&host) {
        Some(company) => company,
        None => {
            eprintln!("Could not find a company for the following domain: {host}.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let db_entry_id = match service::add_entry_to_db(
        &mut conn,
        &body.first_name,
        &body.last_name,
        &body.email,
        &body.phone,
        &body.message,
        &company,
        &None,
    )
    .await
    {
        Ok(db_entry_id) => db_entry_id,
        Err(err) => {
            eprintln!("{err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let recipient = match state.company_data.domain_to_form_address(&host) {
        Some(recipient) => recipient,
        None => {
            eprintln!("Could not find a form recipient for the following domain: {host}.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let recipient_mailbox: Mailbox = match recipient.parse() {
        Ok(recipient_mailbox) => recipient_mailbox,
        Err(err) => {
            eprintln!("Could not parse the form recipient: {err}.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let sender: Mailbox = match "info@tophat.be".parse() {
        Ok(sender) => sender,
        Err(err) => {
            eprintln!("Could not parse the form sender: {err}.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let token = match headers.get("recaptcha") {
        Some(token) => match token.to_str() {
            Ok(token) => token,
            Err(err) => {
                eprintln!("The recaptcha token could not be read as a string: {err}.");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        None => {
            eprintln!("The recaptcha token was not provided.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let score = match recaptcha::verify_token(&state.recaptcha_secret_key, token).await {
        Ok(score) => score,
        Err(err) => {
            eprintln!("The recaptcha token could not be verified: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if service::set_recaptcha_score(&mut *conn, &db_entry_id, (score * 100.0).floor() as u8)
        .await
        .is_err()
    {
        eprintln!(
            "Failed to update recaptcha_score in the database for entry {db_entry_id}, continuing anyway, since the token was verified."
        );
    }

    if score < 0.7 {
        eprintln!("The recaptcha score was {score}, which is lower than the minimum (.7)");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let body = template::form_notification(&body);

    let email = match Message::builder()
        .from(sender)
        .to(recipient_mailbox)
        .subject("Bericht op de website")
        .header(ContentType::TEXT_HTML)
        .body(body.into_string())
    {
        Ok(message) => message,
        Err(err) => {
            eprintln!("The message could not be build: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(match state.forms_mailer.send(&email) {
        Ok(_) => {
            if service::set_sent_to(&mut *conn, &db_entry_id, &Some(recipient.to_string()))
                .await
                .is_err()
            {
                eprintln!(
                    "Failed to update sent_to in the database for entry {db_entry_id}, but the email was sent anyway."
                );
            }
            println!("Email {db_entry_id} sent successfully");
            html!(
                h2 { "Bedankt voor je interesse" }
                small {"Iedere samenwerking start met een fijn gesprek. Tot snel!" }
            )
        }
        Err(e) => {
            eprintln!("Failed to send email: {e}");
            html!(
                h2 { "Er ging iets mis..." }
                small {"Probeer het later opnieuw." }
            )
        }
    })
}
