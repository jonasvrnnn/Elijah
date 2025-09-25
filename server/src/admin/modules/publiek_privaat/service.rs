use sqlx::{query_scalar, SqliteConnection};

pub async fn set_publiek_privaat(
    conn: &mut SqliteConnection,
    project_id: &str,
    publiek_privaat: &str
) -> Result<String, sqlx::Error> {
    query_scalar!("UPDATE PROJECTS SET publiek_privaat=$1 WHERE id=$2 AND draft=1 RETURNING publiek_privaat", publiek_privaat, project_id)
    .fetch_one(conn)
    .await
}

const publiek_privaat_types: [(&str, &str); 3] = [
    ("Publiek", "publiek"),
    ("Privaat", "privaat"),
    ("PPS", "pps"),
];

pub fn get_current_and_next(
    r#type: &str
) -> ((&str, &str), (&str, &str)) {
    let mut index = publiek_privaat_types.iter().position(|t| t.1 == r#type).unwrap_or(0);

    let current = publiek_privaat_types[index];

    if index >= publiek_privaat_types.len() - 1 { 
        index = 0;
    } else {
        index += 1;
    }

    let next = publiek_privaat_types[index];

    (current, next)
}