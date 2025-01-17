use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use chrono::Local;
use chrono_tz::Asia::Kolkata;
use sqlx::PgPool;

use crate::models::attendance::{Attendance, MarkAttendanceInput};

#[derive(Default)]
pub struct AttendanceMutations;

#[Object]
impl AttendanceMutations {
    #[graphql(name = "markAttendance")]
    async fn mark_attendance(
        &self,
        ctx: &Context<'_>,
        input: MarkAttendanceInput,
    ) -> Result<Attendance> {
        let pool = ctx
            .data::<Arc<PgPool>>()
            .expect("Pool not found in context");

        let now = Local::now().with_timezone(&Kolkata).date_naive();
        let attendance = sqlx::query_as::<_, Attendance>(
            "INSERT INTO Attendance (member_id, date, is_present, time_in, time_out, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $6) RETURNING *",
        )
        .bind(input.member_id)
        .bind(input.date)
        .bind(true)
        .bind(input.time_in)
        .bind(input.time_out)
        .bind(now)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(attendance)
    }
}
