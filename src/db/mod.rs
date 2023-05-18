mod contacts;
mod emails;
mod groups;
mod phones;
mod pic;
mod social_nets;
mod util;
mod wallets;

pub mod queries;
pub mod schema;

use super::configuration::Conf;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub async fn init(path: Option<&Path>, conf: &Conf) {
    let cmd: &str = "CREATE TABLE IF NOT EXISTS contacts (
        id          INTEGER PRIMARY KEY NOT NULL,
        name        TEXT NOT NULL,
        pic         TEXT NOT NULL,
        groups      TEXT NOT NULL,
        phones      TEXT NOT NULL,
        emails      TEXT NOT NULL,
        social_nets TEXT NOT NULL,
        wallets     TEXT NOT NULL,
        locations   TEXT NOT NULL,
        events      TEXT NOT NULL,
        notes       TEXT NOT NULL
    ) ;";

    let _ = Command::new("sqlite3")
        .arg(path.unwrap_or(&PathBuf::from(conf.dbfile.to_lowercase())))
        .args(["--cmd", cmd])
        .output();
}
