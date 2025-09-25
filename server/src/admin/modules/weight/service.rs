use sqlx::{SqliteConnection, query};

pub async fn update_weight(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
    weight: &Option<i64>,
) -> Result<(), sqlx::Error> {
    query!(
        "UPDATE PROJECT_COMPANIES SET weight=$1 WHERE project_id=$2 AND company_name=$3 AND draft=1",
        weight,
        project_id,
        company_name
    )
    .execute(conn)
    .await
    .map(|_| ())
}