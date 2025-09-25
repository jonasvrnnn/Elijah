use sqlx::{query, query_as, query_scalar, SqliteConnection};

pub struct TM {
    pub name: String
}

pub struct Company {
    pub name: String,
    pub active: bool
}

pub struct Party {
    pub name: String,
    pub active: bool
}

pub struct CheckInputData {
    pub tms: Vec<TM>,
    pub exact_match_exists: bool
}

pub async fn tm_search_list(conn: &mut SqliteConnection, project_id: &str, r#type: &str, filter: &Option<String>, draft: bool) -> Result<Vec<String>, sqlx::Error> {
    let filter = format!("%{}%", filter.as_deref().unwrap_or_default());

    query_scalar!("
        SELECT
            t.name 
        FROM TMS t 
        JOIN PROJECT_TMS pt
        ON t.name = pt.tm_name 
        WHERE pt.project_id = $1 AND pt.draft = $4 AND pt.type = $3 AND t.name LIKE $2
        ORDER BY t.name LIMIT 10", project_id, filter, r#type, draft)
    .fetch_all(conn)
    .await
}

pub async fn check_input(conn: &mut SqliteConnection, input: &Option<String>) -> Result<CheckInputData, sqlx::Error> {
    let filter = format!("%{}%", input.as_deref().unwrap_or_default());

    let tms = query_as!(TM, "SELECT name from TMS WHERE name LIKE $1 LIMIT 5", filter)
    .fetch_all(&mut *conn)
    .await?;

    let exact_match_exists = query_scalar!(r#"SELECT EXISTS(SELECT name from TMS WHERE name=$1) as "exact_match_exists: bool""#, input)
    .fetch_one(&mut *conn)
    .await?;

    Ok(CheckInputData {
        tms,
        exact_match_exists
    })
}

pub async fn get_all_tms(conn: &mut SqliteConnection) -> Result<Vec<TM>, sqlx::Error> {
    query_as!(TM, "SELECT name from TMS")
    .fetch_all(&mut *conn)
    .await
}

pub async fn get_all_companies(conn: &mut SqliteConnection, tm: &str) -> Result<Vec<Company>, sqlx::Error> {
    query_as!(Company, r#"
        SELECT
            c.name,
            tm.name IS NOT NULL as "active: bool"
        FROM COMPANIES c
        LEFT JOIN TM_COMPANIES tm
        ON c.name = tm.company AND tm.name = $1
        ORDER BY c.name
    "#, tm)
    .fetch_all(&mut *conn)
    .await
}

pub async fn get_all_parties(conn: &mut SqliteConnection, tm: &str) -> Result<Vec<Party>, sqlx::Error> {
    query_as!(Party, r#"
        SELECT
            p.name,
            tm.name IS NOT NULL as "active: bool"
        FROM PARTIES p
        LEFT JOIN TM_PARTY tm
        ON p.name = tm.party AND tm.name = $1
        ORDER BY p.name
    "#, tm)
    .fetch_all(&mut *conn)
    .await
}

pub async fn create_new_tm(conn: &mut SqliteConnection, name: &str) -> Result<TM, sqlx::Error> {
    query_as!(TM, "INSERT INTO TMS(name) VALUES ($1) RETURNING name", name)
    .fetch_one(&mut *conn)
    .await
}

pub async fn add_company_to_tm(conn: &mut SqliteConnection, tm: &str, company: &str) -> Result<(), sqlx::Error>{
    query!("INSERT INTO TM_COMPANIES(name, company) VALUES ($1, $2)", tm, company)
    .execute(&mut *conn)
    .await
    .map(|_| ())
}

pub async fn delete_company_from_tm(conn: &mut SqliteConnection, tm: &str, company: &str) -> Result<(), sqlx::Error> {
    query!("DELETE FROM TM_COMPANIES where name=$1 AND company=$2", tm, company)
    .execute(&mut *conn)
    .await
    .map(|_| ())
}

pub async fn add_party_to_tm(conn: &mut SqliteConnection, tm: &str, party: &str) -> Result<(), sqlx::Error> {
    query!("INSERT INTO TM_PARTY(name, party) VALUES ($1, $2)", tm, party)
    .execute(&mut *conn)
    .await
    .map(|_| ())
}

pub async fn delete_party_from_tm(conn: &mut SqliteConnection, tm: &str, party: &str) -> Result<(), sqlx::Error> {
    query!("DELETE FROM TM_PARTY where name=$1 AND party=$2", tm, party)
    .execute(&mut *conn)
    .await
    .map(|_| ())
}


pub async fn project_tm_data_type(conn: &mut SqliteConnection, project_id: &str, r#type: &str, draft: bool) -> Result<Vec<TM>, sqlx::Error> {
    query_as!(TM, "
    SELECT
        t.name
    FROM
        PROJECT_TMS pt
    INNER JOIN
        TMS t ON pt.tm_name = t.name AND pt.project_id = $1
    WHERE pt.type=$2 AND pt.draft=$3
    ", project_id, r#type, draft)
    .fetch_all(conn)
    .await
}

pub struct ProjectTMData {
    pub clients: Vec<TM>,
    pub architects: Vec<TM>,
    pub contractors: Vec<TM>
}

pub async fn project_tm_data(conn: &mut SqliteConnection, project_id: &str, draft: bool) -> Result<ProjectTMData, sqlx::Error> {
    let clients = project_tm_data_type(conn, project_id, "client", draft).await?;
    let architects = project_tm_data_type(conn, project_id, "architect", draft).await?;
    let contractors = project_tm_data_type(conn, project_id, "contractor", draft).await?;

    Ok(ProjectTMData {
        clients,
        architects,
        contractors
    })
}

pub async fn add_tm_to_project(conn: &mut SqliteConnection, project_id: &str, tm_name: &str, r#type: &str) -> Result<(), sqlx::Error> {
    query!("INSERT INTO PROJECT_TMS(project_id, tm_name, draft, type) VALUES ($1, $2, 1, $3)", project_id, tm_name, r#type)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn delete_party_from_project(conn: &mut SqliteConnection, project_id: &str, tm_name: &str, r#type: &str)  -> Result<(), sqlx::Error>{
    query!("DELETE FROM PROJECT_TMS WHERE project_id=$1 AND tm_name=$2 AND draft=1 AND type=$3", project_id, tm_name, r#type)
    .execute(conn)
    .await
    .map(|_| ())
}


pub async fn add_tm(conn: &mut SqliteConnection, name: &str) -> Result<(), sqlx::Error> {
    query!("INSERT INTO TMS(name) VALUES ($1)", name)
    .execute(conn)
    .await
    .map(|_| ())
}

pub async fn check_if_tm_exists(conn: &mut SqliteConnection, name: &str) -> Result<bool, sqlx::Error> {
    query_scalar!(r#"SELECT EXISTS(SELECT 1 from TMS WHERE name=$1) as 'e: bool'"#, name)
    .fetch_one(conn)
    .await
}