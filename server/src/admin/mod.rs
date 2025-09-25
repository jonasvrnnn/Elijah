use std::time::SystemTime;

use axum::{
    extract::{FromRequestParts, Query}, http::{request::Parts, HeaderMap, StatusCode}, middleware::from_fn_with_state, response::{IntoResponse, Response}, routing::{delete, get, patch, post, put}, Router
};
use index::index_page;
use modules::{
    banner::endpoint::{customise_banner_photo, update_banner_photo},
    company_list::endpoint::{delete_company_from_project_endpoint, get_companies},
    content::endpoint::{
        customise_content_endpoint, delete_content_entry_endpoint, insert_content_entry_endpoint,
        update_content_entry_image_endpoint, update_content_entry_quote_endpoint,
        update_content_entry_text_endpoint,
    },
    core_numbers::endpoint::{create_core_number, delete_core_number, update_core_number},
    header::endpoint::{customise_header_photo, update_header_photo},
    industry::endpoint::{add_industry_project, delete_industry_project, full_industry_list},
    introduction::endpoint::{customise_introduction, update_introduction},
    location::endpoint::update_project_location,
    name::endpoint::set_name,
    party_list::endpoint::{add_party_to_project, delete_party_from_project, project_parties},
    project::endpoint::{get_project, revert_changes, save_changes},
    publiek_privaat::endpoint::set_publiek_privaat,
    show_in_carousel::endpoint::set_show_in_carousel,
    status::endpoint::set_state,
    tms::endpoint::{
        add_company_to_tm, add_party_to_tm, add_tm_to_project, delete_company_from_tm,
        delete_party_from_tm, delete_tm_from_project, tm_input, tms,
    },
    weight::endpoint::update_project_weight,
    year::endpoint::update_project_year,
};
use serde::Deserialize;
use sqlx::{pool::PoolConnection, Sqlite, SqliteConnection, SqliteTransaction};
use url::Url;

use crate::{
    admin::modules::{
        company_list::endpoint::company_search_list, image::endpoint::{add_images_to_collection, customise, delete_image_to_collection}, industry::endpoint::industry_search_list, labels::endpoint::update_labels, party_list::endpoint::party_search_list_id, project::endpoint::{create_new_project, delete_project, name_for_id, projects, unpublish}, thumbnail::endpoint::set_thumbnail, visible::endpoint::set_visible
    }, auth::service::{auth_middleware, create_draft_middleware}, AppState
};

pub mod draft;
pub mod index;
pub mod modules;

pub trait AllowedCompanyType {}

impl AllowedCompanyType for String {}
impl AllowedCompanyType for Option<String> {}

pub struct Company<T: AllowedCompanyType>(pub T);

impl From<String> for Company<String> {
    fn from(value: String) -> Self {
        Company(value)
    }
}

#[derive(Deserialize)]
struct CompanyExtractorQuery {
    company: Option<String>
}

fn get_company_from_url(headers: &HeaderMap, company: &Option<String>) -> Result<Option<String>, &'static str> {
    let header_value = match headers.get("HX-CURRENT-URL") {
        Some(header_value) => header_value,
        None => {
            if let Some(company) = company {
                return Ok(Some(company.to_string()));
            }
            return Err("Missing the HX-CURRENT-URL header, or the 'company' query parameter.");
        }
    };

    let url_string = match header_value.to_str() {
        Ok(url_string) => url_string,
        Err(_) => {
            return Err("Missing the HX-CURRENT-URL header.");
        }
    };

    let url = match Url::parse(&url_string) {
        Ok(url) => url,
        Err(_) => {
            return Err("Missing the HX-CURRENT-URL header.");
        }
    };

    let company = url
        .query_pairs()
        .find(|(key, _)| key == "company")
        .map(|q| q.1.to_string());

    Ok(company)
}

impl<S> FromRequestParts<S> for Company<String>
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<CompanyExtractorQuery>::from_request_parts(parts, state)
        .await
        .unwrap();
        
        let company = match get_company_from_url(&parts.headers, &query.company) {
            Ok(company) => company,
            Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_owned())),
        };

        match company {
            Some(company) => Ok(Company(company)),
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Missing the company query parameter.".to_owned(),
                ));
            }
        }
    }
}

impl<S> FromRequestParts<S> for Company<Option<String>>
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<CompanyExtractorQuery>::from_request_parts(parts, state)
        .await
        .unwrap();
        
        match get_company_from_url(&parts.headers, &query.company) {
            Ok(company) => Ok(Company(company)),
            Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_owned())),
        }
    }
}

pub struct DBTransaction<'a>(pub SqliteTransaction<'a>);

impl FromRequestParts<AppState> for DBTransaction<'_>
{
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let transaction = match state.pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                eprintln!("Failed to start a transaction: {err}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        Ok(DBTransaction(transaction))
    }
}

pub struct DBConnection(pub PoolConnection<Sqlite>);

impl FromRequestParts<AppState> for DBConnection
{
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let connection = match state.pool.acquire().await {
            Ok(connection) => connection,
            Err(err) => {
                eprintln!("Failed to start a connection: {err}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        Ok(DBConnection(connection))
    }
}

async fn server_error() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis().to_string()).into_response()
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(index_page))
        .route("/companies", get(get_companies))
        .route(
            "/companies/search",
            get(company_search_list).layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .route("/industries", get(full_industry_list))
        .route("/industries/search", get(industry_search_list))
        .route("/parties/search", get(party_search_list_id))
        .route("/parties/testtest", get(server_error))
        /*         .route("/parties", get(get_all_parties))
         */
        .route("/tms", get(tms))
        .route("/tms/{id}/companies", post(add_company_to_tm))
        .route("/tms/{id}/companies", delete(delete_company_from_tm))
        .route("/tms/{id}/parties", post(add_party_to_tm))
        .route("/tms/{id}/parties", delete(delete_party_from_tm))
        .route("/tms/search", get(tm_input))
        .route(
            "/projects",
            get(projects).layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .route("/projects", post(create_new_project))
        .route(
            "/projects/{project_id}/show-in-carousel",
            patch(set_show_in_carousel)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/visible",
            patch(set_visible).layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route("/projects/{project_id}/name", get(name_for_id))
        .route(
            "/projects/{project_id}/name",
            patch(set_name).layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/status",
            patch(set_state).layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}",
            get(get_project).layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .route(
            "/projects/{project_id}/revert-changes",
            post(revert_changes).layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .route(
            "/projects/{project_id}/save-changes",
            post(save_changes).layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .route(
            "/projects/{project_id}",
            delete(delete_project).layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .route(
            "/projects/{project_id}/unpublish",
            delete(unpublish)
                .layer(from_fn_with_state(state.clone(), auth_middleware))
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/companies/{project_name}",
            delete(delete_company_from_project_endpoint)
                .layer(from_fn_with_state(state.clone(), auth_middleware))
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/header-image",
            patch(update_header_photo)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/header-image/customise",
            post(customise_header_photo)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/banner-image",
            patch(update_banner_photo)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/banner-image/customise",
            post(customise_banner_photo)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/year",
            patch(update_project_year)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/location",
            patch(update_project_location)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/industries",
            post(add_industry_project)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/industries/{industry}",
            delete(delete_industry_project)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/weight",
            patch(update_project_weight)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/introduction",
            patch(update_introduction)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/thumbnail",
            patch(set_thumbnail)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/lightbox",
            post(add_images_to_collection)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/lightbox/customise",
            post(customise).layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/lightbox/{image_id}",
            delete(delete_image_to_collection)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/publiek-privaat",
            patch(set_publiek_privaat)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/introduction/customise",
            post(customise_introduction)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route("/projects/{project_id}/parties", get(project_parties))
        .route(
            "/projects/{project_id}/parties",
            post(add_party_to_project)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/parties",
            delete(delete_party_from_project)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/parties/search",
            get(party_search_list_id),
        )
        .route(
            "/projects/{project_id}/core_numbers",
            post(create_core_number)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/tms",
            post(add_tm_to_project)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/tms",
            delete(delete_tm_from_project)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/content/customise",
            post(customise_content_endpoint)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/content/{id}/text",
            patch(update_content_entry_text_endpoint)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/content/{id}/quote",
            patch(update_content_entry_quote_endpoint)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/content/{id}/image",
            patch(update_content_entry_image_endpoint)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/content/{id}",
            delete(delete_content_entry_endpoint)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/content",
            post(insert_content_entry_endpoint)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/core_numbers/{id}",
            delete(delete_core_number)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/core_numbers/{id}",
            put(update_core_number)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
        .route(
            "/projects/{project_id}/labels",
            put(update_labels)
                .layer(from_fn_with_state(state.clone(), create_draft_middleware)),
        )
}
