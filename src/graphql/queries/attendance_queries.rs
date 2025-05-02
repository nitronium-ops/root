use std::sync::Arc;

use crate::models::attendance::{AbsentsWithMember, Attendance, AttendanceWithMember};
use async_graphql::{Context, Object, Result};
use chrono::Datelike;
use chrono::NaiveDate;
use sqlx::PgPool;

#[derive(Default)]
pub struct AttendanceQueries;

#[Object]
impl AttendanceQueries {
    async fn attendance(&self, ctx: &Context<'_>, member_id: i32) -> Result<Vec<Attendance>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        Ok(
            sqlx::query_as::<_, Attendance>("SELECT * FROM Attendance WHERE member_id = $1")
                .bind(member_id)
                .fetch_all(pool.as_ref())
                .await?,
        )
    }

    async fn attendance_by_date(
        &self,
        ctx: &Context<'_>,
        date: NaiveDate,
    ) -> Result<Vec<AttendanceWithMember>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let records = sqlx::query_as::<_, AttendanceWithMember>(
            "SELECT att.attendance_id, att.member_id, att.date, att.is_present,
                    att.time_in, att.time_out, mem.name, mem.year, mem.group_id
             FROM Attendance att
             JOIN Member mem ON att.member_id = mem.member_id
             WHERE att.date = $1",
        )
        .bind(date)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(records)
    }

    async fn absents_by_month(
        &self,
        ctx: &Context<'_>,
        date: NaiveDate,
    ) -> Result<Vec<AbsentsWithMember>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let end_date = date
            .with_day(1)
            .unwrap()
            .with_month(date.month() + 1)
            .unwrap();

        let records = sqlx::query_as::<_, AbsentsWithMember>(
            "SELECT mem.name, mem.year, att.member_id,
                COUNT(*) AS absent_days
            FROM attendance att
            JOIN member mem ON att.member_id = mem.member_id
            WHERE
                att.is_present = FALSE
                AND att.date >= $1
                AND att.date < $2
            GROUP BY
                att.member_id, mem.name, mem.year
            ORDER BY
                absent_days DESC, mem.name
            ",
        )
        .bind(date)
        .bind(end_date)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(records)
    }
}
