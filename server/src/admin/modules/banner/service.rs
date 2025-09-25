use sqlx::{query_as, SqliteConnection};

pub struct BannerImageData {
    pub banner_photo: Option<String>,
    pub banner_photo_copyright: Option<String>
}

pub async fn update_project_banner_photo(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    image: &Option<String>,
    copyright: &Option<String>
) -> Result<BannerImageData, sqlx::Error> {
    query_as!(BannerImageData, "UPDATE PROJECT_COMPANIES SET banner_photo=$1, banner_photo_copyright=$2 WHERE project_id=$3 AND (company_name=$4 OR ($4 IS NULL AND company_name IS NULL)) AND draft=1 RETURNING banner_photo, banner_photo_copyright", image, copyright, project_id, company_name)
    .fetch_one(conn)
    .await
}

pub async fn customise_project_banner_photo(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str
) -> Result<BannerImageData, sqlx::Error> {
    query_as!(BannerImageData, "UPDATE PROJECT_COMPANIES SET banner_photo=(SELECT banner_photo FROM PROJECT_COMPANIES WHERE project_id=$1 AND company_name IS NULL and DRAFT=1) WHERE project_id=$1 AND (company_name=$2 OR ($2 IS NULL AND company_name IS NULL)) AND draft=1 RETURNING banner_photo, banner_photo_copyright", project_id, company_name)
    .fetch_one(conn)
    .await
}