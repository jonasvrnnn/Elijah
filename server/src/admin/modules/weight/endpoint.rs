use axum::{
    extract::{Path, Query}, http::{StatusCode}, response::Result, Form
};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{
    modules::project::template::page_buttons_update_wrapper, DBTransaction,
}, TransactionError};

use super::{service, template::project_weight};

#[derive(Deserialize)]
pub struct UpdateWeightBody {
    weight: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateWeightQuery {
    company: String,
}

pub async fn update_project_weight(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<UpdateWeightQuery>,
    Form(body): Form<UpdateWeightBody>,
) -> Result<Markup> {
    service::update_weight(&mut transaction, &project_id, &query.company, &body.weight)
        .await
        .map_err(|err| {
            eprintln!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = project_weight(&project_id, &Some(query.company), &body.weight);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}
