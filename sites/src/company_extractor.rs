use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};
use axum_extra::extract::Host;
use reqwest::StatusCode;

use crate::AppState;

pub struct CompanyExtractor(pub String);

impl<S> FromRequestParts<S> for CompanyExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        
        let Host(host) = Host::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "The hostname could not be found"))?;

        let state = AppState::from_ref(state);

        let company = match state.company_data.domain_to_company(&host) {
            Some(company) => company,
            None => return Err((StatusCode::NOT_FOUND, "Could not recognise the host for this request.")),
        };

        Ok(CompanyExtractor(company.to_string()))
    }
}