# Overview

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

- [Overview](#overview)
  - [System Purpose](#system-purpose)
  - [System Context](#system-context)
  - [Key Components](#key-components)
  - [Technology Stack](#technology-stack)
  - [Application Initialization Flow](#application-initialization-flow)
  - [Integration with Other Services](#integration-with-other-services)
  - [Development and Deployment](#development-and-deployment)

# Overview

Relevant source files

* [.gitignore](https://github.com/amfoss/root/blob/2b58803d/.gitignore)
* [Cargo.lock](https://github.com/amfoss/root/blob/2b58803d/Cargo.lock)
* [Cargo.toml](https://github.com/amfoss/root/blob/2b58803d/Cargo.toml)
* [README.md](https://github.com/amfoss/root/blob/2b58803d/README.md)
* [src/main.rs](https://github.com/amfoss/root/blob/2b58803d/src/main.rs)

Root is a GraphQL backend that serves as the central data management system for amfoss club. It collects and distributes data from and to other services including Home, amD, and Presense, creating a unified data infrastructure while allowing each service to operate independently.

Sources: [README.md7-8](https://github.com/amfoss/root/blob/2b58803d/README.md#L7-L8)

## System Purpose

Root aims to solve the problem of distributed data management by providing a single source of truth for club member information, attendance records, project tracking, and activity streaks. Unlike the club's previous CMS architecture, Root decouples data management from client applications, ensuring there's no single point of failure across the infrastructure.

The system primarily handles:

* Member data management
* Attendance tracking and reporting
* Project management
* Activity streak monitoring

For information about deployment specifics, see [Deployment and CI/CD](/amfoss/root/6-deployment-and-cicd).

Sources: [README.md8-9](https://github.com/amfoss/root/blob/2b58803d/README.md#L8-L9) [src/main.rs14-17](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L14-L17)

## System Context

Root sits at the center of the amfoss digital infrastructure, providing a GraphQL API that other services consume. Each client application interacts with Root through GraphQL queries and mutations to retrieve or modify data, while maintaining its own independent functionality.

Sources: [README.md8-9](https://github.com/amfoss/root/blob/2b58803d/README.md#L8-L9) [src/main.rs10-12](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L10-L12)

## Key Components

The Root system consists of four main components:

1. **GraphQL API Layer**: Built with `async-graphql` and `axum`, this layer handles all client interactions through a GraphQL schema with queries and mutations.
2. **Data Models**: Core data structures representing members, attendance records, projects, and streaks.
3. **Database Layer**: PostgreSQL database accessed via `sqlx` that provides persistent storage.
4. **Background Tasks**: Scheduled operations, particularly the daily attendance task that runs at midnight.

Sources: [src/main.rs10-17](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L10-L17) [src/main.rs40-59](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L40-L59)

## Technology Stack

Root is built on a modern Rust stack with the following key technologies:

| Component | Technology | Purpose |
| --- | --- | --- |
| Language | Rust | Core programming language |
| Web Framework | Axum | HTTP server and routing |
| GraphQL | async-graphql | GraphQL schema and execution |
| Database | PostgreSQL + sqlx | Data storage and access |
| Async Runtime | Tokio | Asynchronous task execution |
| Authentication | HMAC + SHA2 | Securing attendance mutations |
| Configuration | dotenv | Environment variable management |
| Logging | tracing + tracing-subscriber | Application logging |
| Date/Time | chrono + chrono-tz | Date and timezone handling |

Sources: [Cargo.toml6-26](https://github.com/amfoss/root/blob/2b58803d/Cargo.toml#L6-L26) [src/main.rs1-8](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L1-L8)

## Application Initialization Flow

The system follows a structured initialization process when starting up:

At startup, Root loads configuration from environment variables, sets up tracing based on the environment (development or production), connects to the PostgreSQL database, runs any pending migrations, builds the GraphQL schema, spawns the daily attendance task, sets up the router with CORS configuration, and finally starts the HTTP server.

Sources: [src/main.rs40-59](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L40-L59) [src/main.rs62-139](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L62-L139)

## Integration with Other Services

Root integrates with several amfoss services:

* **Home**: The club website consumes member data and project information
* **amD**: Member dashboard that displays personalized information and statistics
* **Presense**: Attendance tracking system that uses HMAC verification to securely record member presence

The CORS configuration specifically allows requests from `http://127.0.0.1:3000` and `https://home.amfoss.in`, indicating the primary client applications.

Sources: [README.md8-9](https://github.com/amfoss/root/blob/2b58803d/README.md#L8-L9) [src/main.rs128-139](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L128-L139)

## Development and Deployment

Root follows a structured development and deployment process:

* Development occurs on the `develop` branch
* Production-ready code is merged into the `main` branch
* GitHub Actions workflows handle linting, testing, and deployment
* The system is containerized using Docker and deployed to GitHub Container Registry
* The production instance is accessible at [root.amfoss.in](https://root.amfoss.in)

For local development, the system provides a GraphQL playground at `http://localhost:8000/graphiql` when running in development mode.

For more detailed information about deployment processes, see [Deployment and CI/CD](/amfoss/root/6-deployment-and-cicd).

Sources: [README.md37-42](https://github.com/amfoss/root/blob/2b58803d/README.md#L37-L42) [README.md10-34](https://github.com/amfoss/root/blob/2b58803d/README.md#L10-L34)