use sqlx::{query_scalar, SqliteConnection};

pub async fn update_introduction(conn: &mut SqliteConnection, project_id: &str, company_name: &Option<String>, introduction: &Option<String>) -> Result<Option<String>, sqlx::Error> {
    query_scalar!("
        UPDATE PROJECT_COMPANIES SET introduction=$1 WHERE project_id=$2 AND ($3 IS NULL OR company_name = $3) AND draft=1 RETURNING introduction
    ", introduction, project_id, company_name)
    .fetch_one(conn)
    .await
}

pub async fn customise_introduction(conn: &mut SqliteConnection, project_id: &str, company_name: &str) -> Result<Option<String>, sqlx::Error> {
    query_scalar!("
        UPDATE PROJECT_COMPANIES SET introduction=(SELECT introduction FROM PROJECT_COMPANIES where project_id=$1 and company_name IS NULL and draft=1) WHERE project_id=$1 AND company_name = $2 AND draft=1 returning introduction
    ", project_id, company_name)
    .fetch_one(conn)
    .await
}
