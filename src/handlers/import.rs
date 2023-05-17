use super::util::{self, OPTIONS};
use crate::db::queries;

pub async fn import_cmd_handler(conn: &mut sqlx::SqliteConnection, contacts: serde_json::Value) {
    for contact in contacts.as_array().unwrap().iter() {
        let Some(name) = contact.get("name").and_then(|name| name.as_str() ) else {
            continue
        };

        if let Err(_) = queries::insert_contact(conn, &name).await {
            println!("error importing contact name");
            continue;
        }

        println!("importing {}", name);
        for option in OPTIONS.into_iter() {
            match option {
                "pic" => {
                    let Some(value) = &contact[option].as_str() else {
                        println!( "error to get pic value");
                        continue
                    };

                    if let Err(_) = queries::insert_pic(conn, &name, value).await {
                        println!(
                            "error to import pic of contact {}",
                            &contact["name"].as_str().unwrap(),
                        );
                    };
                }

                "phones" => {
                    let Ok(value) = util::get_option_value(&contact,option) else {
                        continue
                    };

                    if let Err(_) = queries::insert_phones(conn, &name, &value).await {
                        println!("failed to import phones of contact {}", &name);
                    };
                }

                "emails" => {
                    let Ok(value) = util::get_option_value(&contact,option) else {
                        continue
                    };

                    if let Err(_) = queries::insert_emails(conn, &name, &value).await {
                        println!("failed to import emails of contact {}", &name);
                    };
                }

                "social_nets" => {
                    let Ok(value) = util::get_option_value(&contact,option) else {
                        continue
                    };

                    if let Err(_) = queries::insert_social_nets(conn, &name, &value).await {
                        println!("failed to import social nets of contact {}", &name);
                    };
                }

                "wallets" => {
                    let Ok(value) = util::get_option_value(&contact,option) else {
                        continue
                    };

                    if let Err(_) = queries::insert_wallets(conn, &name, &value).await {
                        println!("failed to import wallets of contact {}", &name);
                    };
                }

                _ => continue,
            }
        }
    }
}
