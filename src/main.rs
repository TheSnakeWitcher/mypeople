pub mod cli;
pub mod db;
pub mod dispatchers;

use serde_json;
use sqlx::{Connection, SqliteConnection};
use std::{fs::File, io::Write, path::Path};

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

            let contacts = db::queries::get_contacts(&mut conn, names).await?;
            for contact in contacts.iter() {
                if let Ok(output) = serde_json::to_string(contact) {
                    println!("{}", output);
                }
            }
            return Ok(());
        }

        Some(("add", sub_matches)) => {
            let names = sub_matches
                .get_many::<String>("name")
                .into_iter()
                .flatten()
                .collect::<Vec<&String>>();

            if names.is_empty() {
                println!("names must be passed");
                return Ok(());
            }

            for name in names.iter() {
                if db::queries::get_contact(&mut conn, name).await.is_err() {
                    db::queries::insert_contact(&mut conn, name).await?;
                }

                dispatchers::add_cmd_dispatcher(&mut conn, name, sub_matches).await?;
            }
        }

        Some(("rm", sub_matches)) => {
            let names = sub_matches
                .get_many::<String>("name")
                .into_iter()
                .flatten()
                .collect::<Vec<&String>>();

            if names.is_empty() {
                println!("names must be passed");
                return Ok(());
            }

            for name in names.iter() {
                if let Err(err) = db::queries::get_contact(&mut conn, name).await {
                    println!("contact not found");
                    continue;
                }
                dispatchers::rm_cmd_dispatcher(&mut conn, name, &sub_matches).await?;
            }
        }

        Some(("init", sub_matches)) => {
            if !sub_matches.args_present() {
                db::init(None).await;
                return Ok(());
            }

            let Some(path_string) = sub_matches.get_one::<String>("path") else {
                println!("error parsing path input") ;
                return Ok(())
            } ;
            let path = Path::new(path_string);

            db::init(Some(&path)).await;
        }

        Some(("import", sub_matches)) => {
            todo!()
        }

        Some(("export", sub_matches)) => {
            let Some(path_string) = sub_matches.get_one::<String>("path") else {
                println!("error parsing path") ;
                return Ok(())
            } ;

            let Ok(mut file) = File::create(Path::new(path_string)) else {
                println!("error opening file") ;
                return Ok(())
            };

            let Ok(data) = serde_json::to_string_pretty(
                &db::queries::get_all_contacts(&mut conn).await?
            ) else {
                println!("failed to convert output data");
                return Ok(())
            };

            if let Err(err) = file.write_all(&data.as_bytes()) {
                println!("error writing data to file");
            };

            return Ok(());
        }

        Some(("config", sub_matches)) => {
            todo!()
        }

        _ => unreachable!(),
    }

    Ok(())
}
