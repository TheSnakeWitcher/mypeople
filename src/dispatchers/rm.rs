use crate::db;
use clap::ArgMatches;
use sqlx::{Error, SqliteConnection};
use std::collections::HashMap;

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

            "phones" => {
                if let Err(error) = db::queries::remove_phone(conn, name, &val).await {
                    println!(
                        "failed to remove phone of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            "emails" => {
                if let Err(error) = db::queries::remove_email(conn, name, &val).await {
                    println!(
                        "failed to remove email of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            "social_nets" => {
                if let Err(error) = db::queries::remove_social_net(conn, name, &val).await {
                    println!(
                        "failed to remove social net of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            "wallets" => {
                if let Err(error) = db::queries::remove_wallet(conn, name, &val).await {
                    println!(
                        "failed to remove wallet of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            _ => unreachable!(),
        }
    }

    Ok(())
}
