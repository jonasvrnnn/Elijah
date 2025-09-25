use axum::response::Result;
use axum::{extract::Path, response::IntoResponse};

use axum_extra::extract::Query;

use maud::Markup;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_with::{NoneAsEmptyString, serde_as};

use crate::projecten::service::Parties;
use crate::{
    company_extractor::CompanyExtractor,
    db_connection_extractor::SqliteConnectionExtractor,
    projecten::service::{GetContentError, GetImagesError},
};

use super::service;
use super::template;

const LIMIT: u8 = 12;

#[serde_as]
#[derive(Deserialize)]
pub struct ProjectenQuery {
    page: Option<u8>,
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    sector: Option<String>,
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    status: Option<bool>,
    #[serde(default, rename = "publiek-privaat")]
    #[serde_as(as = "NoneAsEmptyString")]
    publiek_privaat: Option<String>,
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    bedrijf: Option<String>,
    search: Option<String>,
    tags: Option<Vec<String>>,
}

pub struct P {
    name: String,
    slug: String,
    location: Option<String>,
    header_photo: Option<String>,
}

pub async fn carousel(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    CompanyExtractor(company): CompanyExtractor,
) -> Result<Markup> {
    let projects = service::get_carousel_projects(&mut conn, &company)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::carousel(&projects))
}

pub async fn in_de_kijker(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    CompanyExtractor(company): CompanyExtractor,
) -> Result<Markup> {
    let projects = service::get_carousel_projects(&mut conn, &company)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::in_de_kijker(&projects))
}

pub async fn projecten(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    CompanyExtractor(company): CompanyExtractor,
    Query(query): Query<ProjectenQuery>,
) -> Result<Markup> {
    let page = query.page.unwrap_or(0);

    let offset = page * LIMIT;

    let mut query_string = String::new();

    query_string.push_str(&format!("page={}", page + 1));

    if let Some(sector) = &query.sector {
        query_string.push_str(&format!("&sector={sector}"));
    }
    if let Some(status) = &query.status {
        query_string.push_str(&format!("&status={status}"));
    }
    if let Some(publiek_privaat) = &query.publiek_privaat {
        query_string.push_str(&format!("&publiek-privaat={publiek_privaat}"));
    }
    if let Some(bedrijf) = &query.bedrijf {
        query_string.push_str(&format!("&bedrijf={bedrijf}"));
    }
    if let Some(search) = &query.search {
        query_string.push_str(&format!("&search={search}"));
    }
    if let Some(tags) = &query.tags {
        for tag in tags {
            query_string.push_str(&format!("&tags={tag}"));
        }
    }

    let search = format!("%{}%", query.search.unwrap_or_default());

    let projects = service::get_projects(
        &mut conn,
        &company,
        &query.status,
        &query.sector,
        query.publiek_privaat,
        LIMIT,
        offset,
        &search,
        &query.bedrijf,
        &query.tags,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(template::projects(&projects, LIMIT, &query_string))
}

pub struct ProjectItem {
    pub name: String,
    pub slug: String,
    pub location: Option<String>,
    pub year: Option<i64>,
    pub learn_more: Option<String>,
    pub status: bool,
    pub publiek_privaat: String,
    pub header_photo: Option<String>,
    pub header_photo_copyright: Option<String>,
    pub banner_photo: Option<String>,
    pub banner_photo_copyright: Option<String>,
    pub introduction: Option<String>,
    pub contractors: Parties,
    pub architects: Parties,
    pub clients: Parties,
    pub industries: Parties,
}

pub enum ProjectError {
    HostToCompany,
    ProjectRetrieval(sqlx::Error),
    ClientsRetrieval(sqlx::Error),
    ArchitectsRetrieval(sqlx::Error),
    ContractorsRetrieval(sqlx::Error),
    GroupCompaniesRetrieval(sqlx::Error),
    IndustriesRetrieval(sqlx::Error),
    CoreNumbersRetrieval(sqlx::Error),
    ContentRetrieval(GetContentError),
    ImagesRetrieval(GetImagesError),
}

impl IntoResponse for ProjectError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn project(
    SqliteConnectionExtractor(mut conn): SqliteConnectionExtractor,
    Path(slug): Path<String>,
    CompanyExtractor(company): CompanyExtractor,
) -> Result<Markup> {
    let project = service::get_project_data(&mut *conn, &slug, &company)
        .await
        .map_err(|err| {
            println!("{err}");
            ProjectError::ProjectRetrieval(err);
        })?;

    let group_companies = service::get_group_companies(&mut *conn, &slug)
        .await
        .map_err(|err| ProjectError::GroupCompaniesRetrieval(err))?;
    let core_numbers = service::get_core_numbers(&mut *conn, &slug)
        .await
        .map_err(|err| ProjectError::CoreNumbersRetrieval(err))?;
    let content = service::get_content(&mut *conn, &slug, &company)
        .await
        .map_err(|err| ProjectError::ContentRetrieval(err))?;
    let images = service::get_images(&mut *conn, &slug, &company)
        .await
        .map_err(|err| ProjectError::ImagesRetrieval(err))?;

    let template =
        template::project_template(&project, &group_companies, &core_numbers, &content, &images);

    Ok(template)
}
