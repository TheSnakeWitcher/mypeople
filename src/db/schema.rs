#[derive(Debug,sqlx::FromRow)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub pic: String,
    pub groups: String,
    pub phones: String,
    pub emails: String,
    pub social_nets: String,
    pub wallets: String,
    pub locations: String,
    pub events: String,
    pub notes: String,
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
