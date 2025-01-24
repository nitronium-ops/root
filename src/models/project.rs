use async_graphql::{InputObject, SimpleObject};
use sqlx::FromRow;

#[derive(FromRow, SimpleObject)]
pub struct Project {
    pub project_id: i32,
    pub member_id: i32,
    pub title: Option<String>,
}

#[derive(InputObject)]
pub struct SetProjectInput {
    pub member_id: i32,
    pub title: String,
}
