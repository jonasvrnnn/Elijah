mod projecten;
mod search;
mod forms;
mod company_data;
mod company_extractor;
mod db_connection_extractor;
mod cookies;

use lettre::SmtpTransport;
use search::search_results;

use axum::{
    routing::{get, post}, Router
};

use sqlx::SqlitePool;

use crate::{company_data::CompanyData};

use forms::service as forms_service;

#[derive(Clone)]
struct AppState {
    db_pool: SqlitePool,
    forms_mailer: SmtpTransport,
    company_data: CompanyData,
    recaptcha_secret_key: String
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("The DATABASE_URL environment variable was not provided.");
    let port = std::env::var("WEBSITES_PORT").expect("The WEBSITES_PORT environment variable was not provided.");

    let db_pool = SqlitePool::connect(&database_url).await.expect("Failed to connect to the database.");

    let forms_mailer = forms_service::init().expect("Failed to create the SMTP client.");
    let company_data = CompanyData::init().expect("Failed to create a CompanyData object.");

    let recaptcha_secret_key = std::env::var("RECAPTCHA_SECRET_KEY").expect("The RECAPTCHA_SECRET_KEY environment variable was not provided.");

    let state = AppState {
        db_pool,
        forms_mailer,
        company_data,
        recaptcha_secret_key
    };

    // build our application with a single route
    let app = Router::new()
        .route("/search", get(search_results))
        .route("/projecten", get(projecten::endpoint::projecten))
        .route("/projecten/carousel", get(projecten::endpoint::carousel))
        .route("/projecten/in-de-kijker", get(projecten::endpoint::in_de_kijker))
        .route("/projecten/{id}", get(projecten::endpoint::project))
        .route("/forms", post(forms::endpoint::form))
        .route("/cookies", get(cookies::endpoint::get_cookie_buttons))
        .route("/cookies", post(cookies::endpoint::update_cookie_consent))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .expect("Could not create the TcpListener");
    axum::serve(listener, app).await.expect("Could not start the server");
}
