use std::{
    collections::HashMap,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use crate::{
    AppState,
    admin::{DBConnection, DBTransaction, draft::create_project_draft_if_necessary},
};

use super::models::{Permission, User, UserBase};
use axum::{
    extract::{Path, Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response, Result},
};
use axum_extra::extract::CookieJar;
use core::convert::TryFrom;
use core::time::Duration;
use otpauth::TOTP;
use pasetors::keys::AsymmetricPublicKey;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::AsymmetricSecretKey,
    public,
    token::UntrustedToken,
    version4::V4,
};
use rand::{Rng, distr::Alphanumeric};
use serde::Deserialize;
use sqlx::{SqliteConnection, query_as, query_scalar};
use uuid::Uuid;

pub fn verify_token(token: &Option<String>) -> bool {
    if let Some(token) = token { true } else { false }
}

pub fn generate_totp_uri(secret: &str) -> String {
    let totp = TOTP::new(secret);
    totp.to_uri("GVR Content Editor", "Top Hat")
}

pub async fn get_user_totp_secret(
    conn: &mut SqliteConnection,
    id: &str,
) -> Result<Option<String>, sqlx::Error> {
    query_scalar!("SELECT totp_secret from USERS WHERE id=$1", id)
        .fetch_one(conn)
        .await
}

pub fn verify_totp(secret: &str, code: u32) -> Result<bool, SystemTimeError> {
    let totp = TOTP::new(secret);

    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    Ok(totp.verify(code, 30, timestamp1))
}

pub async fn get_user_from_email(
    conn: &mut SqliteConnection,
    email: &str,
) -> Result<User, sqlx::Error> {
    query_as!(User, "SELECT * from USERS WHERE email=$1", email)
        .fetch_one(conn)
        .await
}

pub async fn get_user_from_id(
    conn: &mut SqliteConnection,
    email: &str,
) -> Result<User, sqlx::Error> {
    query_as!(User, "SELECT * from USERS WHERE id=$1", email)
        .fetch_one(conn)
        .await
}

pub fn generate_totp_secret() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub async fn set_user_totp_secret(
    conn: &mut SqliteConnection,
    email: &str,
    secret: &str,
) -> Result<(), sqlx::Error> {
    query_scalar!(
        "UPDATE USERS SET totp_secret=$1 WHERE email=$2",
        secret,
        email
    )
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn get_permissions(
    conn: &mut SqliteConnection,
    user: &str,
) -> Result<Vec<Permission>, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        SELECT
            company,
            p.`create`,
            p.edit
        FROM
            PERMISSIONS p
        WHERE
            company IS NULL
            AND user = $1
        UNION
        SELECT
            NULL AS company,
            NULL AS `create`,
            NULL AS edit
        WHERE
            NOT EXISTS (
                SELECT
                    1
                FROM
                    PERMISSIONS p
                WHERE
                    p.company IS NULL
                    AND user = $1
            )
        UNION
        SELECT
            name as company,
            "create",
            edit
        FROM
            COMPANIES c
            LEFT JOIN PERMISSIONS p ON c.name = p.company
            AND user = $1
    "#,
        user
    )
    .fetch_all(conn)
    .await
}

pub async fn get_users(conn: &mut SqliteConnection) -> Result<Vec<UserBase>, sqlx::Error> {
    sqlx::query_as!(
        UserBase,
        "
        SELECT
            id,
            email, 
            first_name, 
            last_name,
            role from USERS"
    )
    .fetch_all(conn)
    .await
}

pub enum CreateUserError {
    SQL(sqlx::Error),
    HashError,
}

pub async fn create_user(
    conn: &mut SqliteConnection,
    first_name: &str,
    last_name: &str,
    email: &str,
    role: &str,
    password: &str,
) -> Result<UserBase, CreateUserError> {
    let id = Uuid::new_v4().to_string();

    let hash = match bcrypt::hash_with_result(password, 12) {
        Ok(hash) => hash.to_string(),
        Err(err) => {
            eprintln!("Failed to create a hash: {err}");
            return Err(CreateUserError::HashError);
        }
    };

    sqlx::query_as!(
        UserBase,
        "
        INSERT INTO USERS(
            id,
            email, 
            first_name, 
            last_name,
            password,
            role) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id, email, first_name, last_name, role",
        id,
        email,
        first_name,
        last_name,
        hash,
        role
    )
    .fetch_one(conn)
    .await
    .map_err(|err| CreateUserError::SQL(err))
}

pub async fn update_permission(
    conn: &mut SqliteConnection,
    user: &str,
    company: &Option<String>,
    create: bool,
    edit: bool,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(Permission, r#"
        INSERT INTO PERMISSIONS(user, company, `create`, edit) VALUES($1, $2, $3, $4) ON CONFLICT(user, IFNULL(company, '')) DO UPDATE SET `create`=$3, edit=$4 WHERE user=$1 AND (company=$2 OR (company IS NULL AND $2 IS NULL)) RETURNING company, `create`, edit"#, user, company, create, edit)
    .fetch_one(conn)
    .await
}

pub async fn update_user_data(
    conn: &mut SqliteConnection,
    user: &str,
    email: &Option<String>,
    role: &Option<String>,
) -> Result<UserBase, sqlx::Error> {
    sqlx::query_as!(UserBase, r#"
        UPDATE USERS SET email=COALESCE($1, email), role=COALESCE($2, role) WHERE id=$3 RETURNING id, email, first_name, last_name, role"#, email, role, user)
    .fetch_one(conn)
    .await
}

pub async fn delete_user(conn: &mut SqliteConnection, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM USERS where id=$1", id)
        .execute(conn)
        .await
        .map(|_| ())
}

pub enum GenerateTokenForUserId {
    Pasetors(pasetors::errors::Error),
    SystemTimeError,
}

pub fn generate_token_for_user_id(
    user_id: &str,
    private_key: &AsymmetricSecretKey<V4>,
) -> Result<String, GenerateTokenForUserId> {
    let mut claims = Claims::new().map_err(|err| GenerateTokenForUserId::Pasetors(err))?;
    claims
        .subject(user_id)
        .map_err(|err| GenerateTokenForUserId::Pasetors(err))?;
    claims
        .issuer("tophat.be")
        .map_err(|err| GenerateTokenForUserId::Pasetors(err))?;
    claims
        .set_expires_in(&Duration::from_secs(60 * 60 * 24 * 7))
        .map_err(|err| GenerateTokenForUserId::Pasetors(err))?;
    claims
        .add_additional(
            "last-action",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| GenerateTokenForUserId::SystemTimeError)?
                .as_secs(),
        )
        .map_err(|err| GenerateTokenForUserId::Pasetors(err))?;

    Ok(
        pasetors::public::sign(private_key, &claims, None, Some(b"implicit assertion"))
            .map_err(|err| GenerateTokenForUserId::Pasetors(err))?,
    )
}

#[derive(Clone, Debug)]
pub struct UserData {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct MiddlewareQuery {
    role: Option<String>,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    DBConnection(mut connection): DBConnection,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Response {
    let token = match jar.get("token") {
        Some(token) => token.value(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let (id, last_action) = match validate_paseto(token, &state.paseto.public_key) {
        Ok(val) => val,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let ip = match headers.get("X-Forwarded-For") {
        Some(val) => match val.to_str() {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("Could not parse {val:?} as a string.");
                return (StatusCode::BAD_REQUEST, "Could not get parse client's ip address.").into_response()
            },
        },
        None => return (StatusCode::BAD_REQUEST, "Could not get the client's ip address.").into_response(),
    };

    let required_role = headers.get("required_role").map(|rr| rr.to_str());

    let role = match required_role {
        Some(required_role) => match required_role {
            Ok(required_role) => required_role,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse the required role.").into_response(),
        },
        None => "user",
    };

    let auth_data = match get_user_auth_data(&mut connection, &id, &ip).await {
        Ok(auth_data) => auth_data,
        Err(err) => {
            eprintln!("Failed to get the user auth data for user {id}: {err}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        },
    };

    if role == "admin" && auth_data.role != "admin" {
        return StatusCode::FORBIDDEN.into_response();
    }

    let now = match SystemTime::now()
        .duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(err) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

    let secs_multiplier = (24 * 60 * 60) as u64;
    let secs_until_2fa = (last_action as u64) + 3 * secs_multiplier;
    let secs_until_reauth = (last_action as u64) + 7 * secs_multiplier;

    if now > secs_until_reauth {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    if now > secs_until_2fa {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    req.extensions_mut().insert(UserData { id });

    let res = next.run(req).await;
    res
}

pub async fn create_draft_middleware(
    Path(params): Path<HashMap<String, String>>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    req: Request,
    next: Next,
) -> Response {
    let project_id = match params.get("project_id") {
        Some(project_id) => project_id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "The path did not match the expected format (could not find the project id).",
            )
                .into_response();
        }
    };

    if let Err(err) = create_project_draft_if_necessary(&mut transaction, &project_id).await {
        eprintln!("Could not create a draft for project {project_id}: {err}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create a draft version of this project.",
        )
            .into_response();
    };

    if let Err(err) = transaction.commit().await {
        eprintln!("Could not commit a database transaction: {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response();
    }

    next.run(req).await
}

pub fn validate_paseto(
    token: &str,
    public_key: &AsymmetricPublicKey<pasetors::version4::V4>,
) -> Result<(String, u64)> {
    let mut validation_rules = ClaimsValidationRules::new();
    validation_rules.allow_non_expiring();
    validation_rules.validate_issuer_with("tophat.be");

    let untrusted_token =
        UntrustedToken::<pasetors::Public, pasetors::version4::V4>::try_from(token).map_err(|t| StatusCode::INTERNAL_SERVER_ERROR)?;

    let trusted_token = match public::verify(
        public_key,
        &untrusted_token,
        &validation_rules,
        None,
        Some(b"implicit assertion"),
    ) {
        Ok(trusted_token) => trusted_token,
        Err(err) => {
            eprintln!("{err}");
            return Err((
                StatusCode::UNAUTHORIZED,
                "Failed to verify the PASETO token",
            )
                .into_response()
                .into());
        }
    };

    let id = match trusted_token.payload_claims() {
        Some(claims) => match claims.get_claim("sub") {
            Some(id) => match id.as_str() {
                Some(id) => id.to_string(),
                None => return Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
            },
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Failed to verify the PASETO token",
                )
                    .into_response()
                    .into());
            }
        },
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Failed to verify the PASETO token",
            )
                .into_response()
                .into());
        }
    };

    let last_action = match trusted_token.payload_claims() {
        Some(claims) => match claims.get_claim("last-action") {
            Some(last_action) => match last_action.as_u64() {
                Some(last_action) => last_action,
                None => return Ok((id, 0)),
            },
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Failed to verify the PASETO token",
                )
                    .into_response()
                    .into());
            }
        },
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Failed to verify the PASETO token",
            )
                .into_response()
                .into());
        }
    };

    Ok((id, last_action))
}

#[derive(Debug)]
pub struct UserAuthData {
    pub id: String,
    pub role: String,
    pub trusted_ip: bool,
}

pub async fn get_user_auth_data(
    conn: &mut SqliteConnection,
    user_id: &str,
    ip: &str,
) -> Result<UserAuthData, sqlx::Error> {
    query_as!(UserAuthData, r#"SELECT id, role, EXISTS(SELECT 1 FROM TRUSTED_IPS WHERE user_id=id AND ip=$2) as "trusted_ip!: bool" FROM USERS WHERE id=$1"#, user_id, ip)
    .fetch_one(conn)
    .await
}
