use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::{Markup, PreEscaped};
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, DBTransaction}, AppState, TransactionError};

use super::{service as service, template};

pub async fn create_core_number(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path(project_id): Path<String>
) -> Result<Markup> {
    let core_number = service::create_core_number(&mut *transaction, &project_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::core_number_template(&project_id, &core_number);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}

#[derive(Deserialize)]
pub struct UpdateCoreNumberBody {
    title: String,
    number: String
}

pub async fn update_core_number(
    Path((project_id, core_number_id)): Path<(String, String)>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateCoreNumberBody>
) -> Result<Markup> {
    let core_number = service::update_core_number(&mut *transaction, &core_number_id, &body.number, &body.title).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::core_number_template(&project_id, &core_number);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}

pub async fn delete_core_number(
    Path((project_id, core_number_id)): Path<(String, String)>,
    DBTransaction(mut transaction): DBTransaction<'_>,
) -> Result<Markup>{
    service::delete_core_number(&mut *transaction, &core_number_id).await;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    Ok(page_buttons_update_wrapper(&project_id, true, &PreEscaped::default()))
}