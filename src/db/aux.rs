pub fn to_sqlite_json_key(phone_key: &str) -> String {
    let mut key: String = phone_key.to_owned();
    key.insert_str(0, "$.");
    return key;
}
