use serde_json::Value;
use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};

pub async fn insert_groups(conn: &mut SqliteConnection, name: &str, groups: &[Value]) {
    for group in groups.iter() {
        let Some(group) = group.as_str() else {
            continue
        };
        if let Err(_) = insert_group(conn, name, group).await {
            println!("failed to import group {} of contact {}", group, name);
        }
    }
}

pub async fn insert_group(
    conn: &mut SqliteConnection,
    name: &str,
    group: &str,
) -> Result<SqliteQueryResult, Error> {
    let output =
        query("UPDATE contacts SET groups = json_insert(groups,'$[#]',?) WHERE name = ? ;")
            .bind(group)
            .bind(name)
            .execute(conn)
            .await?;

    return Ok(output);
}

pub async fn remove_group(
    conn: &mut SqliteConnection,
    name: &str,
    group: &str,
) -> Result<SqliteQueryResult, Error> {
    let output = query(
        "UPDATE contacts
        SET groups = json_remove(groups,(
                SELECT fullkey FROM json_each(groups) WHERE value = ?
        ))
        WHERE name = ? ;",
    )
    .bind(group)
    .bind(name)
    .execute(conn)
    .await?;

    return Ok(output);
}
