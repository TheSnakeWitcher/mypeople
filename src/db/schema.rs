use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub pic: String,
    pub groups: Vec<String>,
    pub phones: HashMap<String, String>,
    pub emails: HashMap<String, String>,
    pub social_nets: HashMap<String, String>,
    pub wallets: HashMap<String, String>,
    pub locations: HashMap<String, String>,
    pub events: HashMap<String, String>,
    pub notes: String,
}

impl FromRow<'_, SqliteRow> for Contact {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.get("id"),
            name: row.get("name"),
            pic: row.try_get("pic").unwrap_or_default(),
            groups: serde_json::from_str(row.try_get("groups").unwrap_or_default()).unwrap(),
            emails: serde_json::from_str(row.try_get("emails").unwrap_or_default()).unwrap(),
            phones: serde_json::from_str(row.try_get("phones").unwrap_or_default()).unwrap(),
            social_nets: serde_json::from_str(row.try_get("social_nets").unwrap_or_default())
                .unwrap(),
            wallets: serde_json::from_str(row.try_get("wallets").unwrap_or_default()).unwrap(),
            locations: serde_json::from_str(row.try_get("locations").unwrap_or_default()).unwrap(),
            events: serde_json::from_str(row.try_get("events").unwrap_or_default()).unwrap(),
            notes: row.try_get("notes").unwrap_or_default(),
        })
    }
}

pub struct Event {
    pub name: String,
    pub date: String,
    pub requirements: String,
    pub related_people: String,
    pub notes: String,
}

pub struct Location {
    pub name: String,
    pub pic: String,
    pub address: String,
    pub coordinates: String,
    pub notes: String,
}

pub struct Product {
    pub name: String,
    pub price: f64,
    pub qty: u64,
    pub pic: String,
    pub details: String,
    pub notes: String,
}
