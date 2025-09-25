use sqlx::{query, query_as, query_scalar, QueryBuilder, Sqlite, SqliteConnection};

pub struct Party {
    pub name: String,
    pub url: Option<String>
}

pub async fn full_party_list(conn: &mut SqliteConnection) -> Result<Vec<Party>, sqlx::Error> {
    query_as!(Party, "SELECT name, url FROM PARTIES ORDER BY name")
        .fetch_all(conn)
        .await
}

pub async fn party_search_list(conn: &mut SqliteConnection, filter: &Option<String>, exclude: &Vec<String>) -> Result<Vec<String>, sqlx::Error> {
    let filter = format!("%{}%", filter.as_deref().unwrap_or_default());

    let mut query_builder = QueryBuilder::<Sqlite>::new(r#"
            SELECT
                DISTINCT party_name
            FROM PROJECT_PARTIES
            WHERE party_name LIKE 
    "#);

    query_builder.push_bind(filter);

    query_builder.push("AND party_name NOT IN(");

    let mut separated = query_builder.separated(", ");
    for excluded in exclude {
        separated.push_bind(excluded);
    }
    
    separated.push_unseparated(") ORDER BY party_name LIMIT 10");

    let query=  query_builder.build_query_scalar();

    query
    .fetch_all(conn)
    .await
}

pub async fn project_party_data_type(conn: &mut SqliteConnection, project_id: &str, r#type: &str, draft: bool) -> Result<Vec<Party>, sqlx::Error> {
    query_as!(Party, "
    SELECT
        p.name, p.url
    FROM
        PROJECT_PARTIES pp
    INNER JOIN
        PARTIES p ON pp.party_name = p.name AND pp.project_id = $1
    WHERE pp.type=$2 AND pp.draft=$3
    ", project_id, r#type, draft)
    .fetch_all(conn)
    .await
}

pub struct ProjectPartyData {
    pub clients: Vec<Party>,
    pub architects: Vec<Party>,
    pub contractors: Vec<Party>
}

pub async fn project_party_data(conn: &mut SqliteConnection, project_id: &str, draft: bool) -> Result<ProjectPartyData, sqlx::Error> {
    let clients = project_party_data_type(conn, project_id, "client", draft).await?;
    let architects = project_party_data_type(conn, project_id, "architect", draft).await?;
    let contractors = project_party_data_type(conn, project_id, "contractor", draft).await?;

    Ok(ProjectPartyData {
        clients,
        architects,
        contractors
    })
}

pub async fn add_party(conn: &mut SqliteConnection, name: &str) -> Result<(), sqlx::Error> {
    query!("INSERT INTO PARTIES(name) VALUES ($1)", name)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn check_if_party_exists(conn: &mut SqliteConnection, name: &str) -> Result<bool, sqlx::Error> {
    query_scalar!(r#"SELECT EXISTS(SELECT 1 from PARTIES WHERE name=$1) as 'e: bool'"#, name)
    .fetch_one(conn)
    .await
}

pub async fn add_party_to_project(conn: &mut SqliteConnection, project_id: &str, party_name: &str, r#type: &str) -> Result<(), sqlx::Error> {
    query!("INSERT INTO PROJECT_PARTIES(project_id, party_name, draft, type) VALUES ($1, $2, 1, $3)", project_id, party_name, r#type)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn delete_party_from_project(conn: &mut SqliteConnection, project_id: &str, party_name: &str, r#type: &str) -> Result<(), sqlx::Error> {
    query!("DELETE FROM PROJECT_PARTIES WHERE project_id=$1 AND party_name=$2 AND draft=1 AND type=$3", project_id, party_name, r#type)
    .execute(conn)
    .await
    .map(|_| ())
}