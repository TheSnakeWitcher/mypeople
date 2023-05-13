use super::aux;
use crate::db;
use clap::ArgMatches;
use std::collections::HashMap;

pub fn ls_cmd_dispatcher(
    contacts: db::schema::Contacts,
    sub_matches: &ArgMatches,
) -> Result<(), ()> {
    let options = aux::get_options(sub_matches);

    if !options.iter().any(|option| sub_matches.get_flag(option)) {
        println!("{}", contacts);
        return Ok(());
    }

    contacts.iter().for_each(|contact| {
        let mut out: HashMap<&str, &serde_json::Value> = HashMap::new();
        let val: serde_json::Value = serde_json::to_value(contact).unwrap_or_default();

        options.iter().for_each(|option| {
            if sub_matches.get_flag(option) {
                out.insert(option, &val[option]);
            }
        });

        println!("{}", serde_json::to_string(&out).unwrap_or_default());
    });

    Ok(())
}
