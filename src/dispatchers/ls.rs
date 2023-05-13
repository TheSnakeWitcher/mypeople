use std::collections::HashMap ;
use super::aux;
use crate::db;
use clap::ArgMatches;

pub fn ls_cmd_dispatcher(
    contacts: db::schema::Contacts,
    sub_matches: &ArgMatches,
) -> Result<(), ()> {
    let options = aux::get_options(sub_matches);

    if options.is_empty() {
        println!("{}", contacts);
        return Ok(())
    }

    contacts.iter().for_each(|contact| {
        let mut out : HashMap<&str,&serde_json::Value> = HashMap::new() ;
        let val: serde_json::Value = serde_json::to_value(contact).unwrap_or_default();
        
        options.iter().for_each(|option| {
            out.insert(option,&val[option]) ;
        });

        println!("{}", serde_json::to_string(&out).unwrap_or_default());
    });

    Ok(())
}
