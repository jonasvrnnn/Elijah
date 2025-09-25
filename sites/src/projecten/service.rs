use sqlx::{prelude::FromRow, query_as, Database, Decode, SqliteConnection};
use std::error::Error;

use crate::projecten::endpoint::ProjectItem;

pub struct Parties(pub Vec<String>);

impl<'r, Sqlite: Database> Decode<'r, Sqlite> for Parties
where
    &'r str: Decode<'r, Sqlite>
{
    fn decode(
        value: <Sqlite as Database>::ValueRef<'r>,
    ) -> Result<Parties, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Sqlite>>::decode(value)?.to_string();

        let x: Vec<String> = value.split(",").map(|v| v.to_string()).filter(|v| !v.is_empty()).collect();

        Ok(Parties(x))
    }
}

pub async fn get_project_data<'a>(
    conn: &mut SqliteConnection,
    slug: &str,
    company_name: &str,
) -> Result<ProjectItem, sqlx::Error> {
    query_as!(ProjectItem, r#"
    SELECT
        p.name,
        p.slug,
        p.location,
        p.year,
        p.learn_more,
        p.status,
        p.publiek_privaat,
        COALESCE(pc.introduction, base.introduction) as "introduction: String",
        COALESCE(pc.header_photo, base.header_photo) as "header_photo: String",
        COALESCE(pc.banner_photo, base.banner_photo) as "banner_photo: String",
        pc.header_photo_copyright,
        COALESCE(pc.banner_photo_copyright, base.banner_photo_copyright) as "banner_photo_copyright: String",
        (SELECT pp.party_name
        FROM PROJECT_PARTIES pp 
        WHERE pp.project_id = p.id AND pp.type = 'contractor' AND pp.draft=p.draft) AS "contractors: Parties",
        
        (SELECT pp.party_name
        FROM PROJECT_PARTIES pp 
        WHERE pp.project_id = p.id AND pp.type = 'architect' AND pp.draft=p.draft) AS "architects: Parties",
        
        (SELECT pp.party_name
        FROM PROJECT_PARTIES pp 
        WHERE pp.project_id = p.id AND pp.type = 'client' AND pp.draft=p.draft) AS "clients: Parties",
        
        (SELECT industry
        FROM PROJECT_INDUSTRIES pi
        WHERE pi.project_id=p.id AND pi.draft=p.draft) AS "industries: Parties"
    FROM
        PROJECTS p
    LEFT JOIN
        PROJECT_COMPANIES pc
    ON p.id=pc.project_id AND p.draft=pc.draft AND (pc.company_name = $1 OR ($1 IS NULL AND pc.company_name IS NULL))
    JOIN
        PROJECT_COMPANIES base
    ON
        p.id=base.project_id
        AND base.company_name IS NULL
        AND p.draft=base.draft
    WHERE slug=$2 AND p.draft=0
    "#, company_name, slug)
    .fetch_one(conn)
    .await
}

pub struct GroupCompany {
    pub name: Option<String>,
    pub url: Option<String>,
}

pub async fn get_group_companies<'a>(
    conn: &mut SqliteConnection,
    project_name: &str,
) -> Result<Vec<GroupCompany>, sqlx::Error> {
    query_as!(
        GroupCompany,
        r#"
    SELECT
        company_name as "name: String",
        NULL as "url: String"
    FROM PROJECT_COMPANIES pc
    LEFT JOIN PROJECTS p ON pc.project_id=p.id
    WHERE p.slug=$1 AND company_name IS NOT NULL AND company_name IS NOT 'Corporate' AND pc.draft=0 AND p.draft=0
    "#,
        project_name
    )
    .fetch_all(conn)
    .await
}

pub struct CoreNumber {
    pub number: String,
    pub title: String,
}

pub async fn get_core_numbers<'a>(conn: &mut SqliteConnection, project_name: &str) -> Result<Vec<CoreNumber>, sqlx::Error> {
    query_as!(
        CoreNumber,
        r#"
    SELECT
        number, title
    FROM CORE_NUMBERS cn
    LEFT JOIN PROJECTS p ON cn.project_id=p.id
    WHERE p.slug=$1 AND cn.draft=0 AND p.draft=0
    "#,
        project_name
    )
    .fetch_all(conn)
    .await
}

pub struct ContentEntry {
    pub text: Option<String>,
    pub image: Option<String>,
    pub image_copyright: Option<String>,
    pub quote: Option<String>,
}

#[derive(Debug)]
pub enum GetContentError {
    ContentIdRetrievalError(sqlx::Error),
    ContentIdNotFound,
    ContentRetrievalError(sqlx::Error),
}

pub async fn get_content<'a>(
    conn: &mut SqliteConnection,
    slug: &str,
    company_name: &str,
) -> Result<Vec<ContentEntry>, GetContentError> {
    query_as!(
        ContentEntry,
        r#"
        WITH RECURSIVE ContentItems AS (
            SELECT
                pc.id,
                text,
                image,
                image_copyright,
                quote
                FROM PROJECT_COMPANIES pcs 
            JOIN PROJECTS p ON pcs.project_id=p.id AND pcs.draft=p.draft
            JOIN PROJECT_CONTENT pc ON pc.project_id=pcs.project_id AND pc.draft=pcs.draft AND
            ((custom_content=1 AND pc.company_name=pcs.company_name) OR (custom_content=0 AND pc.company_name IS NULL))
            WHERE p.slug=$1
            AND pcs.company_name=$2
            AND pcs.draft=0
            AND pc.previous_entry IS NULL

            UNION ALL

            SELECT
                pc.id,
                pc.text,
                pc.image,
                pc.image_copyright,
                pc.quote
            FROM PROJECT_CONTENT pc
            JOIN ContentItems pi ON pc.previous_entry = pi.id AND draft = 0
        )

        SELECT text, image, image_copyright, quote from ContentItems"#,
        slug, company_name
    )
    .fetch_all(conn)
    .await
    .map_err(|err| GetContentError::ContentRetrievalError(err))
}

pub struct ImageData {
    pub image: Option<String>,
    pub image_copyright: Option<String>,
    pub alt: Option<String>,
}

#[derive(Debug)]
pub enum GetImagesError {
    LightboxIdRetrievalError(sqlx::Error),
    LightboxIdNotFound,
    LightboxImagesRetrievalError(sqlx::Error),
}

pub async fn get_images<'a>(
    conn: &mut SqliteConnection,
    slug: &str,
    company_name: &str,
) -> Result<Vec<ImageData>, GetImagesError> {
    query_as!(ImageData, "SELECT
            image,
            image_copyright,
            alt
        FROM
            IMAGES i
            JOIN PROJECTS p on i.project_id = p.id
            AND p.slug = $1
            AND p.draft = i.draft
            JOIN PROJECT_COMPANIES pc on i.project_id = pc.project_id
            AND pc.draft = i.draft
            AND (
                (
                    pc.custom_lightbox = 1
                    AND i.company_name = $2
                )
                OR (
                    pc.custom_lightbox = 0
                    AND i.company_name IS NULL
                )
            )
        WHERE
            i.draft = 0
            AND p.slug = 'arts-47'
            AND (pc.company_name = $2 OR ($2 IS NULL AND pc.company_name IS NULL));", slug, company_name)
    .fetch_all(conn)
    .await
    .map_err(|err| GetImagesError::LightboxImagesRetrievalError(err))
}

pub struct CarouselProject {
    pub name: String,
    pub slug: Option<String>,
    pub location: Option<String>,
    pub header_photo: Option<String>,
}

pub async fn get_carousel_projects(
        conn: &mut SqliteConnection,
        company_name: &str
) -> Result<Vec<CarouselProject>, sqlx::Error> {
    query_as!(
        CarouselProject,
        "
        SELECT 
            p.name, 
            p.slug, 
            p.location, 
            COALESCE(pc.header_photo, pcb.header_photo) as header_photo 
        FROM PROJECTS p 
        LEFT JOIN PROJECT_COMPANIES pc ON pc.project_id=p.id AND pc.company_name=$1
        LEFT JOIN PROJECT_COMPANIES pcb ON pcb.project_id=p.id AND pcb.company_name IS NULL
        WHERE pc.show_in_carousel=1
        GROUP BY p.name, p.slug;",
        company_name
    )
    .fetch_all(&mut *conn)
    .await
}

#[derive(FromRow)]
pub struct Project {
    pub name: String,
    pub slug: String,
    pub location: Option<String>,
    pub thumbnail: Option<String>
}

pub async fn get_projects(
        conn: &mut SqliteConnection,
        company: &str,
        status: &Option<bool>,
        sector: &Option<String>,
        publiek_privaat: Option<String>,
        limit: u8,
        offset: u8,
        search: &str,
        bedrijf: &Option<String>,
        tags: &Option<Vec<String>>
) -> Result<Vec<Project>, sqlx::Error> {
    let tags_len = match tags  {
        Some(tags) => tags.len(),
        None => 0,
    };

    let tags = tags.clone().map(|tags| tags.iter().map(|tag| format!("'{tag}'")).collect::<Vec<String>>().join(","));

    let tags = tags.map(|tags| format!("AND pt.tag IN ({tags})")).unwrap_or_default();

    let having = if tags_len > 0 {
        format!("HAVING COUNT(DISTINCT pt.tag) = {tags_len}")
    } else {
        String::new()
    };

    println!("{limit}: {offset}");

    let projects: Vec<Project> = query_as(
        &format!(r#"
        SELECT p.name, p.slug, p.location,
            CASE
                WHEN pc.thumbnail = 'base' THEN COALESCE(pcb.thumbnail, pcb.header_photo)
                WHEN pc.thumbnail IS NULL THEN COALESCE(pc.header_photo, pcb.header_photo)
                ELSE pc.thumbnail
            END as thumbnail
            FROM PROJECTS p 
            LEFT JOIN PROJECT_COMPANIES pc ON p.id = pc.project_id AND p.draft = pc.draft
            LEFT JOIN PROJECT_INDUSTRIES pi ON p.id = pi.project_id AND p.draft = pi.draft
            LEFT JOIN PROJECT_COMPANIES pcb ON pcb.project_id=p.id AND pcb.company_name IS NULL AND p.draft = pcb.draft
            LEFT JOIN PROJECT_TAGS pt ON pt.project_id=p.id AND p.draft = pt.draft
            WHERE 
                pc.company_name = $1
                AND
                ($2 IS NULL OR p.status = $2)
                AND
                ($3 IS NULL OR pi.industry = $3)
                AND
                ($4 IS NULL OR p.publiek_privaat = $4)
                AND
                ($5 IS NULL OR EXISTS(SELECT 1 FROM PROJECT_COMPANIES WHERE project_id=p.id AND company_name=$5))
                {tags}
                AND
                p.name LIKE $7
                AND p.draft=0
                AND pc.visible=1
            GROUP BY p.id
            {having}
            ORDER BY pc.weight DESC, p.year DESC
            LIMIT $8 OFFSET $9"#))
            .bind(company)
            .bind(status)
            .bind(sector)
            .bind(publiek_privaat)
            .bind(bedrijf)
            .bind(tags)
            .bind(search)
            .bind(limit)
            .bind(offset)
    .fetch_all(&mut *conn)
    .await
    .unwrap();

    Ok(projects)
}