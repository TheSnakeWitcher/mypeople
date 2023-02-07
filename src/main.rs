pub mod cli;
pub mod db;
pub mod dispatchers;

use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = SqliteConnection::connect(env!("DATABASE_URL_FILE"))
        .await
        .unwrap();

    let matches = cli::new().get_matches();

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            if !sub_matches.args_present() {
                let contacts = db::queries::get_all_contacts(&mut conn).await?;
                println!("{:#?}", contacts);
                return Ok(());
            }

            let names = sub_matches
                .get_many::<String>("name")
                .into_iter()
                .flatten()
                .collect::<Vec<&String>>();

            for name in names {
                let contact = db::queries::get_contact(&mut conn, name).await?;
                println!("{:#?}", contact);
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
                    db::queries::insert_contact(&mut conn, name).await?;
                }

                dispatchers::add_cmd_dispatcher(&mut conn,name,sub_matches).await? ;
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
                    db::queries::remove_contact(&mut conn, name).await?;
                    continue;
                }


                for arg in ids.iter() {
                    let val = sub_matches.get_one::<String>(arg).unwrap().to_string();
                    match arg.clone() {
                        "groups" => {
                            if let Err(err) = db::queries::remove_group(&mut conn, name, &val).await
                            {
                                println!(
                                    "failed to add groups of contact {}\n error: {}",
                                    &name, err
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
