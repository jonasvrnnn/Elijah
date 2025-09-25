use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Result},
};
use maud::Markup;
use serde::Deserialize;
use serde_with::OneOrMany;
use serde_with::formats::PreferMany;
use serde_with::serde_as;

use crate::admin::{modules::{industry::service as industry_service, labels::service::get_labels}, DBConnection};
use crate::admin::{DBTransaction, modules::party_list::service as party_service};
use crate::{TransactionError, admin::modules::company_list::service as company_list_service};

use crate::{
    AppState,
    admin::{
        draft::{self, check_for_existing_project_draft},
        modules::{
            company_list::service::{
                add_company_to_project, does_project_have_company, get_company_lists,
            },
            content::service::get_content,
            core_numbers::service::get_project_core_numbers,
            industry::service::project_industry_data,
            party_list::service::project_party_data,
            tms::service::project_tm_data,
        },
    },
    auth::service::UserData,
};

use super::{
    service::{self, full_list, get_project_company_info},
    template::{project_list, project_template},
};

use super::super::core_numbers::service as core_numbers_service;

use super::template;

#[derive(Deserialize)]
pub struct GetProjectsQuery {
    filter: Option<String>,
}

pub async fn projects(
    Query(query): Query<GetProjectsQuery>,
    DBConnection(mut connection): DBConnection,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    let projects = full_list(&mut connection, query.filter, &Some(data.id))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::projects(&projects))
}

#[derive(Deserialize)]
pub struct GetProjectQuery {
    company: Option<String>,
}

pub async fn get_project(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<GetProjectQuery>,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    let mut draft = check_for_existing_project_draft(&mut *transaction, &project_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(company) = &query.company {
        let exists = does_project_have_company(&mut *transaction, &project_id, &company, draft)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if !exists {
            if !draft {
                draft::create_project_draft_if_necessary(&mut *transaction, &project_id).await;
                draft = true;
            }
            add_company_to_project(&mut transaction, &project_id, &company).await;
        }
    }

    let (project, images) = get_project_company_info(&mut transaction, &project_id, &query.company)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let company_lists = get_company_lists(&mut transaction, &project_id, project.draft, &data.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let labels = get_labels(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let party_data = project_party_data(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tm_data = project_tm_data(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let industry_data = project_industry_data(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content_company = if project.custom_content.unwrap_or(false) {
        &query.company
    } else {
        &None
    };

    let content = get_content(
        &mut transaction,
        &project_id,
        content_company,
        project.draft,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let core_numbers = get_project_core_numbers(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(project_template(
        &project,
        &query.company,
        &company_lists,
        &party_data,
        &tm_data,
        &industry_data,
        &content,
        &core_numbers,
        &images,
        &labels
    ))
}

pub async fn save_changes(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    service::delete_project(&mut *transaction, &project_id, false)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    service::update_project_draft(&mut *transaction, &project_id, false)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (project, images) = get_project_company_info(&mut transaction, &project_id, &None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let company_lists = get_company_lists(&mut transaction, &project_id, project.draft, &data.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let labels = get_labels(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    let core_numbers = get_project_core_numbers(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(project_template(
        &project,
        &None,
        &company_lists,
        &party_data,
        &tm_data,
        &industry_data,
        &content,
        &core_numbers,
        &images,
        &labels
    ))
}

pub async fn revert_changes(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    service::delete_project(&mut *transaction, &project_id, true)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (project, images) = get_project_company_info(&mut transaction, &project_id, &None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let company_lists = get_company_lists(&mut transaction, &project_id, project.draft, &data.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let labels = get_labels(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    let core_numbers = get_project_core_numbers(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|err| {
        println!("{err}");
        TransactionError::Commit
    })?;

    Ok(project_template(
        &project,
        &None,
        &company_lists,
        &party_data,
        &tm_data,
        &industry_data,
        &content,
        &core_numbers,
        &images,
        &labels
    ))
}

pub async fn unpublish(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Extension(data): Extension<UserData>,
) -> Result<Markup> {
    core_numbers_service::delete_for_project_id(&mut *transaction, &project_id, false).await;
    service::delete_project(&mut *transaction, &project_id, false).await;

    let (project, images) = get_project_company_info(&mut transaction, &project_id, &None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let company_lists = get_company_lists(&mut transaction, &project_id, project.draft, &data.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let labels = get_labels(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    let core_numbers = get_project_core_numbers(&mut transaction, &project_id, project.draft)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(project_template(
        &project,
        &None,
        &company_lists,
        &party_data,
        &tm_data,
        &industry_data,
        &content,
        &core_numbers,
        &images,
        &labels
    ))
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct CreateNewProjectBody {
    name: String,
    #[serde(default)]
    status: bool,
    contract: String,
    year: Option<i64>,
    location: Option<String>,
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    client: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    architect: Vec<String>,
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    #[serde(default)]
    contractor: Vec<String>,
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    #[serde(default)]
    company: Vec<String>,
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    #[serde(default)]
    industry: Vec<String>,
}

pub async fn create_new_project(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Json(body): Json<CreateNewProjectBody>,
) -> Result<HeaderMap> {
    let project_id = service::create_new(
        &mut *transaction,
        &body.name,
        &body.location,
        &body.year,
        body.status,
        &body.contract,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for party in body.client {
        party_service::add_party_to_project(&mut *transaction, &project_id, &party, "client").await;
    }

    for party in body.architect {
        party_service::add_party_to_project(&mut *transaction, &project_id, &party, "architect")
            .await;
    }

    for party in body.contractor {
        party_service::add_party_to_project(&mut *transaction, &project_id, &party, "contractor")
            .await;
    }

    for company in body.company {
        company_list_service::add_company_to_project(&mut *transaction, &project_id, &company)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    for industry in body.industry {
        industry_service::add_industry_to_project(&mut *transaction, &project_id, &industry).await;
    }

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "HX-REDIRECT",
        format!("/projects/{project_id}")
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    Ok(headers)
}

pub async fn name_for_id(
    DBConnection(mut connection): DBConnection,
    Path(project_id): Path<String>,
) -> Result<String> {
    let name = service::name_for_id(&mut *connection, &project_id)
        .await
        .map_err(|err| {
            eprintln!("Failed to retrieve the project's name: {err}");
            StatusCode::NOT_FOUND
        })?;

    Ok(name)
}

pub async fn delete_project(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path(project_id): Path<String>,
) -> Result<()> {
    service::delete_project(&mut *transaction, &project_id, false)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    service::delete_project(&mut *transaction, &project_id, true)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(())
}
