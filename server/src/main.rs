#[macro_use] extern crate markup5ever;

mod admin;
mod auth;
mod blocks;
mod forms;

use std::{
    fs::{self, File},
    io::{Sink, Write},
};

use axum::{Router, http::StatusCode, response::IntoResponse};

use core::convert::TryFrom;
use maud::{Markup, html};
use pasetors::paserk::FormatAsPaserk;
use pasetors::{
    keys::{AsymmetricKeyPair, AsymmetricPublicKey, AsymmetricSecretKey, Generate},
    version4::V4,
};
use sqlx::SqlitePool;

use crate::blocks::config::Block;

fn static_stylesheet(file: &str) -> Markup {
    html!(
        link rel="stylesheet" href={"/static/"(file)};
    )
}

fn static_script(file: &str, module: bool) -> Markup {
    let r#type = if module { Some("module") } else { None };

    html!(
        script type=[r#type] src={"/static/"(file)} {}
    )
}

fn self_replacing_svg(file: &str) -> Markup {
    html!(
        svg hx-get=(file) hx-trigger="load" hx-swap="outerHTML" hx-target="this" hx-cache="true" {}
    )
}

#[derive(Clone)]
struct Paseto {
    private_key: AsymmetricSecretKey<V4>,
    public_key: AsymmetricPublicKey<V4>,
}

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    paseto: Paseto,
}

enum TransactionError {
    Begin,
    Commit,
}

impl IntoResponse for TransactionError {
    fn into_response(self) -> axum::response::Response {
        match self {
            TransactionError::Begin => {
                eprintln!("Failed to begin a database transaction.");
            }
            TransactionError::Commit => {
                eprintln!("Failed to commit a database transaction.");
            }
        };

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let key_path = "./keys/paseto";

    let private_key_file = fs::read_to_string(key_path);
    let public_key_file = fs::read_to_string(&format!("{key_path}.pub"));

    let (private_key_bytes, public_key_bytes) = match (private_key_file, public_key_file) {
        (Ok(private_key_bytes), Ok(public_key_bytes)) => (private_key_bytes, public_key_bytes),
        _ => {
            let kp =
                AsymmetricKeyPair::<V4>::generate().expect("Failed to generate a PASETO keypair.");
            let mut private_key_string = String::new();
            let mut public_key_string = String::new();
            kp.secret
                .fmt(&mut private_key_string)
                .expect("Failed to format the private key to a string.");
            kp.public
                .fmt(&mut public_key_string)
                .expect("Failed to format the public key to a string.");

            let mut private_key_file =
                File::create(key_path).expect("Failed to format the create the private key file.");
            private_key_file
                .write(private_key_string.as_bytes())
                .expect("Failed to format the write to the private key file.");
            let mut public_key_file = File::create(&format!("{key_path}.pub"))
                .expect("Failed to format the create the public key file.");
            public_key_file
                .write(public_key_string.as_bytes())
                .expect("Failed to format the write to the public key file.");

            (private_key_string, public_key_string)
        }
    };

    let private_key = AsymmetricSecretKey::<V4>::try_from(private_key_bytes.as_str())
        .expect("Failed to parse the PASETO private key.");
    let public_key = AsymmetricPublicKey::<V4>::try_from(public_key_bytes.as_str())
        .expect("Failed to parse the PASETO public key.");
    let database_url = std::env::var("DATABASE_URL")
        .expect("Failed to find the DATABASE_URL environment variable.");
    let port =
        std::env::var("ADMIN_PORT").expect("Failed to find the ADMIN_PORT environment variable.");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to start a database pool.");
    let paseto = Paseto {
        private_key,
        public_key,
    };

    let state = AppState { pool, paseto };

    let app = Router::new()
        .merge(admin::router(state.clone()))
        .nest("/auth", auth::router(state.clone()))
        .nest("/forms", forms::router(state.clone()))
        .nest("/blocks", blocks::router(state.clone()))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .expect("Failed to create a TCP listener.");
    axum::serve(listener, app)
        .await
        .expect("Failed to start Axum.");
}
