#  Graphql Api

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

- [GraphQL API](#graphql-api)
  - [API Architecture](#api-architecture)
  - [API Endpoints](#api-endpoints)
  - [Schema Structure](#schema-structure)
  - [Schema Initialization](#schema-initialization)
  - [API Access and CORS](#api-access-and-cors)
  - [GraphiQL Development Interface](#graphiql-development-interface)
  - [Connection with Business Logic](#connection-with-business-logic)
  - [Security Considerations](#security-considerations)
  - [Data Context](#data-context)
  - [API Interaction Examples](#api-interaction-examples)
    - [Basic Query Example](#basic-query-example)
    - [Basic Mutation Example](#basic-mutation-example)
  - [Summary](#summary)

# GraphQL API

Relevant source files

* [src/graphql/mod.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/mod.rs)
* [src/main.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs)
* [src/routes.rs](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/routes.rs)

This document provides a technical overview of the Root GraphQL API, which serves as the central interface for accessing and manipulating data in the system. It covers the API's structure, endpoints, and key components. For details about specific queries and mutations related to particular data models, see their respective pages ([Member Queries and Mutations](/nitronium-ops/root/4.1-member-queries-and-mutations), [Attendance Queries and Mutations](/nitronium-ops/root/4.2-attendance-queries-and-mutations), [Streak Queries and Mutations](/nitronium-ops/root/4.3-streak-queries-and-mutations), and [Project Queries and Mutations](/nitronium-ops/root/4.4-project-queries-and-mutations)).

## API Architecture

The Root system exposes a GraphQL API as its primary interface for client applications. The API is built using the `async-graphql` Rust crate with an Axum web server.

Sources: [src/main.rs118-126](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L118-L126) [src/routes.rs12-39](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/routes.rs#L12-L39) [src/graphql/mod.rs1-22](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/mod.rs#L1-L22)

## API Endpoints

The Root GraphQL API exposes the following endpoints:

| Endpoint | Description | Environment |
| --- | --- | --- |
| `/` | Primary GraphQL endpoint | All |
| `/graphiql` | GraphiQL interactive playground | Development only |

The GraphQL endpoint accepts POST requests with GraphQL queries and mutations. The GraphiQL playground provides a web-based interface for exploring and testing the API when running in development mode.

Sources: [src/routes.rs12-39](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/routes.rs#L12-L39)

## Schema Structure

The GraphQL schema is organized around the main data domains in the system: Members, Attendance, Streaks, and Projects. The schema uses the merged object pattern to compose the root Query and Mutation types from domain-specific subcomponents.

Sources: [src/graphql/mod.rs1-22](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/mod.rs#L1-L22)

## Schema Initialization

The GraphQL schema is initialized in the application's main function and includes the following components:

1. A merged `Query` object composed of domain-specific query types
2. A merged `Mutation` object composed of domain-specific mutation types
3. An `EmptySubscription` type (the API doesn't support GraphQL subscriptions)
4. Shared data including:
   * Database connection pool
   * Secret key for security operations

Sources: [src/main.rs40-60](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L40-L60) [src/main.rs118-126](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L118-L126)

## API Access and CORS

The GraphQL API implements CORS (Cross-Origin Resource Sharing) to control which domains can access the API. The CORS configuration allows specific origins and HTTP methods:

| Allowed Origins | Allowed Methods |
| --- | --- |
| <http://127.0.0.1:3000> | GET |
| <https://home.amfoss.in> | POST |
|  | OPTIONS |

All HTTP headers are allowed through the CORS configuration.

Sources: [src/main.rs128-139](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L128-L139)

## GraphiQL Development Interface

In development mode, the system provides a GraphiQL interface at the `/graphiql` endpoint. This interface allows developers to:

1. Explore the GraphQL schema
2. Test queries and mutations interactively
3. View documentation of available types and operations

The GraphiQL interface is disabled in production for security reasons.

Sources: [src/routes.rs21-29](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/routes.rs#L21-L29) [src/routes.rs32-39](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/routes.rs#L32-L39)

## Connection with Business Logic

The GraphQL API serves as an interface layer that connects client applications to the system's business logic and data access layers. Queries and mutations are resolved by corresponding resolver functions that implement business logic and perform database operations.

Sources: [src/main.rs118-126](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L118-L126) [src/graphql/mod.rs1-22](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/graphql/mod.rs#L1-L22)

## Security Considerations

The GraphQL API incorporates several security measures:

1. **HMAC Verification**: Used for securing attendance marking operations
2. **Environment-Based Configuration**: Different settings for development and production environments
3. **Secret Key**: Used for security operations (e.g., HMAC signing)
4. **CORS Protection**: Limited to specific origins to prevent unauthorized access

For more details on the attendance security implementation, see [Attendance Security](/nitronium-ops/root/5.2-attendance-security).

Sources: [src/main.rs20-38](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L20-L38) [src/main.rs128-139](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L128-L139)

## Data Context

The GraphQL resolvers have access to shared data provided through the schema context:

1. **Database Connection**: A pooled connection to the PostgreSQL database
2. **Secret Key**: Used for security operations

This context is made available to all resolvers through the schema's data injection mechanism.

Sources: [src/main.rs118-126](https://github.com/nitronium-ops/root/blob/f2ed7e90/src/main.rs#L118-L126)

## API Interaction Examples

### Basic Query Example

### Basic Mutation Example

## Summary

The Root GraphQL API serves as the central interface for accessing and manipulating data in the system. It provides a structured, type-safe way for client applications to interact with the system's core functionality. The API is organized around the main data domains (Members, Attendance, Streaks, and Projects) and follows a clean architecture pattern with clear separation of concerns.