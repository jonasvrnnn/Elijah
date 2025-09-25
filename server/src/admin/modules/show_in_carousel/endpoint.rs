use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Result}, Form};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, Company, DBTransaction}, TransactionError};

use super::{service as service, template};

fn default_as_false() -> bool {
    false
}

#[derive(Deserialize)]
pub struct SetShowInCarousel {
    #[serde(default="default_as_false")]
    show_in_carousel: bool
}

pub async fn set_show_in_carousel(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>,
    Form(body): Form<SetShowInCarousel>
) -> Result<Markup> {
    let show_in_carousel = service::set_show_in_carousel(&mut *transaction, &project_id, &company, body.show_in_carousel).await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::show_in_carousel_template(&project_id, show_in_carousel, false);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}