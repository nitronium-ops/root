use sqlx::FromRow;
use async_graphql::SimpleObject;

#[derive(FromRow, SimpleObject)]
pub struct Attendance {
    pub id: i32,
    pub date: String,
    pub timein: String,
    pub timeout: String,
}
