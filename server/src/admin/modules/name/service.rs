use sqlx::{query_scalar, SqliteConnection};

pub async fn set_name(
    conn: &mut SqliteConnection,
    project_id: &str,
    name: &str
) -> Result<String, sqlx::Error> {
    query_scalar!("UPDATE PROJECTS SET name=$1 WHERE id=$2 AND draft=1 RETURNING name", name, project_id)
    .fetch_one(conn)
    .await
}