use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Default)]
pub struct AuthQueries;

#[Object]
impl AuthQueries {
    async fn has_api_key(&self, ctx: &Context<'_>, member_id: i32) -> Result<Option<bool>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        Ok(sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM "apikey" WHERE member_id = $1)"#,
            member_id
        )
        .fetch_one(pool.as_ref())
        .await?)
    }
}
