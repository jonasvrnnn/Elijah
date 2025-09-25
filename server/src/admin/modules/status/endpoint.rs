use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, DBTransaction}, TransactionError};

use super::{service as service, template};

fn default_as_false() -> bool {
    false
}

#[derive(Deserialize)]
pub struct SetStateBody {
    #[serde(default="default_as_false")]
    status: bool
}

pub async fn set_state(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<SetStateBody>
) -> Result<Markup> {
    let status = service::set_status(&mut *transaction, &project_id, body.status).await.map_err(|err| {
        eprintln!("Failed to update the status for project {project_id}: {err}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update the status")
    })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::status_template(&project_id, status, false);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}