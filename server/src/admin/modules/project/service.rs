use serde;
use serde::Deserialize;
use sqlx::{SqliteConnection, query, query_as, query_scalar};
use uuid::Uuid;

use crate::admin::modules::image::service::{self as image_service, ImageData};

pub struct ProjectItem {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub year: Option<i64>,
    pub weight: Option<i64>,
    pub learn_more: Option<String>,
    pub status: bool,
    pub publiek_privaat: String,
    pub header_photo: Option<String>,
    pub custom_header_photo: Option<bool>,
    pub header_photo_copyright: Option<String>,
    pub banner_photo: Option<String>,
    pub custom_banner_photo: Option<bool>,
    pub banner_photo_copyright: Option<String>,
    pub draft: bool,
    pub show_in_carousel: bool,
    pub introduction: Option<String>,
    pub custom_introduction: Option<bool>,
    pub custom_content: Option<bool>,
    pub custom_lightbox: Option<bool>,
    pub visible: bool,
    pub thumbnail: Option<String>,
    pub published: bool,
}

pub async fn get_project_company_info(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
) -> Result<(ProjectItem, Vec<ImageData>), sqlx::Error> {
    let mut project_item= query_as!(ProjectItem, r#"
    SELECT
        p.id, 
        p.name,
        p.location,
        p.year,
        p.learn_more,
        p.status,
        p.publiek_privaat,
        p.draft,
        pc.show_in_carousel,
        pc.weight,
        COALESCE(pc.introduction, base.introduction) as "introduction: String",
        $1 IS NULL OR pc.introduction IS NOT NULL AS "custom_introduction: bool",
        COALESCE(pc.header_photo, base.header_photo) as "header_photo: String",
        $1 IS NULL OR pc.header_photo IS NOT NULL AS "custom_header_photo: bool",
        COALESCE(pc.banner_photo, base.banner_photo) as "banner_photo: String",
        $1 IS NULL OR pc.banner_photo IS NOT NULL AS "custom_banner_photo: bool",
        ((pc.company_name IS NULL AND TRUE) OR pc.custom_lightbox) as "custom_lightbox: bool",
        pc.header_photo_copyright,
        pc.banner_photo_copyright,
        ((pc.company_name IS NULL AND TRUE) OR pc.custom_content) as "custom_content: bool",
        pc.visible,
        pc.thumbnail,
        EXISTS(SELECT 1 FROM PROJECTS p_ WHERE id=p.id AND draft=0) as "published!: bool"
    FROM
        PROJECTS p
    LEFT JOIN
        PROJECT_COMPANIES pc
    ON p.id=pc.project_id AND p.draft=pc.draft AND (pc.company_name = $1 OR ($1 IS NULL AND pc.company_name IS NULL))
    LEFT JOIN
        PROJECT_COMPANIES base
    ON
        p.id=base.project_id
        AND base.company_name IS NULL
        AND p.draft=base.draft
    WHERE id=$2
    ORDER BY p.draft DESC
    "#, company_name, project_id)
    .fetch_one(&mut *conn)
    .await?;

    if company_name.is_none() {
        project_item.weight = None;
    }

    let images =
        image_service::get_images(&mut *conn, &project_id, company_name, project_item.draft).await?;

    Ok((project_item, images))
}

#[derive(Deserialize)]
pub struct ProjectListItem {
    pub id: String,
    pub name: String,
    pub draft: bool,
    pub published: bool,
    pub header_photo: Option<String>,
}

pub async fn full_list(
    conn: &mut SqliteConnection,
    filter: Option<String>,
    user_id: &Option<String>,
) -> Result<Vec<ProjectListItem>, sqlx::Error> {
    let filter = format!("%{}%", filter.unwrap_or_default());

    query_as!(ProjectListItem, r#"
        SELECT 
            p.id, 
            p.name, 
            COALESCE(pc.header_photo, pcb.header_photo) as "header_photo!: String", 
            EXISTS(SELECT 1 FROM PROJECTS p_2 WHERE p_2.id = p.id AND p_2.draft = 1) AS "draft!: bool",
            EXISTS(SELECT 1 FROM PROJECTS p_2 WHERE p_2.id = p.id AND p_2.draft = 0) AS "published!: bool" 
        FROM PROJECTS p
        LEFT JOIN 
            PROJECT_COMPANIES pc
            ON 
            p.id=pc.project_id AND pc.draft=p.draft
        LEFT JOIN 
            PROJECT_COMPANIES pcb
            ON 
            p.id=pcb.project_id AND pc.draft==p.draft
        LEFT JOIN 
            PERMISSIONS pm
            ON 
            (pm.company=pc.company_name OR (pm.company IS NULL AND pc.company_name IS NULL))
        WHERE p.name LIKE $1 AND pm.user=$2 AND pm.edit=1
        GROUP BY p.id
        ORDER BY name"#, filter, user_id)
    .fetch_all(conn)
    .await
}

pub async fn create_new(
    conn: &mut SqliteConnection,
    name: &str,
    location: &Option<String>,
    year: &Option<i64>,
    status: bool,
    publiek_privaat: &str,
) -> Result<String, sqlx::Error> {
    let project_id = Uuid::new_v4().to_string();

    query!(
        "INSERT INTO PROJECTS(
            id, 
            name, 
            location, 
            year, 
            status, 
            publiek_privaat, 
            draft
        ) VALUES ($1, $2, $3, $4, $5, $6, 1)",
        project_id,
        name,
        location,
        year,
        status,
        publiek_privaat
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "INSERT INTO PROJECT_COMPANIES(project_id, draft) VALUES ($1, 1)",
        project_id
    )
    .execute(&mut *conn)
    .await?;

    Ok(project_id)
}

pub async fn delete_project(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool,
) -> Result<(), sqlx::Error> {
    query!(
        "
        PRAGMA defer_foreign_keys=ON;
        DELETE FROM PROJECTS WHERE id=$1 AND draft=$2;
        PRAGMA defer_foreign_keys=OFF;",
        project_id,
        draft
    )
    .execute(&mut *conn)
    .await
    .map(|_| ())
}

pub async fn update_project_draft(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool,
) -> Result<(), sqlx::Error> {
    query!(
        "
        PRAGMA defer_foreign_keys=ON;
        UPDATE PROJECTS SET draft=$2 WHERE id=$1 AND draft=NOT $2;
        PRAGMA defer_foreign_keys=OFF;",
        project_id,
        draft
    )
    .execute(&mut *conn)
    .await
    .map(|_| ())

}

pub async fn name_for_id(
    conn: &mut SqliteConnection,
    project_id: &str,
) -> Result<String, sqlx::Error> {
    query_scalar!("SELECT name FROM PROJECTS WHERE id=$1", project_id)
        .fetch_one(&mut *conn)
        .await
}
