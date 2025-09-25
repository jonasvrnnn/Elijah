use std::time::{SystemTime, UNIX_EPOCH};

use axum_extra::extract::{CookieJar, cookie::Cookie};

use axum::{
    Extension, Form,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response, Result},
};
use maud::{Markup, PreEscaped};
use qrcode::{QrCode, render::svg};

use cookie::time::{Duration, OffsetDateTime};

use crate::{
    AppState, TransactionError,
    admin::{DBConnection, DBTransaction},
    auth::{
        service::{UserData, generate_token_for_user_id, get_user_auth_data, validate_paseto},
        template::twofa_form,
    },
};

use super::service::{self, generate_totp_uri};

use super::dto;
use super::template;

pub async fn login(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<dto::LoginBody>,
) -> Result<Markup> {
    let user = service::get_user_from_email(&mut *transaction, &body.email)
        .await
        .map_err(|err| StatusCode::UNAUTHORIZED)?;

    let matches = bcrypt::verify(body.password, &user.password)
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    if matches {
        let mut svg = None;

        if user.totp_secret.is_none() {
            let rand_string = service::generate_totp_secret();

            service::set_user_totp_secret(&mut *transaction, &user.email, &rand_string).await;

            transaction
                .commit()
                .await
                .map_err(|_| TransactionError::Commit)?;

            let uri = service::generate_totp_uri(&rand_string);
            let code = QrCode::new(uri).map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

            svg = Some(code.render::<svg::Color>().build())
        }

        let form = twofa_form(&user.id, svg);

        Ok(form)
    } else {
        Err(StatusCode::UNAUTHORIZED.into())
    }
}

pub async fn totp_verify(
    State(state): State<AppState>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    mut jar: CookieJar,
    Form(body): Form<dto::TotpVerifyBody>,
) -> Result<(HeaderMap, axum_extra::extract::CookieJar)> {
    let secret = match service::get_user_totp_secret(&mut *transaction, &body.id)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        Some(secret) => secret,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
    };

    let valid = service::verify_totp(&secret, body.code)
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut headers = HeaderMap::new();

    if valid {
        let user = match service::get_user_from_id(&mut *transaction, &body.id).await {
            Ok(user) => user,
            Err(err) => {
                eprintln!("{err}");
                return Err(StatusCode::UNAUTHORIZED.into());
            }
        };

        let permissions = match service::get_permissions(&mut *transaction, &user.id).await {
            Ok(permissions) => permissions,
            Err(err) => {
                eprintln!("{err}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
            }
        };

        let pub_token = generate_token_for_user_id(&user.id, &state.paseto.private_key)
            .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

        headers.append(
            "HX-Redirect",
            HeaderValue::from_str("/").map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?,
        );

        let mut now = OffsetDateTime::now_utc();
        now += Duration::weeks(26);

        let mut token_cookie = Cookie::new("token", pub_token);
        token_cookie.set_http_only(true);
        token_cookie.set_expires(now);
        token_cookie.set_path("/");

        jar = jar.add(token_cookie);
    }

    Ok((headers, jar))
}

pub async fn logout(jar: CookieJar) -> Result<Response> {
    let mut token_cookie = Cookie::from("token");
    token_cookie.set_http_only(true);
    token_cookie.set_path("/");

    let mut headers = HeaderMap::new();
    headers.append(
        "HX-Redirect",
        HeaderValue::from_str("/login").map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    Ok((headers, jar.remove(token_cookie)).into_response())
}

pub async fn get_permissions(
    Path(user_id): Path<String>,
    DBConnection(mut connection): DBConnection,
) -> Result<Markup> {
    let permissions = service::get_permissions(&mut connection, &user_id)
        .await
        .map_err(|err| {
            println!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(template::permissions(&permissions, &user_id))
}

pub async fn get_users(
    DBConnection(mut connection): DBConnection,
    Extension(user_id): Extension<UserData>,
) -> Result<Markup> {
    let users = service::get_users(&mut connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::user_list(&users))
}

pub async fn set_permission(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<dto::SetPermissionQuery>,
    Form(body): Form<dto::SetPermissionBody>,
) -> Result<Markup> {
    let permission = service::update_permission(
        &mut transaction,
        &user_id,
        &query.company,
        body.create,
        body.edit,
    )
    .await
    .map_err(|err| {
        println!("{err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(template::permission_row(
        &permission,
        &user_id,
        &query.company.as_deref().unwrap_or("Base"),
        &query.company,
    ))
}

pub async fn create_user(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<dto::CreateUserBody>,
) -> Result<Markup> {
    let user = service::create_user(
        &mut transaction,
        &body.first_name,
        &body.last_name,
        &body.email,
        &body.role,
        &body.password,
    )
    .await
    .map_err(|err| {
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(template::user_template(&user))
}

pub async fn delete_user(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path(id): Path<String>,
) -> Result<()> {
    service::delete_user(&mut transaction, &id)
        .await
        .map_err(|err| {
            println!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(())
}

pub async fn update_user_data(
    Path(user_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<dto::UpdateUserDataBody>,
) -> Result<Markup> {
    let user = service::update_user_data(&mut transaction, &user_id, &body.email, &body.role)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(template::user_template(&user))
}

pub async fn get_login_page(
    headers: HeaderMap,
    State(state): State<AppState>,
    DBConnection(mut connection): DBConnection,
    jar: CookieJar,
) -> Response {
    let token = match jar.get("token") {
        Some(token) => token.value(),
        None => return (StatusCode::OK, template::login_form()).into_response(),
    };

    let (id, last_action) = match validate_paseto(token, &state.paseto.public_key) {
        Ok(id) => id,
        Err(_) => return (StatusCode::OK, template::login_form()).into_response(),
    };

    let ip = match headers.get("X-Real-IP") {
        Some(ip) => match ip.to_str() {
            Ok(ip) => ip,
            Err(_) => return (StatusCode::OK, template::login_form()).into_response(),
        },
        None => return (StatusCode::OK, template::login_form()).into_response(),
    };

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(err) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let secs_multiplier = (24 * 60 * 60) as u64;
    let secs_until_2fa = last_action + 3 * secs_multiplier;
    let secs_until_reauth = last_action + 7 * secs_multiplier;

    if now > secs_until_reauth {
        return (StatusCode::OK, template::login_form()).into_response();
    }

    if now > secs_until_2fa {
        return (StatusCode::OK, template::twofa_form(&id, None)).into_response();
    }

    return (
        StatusCode::TEMPORARY_REDIRECT,
        PreEscaped("/projects".to_string()),
    ).into_response();
}
