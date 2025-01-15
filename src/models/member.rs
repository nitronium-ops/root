use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(FromRow, SimpleObject)]
pub struct Member {
    pub member_id: i32,
    pub roll_no: String,
    pub name: String,
    pub email: String,
    pub sex: String,
    pub year: i32,
    pub hostel: String,
    pub mac_address: String,
    pub discord_id: String,
    pub group_id: i32,
    pub created_at: chrono::NaiveDateTime,
}
