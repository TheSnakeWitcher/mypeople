use clap::ArgMatches;
use sqlx::{Error, SqliteConnection};
use super::db;

pub async fn add_cmd_dispatcher(
    conn: &mut SqliteConnection,
    name: &str,
    sub_matches: &ArgMatches
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
                if let Err(error) = db::queries::insert_pic(conn, name, &val).await
                {
                    println!(
                        "failed to set pic of contact {}\n error: {}",
                        &name, error
                    );
                };
            }
            "groups" => {
                if let Err(error) =
                    db::queries::insert_group(conn, name, &val).await
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

    Ok(())
}
