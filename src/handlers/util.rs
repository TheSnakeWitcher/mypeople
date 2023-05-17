use clap::ArgMatches;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub const OPTIONS: [&str; 7] = [
    "pic",
    "groups",
    "emails",
    "phones",
    "social_nets",
    "wallets",
    "notes",
];

pub fn try_parse(val: &mut String) -> Result<HashMap<String, String>, ()> {
    if let Ok(insert_val) = serde_json::from_str(val) {
        return Ok(insert_val);
    } else if let Ok(insert_val) = try_prepare(val) {
        return Ok(insert_val);
    } else {
        return Err(());
    }
}

fn try_prepare(arg: &mut String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    if !arg.contains(":") {
        return Err("failed to parse phones args".into());
    }
    let (key, val) = arg.split_once(":").unwrap();

    let mut key = key.to_string();
    key.insert_str(0, "{\"");
    key.push_str("\"");

    let mut val = val.to_string();
    val.insert_str(0, "\"");
    val.push_str("\"}");

    let new_arg = format!("{}:{}", key, val);
    let Ok(result) = serde_json::from_str::<HashMap<String, String>>(&new_arg) else {
        return Err("failed to parse phones args".into());
    };

    return Ok(result);
}

pub fn get_options(sub_matches: &ArgMatches) -> Vec<&str> {
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
    return ids;
}

pub fn value_to_hashmap(value: &Map<String, Value>) -> HashMap<String, String> {
    value
        .into_iter()
        .map(|(key, val): (&String, &Value)| {
            let new_val = serde_json::to_string(val).unwrap();
            return (key.to_owned(), new_val);
        })
        .collect::<HashMap<String, String>>()
}

pub fn get_option_value(contact: &Value, option: &str) -> Result<HashMap<String, String>, ()> {
    let Some(value) = contact.get(option).and_then(|option| option.as_object()) else {
        println!( "error to get {} value",option);
        return Err(())
    };
    Ok(value_to_hashmap(value))
}
