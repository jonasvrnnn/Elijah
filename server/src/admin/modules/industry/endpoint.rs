use axum::{
    extract::{Path, State}, http::StatusCode, response::{IntoResponse, Result}, Form
};
use axum_extra::extract::Query as ExtraQuery;
use maud::Markup;
use serde::Deserialize;
use sqlx::query_scalar;

use crate::{
    admin::{index::index, modules::project::template::page_buttons_update_wrapper, DBConnection, DBTransaction}, AppState, TransactionError
};

use super::{
    service::{self, project_industry_data},
    template::full_list,
};

use super::template;

pub async fn full_industry_list(
    DBConnection(mut connection): DBConnection,
) -> Result<Markup> {
    let industries = query_scalar!("SELECT name FROM INDUSTRIES ORDER BY name")
        .fetch_all(&mut *connection)
        .await
        .map_err(|err| {
            eprintln!("Failed to retrieve industries: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(index(&Some(full_list(&industries)), &None))
}

#[derive(Deserialize)]
pub struct AddIndustryToProjectBody {
    industry: String,
}

pub async fn add_industry_project(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<AddIndustryToProjectBody>,
) -> Result<Markup> {
    service::add_industry_to_project(&mut transaction, &project_id, &body.industry).await;

    let data = project_industry_data(&mut transaction, &project_id, true).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = template::project_industries(&project_id, &data);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

pub async fn delete_industry_project(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path((project_id, industry)): Path<(String, String)>,
) -> Result<Markup> {
    service::delete_industry_from_project(&mut transaction, &project_id, &industry).await;

    let data = project_industry_data(&mut transaction, &project_id, true).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let content = template::project_industries(&project_id, &data);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

#[derive(Deserialize)]
pub struct IndustrySearchListQuery {
    filter: Option<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

pub async fn industry_search_list(
    ExtraQuery(query): ExtraQuery<IndustrySearchListQuery>,
    DBConnection(mut connection): DBConnection
) -> Result<Markup> {
    let industries =
        service::industry_search_list(&mut connection, &query.filter, &query.exclude).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::search_list_for_existing_project(&industries))
}
