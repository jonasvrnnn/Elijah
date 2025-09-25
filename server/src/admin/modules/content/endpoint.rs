use axum::{
    extract::Path, http::StatusCode, response::{IntoResponse, Result}, Form
};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::{page_buttons, page_buttons_update_wrapper}, Company, DBTransaction}, TransactionError};

use super::{service::{customise_content, delete_content_entry, insert_new_content_entry, update_content_entry_image, update_content_entry_quote, update_content_entry_text}, template::{content_entry_image, content_entry_quote, content_entry_template, content_entry_text, content_template}};


#[derive(Deserialize)]
pub struct CreateContentEntryFormData {
    index: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateContentEntryFormData {
    text: Option<String>,
    image: Option<String>,
}

#[derive(Deserialize)]
pub struct InsertContentEntryFormData {
    previous_id: Option<String>,
}

pub async fn insert_content_entry_endpoint(
    Path(project_id): Path<String>,
    Company(company): Company<Option<String>>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<InsertContentEntryFormData>,
) -> Result<Markup> {
    let content_entry = insert_new_content_entry(
        &mut *transaction,
        &project_id,
        &company,
        &body.previous_id
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = content_entry_template(&project_id, &content_entry, false);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

pub async fn delete_content_entry_endpoint(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path((project_id, id)): Path<(String, String)>,
) -> Result<Markup> {
    delete_content_entry(&mut *transaction, &id).await;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    Ok(page_buttons(&project_id, true, true))
}


#[derive(Deserialize)]
pub struct UpdateContentEntryQuote {
    content: Option<String>,
}


pub async fn update_content_entry_quote_endpoint(
    Path((project_id, id)): Path<(String, String)>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateContentEntryQuote>
) -> Result<Markup> {
    let content = match body.content {
        Some(content) => {
            if content.is_empty() {
                None
            } else {
                Some(content)
            }
        },
        None => None
    };

    let new_content = update_content_entry_quote(&mut *transaction, &id, &content).await.map_err(|err| {
        eprintln!("Failed to update the content entry's quote: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = content_entry_quote(&project_id, &id, &new_content);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}



#[derive(Deserialize)]
pub struct UpdateContentEntryImage {
    image: Option<String>,
    copyright: Option<String>
}


pub async fn update_content_entry_image_endpoint(
    Path((project_id, id)): Path<(String, String)>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateContentEntryImage>
) -> Result<Markup> {
    let image = match body.image {
        Some(image) => {
            if image.is_empty() {
                None
            } else {
                Some(image)
            }
        },
        None => None
    };

    let new_image = update_content_entry_image(&mut *transaction, &id, &image, &body.copyright).await.map_err(|err| {
        eprintln!("Failed to update the content entry's image: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = content_entry_image(&project_id, &id, &new_image.image, &new_image.image_copyright);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

#[derive(Deserialize)]
pub struct UpdateContentEntryText {
    content: String,
}


pub async fn update_content_entry_text_endpoint(
    Path((project_id, id)): Path<(String, String)>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateContentEntryText>
) -> Result<Markup> {
    let new_text = update_content_entry_text(&mut *transaction, &id, &body.content).await.map_err(|err| {
        eprintln!("Failed to update the content entry's text: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = content_entry_text(&project_id, &id, &new_text);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

#[derive(Deserialize)]
pub struct CustomiseContentQuery {
    company: Option<String>
}

pub async fn customise_content_endpoint(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>
) -> Result<Markup> {
    let content = customise_content(&mut transaction, &project_id, &company).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = content_template(&project_id, &content, false);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}