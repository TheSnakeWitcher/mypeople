use super::util::to_sqlite_json_key;
use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};
use std::collections::HashMap;


pub async fn insert_phones(
    conn: &mut SqliteConnection,
    name: &str,
    phones: &HashMap<String, String>,
) -> Result<Vec<SqliteQueryResult>, Error> {
    let mut result: Vec<SqliteQueryResult> = Vec::with_capacity(phones.len());
    for (key, val) in phones {
        let output = insert_phone(conn, name, key, val).await.unwrap_or_default();
        result.push(output);
    }

    return Ok(result);
}

pub async fn insert_phone(
    conn: &mut SqliteConnection,
    name: &str,
    phone_key: &str,
    phone_val: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&phone_key);

    let output = query("UPDATE contacts SET phones = json_insert(phones,?,?) WHERE name = ? ;")
        .bind(key)
        .bind(phone_val)
        .bind(name)
        .execute(conn)
        .await?;

    return Ok(output);
}

pub async fn remove_phone(
    conn: &mut SqliteConnection,
    name: &str,
    phone: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&phone);

    let output = query("UPDATE contacts SET phones = json_remove(phones,?) WHERE name = ? ;")
        .bind(key)
        .bind(name)
        .execute(conn)
        .await?;

    return Ok(output);
}
