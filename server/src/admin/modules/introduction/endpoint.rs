use axum::{extract::{Path, Query}, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::{Markup, PreEscaped};
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, Company, DBTransaction}, TransactionError};

use super::{service, template::introduction_template};

#[derive(Deserialize)]
pub struct UpdateIntroductionQuery {
    company: Option<String>
}

#[derive(Deserialize)]
pub struct UpdateIntroductionBody {
    content: Option<String>
}

pub async fn update_introduction(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<UpdateIntroductionQuery>,
    Form(body): Form<UpdateIntroductionBody>
) -> Result<Markup> {
    let content = body.content;

    service::update_introduction(&mut transaction, &project_id, &query.company, &content).await;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = content.map(|c| PreEscaped(c)).unwrap_or_default();
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

#[derive(Deserialize)]
pub struct CustomiseIntroductionQuery {
    company: Option<String>
}

pub async fn customise_introduction(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>
) -> Result<Markup> {
    let introduction = service::customise_introduction(&mut transaction, &project_id, &company).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = introduction_template(&project_id, &Some(company), &introduction, false);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}