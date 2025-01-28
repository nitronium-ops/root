use std::sync::Arc;

use async_graphql::{Context, Error, Object, Result};
use sqlx::PgPool;

use crate::models::status_update_streak::{StatusUpdateStreak as Streak, StreakInput};

#[derive(Default)]
pub struct StreakMutations;

#[Object]
impl StreakMutations {
    #[graphql(name = "incrementStreak")]
    async fn increment_streak(&self, ctx: &Context<'_>, input: StreakInput) -> Result<Streak> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        // Ensure at least one identifier is provided
        if input.member_id.is_none() && input.discord_id.is_none() {
            return Err(Error::new(
                "Either `member_id` or `discord_id` must be provided.",
            ));
        }

        let mut sql = String::from("UPDATE Streaks SET current_streak = current_streak + 1, max_streak = GREATEST(max_streak, current_streak + 1) WHERE ");
        if let Some(_) = input.member_id {
            sql.push_str("member_id = $1");
        } else if let Some(_) = input.discord_id {
            sql.push_str("discord_id = $1");
        }

        sql.push_str(" RETURNING *");

        let query = if let Some(member_id) = input.member_id {
            sqlx::query_as::<_, Streak>(&sql).bind(member_id)
        } else if let Some(discord_id) = input.discord_id {
            sqlx::query_as::<_, Streak>(&sql).bind(discord_id)
        } else {
            return Err(Error::new("Invalid input."));
        };

        let updated_streak = query.fetch_one(pool.as_ref()).await?;

        Ok(updated_streak)
    }

    async fn reset_streak(&self, ctx: &Context<'_>, input: StreakInput) -> Result<Streak> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        // Ensure at least one identifier is provided
        if input.member_id.is_none() && input.discord_id.is_none() {
            return Err(Error::new(
                "Either `member_id` or `discord_id` must be provided.",
            ));
        }

        let mut sql = String::from("UPDATE Streaks SET current_streak = 0 WHERE ");

        if let Some(_) = input.member_id {
            sql.push_str("member_id = $1");
        } else if let Some(_) = input.discord_id {
            sql.push_str("discord_id = $1");
        }

        sql.push_str(" RETURNING *");

        let query = if let Some(member_id) = input.member_id {
            sqlx::query_as::<_, Streak>(&sql).bind(member_id)
        } else if let Some(discord_id) = input.discord_id {
            sqlx::query_as::<_, Streak>(&sql).bind(discord_id)
        } else {
            return Err(Error::new("Invalid input."));
        };

        let updated_streak = query.fetch_one(pool.as_ref()).await?;
        Ok(updated_streak)
    }
}
