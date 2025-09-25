use sqlx::{query_as, SqliteConnection};

pub struct HeaderContent {
    pub name: String,
    pub header_photo: Option<String>,
    pub header_photo_copyright: Option<String>,
    pub custom_header_photo: Option<bool>
}

pub struct HeaderImageData {
    pub header_photo: Option<String>,
    pub header_photo_copyright: Option<String>
}

pub async fn update_project_header_photo(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    image: &Option<String>,
    copyright: &Option<String>
) -> Result<HeaderImageData, sqlx::Error> {
    query_as!(HeaderImageData, "UPDATE PROJECT_COMPANIES SET header_photo=$1, header_photo_copyright=$2 WHERE project_id=$3 AND (company_name=$4 OR ($4 IS NULL AND company_name IS NULL)) AND draft=1 RETURNING header_photo, header_photo_copyright", image, copyright, project_id, company_name)
    .fetch_one(conn)
    .await
}

pub async fn customise_project_header_photo(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str
) -> Result<HeaderImageData, sqlx::Error> {
    query_as!(HeaderImageData, "UPDATE PROJECT_COMPANIES SET header_photo=(SELECT header_photo FROM PROJECT_COMPANIES WHERE project_id=$1 AND company_name IS NULL and DRAFT=1) WHERE project_id=$1 AND (company_name=$2 OR ($2 IS NULL AND company_name IS NULL)) AND draft=1 RETURNING header_photo, header_photo_copyright", project_id, company_name)
    .fetch_one(conn)
    .await
}

pub async fn get_header_data(conn: &mut SqliteConnection, project_id: &str, company_name: &Option<String>) -> Result<HeaderContent, sqlx::Error> {    
    query_as!(HeaderContent, r#"
    SELECT
        p.name,
        COALESCE(pc.header_photo, base.header_photo) as "header_photo: String",
        COALESCE(pc.header_photo_copyright, base.header_photo_copyright) as "header_photo_copyright: String",
        $1 IS NULL OR pc.header_photo IS NOT NULL AS "custom_header_photo: bool"
    FROM
        PROJECTS p
    LEFT JOIN
        PROJECT_COMPANIES pc
    ON p.id=pc.project_id AND p.draft=pc.draft AND (pc.company_name = $1 OR ($1 IS NULL AND pc.company_name IS NULL))
    LEFT JOIN
        PROJECT_COMPANIES base
    ON
        pc.project_id=base.project_id
        AND base.company_name IS NULL
        AND p.draft=base.draft
    WHERE id=$2
    ORDER BY p.draft DESC
    "#, company_name, project_id)
    .fetch_one(conn)
    .await
}