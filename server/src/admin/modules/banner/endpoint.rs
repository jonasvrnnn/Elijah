use axum::http::StatusCode;
use axum::response::Result;
use axum::{extract::Path, response::IntoResponse, Form};
use maud::Markup;
use serde::Deserialize;

use crate::admin::{modules::project::template::page_buttons_update_wrapper, Company, DBTransaction};
use crate::TransactionError;

use super::service;
use super::template;

#[derive(Deserialize)]
pub struct UpdateBannerPhotoBody {
    image: Option<String>,
    copyright: Option<String>
}

pub async fn update_banner_photo(
    Path(project_id): Path<String>,
    Company(company): Company<Option<String>>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateBannerPhotoBody>
) -> Result<Markup> {
    let data = service::update_project_banner_photo(&mut transaction, &project_id, &company, &body.image, &body.copyright).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = template::project_banner(&project_id, &data.banner_photo, &data.banner_photo_copyright, false);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

pub async fn customise_banner_photo(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>
) -> Result<Markup> {
    let data = service::customise_project_banner_photo(&mut transaction, &project_id, &company).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = template::project_banner(&project_id, &data.banner_photo, &data.banner_photo_copyright, false);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}