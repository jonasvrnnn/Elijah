use std::collections::HashMap;

use sqlx::{SqliteConnection, prelude::FromRow, query, query_as, query_scalar};
use uuid::Uuid;


pub async fn delete_content_entry(conn: &mut SqliteConnection, id: &str) -> Result<(), sqlx::Error> {
    let existing_previous_entry = query_scalar!(
        "PRAGMA defer_foreign_keys = ON;
        DELETE FROM PROJECT_CONTENT WHERE id=$1 and draft=1 RETURNING previous_entry",
        id
    )
    .fetch_one(&mut *conn)
    .await?;

    query!(
        r#"
        UPDATE PROJECT_CONTENT
            SET previous_entry = $1
            WHERE previous_entry = $2 AND draft = 1;

        PRAGMA defer_foreign_keys = OFF;
    "#,
        existing_previous_entry,
        id
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn insert_new_content_entry(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    previous_id: &Option<String>,
) -> Result<ContentEntry, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    query!(
        "PRAGMA defer_foreign_keys = ON;

        UPDATE PROJECT_CONTENT
            SET previous_entry=$1
        WHERE
            project_id = $2
            AND (company_name = $3 OR ($3 IS NULL AND company_name IS NULL))
            AND (previous_entry = $4 OR ($4 IS NULL AND previous_entry IS NULL))
            AND draft=1;
        ", id, project_id, company_name, previous_id
    )
    .execute(&mut *conn)
    .await?;

    query_as!(
        ContentEntry,
        "
        INSERT INTO PROJECT_CONTENT(
            project_id,
            company_name,
            id,
            previous_entry,
            draft,
            text
        ) VALUES (
            $1,$2,$3,$4,1,'<h3>Default text</h3>'
        ) RETURNING
            id,
            text,
            image,
            image_copyright,
            quote;

        PRAGMA defer_foreign_keys = OFF;
    ",
        project_id,
        company_name,
        id,
        previous_id
    )
    .fetch_one(&mut *conn)
    .await
}

pub struct ContentEntry {
    pub id: String,
    pub text: String,
    pub image: Option<String>,
    pub image_copyright: Option<String>,
    pub quote: Option<String>,
}

pub async fn get_content<'a>(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    draft: bool,
) -> Result<Vec<ContentEntry>, sqlx::Error> {
    query_as!(
        ContentEntry,
        r#"
        WITH RECURSIVE ContentItems AS (
            SELECT
                id,
                text,
                image,
                image_copyright,
                quote
            FROM PROJECT_CONTENT
                where 
                project_id=$1
                AND (company_name=$2 OR ($2 IS NULL and company_name IS NULL))
                AND draft = $3
                AND previous_entry IS NULL

            UNION ALL

            SELECT
                pc.id,
                pc.text,
                pc.image,
                pc.image_copyright,
                pc.quote
            FROM PROJECT_CONTENT pc
            INNER JOIN ContentItems pi ON pc.previous_entry = pi.id AND draft = $3
        )

        SELECT * from ContentItems"#,
        project_id,
        company_name,
        draft
    )
    .fetch_all(conn)
    .await
}

pub async fn get_content_ids<'a>(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &Option<String>,
    draft: bool,
) -> Result<Vec<String>, sqlx::Error> {
    query_scalar!(
        r#"
        WITH RECURSIVE ContentItems AS (
            SELECT
                id
            FROM PROJECT_CONTENT
                where 
                project_id=$1
                AND company_name=$2
                AND draft = $3
                AND previous_entry IS NULL

            UNION ALL

            SELECT
                pc.id
            FROM PROJECT_CONTENT pc
            INNER JOIN ContentItems pi ON pc.previous_entry = pi.id AND draft = $3
        )

        SELECT id from ContentItems"#,
        project_id,
        company_name,
        draft
    )
    .fetch_all(conn)
    .await
}

pub async fn update_content_entry_text(
    conn: &mut SqliteConnection,
    id: &str,
    text: &str,
) -> Result<String, sqlx::Error> {
    query_scalar!(
        "UPDATE PROJECT_CONTENT
            SET text=$1 
            WHERE id=$2
            AND draft=1 
            RETURNING text",
        text,
        id
    )
    .fetch_one(conn)
    .await
}

pub async fn update_content_entry_quote(
    conn: &mut SqliteConnection,
    id: &str,
    quote: &Option<String>,
) -> Result<Option<String>, sqlx::Error> {
    query_scalar!(
        "UPDATE PROJECT_CONTENT
            SET quote=$1 
            WHERE id=$2
            AND draft=1 
            RETURNING quote",
        quote,
        id
    )
    .fetch_one(conn)
    .await
}

#[derive(FromRow)]
pub struct ImageData {
    pub image: Option<String>,
    pub image_copyright: Option<String>,
}

pub async fn update_content_entry_image(
    conn: &mut SqliteConnection,
    id: &str,
    image: &Option<String>,
    copyright: &Option<String>,
) -> Result<ImageData, sqlx::Error> {
    query_as!(
        ImageData,
        "UPDATE PROJECT_CONTENT
            SET image=$1, image_copyright=$2
            WHERE id=$3
            AND draft=1 
            RETURNING image, image_copyright",
        image,
        copyright,
        id
    )
    .fetch_one(conn)
    .await
}

struct CustomiseContentEntry {
    id: String,
    previous_entry: Option<String>,
}

pub async fn customise_content(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
) -> Result<Vec<ContentEntry>, sqlx::Error> {
    let mut old_to_new = HashMap::new();

    let entries = query_as!(CustomiseContentEntry, "SELECT id, previous_entry FROM PROJECT_CONTENT WHERE draft=1 AND project_id=$1 AND company_name IS NULL", project_id)
    .fetch_all(&mut *conn)
    .await?;

    for entry in &entries {
        old_to_new.insert(entry.id.to_string(), Uuid::new_v4().to_string());
    }

    let mut new_entries = vec![];

    for entry in &entries {
        let new_id = &old_to_new[&entry.id];
        let previous_id = entry
            .previous_entry
            .as_ref()
            .map(|pe| old_to_new[pe].clone());

        let new_entry = query_as!(ContentEntry, "
        INSERT INTO PROJECT_CONTENT(
            project_id, 
            company_name, 
            id, 
            previous_entry, 
            draft, 
            text, 
            image, 
            image_copyright, 
            quote, 
            quote_small
        )

        SELECT
            project_id,
            $1,
            $2,
            $3,
            draft,
            text,
            image,
            image_copyright,
            quote,
            quote_small FROM PROJECT_CONTENT WHERE draft=1 AND id=$4
            RETURNING id, text, image, image_copyright, quote", company_name, new_id, previous_id, entry.id)
            .fetch_one(&mut *conn)
            .await?;

        new_entries.push(new_entry);
    }

    query!("UPDATE PROJECT_COMPANIES SET custom_content=1 WHERE draft=1 AND project_id=$1 AND company_name=$2", project_id, company_name)
    .execute(&mut *conn)
    .await?;

    Ok(new_entries)
}
