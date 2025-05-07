use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKey {
    pub key_id: i32,
    pub member_id: i32,
    pub key_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
