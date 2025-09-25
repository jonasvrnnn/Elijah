use axum::{
    extract::Path, http::StatusCode, response::Result, Form
};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::{labels::service::{get_labels, set_labels}, project::template::page_buttons_update_wrapper, thumbnail}, Company, DBTransaction}, TransactionError};

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
pub struct UpdateLabelsBody {
    #[serde(default)]
    labels: Vec<String>
}

pub async fn update_labels(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    axum_extra::extract::Form(body): axum_extra::extract::Form<UpdateLabelsBody>,
) -> Result<Markup> {
    set_labels(&mut transaction, &project_id, &body.labels).await.unwrap();

    let labels = get_labels(&mut transaction, &project_id, true).await.unwrap();

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::labels_template(&project_id, &labels, false);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}
