#  Background Tasks

## Quick Navigation

- [overview](1-overview.md)
- [system-architecture](2-system-architecture.md)
  - [application-configuration](2.1-application-configuration.md)
- [database-models](3-database-models.md)
  - [member-model](3.1-member-model.md)
  - [attendance-model](3.2-attendance-model.md)
  - [streak-model](3.3-streak-model.md)
  - [project-model](3.4-project-model.md)
- [graphql-api](4-graphql-api.md)
  - [graphql-queries](4.1-graphql-queries.md)
    - [member-queries](4.1.1-member-queries.md)
    - [attendance-queries](4.1.2-attendance-queries.md)
    - [streak-queries](4.1.3-streak-queries.md)
    - [project-queries](4.1.4-project-queries.md)
  - [graphql-mutations](4.2-graphql-mutations.md)
    - [member-mutations](4.2.1-member-mutations.md)
    - [attendance-mutations](4.2.2-attendance-mutations.md)
    - [streak-mutations](4.2.3-streak-mutations.md)
    - [project-mutations](4.2.4-project-mutations.md)
- [background-tasks](5-background-tasks.md)
  - [daily-attendance-task](5.1-daily-attendance-task.md)
- [deployment-and-cicd](6-deployment-and-cicd.md)
  - [github-actions-workflows](6.1-github-actions-workflows.md)
  - [docker-deployment](6.2-docker-deployment.md)
- [security-features](7-security-features.md)
  - [hmac-authentication](7.1-hmac-authentication.md)

## Table of Contents

- [Background Tasks](#background-tasks)
  - [Introduction](#introduction)
  - [Background Task Architecture](#background-task-architecture)
  - [Task Scheduling](#task-scheduling)
  - [Daily Attendance Processing](#daily-attendance-processing)
    - [Task Components](#task-components)
    - [Attendance Record Creation](#attendance-record-creation)
    - [Attendance Summary Updates](#attendance-summary-updates)
  - [Database Interaction Model](#database-interaction-model)
  - [Error Handling and Logging](#error-handling-and-logging)
  - [Integration with System Architecture](#integration-with-system-architecture)
  - [Performance Considerations](#performance-considerations)

# Background Tasks

Relevant source files

* [src/daily\_task/mod.rs](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs)

## Introduction

The Root system includes automated background tasks that run independently of client requests to perform critical maintenance and data processing operations. These tasks are essential for maintaining data integrity, generating scheduled records, and ensuring the system remains up to date without manual intervention.

For information about specific attendance functionality, see [Daily Attendance Task](/amfoss/root/5.1-daily-attendance-task).

Sources: [src/daily\_task/mod.rs37-40](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L37-L40)

## Background Task Architecture

The background task system in Root is designed to execute scheduled operations at specific intervals. The current implementation focuses on daily tasks that run at midnight (Kolkata time). These tasks are launched during application startup and run in separate asynchronous processes to avoid interfering with normal API operations.

Sources: [src/daily\_task/mod.rs10-35](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L10-L35)

## Task Scheduling

Background tasks in Root use Tokio's time utilities to schedule execution. The daily task is precisely scheduled to run at 00:30:00 Kolkata time (Asia/Kolkata timezone). The system calculates the duration until the next execution time and sleeps until then.

Sources: [src/daily\_task/mod.rs10-35](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L10-L35)

## Daily Attendance Processing

The primary background task in Root is the daily attendance processor. This task performs several critical functions:

1. Creates new attendance records for all members for the current day
2. Updates attendance summaries based on the previous day's attendance
3. Manages monthly attendance statistics

### Task Components

The daily attendance task is structured as a hierarchy of focused functions:

| Function | Purpose | Description |
| --- | --- | --- |
| `run_daily_task_at_midnight` | Task Scheduler | Manages timing and scheduling of execution |
| `execute_daily_task` | Coordination | Coordinates the overall task execution |
| `update_attendance` | Record Creation | Creates new daily attendance records |
| `update_attendance_summary` | Summary Processing | Processes attendance summary data |
| `update_days_attended` | Statistics Maintenance | Updates monthly attendance statistics |

Sources: [src/daily\_task/mod.rs10-202](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L10-L202)

### Attendance Record Creation

Every day at the scheduled time, the system creates default attendance records for all members. These records are initialized with `is_present = false` and empty time\_in/time\_out values. External systems like [Presense](https://github.com/amfoss/root/blob/2b58803d/Presense) will later update these records based on actual attendance data.

Sources: [src/daily\_task/mod.rs53-93](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L53-L93)

### Attendance Summary Updates

The system maintains monthly attendance summaries for each member, tracking how many days they were present in a given month. The summary update process:

1. Checks if the member was present on the previous day
2. If they were present, updates the current month's attendance count
3. If no summary exists for the current month, creates a new one

Sources: [src/daily\_task/mod.rs95-202](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L95-L202)

## Database Interaction Model

Background tasks interact extensively with the database, particularly with the `Attendance` and `AttendanceSummary` tables.

Key database operations:

1. **Member Retrieval**: Fetches all members to process their attendance
2. **Attendance Insertion**: Creates default attendance records with ON CONFLICT handling
3. **Attendance Summary**: Updates monthly attendance statistics based on previous day's attendance

Sources: [src/daily\_task/mod.rs42-44](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L42-L44) [src/daily\_task/mod.rs62-73](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L62-L73) [src/daily\_task/mod.rs154-167](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L154-L167) [src/daily\_task/mod.rs177-188](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L177-L188)

## Error Handling and Logging

The background task system incorporates robust error handling and logging to ensure operational visibility and reliability:

1. **Debug Logging**: Records operational details for debugging purposes
2. **Info Logging**: Records significant events like task scheduling
3. **Error Logging**: Records failures in database operations or task execution
4. **Error Recovery**: Task failures are contained to prevent cascade failures

Errors are handled locally where possible to prevent a failure in one member's processing from affecting others.

Sources: [src/daily\_task/mod.rs25-28](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L25-L28) [src/daily\_task/mod.rs49](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L49-L49) [src/daily\_task/mod.rs82-87](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L82-L87) [src/daily\_task/mod.rs126-128](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L126-L128) [src/daily\_task/mod.rs195-200](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L195-L200)

## Integration with System Architecture

Background tasks form a crucial component in the overall Root system architecture, working alongside the GraphQL API to maintain system data.

The background tasks operate independently of client requests, which allows the system to:

1. Maintain data consistency even during periods of low API activity
2. Ensure critical operations happen at precise times regardless of client behavior
3. Reduce load on the API by offloading routine maintenance operations

Sources: [src/daily\_task/mod.rs10-35](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L10-L35) [src/daily\_task/mod.rs37-40](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L37-L40)

## Performance Considerations

The current background task implementation has several performance characteristics worth noting:

1. Tasks run in a single thread and process members sequentially
2. Database operations use prepared statements for efficiency
3. The ON CONFLICT clause prevents duplicate attendance records
4. Error handling is designed to ensure task completion even if individual operations fail

For large member bases, the sequential processing approach may become a performance bottleneck. Future enhancements could include:

* Parallel processing of members
* Batched database operations
* More granular error recovery

Sources: [src/daily\_task/mod.rs61-73](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L61-L73) [src/daily\_task/mod.rs42-50](https://github.com/amfoss/root/blob/2b58803d/src/daily_task/mod.rs#L42-L50)