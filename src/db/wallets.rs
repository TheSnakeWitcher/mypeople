use super::aux::to_sqlite_json_key;
use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult, SqliteRow},
    Error,
};
use std::collections::HashMap;

pub async fn insert_wallets(
    conn: &mut SqliteConnection,
    name: &str,
    wallets: &HashMap<String, String>,
) -> Result<Vec<SqliteQueryResult>, Error> {
    let mut result: Vec<SqliteQueryResult> = Vec::with_capacity(wallets.len());
    for (key, val) in wallets {
        let output = insert_wallet(conn, name, key, val)
            .await
            .unwrap_or_default();
        result.push(output);
    }

    return Ok(result);
}

pub async fn insert_wallet(
    conn: &mut SqliteConnection,
    name: &str,
    wallet_key: &str,
    wallet_val: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&wallet_key);

    let output = query("UPDATE contacts SET wallets = json_insert(wallets,?,?) WHERE name = ? ;")
        .bind(key)
        .bind(wallet_val)
        .bind(name)
        .execute(conn)
        .await?;

    return Ok(output);
}

pub async fn remove_wallet(
    conn: &mut SqliteConnection,
    name: &str,
    wallet: &str,
) -> Result<SqliteQueryResult, Error> {
    let key = to_sqlite_json_key(&wallet);

    let output = query("UPDATE contacts SET wallets = json_remove(wallets,?) WHERE name = ? ;")
        .bind(key)
        .bind(name)
        .execute(conn)
        .await?;

    return Ok(output);
}
