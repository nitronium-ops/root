use chrono::{Local, NaiveTime};
use chrono_tz::Asia::Kolkata;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::models::member::Member;

// TODO: Abstract this down to functions for each task.
// We need to add a record for every member because otherwise Presense will only add present members to the DB, and we will have to JOIN Members and Attendance records for the day to get the absent members. In exchange for increased storage use, we get simpler queries for Home which needs the data for every member for every day so far. But as of Jan 2025, there are less than 50 members in the club and thus shouldn't really create that many rows over time.
/// This function does a number of things, including:
/// * Insert new attendance records everyday for [`presense`](https://www.github.com/amfoss/presense) to update them later in the day.
/// * Fetch stats from both codeforces and leetcode
/// * Use those fetched stats to update the leaderboard
/// * Update attendance streaks for some reason?
pub async fn execute_daily_task(pool: Arc<PgPool>) {
    add_daily_attendance_records(&*pool).await;

    //fetching the username from tables
    //              let leetcode_username = sqlx::query_as::<_, LeetCodeStats>(
    //                  "SELECT * FROM leetcode_stats WHERE member_id = $1",
    //              )
    //              .bind(member.id)
    //              .fetch_optional(pool.as_ref())
    //              .await;

    //              if let Ok(Some(leetcode_stats)) = leetcode_username {
    //                  let username = leetcode_stats.leetcode_username.clone();

    //                  // Fetch and update LeetCode stats
    //                  match fetch_leetcode_stats(pool.clone(), member.id, &username).await {
    //                      Ok(_) => println!("LeetCode stats updated for member ID: {}", member.id),
    //                      Err(e) => eprintln!(
    //                          "Failed to update LeetCode stats for member ID {}: {:?}",
    //                          member.id, e
    //                      ),
    //                  }
    //              }

    //              // Fetch Codeforces username
    //              let codeforces_username = sqlx::query_as::<_, CodeforcesStats>(
    //                  "SELECT * FROM codeforces_stats WHERE member_id = $1",
    //              )
    //              .bind(member.id)
    //              .fetch_optional(pool.as_ref())
    //              .await;

    //              if let Ok(Some(codeforces_stats)) = codeforces_username {
    //                  let username = codeforces_stats.codeforces_handle.clone();

    //                  // Fetch and update Codeforces stats
    //                  match fetch_codeforces_stats(pool.clone(), member.id, &username).await {
    //                      Ok(_) => println!("Codeforces stats updated for member ID: {}", member.id),
    //                      Err(e) => eprintln!(
    //                          "Failed to update Codeforces stats for member ID {}: {:?}",
    //                          member.id, e
    //                      ),
    //                  }
    //              }

    //              match update_leaderboard(pool.clone()).await {
    //                  Ok(_) => println!("Leaderboard updated."),
    //                  Err(e) => eprintln!("Failed to update leaderboard: {:?}", e),
    //              }

    //              // Update attendance streak
    //              update_attendance_streak(member.id, pool.as_ref()).await;
}

// We need to add a record for every member because otherwise [`Presense`](https://www.github.com/presense) will only add present members to the DB, and we will have to JOIN Members and Attendance records for the day to get the absent members. In exchange for increased storage use, we get simpler queries for Home which needs the data for every member for every day so far. But as of Jan 2025, there are less than 50 members in the club and thus storage really shouldn't be an issue.
/// Inserts new attendance records everyday for [`presense`](https://www.github.com/amfoss/presense) to update them later in the day.
async fn add_daily_attendance_records(pool: &PgPool) {
    info!("Adding daily attendance records...");

    let members: Result<Vec<Member>, sqlx::Error> =
        sqlx::query_as::<_, Member>("SELECT * FROM Member")
            .fetch_all(pool)
            .await;

    match members {
        Ok(members) => {
            let today = Local::now().with_timezone(&Kolkata).date_naive();

            for member in members {
                let attendance = sqlx::query(
                    "INSERT INTO Attendance (member_id, date, is_present, time_in, time_out) 
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (member_id, date) DO NOTHING",
                )
                .bind(member.member_id)
                .bind(today)
                .bind(false) // Default `is_present` is False
                .bind(None::<NaiveTime>) // Default `time_in` is NULL
                .bind(None::<NaiveTime>) // Default `time_out` is NULL
                .execute(pool)
                .await;

                match attendance {
                    Ok(_) => {
                        debug!(
                            "Attendance record added for member ID: {}",
                            member.member_id
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to insert attendance for member ID: {}: {:?}",
                            member.member_id, e
                        );
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch members: {:?}", e);
        }
    }
}

// Function to update attendance streak
// async fn update_attendance_streak(member_id: i32, pool: &sqlx::PgPool) {
//     let today = chrono::Local::now()
//         .with_timezone(&chrono_tz::Asia::Kolkata)
//         .naive_local();
//     let yesterday = today
//         .checked_sub_signed(chrono::Duration::hours(12))
//         .unwrap()
//         .date();
//
//     if today.day() == 1 {
//         let _ = sqlx::query(
//             r#"
//                 INSERT INTO AttendanceStreak (member_id, month, streak)
//                 VALUES ($1, date_trunc('month', $2::date AT TIME ZONE 'Asia/Kolkata'), 0)
//             "#,
//         )
//         .bind(member_id)
//         .bind(today)
//         .execute(pool)
//         .await;
//         println!("Attendance streak created for member ID: {}", member_id);
//     }
//
//     let present_attendance = sqlx::query_scalar::<_, i64>(
//         r#"
//             SELECT COUNT(*)
//             FROM Attendance
//             WHERE id = $1
//             AND is_present = true
//             AND date = $2
//         "#,
//     )
//     .bind(member_id)
//     .bind(yesterday)
//     .fetch_one(pool)
//     .await;
//
//     match present_attendance {
//         Ok(1) => {
//             let existing_streak = sqlx::query_scalar::<_, i32>(
//                 r#"
//                     SELECT streak
//                     FROM AttendanceStreak
//                     WHERE member_id = $1
//                     AND month = date_trunc('month', $2::date AT TIME ZONE 'Asia/Kolkata')
//                 "#,
//             )
//             .bind(member_id)
//             .bind(today)
//             .fetch_optional(pool)
//             .await;
//
//             match existing_streak {
//                 Ok(Some(streak)) => {
//                     let _ = sqlx::query(
//                         r#"
//                             UPDATE AttendanceStreak
//                             SET streak = $1
//                             WHERE member_id = $2
//                             AND month = date_trunc('month', $3::date AT TIME ZONE 'Asia/Kolkata')
//                         "#,
//                     )
//                     .bind(streak + 1)
//                     .bind(member_id)
//                     .bind(today)
//                     .execute(pool)
//                     .await;
//                 }
//                 Ok(None) => {
//                     println!("No streak found for member ID: {}", member_id);
//                 }
//                 Err(e) => eprintln!("Error checking streak for member ID {}: {:?}", member_id, e),
//             }
//         }
//         Ok(0) => {
//             println!("Sreak not incremented for member ID: {}", member_id);
//         }
//         Ok(_) => eprintln!("Unexpected attendance value for member ID: {}", member_id),
//         Err(e) => eprintln!(
//             "Error checking attendance for member ID {}: {:?}",
//             member_id, e
//         ),
//    }
//}
