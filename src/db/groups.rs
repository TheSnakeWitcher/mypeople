use super::queries::get_contact;
use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};

pub async fn insert_group(
    conn: &mut SqliteConnection,
    name: &str,
    group: &str,
) -> Result<SqliteQueryResult, Error> {
    let output = query!(
        "UPDATE contacts SET groups = json_insert(groups,'$[#]',?)
        WHERE name = ? ;",
        group,
        name
    )
    .execute(conn)
    .await?;

    return Ok(output);
}

pub async fn remove_group(
    conn: &mut SqliteConnection,
    name: &str,
    group: &str,
) -> Result<SqliteQueryResult, Error> {
    let output = query!(
        "
        UPDATE contacts
        SET groups = json_remove(groups,(
                SELECT fullkey FROM json_each(groups) WHERE value = ?
        ))
        WHERE name = ? ;
    ",
        group,
        name
    )
    .execute(conn)
    .await?;

    return Ok(output);
}
