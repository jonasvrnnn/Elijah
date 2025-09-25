use sqlx::{query, query_as, QueryBuilder, Sqlite, SqliteConnection};

pub struct IndustryData {
    pub active: Vec<IndustryDataItem>,
    pub non_active: Vec<IndustryDataItem>
}

pub struct IndustryDataItem {
    pub industry: String,
    pub active: bool
}

pub async fn project_industry_data(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool,
) -> Result<IndustryData, sqlx::Error> {
    query_as!(
        IndustryDataItem,
        r#"SELECT 
            i.name AS industry,
            pi.industry IS NOT NULL as "active: bool"
        FROM 
            INDUSTRIES i
        LEFT JOIN 
            PROJECT_INDUSTRIES pi ON i.name = pi.industry AND pi.project_id = $1 AND pi.draft = $2
            ORDER BY i.name"#,
        project_id,
        draft
    )
    .fetch_all(conn)
    .await
    .map(|r| {
        let lists = r.into_iter().partition(|i| i.active);
        IndustryData {
            active: lists.0,
            non_active: lists.1
        }
    })
}

pub async fn add_industry_to_project(conn: &mut SqliteConnection, project_id: &str, industry: &str) -> Result<(), sqlx::Error> {
    query!("
        INSERT INTO PROJECT_INDUSTRIES(project_id, industry, draft) VALUES ($1, $2, 1)
    ", project_id, industry)
    .execute(conn)
    .await
    .map(|_| ())
}


pub async fn delete_industry_from_project(conn: &mut SqliteConnection, project_id: &str, industry: &str) -> Result<(), sqlx::Error> {
    query!("
        DELETE FROM PROJECT_INDUSTRIES WHERE project_id=$1 AND industry=$2 AND draft=1
    ", project_id, industry)
    .execute(conn)
    .await
    .map(|_| ())
}


pub async fn industry_search_list(conn: &mut SqliteConnection, filter: &Option<String>, exclude: &Vec<String>) -> Result<Vec<String>, sqlx::Error> {
    let filter = format!("%{}%", filter.as_deref().unwrap_or_default());

    let mut query_builder = QueryBuilder::<Sqlite>::new(r#"
            SELECT
                name
            FROM INDUSTRIES
            WHERE name LIKE 
    "#);

    query_builder.push_bind(filter);

    query_builder.push("AND name NOT IN(");

    let mut separated = query_builder.separated(", ");
    for excluded in exclude {
        separated.push_bind(excluded);
    }
    
    separated.push_unseparated(") ORDER BY name LIMIT 15");

    let query=  query_builder.build_query_scalar();

    query
    .fetch_all(conn)
    .await
}