use axum::{
    extract::Path, http::StatusCode, response::Result, Form
};
use maud::Markup;
use serde::Deserialize;

use crate::{admin::{modules::project::template::page_buttons_update_wrapper, Company, DBTransaction}, TransactionError};

use super::{service, template};

fn default_as_false() -> bool {
    false
}

#[derive(Deserialize)]
pub struct SetVisible {
    #[serde(default = "default_as_false")]
    visible: bool,
}

pub async fn set_visible(
    Path(project_id): Path<String>,
    DBTransaction(mut transaction): DBTransaction<'_>,
    Company(company): Company<String>,
    Form(body): Form<SetVisible>,
) -> Result<Markup> {
    let visible = service::set_visible(&mut *transaction, &project_id, &company, body.visible)
        .await
        .map_err(|err| {
            eprintln!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    transaction.commit().await.map_err(|_| TransactionError::Commit)?;

    let template = template::visible_template(&project_id, visible, false);

    Ok(page_buttons_update_wrapper(&project_id, true, &template))
}
