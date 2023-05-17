pub fn to_sqlite_json_key(phone_key: &str) -> String {
    let mut key: String = phone_key.to_owned();
    key.insert_str(0, "$.");
    return key;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_sqlite_json_key_should_append_dollar() {
        assert_eq!(to_sqlite_json_key("key"), "$.key")
    }
}
