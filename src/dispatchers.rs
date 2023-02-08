use super::db;
use clap::ArgMatches;
use sqlx::{Error, SqliteConnection};
use std::collections::HashMap;

fn try_parse(val: &mut String) -> Result<HashMap<String, String>, ()> {
    if let Ok(insert_val) = serde_json::from_str(val) {
        return Ok(insert_val);
    } else if let Ok(insert_val) = try_prepare(val) {
        return Ok(insert_val);
    } else {
        return Err(());
    }
}

fn try_prepare(arg: &mut String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    if !arg.contains(":") {
        return Err("failed to parse phones args".into());
    }
    let (key, val) = arg.split_once(":").unwrap();

    let mut key = key.to_string();
    key.insert_str(0, "{\"");
    key.push_str("\"");

    let mut val = val.to_string();
    val.insert_str(0, "\"");
    val.push_str("\"}");

    let new_arg = format!("{}:{}", key, val);
    let result: HashMap<String, String>;
    let Ok(result) = serde_json::from_str(&new_arg) else {
        return Err("failed to parse phones args".into());
    };

    return Ok(result);
}

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
                let Ok(insert_val) = try_parse(&mut val) else {
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
                let Ok(insert_val) = try_parse(&mut val) else {
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
                let Ok(insert_val) = try_parse(&mut val) else {
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
                        "failed to remove email of contact {}\n error: {}",
                        &name, error
                    );
                };
            }

            _ => unreachable!(),
        }
    }

    Ok(())
}
