use super::schema::Contact;
use sqlx::{
    query, query_as,
    sqlite::{SqliteConnection, SqliteQueryResult, SqliteRow},
    Error, FromRow,
};

pub async fn get_all_contacts(conn: &mut SqliteConnection) -> Result<Vec<Contact>, Error> {
    let rows = query("SELECT * FROM contacts ORDER BY name ;")
        .map(|item: SqliteRow| Contact::from_row(&item))
        .fetch_all(conn)
        .await?;
    let out = rows
        .into_iter()
        .filter_map(|item| {
            if item.is_ok() {
                Some(item.unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<Contact>>();
    Ok(out)
}

pub async fn get_contacts(
    conn: &mut SqliteConnection,
    names: Vec<&String>,
) -> Result<Vec<Contact>, Error> {
    let mut contacts: Vec<Contact> = Vec::with_capacity(names.len());
    for name in names {
        let contact = get_contact(conn, &name).await?;
        contacts.push(contact);
    }

    return Ok(contacts);
}

pub async fn get_contact(conn: &mut SqliteConnection, name: &str) -> Result<Contact, Error> {
    let contact = query_as::<_, Contact>("SELECT * FROM contacts WHERE name = ? LIMIT 1 ;")
        .bind(name)
        .fetch_one(conn)
        .await?;

    Ok(contact)
}

pub async fn insert_contact(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<SqliteQueryResult, Error> {
    let output = query!("
        INSERT INTO contacts(id,name, pic, groups, emails, phones, social_nets, wallets, locations, events, notes)
        VALUES(NULL,?,'','[\"all\"]','{}','{}','{}','{}','{}','{}','') ;",name
    )
    .execute(conn)
    .await? ;

    return Ok(output);
}

pub async fn insert_contacts(
    conn: &mut SqliteConnection,
    names: Vec<&String>,
) -> Result<Vec<SqliteQueryResult>, Error> {
    let mut results: Vec<SqliteQueryResult> = Vec::with_capacity(names.len());
    for name in names {
        let result = insert_contact(conn, &name).await?;
        results.push(result);
    }

    return Ok(results);
}

pub async fn remove_contact(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<SqliteQueryResult, Error> {
    let output = query!("DELETE FROM contacts WHERE name = ? ;", name)
        .execute(conn)
        .await?;

    Ok(output)
}

pub async fn remove_contacts(
    conn: &mut SqliteConnection,
    names: Vec<&String>,
) -> Result<Vec<SqliteQueryResult>, Error> {
    let mut results: Vec<SqliteQueryResult> = Vec::with_capacity(names.len());
    for name in names {
        let result = remove_contact(conn, &name).await?;
        results.push(result);
    }

    return Ok(results);
}
