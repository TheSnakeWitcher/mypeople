pub mod cli;
pub mod db;

pub use sqlx::{
    sqlite::{SqliteConnection, SqliteRow},
    Connection,
};

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

            if names.len() == 1 {
                db::queries::insert_contact(&mut conn, names[0].as_str(), None).await?;
            } else {
                db::queries::insert_contacts(&mut conn, names).await?;
            }

        }

        Some(("rm", sub_matches)) => {
            let names = sub_matches
                .get_many::<String>("name")
                .into_iter()
                .flatten()
                .collect::<Vec<&String>>();

            if names.len() == 1 {
                db::queries::remove_contact(&mut conn, names[0].as_str()).await?;
            } else {
                db::queries::remove_contacts(&mut conn, names).await?;
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

        _ => unreachable!()
    }

    Ok(())
}
