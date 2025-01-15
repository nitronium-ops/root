use async_graphql::{Context, Object};
use chrono::NaiveDate;
use root::models::{
    attendance::Attendance,
    member::Member,
};
use sqlx::PgPool;
use std::sync::Arc;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_member(&self, ctx: &Context<'_>) -> Result<Vec<Member>, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");
        let users = sqlx::query_as::<_, Member>("SELECT * FROM Member")
            .fetch_all(pool.as_ref())
            .await?;
        Ok(users)
    }

    //Query for retrieving the attendance based on date
    async fn get_attendance(
        &self,
        ctx: &Context<'_>,
        date: NaiveDate,
    ) -> Result<Vec<Attendance>, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");

        let attendance_list = sqlx::query_as::<_, Attendance>(
            "SELECT id, date, timein, timeout, is_present 
            FROM Attendance WHERE date = $1",
        )
        .bind(date)
        .fetch_all(pool.as_ref())
        .await?;
        Ok(attendance_list)
    }
    async fn get_streak(&self, ctx: &Context<'_>, id: i32) -> Result<StreakUpdate, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");
        let streak = sqlx::query_as::<_, StreakUpdate>("SELECT * FROM StreakUpdate WHERE id = $1")
            .bind(id)
            .fetch_one(pool.as_ref())
            .await?;

        Ok(streak)
    }

    async fn get_update_streak(&self, ctx: &Context<'_>) -> Result<Vec<StreakUpdate>, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");
        let streak = sqlx::query_as::<_, StreakUpdate>("SELECT * FROM StreakUpdate")
            .fetch_all(pool.as_ref())
            .await?;

        Ok(streak)
    }

    async fn get_attendance_streak(
        &self,
        ctx: &Context<'_>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AttendanceStreak>, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");
        let attendance_streak = sqlx::query_as::<_, AttendanceStreak>(
            "SELECT * from AttendanceStreak 
            WHERE month >= $1 AND month < $2
            ",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(attendance_streak)
    }

    async fn get_attendance_summary(
        &self,
        ctx: &Context<'_>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<AttendanceSummary, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");
        let attendance_days = sqlx::query_as::<_, (NaiveDate, i64)>(
            "SELECT date, COUNT(*) FROM Attendance 
            WHERE date >= $1 AND date < $2 
            AND is_present = true 
            GROUP BY date ORDER BY date",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool.as_ref())
        .await?;

        let member_attendance = sqlx::query_as::<_, (i32, i64)>(
            "SELECT id, COUNT(*) FROM Attendance 
            WHERE date >= $1 AND date < $2 
            AND is_present = true 
            GROUP BY id",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool.as_ref())
        .await?;

        let max_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM Attendance 
            WHERE date >= $1 AND date < $2 
            AND is_present = true",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool.as_ref())
        .await?;

        let daily_count = attendance_days
            .into_iter()
            .map(|(date, count)| DailyCount { date, count })
            .collect();

        let member_attendance = member_attendance
            .into_iter()
            .map(|(id, present_days)| MemberAttendance { id, present_days })
            .collect();

        let summaries: AttendanceSummary = AttendanceSummary {
            max_days: max_count[0],
            member_attendance,
            daily_count,
        };

        Ok(summaries)
    }

    pub async fn get_non_working_days(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<NaiveDate>, sqlx::Error> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");

        let dates = sqlx::query_scalar::<_, NaiveDate>(
            "SELECT date
            FROM Attendance
            GROUP BY date
            HAVING BOOL_AND(NOT is_present)
            ORDER BY date",
        )
        .fetch_all(pool.as_ref())
        .await?;

        Ok(dates)
    }
}
