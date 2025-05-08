#  Attendance System

## Quick Navigation

- [root-graphql-backend-overview](1-root-graphql-backend-overview.md)
  - [system-architecture](1.1-system-architecture.md)
- [project-setup](2-project-setup.md)
  - [environment-configuration](2.1-environment-configuration.md)
- [data-models](3-data-models.md)
  - [member-model](3.1-member-model.md)
  - [attendance-models](3.2-attendance-models.md)
  - [status-update-streak-model](3.3-status-update-streak-model.md)
  - [project-model](3.4-project-model.md)
- [graphql-api](4-graphql-api.md)
  - [member-queries-and-mutations](4.1-member-queries-and-mutations.md)
  - [attendance-queries-and-mutations](4.2-attendance-queries-and-mutations.md)
  - [streak-queries-and-mutations](4.3-streak-queries-and-mutations.md)
  - [project-queries-and-mutations](4.4-project-queries-and-mutations.md)
- [attendance-system](5-attendance-system.md)
  - [daily-attendance-task](5.1-daily-attendance-task.md)
  - [attendance-security](5.2-attendance-security.md)
- [cicd-and-deployment](6-cicd-and-deployment.md)
  - [docker-deployment](6.1-docker-deployment.md)
  - [code-quality-and-releases](6.2-code-quality-and-releases.md)
  - [documentation-generation](6.3-documentation-generation.md)

## Table of Contents

- [Attendance System](#attendance-system)
  - [System Overview](#system-overview)
  - [Core Data Models](#core-data-models)
    - [Attendance Model](#attendance-model)
    - [Attendance Summary Model](#attendance-summary-model)
  - [Database Schema](#database-schema)
  - [Automated Daily Task System](#automated-daily-task-system)
  - [Attendance Operations](#attendance-operations)
    - [Creating Attendance Records](#creating-attendance-records)
    - [Marking Attendance](#marking-attendance)
    - [Updating Attendance Summaries](#updating-attendance-summaries)
  - [Integration with Other Components](#integration-with-other-components)
  - [Summary](#summary)

# Attendance System

Relevant source files

* [src/daily\_task/mod.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs)
* [src/models/attendance.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs)
* [src/models/member.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs)

The Attendance System in Root serves as a comprehensive solution for tracking member presence, generating attendance records, and compiling attendance statistics. This page documents the core components, data models, automated processes, and operations of the attendance tracking functionality within the Root GraphQL backend.

For details about the daily task automation, see [Daily Attendance Task](/nitronium-ops/root/5.1-daily-attendance-task). For information about the security mechanism, see [Attendance Security](/nitronium-ops/root/5.2-attendance-security).

## System Overview

The Attendance System consists of several interconnected components that work together to maintain accurate attendance records for all members:

The system follows a two-step process for attendance tracking:

1. Automatically creating attendance records for all members every day
2. Updating these records when members are marked present (either manually or through an external system)

Sources: [src/daily\_task/mod.rs37-51](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L37-L51) [src/models/attendance.rs5-59](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L59)

## Core Data Models

The attendance system is built around two primary data models:

### Attendance Model

Represents a daily attendance record for a specific member:

### Attendance Summary Model

Aggregates monthly attendance statistics for each member:

Additional models exist to support various operations:

* `AttendanceInfo`: Simplified attendance view
* `AttendanceSummaryInfo`: Simplified summary view
* `MarkAttendanceInput`: Input format for marking attendance, including HMAC security
* `AttendanceWithMember`: Join model combining attendance with member information

Sources: [src/models/attendance.rs5-17](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L17) [src/models/attendance.rs19-25](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L19-L25) [src/models/attendance.rs27-59](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L27-L59)

## Database Schema

The attendance system relies on the following database tables:

| Table | Description | Key Fields |
| --- | --- | --- |
| Attendance | Stores daily attendance records | attendance\_id (PK), member\_id (FK), date, is\_present, time\_in, time\_out |
| AttendanceSummary | Stores monthly attendance summaries | member\_id (FK), year, month, days\_attended |
| Member | Referenced by attendance records | member\_id (PK), name, roll\_no, etc. |

The tables maintain relationships through the `member_id` foreign key:

Sources: [src/models/attendance.rs5-59](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L59) [src/models/member.rs13-28](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs#L13-L28)

## Automated Daily Task System

A core feature of the attendance system is the automated daily task that runs at a scheduled time each day. This task performs two essential functions:

1. Creating attendance records for all members
2. Updating monthly attendance summaries based on previous day's attendance

The daily task performs the following operations:

1. Scheduled to run at 00:30 Kolkata time every day
2. Fetches all members from the database
3. Creates new attendance records for each member (defaulting to absent)
4. For each member who was present the previous day, updates their monthly attendance summary

Sources: [src/daily\_task/mod.rs10-35](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L10-L35) [src/daily\_task/mod.rs37-51](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L37-L51) [src/daily\_task/mod.rs53-93](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L53-L93) [src/daily\_task/mod.rs95-202](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L95-L202)

## Attendance Operations

The attendance system supports several key operations:

### Creating Attendance Records

Attendance records are automatically created by the daily task:

```
INSERT INTO Attendance (member_id, date, is_present, time_in, time_out) 
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (member_id, date) DO NOTHING

```

The system handles duplicate records using the `ON CONFLICT` clause, ensuring each member has exactly one attendance record per day.

Sources: [src/daily\_task/mod.rs62-73](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L62-L73)

### Marking Attendance

Attendance can be marked through the GraphQL API using the `MarkAttendanceInput` structure, which includes:

* `member_id`: The ID of the member
* `date`: The date of attendance
* `hmac_signature`: Security verification hash

When a member is marked present, the system updates:

1. The individual attendance record for that day
2. The monthly attendance summary (on the next day via the daily task)

Sources: [src/models/attendance.rs42-47](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L42-L47)

### Updating Attendance Summaries

The system maintains monthly attendance summaries through two operations:

1. Checking if a member was present the previous day:

2. Updating the monthly attendance count:

If no summary exists for the current month, a new record is created:

Sources: [src/daily\_task/mod.rs95-202](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L95-L202)

## Integration with Other Components

The Attendance System integrates with several other parts of the Root system:

Key integration points include:

* Member data is referenced through foreign keys in the attendance tables
* The GraphQL API exposes attendance data through queries and mutations
* HMAC security system validates attendance marking requests
* External systems like Presense can interact with the attendance data through the API

Sources: [src/models/attendance.rs42-47](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L42-L47) [src/daily\_task/mod.rs37-39](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/daily_task/mod.rs#L37-L39)

## Summary

The Attendance System provides a robust mechanism for tracking member attendance with these key features:

* Automatic daily record creation for all members
* Secure attendance marking with HMAC verification
* Monthly attendance summaries for statistical analysis
* Integration with the broader member management system

The automated nature of the system ensures consistent record-keeping without manual intervention, while the security features prevent unauthorized attendance marking.