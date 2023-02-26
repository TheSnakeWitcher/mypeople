pub mod cli;
pub mod db;
pub mod dispatchers;

use sqlx::{Connection, SqliteConnection};
use std::path::Path;

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
            println!("{:#?}", contacts);
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

        _ => unreachable!(),
    }

    Ok(())
}
