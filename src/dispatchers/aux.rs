use std::collections::HashMap;

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
    let result: HashMap<String, String>;
    let Ok(result) = serde_json::from_str(&new_arg) else {
        return Err("failed to parse phones args".into());
    };

    return Ok(result);
}
