use super::schema::Contact;
use sqlx::{
    query,
    sqlite::{SqliteConnection, SqliteQueryResult},
    Error,
};

pub async fn get_all_contacts(conn: &mut SqliteConnection) -> Result<Vec<Contact>, Error> {
    let rows = query!("SELECT * FROM contacts ORDER BY name ;")
        .map(|item| Contact {
            id: item.id,
            name: item.name,
            pic: item.pic,
            groups: serde_json::from_str(&item.groups).unwrap(),
            emails: serde_json::from_str(&item.emails).unwrap_or_default(),
            phones: serde_json::from_str(&item.phones).unwrap_or_default(),
            social_nets: serde_json::from_str(&item.phones).unwrap_or_default(),
            wallets: serde_json::from_str(&item.wallets).unwrap_or_default(),
            locations: serde_json::from_str(&item.locations).unwrap_or_default(),
            events: serde_json::from_str(&item.events).unwrap_or_default(),
            notes: item.notes,
        })
        .fetch_all(conn)
        .await?;

    Ok(rows)
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
    let contact = query!("SELECT * FROM contacts WHERE name = ? LIMIT 1 ;", name)
        .map(|item| Contact {
            id: item.id,
            name: item.name,
            pic: item.pic,
            groups: serde_json::from_str(&item.groups).unwrap(),
            emails: serde_json::from_str(&item.emails).unwrap_or_default(),
            phones: serde_json::from_str(&item.phones).unwrap_or_default(),
            social_nets: serde_json::from_str(&item.phones).unwrap_or_default(),
            wallets: serde_json::from_str(&item.wallets).unwrap_or_default(),
            locations: serde_json::from_str(&item.locations).unwrap_or_default(),
            events: serde_json::from_str(&item.events).unwrap_or_default(),
            notes: item.notes,
        })
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
