use super::aux::to_sqlite_json_key;

use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};

use std::collections::HashMap;

pub async fn get_social_nets(conn: &mut SqliteConnection, name: &str) -> Result<String, Error> {
    let data = query!(
        "SELECT social_nets FROM contacts WHERE name = ? LIMIT 1 ;",
        name
    )
    .map(|item| item.social_nets.to_string())
    .fetch_one(conn)
    .await?;

    Ok(data)
}

pub async fn insert_social_nets(
    conn: &mut SqliteConnection,
    name: &str,
    social_nets: &HashMap<String, String>,
) -> Result<Vec<SqliteQueryResult>, Error> {
    let mut result: Vec<SqliteQueryResult> = Vec::with_capacity(social_nets.len());
    for (key, val) in social_nets {
        let output = insert_social_net(conn, name, key, val)
            .await
            .unwrap_or_default();
        result.push(output);
    }

    return Ok(result);
}

pub async fn insert_social_net(
    conn: &mut SqliteConnection,
    name: &str,
    social_net_key: &str,
    social_net_val: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&social_net_key);

    let output = query!(
        "UPDATE contacts SET social_nets = json_insert(social_nets,?,?)
        WHERE name = ? ;",
        key,
        social_net_val,
        name
    )
    .execute(conn)
    .await?;

    return Ok(output);
}

pub async fn remove_social_net(
    conn: &mut SqliteConnection,
    name: &str,
    social_net: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&social_net);

    let output = query!(
        "
        UPDATE contacts
        SET social_nets = json_remove(social_nets,?)
        WHERE name = ? ;
    ",
        key,
        name
    )
    .execute(conn)
    .await?;

    return Ok(output);
}