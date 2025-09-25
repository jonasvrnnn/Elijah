use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, Company, DBTransaction}, AppState, TransactionError};

use super::{service::{self as service, get_header_data}, template::project_header};

#[derive(Deserialize)]
pub struct UpdateHeaderPhotoBody {
    image: Option<String>,
    copyright: Option<String>
}

pub async fn update_header_photo(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<Option<String>>,
    Form(body): Form<UpdateHeaderPhotoBody>
) -> Result<Markup> {
    service::update_project_header_photo(&mut transaction, &project_id, &company, &body.image, &body.copyright).await;

    let header_data = get_header_data(&mut transaction, &project_id, &company).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = project_header(&project_id, &header_data.name, &header_data.header_photo, &header_data.header_photo_copyright, !header_data.custom_header_photo.unwrap_or(false), company.is_some());
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

pub async fn customise_header_photo(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>
) -> Result<Markup> {
    let data = service::customise_project_header_photo(&mut transaction, &project_id, &company).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = project_header(&project_id, "", &data.header_photo, &data.header_photo_copyright, false, true);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}