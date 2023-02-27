use super::aux::to_sqlite_json_key;
use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};
use std::collections::HashMap;

pub async fn insert_emails(
    conn: &mut SqliteConnection,
    name: &str,
    emails: &HashMap<String, String>,
) -> Result<Vec<SqliteQueryResult>, Error> {
    let mut result: Vec<SqliteQueryResult> = Vec::with_capacity(emails.len());
    for (key, val) in emails {
        let output = insert_email(conn, name, key, val).await.unwrap_or_default();
        result.push(output);
    }

    return Ok(result);
}

pub async fn insert_email(
    conn: &mut SqliteConnection,
    name: &str,
    email_key: &str,
    email_val: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&email_key);

    let output = query("UPDATE contacts SET emails = json_insert(emails,?,?) WHERE name = ? ;")
        .bind(key)
        .bind(email_val)
        .bind(name)
        .execute(conn)
        .await?;

    return Ok(output);
}

pub async fn remove_email(
    conn: &mut SqliteConnection,
    name: &str,
    email: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&email);

    let output = query(" UPDATE contacts SET emails = json_remove(emails,?) WHERE name = ? ;")
        .bind(key)
        .bind(name)
        .execute(conn)
        .await?;

    return Ok(output);
}
