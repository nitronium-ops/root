use sqlx::FromRow;
use async_graphql::SimpleObject;

#[derive(FromRow, SimpleObject)]
pub struct Member {
    pub id: i32,
    pub rollno: String,
    pub name: String,
    pub hostel: String,
    pub email: String,
    pub sex: String,
    pub year: i32,
}
