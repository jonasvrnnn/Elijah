use sqlx::{query_scalar, SqliteConnection};

pub async fn set_visible(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
    value: bool
) -> Result<bool, sqlx::Error> {
    query_scalar!("UPDATE PROJECT_COMPANIES SET visible=$1 WHERE project_id=$2 AND company_name=$3 AND draft=1 RETURNING visible", value, project_id, company_name)
    .fetch_one(conn)
    .await
}
