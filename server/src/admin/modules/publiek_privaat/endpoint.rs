
use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, DBTransaction}, AppState, TransactionError};

use super::{service as service, template};

#[derive(Deserialize)]
pub struct SetPubliekPrivaatBody {
    r#type: String
}

pub async fn set_publiek_privaat(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<SetPubliekPrivaatBody>
) -> Result<Markup> {
    let r#type = service::set_publiek_privaat(&mut *transaction, &project_id, &body.r#type).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::publiek_privaat_template(&project_id, &r#type, false);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}