use axum::{
    Form,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Result},
};
use axum_extra::extract::Query as ExtraQuery;
use maud::Markup;
use serde::Deserialize;

use crate::{
    admin::{
        index::index, modules::{
            project::template::page_buttons_update_wrapper, tms::service::project_tm_data_type,
        }, DBConnection, DBTransaction
    }, AppState, TransactionError
};

use super::{
    service::{self, project_party_data_type},
    template::{party_list_template, search_list_for_existing_project},
};

use super::super::tms::service as tm_service;

#[derive(Deserialize)]
pub struct PartySearchListQuery {
    filter: Option<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

pub async fn party_search_list_id(
    DBConnection(mut connection): DBConnection,
    ExtraQuery(query): ExtraQuery<PartySearchListQuery>,
) -> Result<Markup> {
    let parties = service::party_search_list(&mut connection, &query.filter, &query.exclude)
        .await
        .map_err(|err| {
            eprintln!("Failed to get the party search list: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(search_list_for_existing_project(
        &parties,
        &vec![],
        &query.filter,
    ))
}

#[derive(Deserialize)]
pub struct ProjectPartiesQuery {
    r#type: String,
    draft: Option<bool>,
}

pub async fn project_parties(
    Path(project_id): Path<String>,
    DBConnection(mut connection): DBConnection,
    Query(query): Query<ProjectPartiesQuery>,
) -> Result<Markup> {
    let data = project_party_data_type(
        &mut connection,
        &project_id,
        &query.r#type,
        query.draft.unwrap_or(false),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tms = project_tm_data_type(&mut connection, &project_id, &query.r#type, true).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    let list = party_list_template(&project_id, &data, &tms, &query.r#type);

    let content = index(&Some(list), &None);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

#[derive(Deserialize)]
pub struct AddPartyToProject {
    party: String,
    r#type: String,
}

pub async fn add_party_to_project(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<AddPartyToProject>,
) -> Result<Markup> {
    service::add_party_to_project(&mut transaction, &project_id, &body.party, &body.r#type).await;

    let parties = project_party_data_type(&mut transaction, &project_id, &body.r#type, true)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tms: Vec<tm_service::TM> =
        project_tm_data_type(&mut transaction, &project_id, &body.r#type, true).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    let content = party_list_template(&project_id, &parties, &tms, &body.r#type);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}

#[derive(Deserialize)]
pub struct DeletePartyFromProjectQuery {
    party: String,
    r#type: String,
}

pub async fn delete_party_from_project(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<DeletePartyFromProjectQuery>,
) -> Result<Markup> {
    service::delete_party_from_project(&mut transaction, &project_id, &query.party, &query.r#type)
        .await;

    let parties = project_party_data_type(&mut transaction, &project_id, &query.r#type, true)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tms = project_tm_data_type(&mut transaction, &project_id, &query.r#type, true).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    let content = party_list_template(&project_id, &parties, &tms, &query.r#type);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}
