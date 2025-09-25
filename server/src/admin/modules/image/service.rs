use serde::Deserialize;
use sqlx::{SqliteConnection, query, query_as};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ImageData {
    pub id: Option<String>,
    pub image: Option<String>,
    pub image_copyright: Option<String>,
    pub alt: Option<String>,
}

pub async fn customise(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
) -> Result<Vec<ImageData>, sqlx::Error> {
    let images = query_as!(
        ImageData,
        "
        INSERT INTO IMAGES(
            project_id,
            company_name,
            id,
            draft,
            image,
            image_copyright,
            alt
        ) SELECT
            project_id,
            $1,
            id,
            draft,
            image,
            image_copyright,
            alt
        FROM IMAGES
        WHERE project_id=$2 AND company_name IS NULL
        RETURNING id, image, image_copyright, alt",
        project_id,
        company_name
    )
    .fetch_all(&mut *conn)
    .await?;

    query!("UPDATE PROJECT_COMPANIES SET custom_lightbox=1 WHERE draft=1 AND project_id=$1 AND company_name=$2", project_id, company_name)
    .execute(&mut *conn)
    .await?;

    Ok(images)
}

pub async fn get_images(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    draft: bool,
) -> Result<Vec<ImageData>, sqlx::Error> {
    query_as!(
        ImageData,
        "SELECT id, image, image_copyright, alt FROM IMAGES where project_id=$1 AND (($2 IS NULL AND company_name IS NULL) OR company_name=$2) AND draft=$3",
        project_id,
        company_name,
        draft
    )
    .fetch_all(conn)
    .await
}

pub async fn delete_image(conn: &mut SqliteConnection, image_id: &str) -> Result<(), sqlx::Error> {
    query!("DELETE FROM IMAGES where id=$1 AND draft=1", image_id)
        .execute(&mut *conn)
        .await
        .map(|_| ())
}

pub async fn add_image(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    image: &ImageData,
) -> Result<ImageData, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    query_as!(
        ImageData,
        "INSERT INTO IMAGES(
        id,
        project_id,
        company_name,
        draft,
        image,
        image_copyright,
        alt
    ) VALUES (
        $1, $2, $3, 1, $4, $5, $6
    ) RETURNING id, image, image_copyright, alt",
        id,
        project_id,
        company_name,
        image.image,
        image.image_copyright,
        image.alt
    )
    .fetch_one(&mut *conn)
    .await
}
