use sqlx::{SqliteConnection, query, query_as, query_scalar};

pub struct Label {
    pub name: String,
    pub active: bool,
}

pub async fn get_labels(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool,
) -> Result<Vec<Label>, sqlx::Error> {
    query_as!(Label, r#"SELECT name, pt.tag IS NOT NULL as "active!: bool" FROM TAGS t LEFT JOIN PROJECT_TAGS pt ON t.name=pt.tag AND pt.project_id=$1 AND pt.draft=$2 ORDER BY t.name"#, project_id, draft)
    .fetch_all(conn)
    .await
}

pub async fn set_labels(
    conn: &mut SqliteConnection,
    project_id: &str,
    labels: &Vec<String>,
) -> Result<(), sqlx::Error> {
    query!(
        "DELETE FROM PROJECT_TAGS WHERE project_id=$1 AND draft=1",
        project_id
    )
    .execute(&mut *conn)
    .await?;

    for label in labels {
        query!(
            "INSERT INTO PROJECT_TAGS(project_id, tag, draft) VALUES ($1, $2, 1)",
            project_id,
            label
        )
        .execute(&mut *conn)
        .await?;
    }

    Ok(())
}
