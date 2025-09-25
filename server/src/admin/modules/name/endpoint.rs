use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, DBTransaction}, TransactionError};

use super::{service as service, template};

fn default_as_false() -> bool {
    false
}

#[derive(Deserialize)]
pub struct SetNameBody {
    name: String
}

pub async fn set_name(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<SetNameBody>
) -> Result<Markup> {
    let name = service::set_name(&mut *transaction, &project_id, &body.name).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::name_template(&project_id, &name, true);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}