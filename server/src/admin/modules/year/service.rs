use sqlx::{query, SqliteConnection};

pub async fn update_project_year(
    conn: &mut SqliteConnection,
    project_id: &str,
    year: Option<i64>,
) -> Result<(), sqlx::Error> {
    query!("UPDATE PROJECTS SET year=$1 WHERE id=$2 AND draft=1", year, project_id)
    .execute(conn)
    .await
    .map(|_| ())
}