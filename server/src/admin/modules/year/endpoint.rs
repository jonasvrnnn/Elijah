use axum::response::Result;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
};
use maud::Markup;
use serde::Deserialize;

use crate::admin::modules::project::template::page_buttons_update_wrapper;
use crate::admin::DBTransaction;
use crate::{AppState, TransactionError};

use super::{service, template::project_year};

#[derive(Deserialize)]
pub struct UpdateProjectYearBody {
    year: Option<i64>,
}

pub async fn update_project_year(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<UpdateProjectYearBody>,
) -> Result<Markup> {
    service::update_project_year(&mut transaction, &project_id, body.year)
        .await
        .map_err(|err| {
            eprintln!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = project_year(&project_id, &body.year);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}
