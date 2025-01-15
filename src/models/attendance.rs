use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(FromRow, SimpleObject, Debug)]
pub struct Attendance {
    pub attendance_id: i32,
    pub member_id: i32,
    pub date: chrono::NaiveDate,
    pub is_present: bool,
    pub time_in: Option<chrono::NaiveTime>,
    pub time_out: Option<chrono::NaiveTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(FromRow, SimpleObject, Debug)]
pub struct AttendanceSummary {
    pub member_id: i32,
    pub year: i32,
    pub month: i32,
    pub days_attended: i32,
}
