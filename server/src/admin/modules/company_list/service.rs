use sqlx::{QueryBuilder, Sqlite, SqliteConnection, query, query_as, query_scalar};

pub struct CompanyLists {
    pub active: Vec<CompanyListItem>,
    pub non_active: Vec<CompanyListItem>,
}

pub struct CompanyListItem {
    pub name: String,
    pub active: i64,
}

impl CompanyListItem {
    fn is_active(&self) -> bool {
        self.active != 0
    }
}

pub async fn get_company_lists(
    conn: &mut SqliteConnection,
    project_id: &str,
    draft: bool,
    user_id: &str,
) -> Result<CompanyLists, sqlx::Error> {
    let lists = match query_as!(
        CompanyListItem,
        r#"
    SELECT 
        c.name, 
        COALESCE(pc.company_name IS NOT NULL, 0) AS "active!: bool"
    FROM 
        COMPANIES c 
    LEFT JOIN 
        PROJECT_COMPANIES pc ON c.name = pc.company_name 
        AND pc.project_id = $1
        AND pc.draft=$2
    LEFT JOIN 
        PERMISSIONS pm ON c.name = pm.company 
        AND pm.user=$3
    WHERE (pm.edit IS NULL OR pm.edit=1)
    "#,
        project_id,
        draft,
        user_id
    )
    .fetch_all(conn)
    .await
    {
        Ok(list) => list.into_iter().partition(|c| c.is_active()),
        Err(err) => return Err(err),
    };

    Ok(CompanyLists {
        active: lists.0,
        non_active: lists.1,
    })
}

pub async fn does_project_have_company(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
    draft: bool,
) -> Result<bool, sqlx::Error> {
    query_scalar!(r#"SELECT EXISTS(
        SELECT project_id FROM PROJECT_COMPANIES WHERE project_id=$1 AND company_name=$2 AND draft=$3) as 'exists: bool'"#, project_id, company_name, draft
    )
    .fetch_one(&mut *conn)
    .await
}

pub async fn add_company_to_project(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
) -> Result<(), sqlx::Error> {
    query!(
        "
        INSERT INTO PROJECT_COMPANIES(
            project_id,
            company_name,
            draft
        ) VALUES ($1, $2, 1)
    ",
        project_id,
        company_name
    )
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn delete_company_from_project(
    conn: &mut SqliteConnection,
    project_id: &str,
    company_name: &str,
) -> Result<(), sqlx::Error> {
    query!(
        "
        DELETE FROM PROJECT_COMPANIES
        WHERE project_id=$1
        AND company_name=$2
        AND draft=1
    ",
        project_id,
        company_name
    )
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn full_company_list(conn: &mut SqliteConnection) -> Result<Vec<String>, sqlx::Error> {
    query_scalar!("SELECT name FROM COMPANIES ORDER by name")
        .fetch_all(conn)
        .await
}

pub async fn company_search_list(
    conn: &mut SqliteConnection,
    filter: &Option<String>,
    user_id: &Option<String>,
    exclude: &Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    let filter = format!("%{}%", filter.as_deref().unwrap_or_default());

    let mut query_builder = QueryBuilder::<Sqlite>::new(
        r#"
            SELECT
                name
            FROM COMPANIES c
            JOIN PERMISSIONS p ON p.user=
    "#,
    );

    query_builder.push_bind(user_id);

    query_builder.push(" AND c.name=p.company WHERE p.`create`=1 AND name LIKE ");

    query_builder.push_bind(filter);

    query_builder.push(" AND name NOT IN(");

    let mut separated = query_builder.separated(", ");
    for excluded in exclude {
        separated.push_bind(excluded);
    }

    separated.push_unseparated(") LIMIT 10");

    let query = query_builder.build_query_scalar();

    query.fetch_all(conn).await
}
