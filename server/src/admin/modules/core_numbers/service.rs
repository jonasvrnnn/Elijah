use sqlx::{query, query_as, query_scalar, SqliteConnection};
use uuid::Uuid;

pub struct CoreNumber {
    pub id: String,
    pub title: String,
    pub number: String
}

pub async fn get_project_core_numbers(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool
) -> Result<Vec<CoreNumber>, sqlx::Error> {
    query_as!(CoreNumber, "SELECT id, title, number FROM CORE_NUMBERS WHERE project_id=$1 AND draft=$2", project_id, draft)
    .fetch_all(conn)
    .await
}

pub async fn delete_core_number(
    conn: &mut SqliteConnection,
    core_number_id: &str
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM CORE_NUMBERS WHERE id=$1 AND draft=1", core_number_id)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn delete_for_project_id(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM CORE_NUMBERS WHERE project_id=$1 AND draft=$2", project_id, draft)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn save_changes(
    conn: &mut SqliteConnection,
    project_id: &str
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM CORE_NUMBERS WHERE project_id=$1 AND draft=0", project_id)
    .execute(&mut *conn)
    .await?;

    query!("UPDATE CORE_NUMBERS SET draft=0 WHERE project_id=$1 AND draft=1", project_id)
    .execute(&mut *conn)
    .await
    .map(|_| ())
}

pub async fn create_core_number(
    conn: &mut SqliteConnection,
    project_id: &str
) -> Result<CoreNumber, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    query_as!(CoreNumber, "INSERT INTO CORE_NUMBERS(id, project_id, title, number, draft) VALUES($1, $2, $3, $4, $5) RETURNING id, title, number", id, project_id, "", "", 1)
    .fetch_one(conn)
    .await
}

pub async fn update_core_number(
    conn: &mut SqliteConnection,
    id: &str,
    number: &str,
    title: &str
) -> Result<CoreNumber, sqlx::Error> {
    query_as!(CoreNumber, "UPDATE CORE_NUMBERS SET title=$1, number=$2 WHERE id=$3 AND draft=1 RETURNING id, title, number", title, number, id)
    .fetch_one(conn)
    .await
}

pub async fn get_project_id_for_core_number_id(conn: &mut SqliteConnection, core_number_id: &str) -> Result<String, sqlx::Error> {
    query_scalar!("SELECT project_id FROM CORE_NUMBERS WHERE id=$1", core_number_id)
    .fetch_one(&mut *conn)
    .await
}