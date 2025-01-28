use async_graphql::{ComplexObject, Context, Object, Result};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::{
    attendance::{AttendanceInfo, AttendanceSummaryInfo},
    member::Member,
    status_update_streak::StatusUpdateStreakInfo,
};

/// Sub-query for the [`Member`] table. The queries are:
/// * members - get list of all members and their details, optionally filtered by year and id
/// * member - get a specific member's details using their id, roll_no or discord_id
#[derive(Default)]
pub struct MemberQueries;

#[Object]
impl MemberQueries {
    pub async fn member(
        &self,
        ctx: &Context<'_>,
        member_id: Option<i32>,
        roll_no: Option<String>,
        discord_id: Option<String>,
    ) -> Result<Member> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        // Base SQL query
        let mut sql = String::from("SELECT * FROM Member WHERE ");

        // Push filter
        let query = if let Some(id) = member_id {
            sql.push_str("member_id = $1");
            sqlx::query_as::<_, Member>(&sql).bind(id)
        } else if let Some(roll) = roll_no {
            sql.push_str("roll_no = $1");
            sqlx::query_as::<_, Member>(&sql).bind(roll)
        } else if let Some(discord) = discord_id {
            sql.push_str("discord_id = $1");
            sqlx::query_as::<_, Member>(&sql).bind(discord)
        } else {
            return Err(async_graphql::Error::new(
                "At least one key must be specified.",
            ));
        };

        let result = query.fetch_one(pool.as_ref()).await?;
        Ok(result)
    }

    pub async fn members(
        &self,
        ctx: &Context<'_>,
        year: Option<i32>,
        group_id: Option<i32>,
    ) -> Result<Vec<Member>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        // Base standalone query
        let mut query = sqlx::QueryBuilder::new("SELECT * FROM Member WHERE 1=1");

        // Push filters if necessary
        if let Some(y) = year {
            query.push(" AND year = ");
            query.push_bind(y);
        }

        if let Some(g) = group_id {
            query.push(" AND group_id = ");
            query.push_bind(g);
        }

        let members = query
            .build_query_as::<Member>()
            .fetch_all(pool.as_ref())
            .await?;

        Ok(members)
    }
}

/// These are resolvers to handle nested queries i.e if the end-user wants details of members held in other tables. Currently you can query the following for each Member:
/// * date, is_present, time_in and time_out fields of [`Attendance`]
/// * year, month and days_attendned fields of [`AttendanceSummary`]
/// * current_streak and max_streak fields of [`StatusUpdateStreak`]
/// Resolvers can be easily changed to show more fields by editing the SQLx query in their respective methods.
#[ComplexObject]
impl Member {
    async fn attendance(&self, ctx: &Context<'_>) -> Vec<AttendanceInfo> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");
        let query = sqlx::query_as::<_, AttendanceInfo>(
            "SELECT date, is_present, time_in, time_out FROM Attendance WHERE member_id = $1",
        )
        .bind(self.member_id)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();

        query
    }

    #[graphql(name = "attendanceSummary")]
    async fn attendance_summary(&self, ctx: &Context<'_>) -> Vec<AttendanceSummaryInfo> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");
        let query = sqlx::query_as::<_, AttendanceSummaryInfo>(
            "SELECT year, month, days_attended FROM AttendanceSummary WHERE member_id = $1",
        )
        .bind(self.member_id)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();

        query
    }

    async fn streak(&self, ctx: &Context<'_>) -> Vec<StatusUpdateStreakInfo> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");
        let query = sqlx::query_as::<_, StatusUpdateStreakInfo>(
            "SELECT current_streak, max_streak FROM StatusUpdateStreak WHERE member_id = $1",
        )
        .bind(self.member_id)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();

        query
    }
}
