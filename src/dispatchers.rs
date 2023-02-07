use super::db;
use clap::ArgMatches;
use sqlx::{Error, SqliteConnection};

pub async fn add_cmd_dispatcher(
    conn: &mut SqliteConnection,
    name: &str,
    sub_matches: &ArgMatches,
) -> Result<(), Error> {
    let ids = sub_matches
        .ids()
        .filter_map(|id| {
            if id.as_str() != "name" {
                Some(id.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    for arg in ids.iter() {
        let val = sub_matches.get_one::<String>(arg).unwrap().to_string();
        match arg.clone() {
            "pic" => {
                if let Err(error) = db::queries::insert_pic(conn, name, &val).await {
                    println!("failed to set pic of contact {}\n error: {}", &name, error);
                };
            }
            "groups" => {
                if let Err(error) = db::queries::insert_group(conn, name, &val).await {
                    println!(
                        "failed to add groups of contact {}\n error: {}",
                        &name, error
                    );
                };
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

pub async fn rm_cmd_dispatcher(
    conn: &mut SqliteConnection,
    name: &str,
    sub_matches: &ArgMatches,
) -> Result<(), Error> {
    let ids = sub_matches
        .ids()
        .filter_map(|id| {
            if id.as_str() != "name" {
                Some(id.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    if ids.is_empty() {
        db::queries::remove_contact(conn, name).await?;
        return Ok(());
    }

    for arg in ids.iter() {
        let val = sub_matches.get_one::<String>(arg).unwrap().to_string();
        match arg.clone() {
            "groups" => {
                if let Err(err) = db::queries::remove_group(conn, name, &val).await {
                    println!("failed to add groups of contact {}\n error: {}", name, err);
                };
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
