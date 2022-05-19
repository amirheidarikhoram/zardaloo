use std::hash::Hash;

use chrono::{DateTime, Utc};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LifeIndicator {
    pub id: String,
    pub death: DateTime<Utc>,
}