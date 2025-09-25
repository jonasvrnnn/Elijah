use std::fmt::Debug;

use sqlx::{SqliteConnection, query, query_scalar};

pub enum DraftError {
    AlreadyExists(String),
}

impl Debug for DraftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists(arg0) => f.debug_tuple("AlreadyExists").field(arg0).finish(),
        }
    }
}

pub async fn check_for_existing_project_draft(
    conn: &mut SqliteConnection,
    id: &str,
) -> Result<bool, sqlx::Error> {
    query_scalar!(
        r#"
        SELECT
        EXISTS(
            SELECT 
                id
            FROM
                PROJECTS
            WHERE id=$1
            AND draft=1
        ) as "draft: bool""#,
        id
    )
    .fetch_one(conn)
    .await
}

pub async fn create_project_draft_if_necessary(
    conn: &mut SqliteConnection,
    id: &str,
) -> Result<(), sqlx::Error> {
    if check_for_existing_project_draft(&mut *conn, id).await? {
        println!("Project {id} already has a draft version. Continuing...");
        return Ok(());
    }

    query!(
        "INSERT INTO PROJECTS (
            id,
            name,
            location,
            year,
            learn_more,
            status,
            publiek_privaat,
            draft
        )
        SELECT
            id,
            name,
            location,
            year,
            learn_more,
            status,
            publiek_privaat,
            1
            FROM PROJECTS
        WHERE id = $1 AND draft = 0
    ",
        id
    )
    .execute(&mut *conn)
    .await
    .map(|_| ())
}

struct CreateDraftReturn {
    content: Option<String>,
    lightbox: Option<String>,
}
