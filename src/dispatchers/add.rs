use super::aux;
use crate::db;
use clap::ArgMatches;
use sqlx::{Error, SqliteConnection};
use std::collections::HashMap;

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
        let mut val = sub_matches.get_one::<String>(arg).unwrap().to_string();
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

            "phones" => {
                let Ok(insert_val) = aux::try_parse(&mut val) else {
                    println!("invalid format of phones args passed");
                    return Ok(())
                } ;

                if let Err(error) = db::queries::insert_phones(conn, name, &insert_val).await {
                    println!(
                        "failed to add phones of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            "emails" => {
                let Ok(insert_val) = aux::try_parse(&mut val) else {
                    println!("invalid format of emails args passed");
                    return Ok(())
                } ;

                if let Err(error) = db::queries::insert_emails(conn, name, &insert_val).await {
                    println!(
                        "failed to add emails of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            "social_nets" => {
                let Ok(insert_val) = aux::try_parse(&mut val) else {
                    println!("invalid format of social_nets args passed");
                    return Ok(())
                } ;

                if let Err(error) = db::queries::insert_social_nets(conn, name, &insert_val).await {
                    println!(
                        "failed to add social_nets of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            "wallets" => {
                let Ok(insert_val) = aux::try_parse(&mut val) else {
                    println!("invalid format of wallets args passed");
                    return Ok(())
                } ;

                if let Err(error) = db::queries::insert_wallets(conn, name, &insert_val).await {
                    println!(
                        "failed to add wallets of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            _ => unreachable!(),
        }
    }

    Ok(())
}
