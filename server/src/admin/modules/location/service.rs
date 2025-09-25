use sqlx::{query, SqliteConnection};

pub async fn update_project_location(
    conn: &mut SqliteConnection,
    project_id: &str,
    location: &Option<String>,
) -> Result<(), sqlx::Error> {
    query!("UPDATE PROJECTS SET location=$1 WHERE id=$2 AND draft=1", location, project_id)
    .execute(conn)
    .await
    .map(|_| ())
}