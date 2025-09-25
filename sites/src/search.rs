use axum::{extract::{Query, State}, response::Html};
use axum_extra::extract::Host;
use serde::Deserialize;
use sqlx::query_scalar;
use templr::templ;

use crate::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String
}

pub async fn search_results(State(state): State<AppState>, Host(host): Host, Query(query): Query<SearchQuery>) -> Html<String> {
    let filter = format!("%{}%", query.query);

    let company = match host.as_str() {
        "alfa.localhost" => "Alfa Group",
        "architon.localhost" => "Architon",
        "artem.localhost" => "Artem",
        "corporate.localhost" => "Corporate",
        "iftech.localhost" => "IFTech",
        "maes.localhost" => "Algemene Bouw Maes",
        "services.localhost" => "Van Roey Services",
        "staalbeton.localhost" => "Staalbeton",
        "vanhout.localhost" => "vanhout.pro",
        "vanroey.localhost" => "Van Roey",
        _ => ""
    };
    
    let projects = query_scalar!(
        r#"SELECT
                name
            FROM PROJECTS p
            JOIN PROJECT_COMPANIES pc ON p.id = pc.project_id
        WHERE
            pc.company_name = $1
            AND
            name LIKE $2"#, company, filter)
        .fetch_all(&state.db_pool)
        .await
        .unwrap();

    Html(templ! {
        #for project in &projects {
            <div>{project}</div>
        }
    }.to_string())
}