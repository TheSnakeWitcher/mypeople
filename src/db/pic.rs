use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};

pub async fn insert_pic(
    conn: &mut SqliteConnection,
    name: &str,
    pic: &str,
) -> Result<SqliteQueryResult, Error> {
    let output = query!("UPDATE contacts SET pic = ? WHERE name = ? ;", pic,name)
        .execute(conn)
        .await?;

    return Ok(output);
}
