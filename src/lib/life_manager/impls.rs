use chrono::{Duration, Utc};

use super::LifeIndicator;

impl LifeIndicator {
    pub fn new(id: String, lifetime: u64) -> Self {
        let now = Utc::now();
        Self {
            id,
            death: now + Duration::milliseconds(lifetime as i64),
        }
    }
}
