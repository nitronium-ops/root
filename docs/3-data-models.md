#  Data Models

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

- [Data Models](#data-models)
  - [Core Data Models Overview](#core-data-models-overview)
  - [Model Implementation and GraphQL Integration](#model-implementation-and-graphql-integration)
  - [Member Model](#member-model)
  - [Attendance Models](#attendance-models)
    - [Attendance](#attendance)
    - [AttendanceSummary](#attendancesummary)
    - [Supporting Models](#supporting-models)
  - [Project Model](#project-model)
  - [Status Update Streak Model](#status-update-streak-model)
  - [Data Model Common Features](#data-model-common-features)
  - [Model Usage in GraphQL API](#model-usage-in-graphql-api)
  - [Summary](#summary)

# Data Models

Relevant source files

* [migrations/20250124095543\_create\_project\_table.sql](https://github.com/nitronium-ops/root/blob/f2ed7e90/migrations/20250124095543_create_project_table.sql)
* [src/graphql/mutations/mod.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/mutations/mod.rs)
* [src/graphql/queries/mod.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/queries/mod.rs)
* [src/models/attendance.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs)
* [src/models/member.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs)
* [src/models/project.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/project.rs)

This document provides a comprehensive overview of the core data models in the Root GraphQL Backend system. It describes the primary entities, their relationships, and how they are implemented in the codebase. For detailed information about specific queries and mutations related to these models, see [GraphQL API](/nitronium-ops/root/4-graphql-api).

## Core Data Models Overview

The Root GraphQL Backend is built around several key data models that represent the core entities in the system. Each model is defined as a Rust struct and maps to a corresponding PostgreSQL database table.

**Diagram Title: Entity Relationship Diagram of Root System Data Models**

Sources: [src/models/member.rs13-28](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs#L13-L28) [src/models/attendance.rs5-17](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L17) [src/models/attendance.rs19-25](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L19-L25) [src/models/project.rs4-9](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/project.rs#L4-L9) [migrations/20250124095543\_create\_project\_table.sql1-6](https://github.com/nitronium-ops/root/blob/f2ed7e90/migrations/20250124095543_create_project_table.sql#L1-L6)

## Model Implementation and GraphQL Integration

The data models are implemented as Rust structs with GraphQL integrations using the `async-graphql` crate. This approach allows seamless translation between database records and GraphQL types.

**Diagram Title: Model Integration Between Rust, Database, and GraphQL**

Sources: [src/models/member.rs13-28](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs#L13-L28) [src/models/attendance.rs5-17](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L17) [src/models/attendance.rs19-25](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L19-L25) [src/models/project.rs4-9](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/project.rs#L4-L9)

## Member Model

The `Member` model is the central entity in the system, representing club members. Each member has various attributes including personal information and identifiers.

The model also defines an enum type for gender:

For member creation, an input type is defined:

Sources: [src/models/member.rs5-41](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs#L5-L41)

## Attendance Models

The attendance system consists of two primary models: `Attendance` for daily records and `AttendanceSummary` for monthly aggregations.

### Attendance

The `Attendance` model represents daily attendance records for each member:

### AttendanceSummary

The `AttendanceSummary` model stores monthly attendance statistics:

### Supporting Models

Several additional models support attendance operations:

* `AttendanceInfo`: Simplified view for API responses
* `AttendanceSummaryInfo`: Simplified summary view
* `MarkAttendanceInput`: Input for attendance marking with security
* `AttendanceWithMember`: Combined attendance and member details

Sources: [src/models/attendance.rs5-59](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L59)

## Project Model

The `Project` model represents projects associated with members:

For creating or updating a project, an input type is defined:

The database schema includes a foreign key constraint linking projects to members:

Sources: [src/models/project.rs4-15](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/project.rs#L4-L15) [migrations/20250124095543\_create\_project\_table.sql1-6](https://github.com/nitronium-ops/root/blob/f2ed7e90/migrations/20250124095543_create_project_table.sql#L1-L6)

## Status Update Streak Model

The `StatusUpdateStreak` model tracks member engagement through continuous status updates:

This model helps track member consistency in providing status updates, serving as a metric for engagement and participation.

## Data Model Common Features

All data models in the system share several common implementation features:

| Feature | Description | Implementation |
| --- | --- | --- |
| GraphQL Integration | Models are exposed to GraphQL API | `SimpleObject` derive macro |
| Database Mapping | Models map directly to database tables | `FromRow` derive macro |
| Input Types | Separate structs for input operations | `InputObject` derive macro |
| Relationship Handling | Foreign keys and nested resolvers | GraphQL complex object resolvers |
| Security Controls | Field-level visibility control | `#[graphql(skip)]` attributes |

Sources: [src/models/member.rs13-28](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/member.rs#L13-L28) [src/models/attendance.rs5-17](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/attendance.rs#L5-L17) [src/models/project.rs4-9](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/models/project.rs#L4-L9)

## Model Usage in GraphQL API

The models are integrated into the GraphQL API through queries and mutations organized by entity type:

**Diagram Title: Model Integration with GraphQL API Structure**

Sources: [src/graphql/queries/mod.rs1-9](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/queries/mod.rs#L1-L9) [src/graphql/mutations/mod.rs1-9](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/mutations/mod.rs#L1-L9)

## Summary

The data models in the Root GraphQL Backend form a cohesive system centered around the `Member` entity, with related models for tracking attendance, projects, and engagement. Each model is implemented as a Rust struct with appropriate GraphQL and database integrations.

The models are designed with the following principles:

* Clear separation of concerns between different entity types
* Strong relationships defined through foreign keys
* Specialized input types for mutations
* GraphQL-specific annotations for controlling field visibility
* Efficient data representation through summary and info variants

For detailed information about specific models, see their dedicated pages:

* [Member Model](/nitronium-ops/root/3.1-member-model)
* [Attendance Models](/nitronium-ops/root/3.2-attendance-models)
* [Status Update Streak Model](/nitronium-ops/root/3.3-status-update-streak-model)
* [Project Model](/nitronium-ops/root/3.4-project-model)