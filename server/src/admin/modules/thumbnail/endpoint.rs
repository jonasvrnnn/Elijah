use axum::{
    extract::Path, http::StatusCode, response::Result, Form
};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::{project::template::page_buttons_update_wrapper, thumbnail}, Company, DBTransaction}, TransactionError};

use super::{service, template};

#[derive(Deserialize)]
enum ThumbnailSource {
    #[serde(rename="base")]
    Base,
    #[serde(rename="headerphoto")]
    Headerphoto,
    #[serde(rename="custom")]
    Custom
}

#[derive(Deserialize)]
pub struct SetThumbnail {
    source: ThumbnailSource,
    image: Option<String>
}

pub async fn set_thumbnail(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<Option<String>>,
    Form(body): Form<SetThumbnail>,
) -> Result<Markup> {
    let value = match body.source {
        ThumbnailSource::Base => {
            if company.is_none() {
                return Err(StatusCode::BAD_REQUEST.into());
            }
            Some("base".to_string())
        },
        ThumbnailSource::Headerphoto => None,
        ThumbnailSource::Custom => {
            let image = body.image;

            if image.is_none() {
                return Err(StatusCode::BAD_REQUEST.into());
            }

            image
        },
    };

    let thumbnail = service::set_thumbnail(&mut *transaction, &project_id, &company, &value)
        .await
        .map_err(|err| {
            eprintln!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::thumbnail_template(&project_id, &thumbnail, false);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}
