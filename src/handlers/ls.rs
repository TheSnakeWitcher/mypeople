use super::util;
use crate::db;
use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;

pub fn ls_cmd_handler(contacts: db::schema::Contacts, sub_matches: &ArgMatches) {
    let options = util::get_options(sub_matches);

    if !options.iter().any(|option| sub_matches.get_flag(option)) {
        println!("{}", contacts);
        return;
    }

    for contact in contacts.iter() {
        let mut out: HashMap<&str, &Value> = HashMap::new();

        let Ok(val) = serde_json::to_value(contact) else {
            continue
        };

        options.iter().for_each(|option| {
            if sub_matches.get_flag(option) {
                out.insert(option, &val[option]);
            }
        });

        let Ok(data) = serde_json::to_string(&out) else {
            continue;
        } ;
        println!("{}", data);
    }
}
