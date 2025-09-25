use sqlx::{query_scalar, SqliteConnection};

pub async fn set_status(
    conn: &mut SqliteConnection,
    project_id: &str,
    status: bool
) -> Result<bool, sqlx::Error> {
    query_scalar!("UPDATE PROJECTS SET status=$1 WHERE id=$2 AND draft=1 RETURNING status", status, project_id)
    .fetch_one(conn)
    .await
}