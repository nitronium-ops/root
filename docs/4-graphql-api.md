#  Graphql Api

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

- [GraphQL API](#graphql-api)
  - [Purpose and Scope](#purpose-and-scope)
  - [Architecture Overview](#architecture-overview)
  - [API Structure and Components](#api-structure-and-components)
    - [Schema Composition](#schema-composition)
    - [Domain Organization](#domain-organization)
  - [API Endpoints](#api-endpoints)
  - [Request Flow](#request-flow)
  - [Router Configuration](#router-configuration)
  - [API Integration](#api-integration)
    - [Database Integration](#database-integration)
    - [Client Application Integration](#client-application-integration)
  - [Development Tools](#development-tools)
    - [GraphiQL Playground](#graphiql-playground)
  - [Summary](#summary)

# GraphQL API

Relevant source files

* [src/graphql/mod.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs)
* [src/routes.rs](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs)

## Purpose and Scope

This document provides an overview of Root's GraphQL API, which serves as the primary interface for client applications to interact with the system's data. The API provides a unified entry point for querying and modifying data related to members, attendance, streaks, and projects. For detailed information about specific queries and mutations, see [GraphQL Queries](/amfoss/root/4.1-graphql-queries) and [GraphQL Mutations](/amfoss/root/4.2-graphql-mutations).

## Architecture Overview

The GraphQL API sits at the core of the Root system, acting as the bridge between client applications and the underlying data models. It provides a type-safe, self-documenting interface for data access and manipulation.

Sources: [src/graphql/mod.rs1-22](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L1-L22) [src/routes.rs1-39](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L1-L39)

## API Structure and Components

The GraphQL API is built using the `async-graphql` crate and follows a modular architecture that separates concerns by domain area.

### Schema Composition

The GraphQL schema is composed of two main types:

1. **Query**: Contains all read operations
2. **Mutation**: Contains all write operations

Both types are constructed by merging domain-specific query and mutation types, which keeps the codebase organized by domain area.

Sources: [src/graphql/mod.rs8-22](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L8-L22)

### Domain Organization

The API is organized into four primary domains:

| Domain | Description | Query Type | Mutation Type |
| --- | --- | --- | --- |
| Member | User profile and account information | MemberQueries | MemberMutations |
| Attendance | Daily attendance records and summaries | AttendanceQueries | AttendanceMutations |
| Streak | Status update streak tracking for members | StreakQueries | StreakMutations |
| Project | Member project information | ProjectQueries | ProjectMutations |

Sources: [src/graphql/mod.rs2-3](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L2-L3) [src/graphql/mod.rs9-22](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L9-L22)

## API Endpoints

Root exposes the following HTTP endpoints for interacting with the GraphQL API:

| Endpoint | HTTP Methods | Description | Availability |
| --- | --- | --- | --- |
| `/` | POST | Main GraphQL endpoint for all queries and mutations | Development/Production |
| `/graphiql` | GET, POST | GraphiQL playground for interactive API exploration | Development only |

The GraphiQL playground is only enabled in development mode, providing a convenient way to explore and test the API during development.

Sources: [src/routes.rs12-39](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L12-L39)

## Request Flow

The following diagram illustrates how a typical request flows through the GraphQL API:

Sources: [src/routes.rs17-19](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L17-L19)

## Router Configuration

The GraphQL API is served through an Axum router that is configured during application startup. The router includes CORS middleware and conditionally enables the GraphiQL playground in development environments.

Sources: [src/routes.rs12-30](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L12-L30)

## API Integration

### Database Integration

The GraphQL API interacts with the PostgreSQL database through domain-specific queries and mutations. Each domain area has its own query and mutation implementations that access and modify the corresponding database models.

Sources: [src/graphql/mod.rs1-22](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L1-L22)

### Client Application Integration

Client applications interact with the GraphQL API by sending HTTP POST requests to the `/` endpoint with a GraphQL payload. The API processes these requests and returns JSON responses containing the requested data or error information.

For development and testing purposes, clients can also use the GraphiQL playground to interactively explore the API and construct queries.

Sources: [src/routes.rs17-27](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L17-L27)

## Development Tools

### GraphiQL Playground

In development mode, Root provides a GraphiQL playground at the `/graphiql` endpoint. This interactive tool allows developers to:

1. Explore the GraphQL schema
2. Compose and execute queries and mutations
3. View documentation for all available operations
4. Test API functionality directly in the browser

The playground is automatically disabled in production environments for security reasons.

Sources: [src/routes.rs21-27](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L21-L27) [src/routes.rs32-39](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L32-L39)

## Summary

The GraphQL API is the central interface for accessing and manipulating data in Root. It provides a type-safe, domain-organized approach to data operations that client applications can consume. The API follows GraphQL best practices with:

* Clear separation of concerns between queries and mutations
* Domain-specific organization of operations
* Simple HTTP-based access for clients
* Development tools for interactive exploration

For detailed information about specific queries and mutations, refer to the dedicated documentation pages:

* [GraphQL Queries](/amfoss/root/4.1-graphql-queries)
* [GraphQL Mutations](/amfoss/root/4.2-graphql-mutations)