pub mod cli;
mod configuration;
pub mod db;
pub mod handlers;

use clap::ArgMatches;
use console::{Style, Term};
use db::queries;
use serde_json::Value;
use sqlx::{Connection, SqliteConnection};
use std::{env, fs::File, io::Write, path::Path, process::Command};

const EDITOR: &str = "EDITOR";
const VISUAL: &str = "VISUAL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conf, matches) = (
        configuration::init().expect("configuration error"),
        cli::new().get_matches(),
    );

    let (mut term, error_style) = (Term::stdout(), Style::new().red());

    if let Some(sub_matches) = matches.subcommand_matches("init") {
        if !sub_matches.args_present() {
            db::init(None, &conf).await;
            return Ok(());
        }

        let path = {
            let Some(path_string) = sub_matches.get_one::<String>("path") else {
                    println!("{}",error_style.apply_to("error parsing path input"));
                    return Ok(())
            } ;
            Path::new(path_string)
        };

        db::init(Some(path), &conf).await;
        return Ok(())
    }

    let mut conn = SqliteConnection::connect(&conf.dbfile)
        .await
        .expect("failed to set db connection");

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            let names = get_names(sub_matches);

            let contacts = if names.is_empty() {
                queries::get_all_contacts(&mut conn).await?
            } else {
                queries::get_contacts(&mut conn, names).await?
            };

            handlers::ls_cmd_handler(contacts, sub_matches);
            return Ok(());
        }

        Some(("add", sub_matches)) => {
            let names = get_names(sub_matches);

            if names.is_empty() {
                println!("{}", error_style.apply_to("error: names must be passed"));
                return Ok(());
            }

            for name in names.iter() {
                if queries::get_contact(&mut conn, name).await.is_err() {
                    queries::insert_contact(&mut conn, name).await?;
                }

                handlers::add_cmd_handler(&mut conn, name, sub_matches).await?;
            }
        }

        Some(("rm", sub_matches)) => {
            let names = get_names(sub_matches);

            if names.is_empty() {
                println!("{}", error_style.apply_to("error: names must be passed"));
                return Ok(());
            }

            for name in names.iter() {
                if let Err(err) = queries::get_contact(&mut conn, name).await {
                    println!("{}", error_style.apply_to("contact not found"));
                    continue;
                }
                handlers::rm_cmd_handler(&mut conn, name, sub_matches).await?;
            }
        }

        Some(("import", sub_matches)) => {
            let Some(path) = sub_matches.get_one::<String>("path") else {
                println!("{}",error_style.apply_to("error parsing path"));
                return Ok(())
            } ;

            let Ok(file) = File::open(path) else {
                println!("{}",error_style.apply_to("error opening file"));
                return Ok(())
            };

            let Ok(contacts) = serde_json::from_reader::<File,Value>(file) else {
                println!("{}", error_style.apply_to("error reading data from file"));
                return Ok(());
            };

            handlers::import_cmd_handler(&mut conn, contacts).await;
        }

        Some(("export", sub_matches)) => {
            let Some(path) = sub_matches.get_one::<String>("path") else {
                println!("{}",error_style.apply_to("error parsing path"));
                return Ok(())
            } ;

            let Ok(mut file) = File::create(path) else {
                println!("{}",error_style.apply_to("error opening file"));
                return Ok(())
            };

            let Ok(data) = serde_json::to_string(
                &queries::get_all_contacts(&mut conn).await?
            ) else {
                println!("{}",error_style.apply_to("error reading contacts"));
                return Ok(())
            };

            if let Err(err) = file.write_all(&data.as_bytes()) {
                println!("{}", error_style.apply_to("error writing data to file"));
            };

            return Ok(());
        }

        Some(("config", sub_matches)) => {
            let Ok(editor) = env::var(EDITOR).or(env::var(VISUAL)) else {
                println!("{}",error_style.apply_to("error: set $EDITOR or $VISUAL environment variables to edit files"));
                return Ok(())
            };

            let Ok(status) = Command::new(editor)
                .arg(conf.user_config_file)
                .status()
            else {
                println!("{}",error_style.apply_to("error: failed to open user config file"));
                return Ok(())
            };

            return Ok(());
        }

        _ => unreachable!(),
    }

    if let Err(_) = conn.close().await {
        println!("{}", error_style.apply_to("error closing db"));
    };
    Ok(())
}

pub fn get_names(sub_matches: &ArgMatches) -> Vec<&String> {
    let names = sub_matches
        .get_many::<String>("name")
        .into_iter()
        .flatten()
        .collect::<Vec<&String>>();
    return names;
}
