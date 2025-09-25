use axum::{
    Form, Json,
    extract::{Path, State},
    http::StatusCode,
    response::Result,
};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

use crate::{
    AppState, TransactionError,
    admin::{
        Company, DBTransaction,
        modules::{image::service::ImageData, project::template::page_buttons_update_wrapper},
    },
};

use super::service;
use super::template;

#[derive(Deserialize)]
pub struct AddImageBodyEntry {
    image: Option<String>,
    copyright: Option<String>,
    alt: Option<String>,
}

#[derive(Deserialize)]
pub struct AddImageBody {
    images: Vec<AddImageBodyEntry>
}

pub async fn add_images_to_collection(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<Option<String>>,
    Json(body): Json<AddImageBody>,
) -> Result<Markup> {
    let mut content_items = vec![];

    for image in body.images {
        let image_data = ImageData {
            id: None,
            image: image.image,
            image_copyright: image.copyright,
            alt: image.alt,
        };

        let image_data = service::add_image(&mut *transaction, &project_id, &company, &image_data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let content = template::lightbox_image(&project_id, &image_data);

        content_items.push(content);
    }

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    let content = html!(@for item in content_items {
        (item)
    });
    
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

pub async fn delete_image_to_collection(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path((project_id, image_id)): Path<(String, String)>,
) -> Result<Markup> {
    service::delete_image(&mut *transaction, &image_id).await;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(page_buttons_update_wrapper(
        &project_id,
        true,
        &PreEscaped("".to_string()),
    ))
}

pub async fn customise(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>,
) -> Result<Markup> {
    let images = service::customise(&mut *transaction, &project_id, &company)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    let content = template::lightbox(&project_id, &images, false);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}
