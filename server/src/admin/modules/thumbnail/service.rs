use sqlx::{query_scalar, SqliteConnection};

pub async fn set_thumbnail(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    value: &Option<String>
) -> Result<Option<String>, sqlx::Error> {
    query_scalar!("UPDATE PROJECT_COMPANIES SET thumbnail=$1 WHERE project_id=$2 AND (company_name=$3 OR ($3 IS NULL AND company_name IS NULL)) AND draft=1 RETURNING thumbnail", value, project_id, company_name)
    .fetch_one(conn)
    .await
}
