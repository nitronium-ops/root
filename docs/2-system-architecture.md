#  System Architecture

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

- [System Architecture](#system-architecture)
  - [Core Architecture Components](#core-architecture-components)
  - [Application Initialization Flow](#application-initialization-flow)
  - [GraphQL API Structure](#graphql-api-structure)
    - [Routing Configuration](#routing-configuration)
  - [Database Integration](#database-integration)
  - [Background Processing](#background-processing)
  - [Cross-Origin Resource Sharing (CORS)](#cross-origin-resource-sharing-cors)
  - [Data Flow](#data-flow)
  - [Configuration Management](#configuration-management)
  - [System Component Relationships](#system-component-relationships)
  - [Conclusion](#conclusion)

# System Architecture

Relevant source files

* [.gitignore](https://github.com/amfoss/root/blob/2b58803d/.gitignore)
* [Cargo.lock](https://github.com/amfoss/root/blob/2b58803d/Cargo.lock)
* [Cargo.toml](https://github.com/amfoss/root/blob/2b58803d/Cargo.toml)
* [src/graphql/mod.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs)
* [src/main.rs](https://github.com/amfoss/root/blob/2b58803d/src/main.rs)
* [src/routes.rs](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs)

This page provides a comprehensive overview of the Root system architecture, detailing its components, data flow, and initialization process. Root is a GraphQL backend built with Rust that manages club member data, attendance records, projects, and status update streaks. For detailed information about specific configurations, see [Application Configuration](/amfoss/root/2.1-application-configuration).

## Core Architecture Components

The Root backend is structured as a modern web application with clear separation of concerns between the API layer, domain logic, and data persistence.

Sources: [src/main.rs1-140](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L1-L140) [src/graphql/mod.rs1-23](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L1-L23)

## Application Initialization Flow

The initialization process follows a strict sequence to ensure all components are properly configured before the server starts handling requests.

Sources: [src/main.rs40-60](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L40-L60)

The application is initialized in `main()` with the following key steps:

1. **Configuration Loading**: Environment variables are loaded using `dotenv` to configure the application
2. **Tracing Setup**: Different tracing configurations are applied based on the environment (development/production)
3. **Database Setup**: A connection pool is created and migrations are run to ensure the database schema is up-to-date
4. **GraphQL Schema Building**: The schema is built combining queries and mutations for all domain areas
5. **Background Task Spawning**: The daily attendance task is spawned in a separate Tokio task
6. **Router Setup**: The Axum router is configured with the appropriate routes and middleware
7. **Server Startup**: The HTTP server is started on the configured port

Sources: [src/main.rs40-140](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L40-L140)

## GraphQL API Structure

The GraphQL API is organized into queries (read operations) and mutations (write operations), further divided by domain areas.

Sources: [src/graphql/mod.rs1-23](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L1-L23) [src/main.rs118-126](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L118-L126)

The GraphQL schema does not implement subscriptions, as indicated by the use of `EmptySubscription`. The schema is built in the application initialization phase and passed to the Axum router.

### Routing Configuration

The application's routing is handled by Axum, which provides HTTP endpoints for the GraphQL API.

Sources: [src/routes.rs1-40](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L1-L40)

In development mode, a GraphiQL playground is available at `/graphiql` for interactive testing of the GraphQL API. This feature is disabled in production.

## Database Integration

The application uses PostgreSQL for data persistence, with connection pooling managed by `sqlx`.

Sources: [src/main.rs102-116](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L102-L116)

Key database integration features:

* Connection pool with 2-3 connections
* Automatic migrations at startup using SQLx migrations
* Arc-wrapped pool shared across components

## Background Processing

The application includes a background task that runs daily operations, particularly related to attendance.

Sources: [src/main.rs48-50](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L48-L50) [src/daily\_task.rs](https://github.com/amfoss/root/blob/2b58803d/src/daily_task.rs) (referenced in main.rs)

The daily task is spawned as an asynchronous Tokio task during application initialization. It calculates the time until midnight, sleeps until then, and performs attendance-related operations.

## Cross-Origin Resource Sharing (CORS)

The application implements CORS to control which domains can access the API.

Sources: [src/main.rs128-139](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L128-L139)

The CORS configuration allows requests from the development server and the production domain, with specific HTTP methods permitted.

## Data Flow

The following diagram illustrates how data flows through the system during a typical request-response cycle:

Sources: [src/main.rs1-140](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L1-L140) [src/routes.rs1-40](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L1-L40) [src/graphql/mod.rs1-23](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L1-L23)

## Configuration Management

The application uses environment variables for configuration, with fallbacks for development settings.

Sources: [src/main.rs20-38](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L20-L38)

The configuration includes:

* Environment indicator (development/production)
* Secret key for HMAC authentication
* Database connection URL
* HTTP server port

## System Component Relationships

The following table summarizes the key components and their relationships:

| Component | Responsibility | Depends On | Source Files |
| --- | --- | --- | --- |
| Axum Router | HTTP request handling | GraphQL Schema, CORS Layer | [src/routes.rs](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs) |
| GraphQL API | API entry point | Query, Mutation | [src/graphql/mod.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs) |
| PostgreSQL Database | Data persistence | - | [src/main.rs102-116](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L102-L116) |
| Daily Task | Background processing | PostgreSQL Database | [src/main.rs48-50](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L48-L50) |
| Configuration | App configuration | Environment variables | [src/main.rs20-38](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L20-L38) |
| Tracing | Logging | Environment configuration | [src/main.rs62-100](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L62-L100) |

Sources: [src/main.rs1-140](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L1-L140) [src/graphql/mod.rs1-23](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mod.rs#L1-L23) [src/routes.rs1-40](https://github.com/amfoss/root/blob/2b58803d/src/routes.rs#L1-L40)

## Conclusion

The Root system architecture follows modern patterns for a GraphQL API backend, with clean separation of concerns, efficient database access through connection pooling, and background processing capabilities. The architecture is designed to be maintainable, with domain-specific operations organized into distinct modules.