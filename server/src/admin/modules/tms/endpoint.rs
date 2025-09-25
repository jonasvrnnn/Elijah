use axum::{
    Form,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Result},
};
use maud::{Markup, html};
use serde::Deserialize;

use crate::{
    AppState, TransactionError,
    admin::{
        DBConnection, DBTransaction,
        index::index,
        modules::{
            party_list::{service::project_party_data_type, template::party_list_template},
            project::template::page_buttons_update_wrapper,
            tms::template::{list_companies, list_parties, tm_list_companies, tm_list_parties},
        },
    },
};

use super::{
    service::{self, Company, Party, check_input, get_all_companies, get_all_parties},
    template::{self, main},
};

#[derive(Deserialize)]
pub struct TMsQuery {
    tm: Option<String>,
}

pub async fn tms(
    DBConnection(mut connection): DBConnection,
    Query(query): Query<TMsQuery>,
) -> Result<Markup> {
    let tm = query.tm;

    let companies: (Vec<Company>, Vec<Company>) = if let Some(ref tm) = tm {
        get_all_companies(&mut *connection, &tm)
            .await
            .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .partition(|c| !c.active)
    } else {
        (vec![], vec![])
    };

    let parties: (Vec<Party>, Vec<Party>) = if let Some(ref tm) = tm {
        get_all_parties(&mut *connection, &tm)
            .await
            .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .partition(|p| !p.active)
    } else {
        (vec![], vec![])
    };

    let content = main(&tm, &companies, &parties, &vec![]);

    Ok(index(&Some(content), &None))
}

#[derive(Deserialize)]
pub struct TMInputQuery {
    input: String,
}

pub async fn tm_input(
    DBConnection(mut connection): DBConnection,
    Query(query): Query<TMInputQuery>,
) -> Result<Markup> {
    let input = match query.input.as_str() {
        "" => None,
        _ => Some(query.input),
    };

    let check_input_data = check_input(&mut *connection, &input)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tm_input = template::tm_input(
        &input,
        &check_input_data.tms,
        check_input_data.exact_match_exists,
        true,
    );

    Ok(tm_input)
}

#[derive(Deserialize)]
pub struct CreateNewTMBody {
    name: String,
}

pub async fn create_new_tm(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<CreateNewTMBody>,
) {
    let new_tm = service::create_new_tm(&mut *transaction, &body.name).await;
}

#[derive(Deserialize)]
pub struct AddCompanyToTMBody {
    company: String,
}

pub async fn add_company_to_tm(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path(tm): Path<String>,
    Form(body): Form<AddCompanyToTMBody>,
) -> Result<Markup> {
    service::add_company_to_tm(&mut *transaction, &tm, &body.company).await;

    let companies: (Vec<Company>, Vec<Company>) = get_all_companies(&mut *transaction, &tm)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .partition(|c| !c.active);

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(html!((list_companies(&tm, &companies.0))(
        tm_list_companies(&tm, &companies.1, true)
    )))
}

#[derive(Deserialize)]
pub struct DeleteCompanyFromTMQuery {
    company: String,
}

pub async fn delete_company_from_tm(
    Path(tm): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<DeleteCompanyFromTMQuery>,
) -> Result<Markup> {
    service::delete_company_from_tm(&mut *transaction, &tm, &query.company).await;

    let companies: (Vec<Company>, Vec<Company>) = get_all_companies(&mut *transaction, &tm)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .partition(|c| !c.active);

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(html!((list_companies(&tm, &companies.0))(
        tm_list_companies(&tm, &companies.1, true)
    )))
}

#[derive(Deserialize)]
pub struct AddPartyToTMBody {
    party: String,
}

pub async fn add_party_to_tm(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path(tm): Path<String>,
    Form(body): Form<AddPartyToTMBody>,
) -> Result<Markup> {
    service::add_party_to_tm(&mut *transaction, &tm, &body.party).await;

    let parties: (Vec<Party>, Vec<Party>) = get_all_parties(&mut *transaction, &tm)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .partition(|p| !p.active);

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(html!((list_parties(&tm, &parties.0))(tm_list_parties(
        &tm, &parties.1, true
    ))))
}

#[derive(Deserialize)]
pub struct DeletePartyFromTMQuery {
    party: String,
}

pub async fn delete_party_from_tm(
    DBTransaction(mut transaction): DBTransaction<'_>,
    Path(tm): Path<String>,
    Query(query): Query<DeletePartyFromTMQuery>,
) -> Result<Markup> {
    service::delete_party_from_tm(&mut *transaction, &tm, &query.party).await;

    let parties: (Vec<Party>, Vec<Party>) = get_all_parties(&mut *transaction, &tm)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .partition(|p| !p.active);

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    Ok(html!((list_parties(&tm, &parties.0))(tm_list_parties(
        &tm, &parties.1, true
    ))))
}

#[derive(Deserialize)]
pub struct AddTMToProject {
    tm: String,
    r#type: String,
}

pub async fn add_tm_to_project(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Form(body): Form<AddTMToProject>,
) -> Result<Markup> {
    let tm_exists = service::check_if_tm_exists(&mut *transaction, &body.tm)
        .await
        .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !tm_exists {
        service::add_tm(&mut *transaction, &body.tm).await;
    }

    service::add_tm_to_project(&mut transaction, &project_id, &body.tm, &body.r#type).await;

    let parties = project_party_data_type(&mut transaction, &project_id, &body.r#type, true)
        .await
        .map_err(|err| {
            eprintln!("Failed to retrieve the project's party data: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let tms =
        service::project_tm_data_type(&mut transaction, &project_id, &body.r#type, true).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

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

pub async fn delete_tm_from_project(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Query(query): Query<DeletePartyFromProjectQuery>,
) -> Result<Markup> {
    service::delete_party_from_project(&mut transaction, &project_id, &query.party, &query.r#type)
        .await;

    let parties = project_party_data_type(&mut transaction, &project_id, &query.r#type, true)
        .await
        .map_err(|err| {
            eprintln!("Failed to retrieve the project's party data: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let tms =
        service::project_tm_data_type(&mut transaction, &project_id, &query.r#type, true).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction
        .commit()
        .await
        .map_err(|_| TransactionError::Commit)?;

    let content = party_list_template(&project_id, &parties, &tms, &query.r#type);
    Ok(page_buttons_update_wrapper(&project_id, true, &content))
}
