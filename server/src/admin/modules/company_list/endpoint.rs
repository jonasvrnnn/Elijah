use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Result},
};
use axum_extra::extract::Query as ExtraQuery;
use maud::Markup;
use serde::Deserialize;

use crate::{
    admin::{
        index::index, modules::{
            company_list::template, content::service::get_content, industry::service::project_industry_data, labels::service::get_labels, party_list::service::project_party_data, project::{service::get_project_company_info, template::project_template}, tms::service::project_tm_data
        }, DBConnection, DBTransaction
    }, auth::service::UserData, TransactionError
};

use super::{
    service::{delete_company_from_project, full_company_list, get_company_lists},
    template::full_list,
};

use super::service;

#[derive(Deserialize)]
pub struct DeleteCompanyFromProjectQuery {
    current_company: Option<String>,
}

pub async fn delete_company_from_project_endpoint(
    Path((project_id, company)): Path<(String, String)>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<DeleteCompanyFromProjectQuery>,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    delete_company_from_project(&mut *transaction, &project_id, &company).await;

    let current_company = query.current_company.unwrap_or_default();

    let company_name = if current_company == company {
        None
    } else {
        Some(current_company)
    };

    let (project, images) = get_project_company_info(&mut transaction, &project_id, &company_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let company_lists =
        match get_company_lists(&mut transaction, &project_id, project.draft, &data.id).await {
            Ok(company_lists) => company_lists,
            Err(err) => {
                eprintln!("Failed to retrieve company lists: {err}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
            }
        };

    let party_data = project_party_data(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tm_data = project_tm_data(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let industry_data = project_industry_data(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let content = get_content(&mut transaction, &project_id, &None, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let labels = get_labels(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(index(
        &None,
        &Some(project_template(
            &project,
            &company_name,
            &company_lists,
            &party_data,
            &tm_data,
            &industry_data,
            &content,
            &vec![],
            &images,
            &labels,
        )),
    ))
}

pub async fn get_companies(DBConnection(mut connection): DBConnection) -> Result<Markup> {
    let companies = full_company_list(&mut connection).await.map_err(|err| {
        eprintln!("Failed to retrieve the company list: {err}.");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(index(&Some(full_list(&companies)), &None))
}

#[derive(Deserialize)]
pub struct CompanySearchListQuery {
    filter: Option<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

pub async fn company_search_list(
    ExtraQuery(query): ExtraQuery<CompanySearchListQuery>,
    DBConnection(mut connection): DBConnection,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    let companies = service::company_search_list(
        &mut connection,
        &query.filter,
        &Some(data.id),
        &query.exclude,
    )
    .await
    .map_err(|err| {
        eprintln!("Failed to retrieve the company search list: {err}.");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(template::search_list(&companies))
}
