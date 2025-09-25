use axum::{extract::{Path, State}, http::StatusCode, response::Result, Form};
use maud::Markup;
use serde::Deserialize;


use crate::{admin::{modules::project::template::page_buttons_update_wrapper, DBTransaction}, AppState, TransactionError};

use super::{service as service, template::project_location};

#[derive(Deserialize)]
pub struct UpdateProjectLocation {
    location: Option<String>
}

pub async fn update_project_location(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateProjectLocation>
) -> Result<Markup> {
    service::update_project_location(&mut transaction, &project_id, &body.location).await.map_err(|err| {
        eprintln!("Could not update the location for project {project_id}: {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update the location.")
    })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = project_location(&project_id, &body.location);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}