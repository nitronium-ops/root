# Database Models

## Quick Navigation

- [Overview](1-overview.md)
- [System Architecture](2-system-architecture.md)
  - [Application Configuration](2.1-application-configuration.md)
- [Database Models](3-database-models.md)
  - [Member Model](3.1-member-model.md)
  - [Attendance Model](3.2-attendance-model.md)
  - [Streak Model](3.3-streak-model.md)
  - [Project Model](3.4-project-model.md)
- [GraphQL API](4-graphql-api.md)
  - [GraphQL Queries](4.1-graphql-queries.md)
    - [Member Queries](4.1.1-member-queries.md)
    - [Attendance Queries](4.1.2-attendance-queries.md)
    - [Streak Queries](4.1.3-streak-queries.md)
    - [Project Queries](4.1.4-project-queries.md)
  - [GraphQL Mutations](4.2-graphql-mutations.md)
    - [Member Mutations](4.2.1-member-mutations.md)
    - [Attendance Mutations](4.2.2-attendance-mutations.md)
    - [Streak Mutations](4.2.3-streak-mutations.md)
    - [Project Mutations](4.2.4-project-mutations.md)
- [Background Tasks](5-background-tasks.md)
  - [Daily Attendance Task](5.1-daily-attendance-task.md)
- [Deployment and CI/CD](6-deployment-and-cicd.md)
  - [GitHub Actions Workflows](6.1-github-actions-workflows.md)
  - [Docker Deployment](6.2-docker-deployment.md)
- [Security Features](7-security-features.md)
  - [HMAC Authentication](7.1-hmac-authentication.md)

## Table of Contents

- [Database Models](#database-models)
  - [Model Implementation Details](#model-implementation-details)
  - [Member Model](#member-model)
    - [Member Input](#member-input)
  - [Attendance Models](#attendance-models)
    - [Attendance Record](#attendance-record)
    - [Attendance Summary](#attendance-summary)
    - [Supplementary Attendance Models](#supplementary-attendance-models)
  - [Status Update Streak Model](#status-update-streak-model)
  - [Project Model](#project-model)
  - [Model Usage in the Application](#model-usage-in-the-application)

# Database Models

Relevant source files

* [migrations/20250124095543\_create\_project\_table.sql](https://github.com/amfoss/root/blob/2b58803d/migrations/20250124095543_create_project_table.sql)
* [src/graphql/mutations/mod.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/mod.rs)
* [src/graphql/queries/mod.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/queries/mod.rs)
* [src/models/attendance.rs](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs)
* [src/models/member.rs](https://github.com/amfoss/root/blob/2b58803d/src/models/member.rs)
* [src/models/project.rs](https://github.com/amfoss/root/blob/2b58803d/src/models/project.rs)
* [src/models/status\_update\_streak.rs](https://github.com/amfoss/root/blob/2b58803d/src/models/status_update_streak.rs)

This page provides a comprehensive overview of the database models used in the Root system. These models serve as the foundation for storing and managing data related to club members, their attendance records, status update streaks, and projects. For information about how these models are accessed and modified through the GraphQL API, see [GraphQL API](/amfoss/root/4-graphql-api).



## Model Implementation Details

The database models are implemented using Rust structs that are annotated for GraphQL and SQL compatibility. Each model implements traits such as `SimpleObject` for GraphQL schema generation and `FromRow` for SQL query result mapping.

Sources: [src/models/member.rs13-28](https://github.com/amfoss/root/blob/2b58803d/src/models/member.rs#L13-L28) [src/models/attendance.rs5-17](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L5-L17) [src/models/attendance.rs19-25](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L19-L25) [src/models/status\_update\_streak.rs4-9](https://github.com/amfoss/root/blob/2b58803d/src/models/status_update_streak.rs#L4-L9) [src/models/project.rs4-9](https://github.com/amfoss/root/blob/2b58803d/src/models/project.rs#L4-L9)

## Member Model

The `Member` struct represents club members and serves as the central entity in the system.

| Field | Type | Description | Constraints |
| --- | --- | --- | --- |
| `member_id` | i32 | Unique identifier for the member | Primary Key |
| `roll_no` | String | Member's roll number | Unique |
| `name` | String | Member's name |  |
| `email` | String | Member's email address | Unique |
| `sex` | Sex | Member's gender (M, F, or Other) | Enum |
| `year` | i32 | Academic year |  |
| `hostel` | String | Member's hostel name |  |
| `mac_address` | String | MAC address for attendance tracking | Unique |
| `discord_id` | String | Discord identifier | Unique |
| `group_id` | i32 | Group identifier |  |
| `created_at` | NaiveDateTime | Timestamp when record was created | Not exposed in GraphQL |

The Member model includes a custom enum `Sex` with values `M`, `F`, and `Other`, which is mapped to a PostgreSQL enum type `sex_type`.

Sources: [src/models/member.rs5-11](https://github.com/amfoss/root/blob/2b58803d/src/models/member.rs#L5-L11) [src/models/member.rs13-28](https://github.com/amfoss/root/blob/2b58803d/src/models/member.rs#L13-L28)

### Member Input

For creating new members, the system uses a separate `CreateMemberInput` struct:

| Field | Type | Description |
| --- | --- | --- |
| `roll_no` | String | Member's roll number |
| `name` | String | Member's name |
| `email` | String | Member's email address |
| `sex` | Sex | Member's gender |
| `year` | i32 | Academic year |
| `hostel` | String | Member's hostel name |
| `mac_address` | String | MAC address for attendance tracking |
| `discord_id` | String | Discord identifier |
| `group_id` | i32 | Group identifier |

Sources: [src/models/member.rs30-41](https://github.com/amfoss/root/blob/2b58803d/src/models/member.rs#L30-L41)

## Attendance Models

The attendance system uses multiple models to track daily attendance and monthly summaries.

### Attendance Record

The `Attendance` struct stores individual daily attendance records for members:

| Field | Type | Description | Constraints |
| --- | --- | --- | --- |
| `attendance_id` | i32 | Unique identifier for the attendance record | Primary Key |
| `member_id` | i32 | Reference to the member | Foreign Key |
| `date` | NaiveDate | Date of the attendance record |  |
| `is_present` | bool | Whether the member was present |  |
| `time_in` | Option | Time when member checked in | Optional |
| `time_out` | Option | Time when member checked out | Optional |
| `created_at` | NaiveDateTime | Timestamp when record was created | Not exposed in GraphQL |
| `updated_at` | NaiveDateTime | Timestamp when record was last updated | Not exposed in GraphQL |

Sources: [src/models/attendance.rs5-17](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L5-L17)

### Attendance Summary

The `AttendanceSummary` struct aggregates monthly attendance statistics for members:

| Field | Type | Description | Constraints |
| --- | --- | --- | --- |
| `member_id` | i32 | Reference to the member | Primary Key, Foreign Key |
| `year` | i32 | Year of the summary | Primary Key |
| `month` | i32 | Month of the summary (1-12) | Primary Key |
| `days_attended` | i32 | Number of days attended in the month |  |

Sources: [src/models/attendance.rs19-25](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L19-L25)

### Supplementary Attendance Models

The system also includes several auxiliary attendance-related models:

1. `AttendanceInfo`: A simplified view of attendance data

   * Fields: `date`, `is_present`, `time_in`, `time_out`
2. `AttendanceSummaryInfo`: A simplified view of monthly attendance summaries

   * Fields: `year`, `month`, `days_attended`
3. `AttendanceWithMember`: Combines attendance and member data for queries

   * Includes both attendance fields and key member fields like `name` and `year`
4. `MarkAttendanceInput`: Input type for marking attendance

   * Fields: `member_id`, `date`, `hmac_signature` (for verification)

Sources: [src/models/attendance.rs27-33](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L27-L33) [src/models/attendance.rs35-40](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L35-L40) [src/models/attendance.rs49-59](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L49-L59) [src/models/attendance.rs42-47](https://github.com/amfoss/root/blob/2b58803d/src/models/attendance.rs#L42-L47)

## Status Update Streak Model

The `StatusUpdateStreak` struct tracks member activity streaks:

| Field | Type | Description | Constraints |
| --- | --- | --- | --- |
| `member_id` | i32 | Reference to the member | Primary Key, Foreign Key |
| `current_streak` | i32 | Current consecutive days of status updates |  |
| `max_streak` | i32 | Maximum streak achieved |  |

The system also includes:

1. `StatusUpdateStreakInfo`: A view model that excludes the `member_id` field

   * Fields: `current_streak`, `max_streak`
2. `StreakInput`: Input type for streak-related operations

   * Field: `member_id`

Sources: [src/models/status\_update\_streak.rs4-9](https://github.com/amfoss/root/blob/2b58803d/src/models/status_update_streak.rs#L4-L9) [src/models/status\_update\_streak.rs11-15](https://github.com/amfoss/root/blob/2b58803d/src/models/status_update_streak.rs#L11-L15) [src/models/status\_update\_streak.rs17-20](https://github.com/amfoss/root/blob/2b58803d/src/models/status_update_streak.rs#L17-L20)

## Project Model

The `Project` struct represents projects associated with members:

| Field | Type | Description | Constraints |
| --- | --- | --- | --- |
| `project_id` | i32 | Unique identifier for the project | Primary Key |
| `member_id` | i32 | Reference to the member | Foreign Key |
| `title` | Option | Project title | Optional |

The SQL schema for the Project table includes a foreign key constraint that ensures referential integrity with the Member table, including a cascade delete:

For creating or updating projects, the system uses a `SetProjectInput` struct:

| Field | Type | Description |
| --- | --- | --- |
| `member_id` | i32 | Reference to the member |
| `title` | String | Project title |

Sources: [src/models/project.rs4-9](https://github.com/amfoss/root/blob/2b58803d/src/models/project.rs#L4-L9) [src/models/project.rs11-15](https://github.com/amfoss/root/blob/2b58803d/src/models/project.rs#L11-L15) [migrations/20250124095543\_create\_project\_table.sql1-6](https://github.com/amfoss/root/blob/2b58803d/migrations/20250124095543_create_project_table.sql#L1-L6)


## Model Usage in the Application

The database models serve as the foundation for the GraphQL API, which is divided into queries and mutations for each domain area. These models are exposed through the GraphQL API, with certain fields (like timestamps) being hidden from the API for security and simplicity.

For more information about how these models are used in GraphQL queries and mutations, see the [GraphQL API](/amfoss/root/4-graphql-api) documentation.