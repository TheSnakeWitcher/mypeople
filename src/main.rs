pub mod cli;
pub mod db;
pub mod dispatcher;

use sqlx::{
    sqlite::{SqliteConnection, SqliteRow},
    Connection,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = SqliteConnection::connect(env!("DATABASE_URL_FILE"))
        .await
        .unwrap();

    let matches = cli::new().get_matches();

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            if !sub_matches.args_present() {
                let data = db::queries::get_all_contacts(&mut conn).await?;
                println!("{:#?}", data);
            } else {
                let names = sub_matches
                    .get_many::<String>("name")
                    .into_iter()
                    .flatten()
                    .collect::<Vec<&String>>();

                if names.len() == 1 {
                    let data = db::queries::get_contact(&mut conn, &names[0]).await?;
                    println!("{:#?}", data);
                } else {
                    let data = db::queries::get_contacts(&mut conn, names).await?;
                    println!("{:#?}", data);
                }
            }
        }

        Some(("add", sub_matches)) => {
            let names = sub_matches
                .get_many::<String>("name")
                .into_iter()
                .flatten()
                .collect::<Vec<&String>>();

            for name in names.iter() {
                if db::queries::get_contact(&mut conn, name).await.is_err() {
                    db::queries::insert_contact(&mut conn, name, None).await?;
                }

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
                            if let Err(error) = db::queries::insert_pic(&mut conn, name, &val).await
                            {
                                println!(
                                    "failed to set pic of contact {}\n error: {}",
                                    &name, error
                                );
                            };
                        }
                        "groups" => {
                            if let Err(error) =
                                db::queries::insert_group(&mut conn, name, &val).await
                            {
                                println!(
                                    "failed to add groups of contact {}\n error: {}",
                                    &name, error
                                );
                            };
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        Some(("rm", sub_matches)) => {
            let names = sub_matches
                .get_many::<String>("name")
                .into_iter()
                .flatten()
                .collect::<Vec<&String>>();

            for name in names.iter() {
                if let Err(err) = db::queries::get_contact(&mut conn, name).await {
                    println!("contact not found");
                    continue;
                }

                if !sub_matches.args_present() {
                    db::queries::remove_contact(&mut conn, name).await?;
                    continue;
                }

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
                        "groups" => {
                            if let Err(error) =
                                db::queries::remove_group(&mut conn, name, &val).await
                            {
                                println!(
                                    "failed to add groups of contact {}\n error: {}",
                                    &name, error
                                );
                            };
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        Some(("config", sub_matches)) => {
            todo!()
        }

        Some(("import", sub_matches)) => {
            todo!()
        }

        Some(("export", sub_matches)) => {
            todo!()
        }

        Some(("init", sub_matches)) => {
            todo!()
        }

        _ => unreachable!(),
    }

    Ok(())
}
