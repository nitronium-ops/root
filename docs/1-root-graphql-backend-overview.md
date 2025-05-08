#  Root Graphql Backend Overview

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

- [Root GraphQL Backend Overview](#root-graphql-backend-overview)
  - [Purpose and Scope](#purpose-and-scope)
  - [System Architecture](#system-architecture)
    - [High-Level Architecture](#high-level-architecture)
    - [Core Components](#core-components)
  - [Key Components](#key-components)
    - [Configuration System](#configuration-system)
    - [Database Connection](#database-connection)
    - [GraphQL Engine](#graphql-engine)
    - [Daily Task System](#daily-task-system)
    - [CORS Configuration](#cors-configuration)
  - [Technical Stack](#technical-stack)
  - [Integration with Other Services](#integration-with-other-services)
  - [Development and Production Modes](#development-and-production-modes)
  - [Conclusion](#conclusion)

# Root GraphQL Backend Overview

Relevant source files

* [.gitignore](https://github.com/nitronium-ops/root/blob/f2ed7e90/.gitignore)
* [Cargo.lock](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.lock)
* [Cargo.toml](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml)
* [README.md](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md)
* [src/main.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs)

Root is a GraphQL backend service designed to serve as a central data hub for managing club member information. It provides a unified API for data access and manipulation across various applications such as Home, amD, and Presense. This document offers an overview of the architecture, key components, and functionality of the Root system.

For specific setup instructions, see [Project Setup](/nitronium-ops/root/2-project-setup). For detailed API documentation, refer to [GraphQL API](/nitronium-ops/root/4-graphql-api).

## Purpose and Scope

Root serves as the single source of truth for club member data, attendance records, and related information. By centralizing data management, Root ensures consistency across multiple client applications while preventing any single point of failure in the overall infrastructure. The service handles:

* Member profile management
* Attendance tracking
* Status update streak monitoring
* Project information

Sources: [README.md7-8](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L7-L8) [src/main.rs1-18](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L1-L18)

## System Architecture

Root follows a modern Rust-based architecture leveraging the async ecosystem and GraphQL for API interactions.

### High-Level Architecture

Sources: [src/main.rs40-59](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L40-L59) [README.md7-8](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L7-L8)

### Core Components

Sources: [src/main.rs40-59](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L40-L59) [src/main.rs102-126](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L102-L126)

## Key Components

### Configuration System

Root uses environment variables for configuration, managed through a central `Config` struct that extracts values from the environment.

Key configuration parameters:

* `ROOT_ENV`: Environment (development/production)
* `ROOT_SECRET`: Secret key for security operations
* `DATABASE_URL`: PostgreSQL connection string
* `ROOT_PORT`: Server port

Sources: [src/main.rs20-38](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L20-L38)

### Database Connection

The system establishes a connection pool to PostgreSQL using SQLx:

The database connection is wrapped in an `Arc` for safe sharing across async tasks.

Sources: [src/main.rs102-116](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L102-L116)

### GraphQL Engine

Root builds its GraphQL schema using the async-graphql crate:

The schema combines query and mutation capabilities while storing shared data like the database pool and secret key.

Sources: [src/main.rs118-126](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L118-L126) [src/main.rs11](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L11-L11)

### Daily Task System

A critical component is the daily task system that runs at midnight to handle attendance-related operations:

This automated process ensures consistent attendance tracking without manual intervention.

Sources: [src/main.rs48-50](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L48-L50) [src/main.rs10](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L10-L10)

### CORS Configuration

Root implements CORS protection for secure cross-origin resource sharing:

The system allows specific origins including local development and production domains.

Sources: [src/main.rs128-139](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L128-L139)

## Technical Stack

The Root backend is built with the following key technologies:

| Component | Technology | Purpose |
| --- | --- | --- |
| Server Framework | Axum | HTTP server and routing |
| GraphQL Engine | async-graphql | GraphQL schema and resolution |
| Database | PostgreSQL via SQLx | Data persistence |
| Runtime | Tokio | Async runtime |
| Authentication | HMAC (sha2) | Secure request verification |
| Logging | tracing/tracing-subscriber | Structured logging |
| Configuration | dotenv | Environment variable loading |
| Time Handling | chrono, time | Date/time operations |

Sources: [Cargo.toml6-27](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L6-L27)

## Integration with Other Services

Root serves as a central hub that other applications can interact with through its GraphQL API:

This distributed architecture ensures there's no single point of failure while maintaining data consistency across services.

Sources: [README.md7-8](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L7-L8)

## Development and Production Modes

Root supports both development and production modes, with different behaviors:

| Feature | Development Mode | Production Mode |
| --- | --- | --- |
| GraphiQL | Enabled | Disabled |
| Logging | Console + File (verbose) | File only (info level) |
| CORS | Development origins | Production origins only |

The mode is controlled by the `ROOT_ENV` environment variable.

Sources: [src/main.rs62-100](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L62-L100) [src/main.rs53](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L53-L53)

## Conclusion

Root provides a robust, centralized GraphQL API for club membership management, attendance tracking, and related functionality. Its modular architecture and integration with other services make it a core component of the club's technical infrastructure.

The system follows Rust best practices with clear separation of concerns, type safety through SQLx, and efficient async processing via Tokio and async-graphql.

For more details on specific components, refer to the related wiki pages:

* [System Architecture](/nitronium-ops/root/1.1-system-architecture) for detailed architecture diagrams
* [Data Models](/nitronium-ops/root/3-data-models) for information about the core data entities
* [GraphQL API](/nitronium-ops/root/4-graphql-api) for API documentation
* [Attendance System](/nitronium-ops/root/5-attendance-system) for the attendance tracking functionality