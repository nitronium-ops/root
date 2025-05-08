#  Security Features

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

- [Security Features](#security-features)
  - [Overview of Security Measures](#overview-of-security-measures)
  - [HMAC-Based Authentication](#hmac-based-authentication)
    - [Implementation Details](#implementation-details)
  - [Environment Variable Security](#environment-variable-security)
  - [CORS Configuration](#cors-configuration)
  - [Database Security Measures](#database-security-measures)
    - [Connection Pooling](#connection-pooling)
    - [Constraints and Data Validation](#constraints-and-data-validation)
  - [Logging and Audit](#logging-and-audit)
  - [Overall Security Architecture](#overall-security-architecture)

# Security Features

Relevant source files

* [migrations/20250114180047\_create\_tables.sql](https://github.com/amfoss/root/blob/2b58803d/migrations/20250114180047_create_tables.sql)
* [src/graphql/mutations/attendance\_mutations.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/attendance_mutations.rs)
* [src/graphql/mutations/member\_mutations.rs](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/member_mutations.rs)
* [src/main.rs](https://github.com/amfoss/root/blob/2b58803d/src/main.rs)

This page documents the security features implemented in the Root system. It covers authentication mechanisms, data protection, access controls, and other security measures used to protect the integrity and confidentiality of the system.

For information about HMAC-based authentication used specifically for attendance marking, see [HMAC Authentication](/amfoss/root/7.1-hmac-authentication).

## Overview of Security Measures

Root implements several security features to protect its data and ensure the integrity of its operations. These features work together to create a comprehensive security framework.

Sources: [src/main.rs21-37](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L21-L37) [src/graphql/mutations/attendance\_mutations.rs29-43](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/attendance_mutations.rs#L29-L43)

## HMAC-Based Authentication

Root uses HMAC (Hash-based Message Authentication Code) with SHA-256 for verifying the authenticity of attendance marking requests. This cryptographic technique ensures that only legitimate requests with valid signatures can update attendance records.

The HMAC verification process:

1. The client sends a request with the member ID, date, and an HMAC signature
2. The server retrieves the secret key stored in environment variables
3. The server generates an expected signature by applying HMAC-SHA256 to the concatenation of member ID and date
4. The server compares the expected signature with the received signature
5. If the signatures match, the request is processed; otherwise, it's rejected

Sources: [src/graphql/mutations/attendance\_mutations.rs19-62](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/attendance_mutations.rs#L19-L62)

### Implementation Details

The HMAC verification is implemented in the `mark_attendance` mutation:

The secret key used for HMAC is stored as an environment variable and passed to the GraphQL context during application initialization.

Sources: [src/main.rs119-126](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L119-L126) [src/graphql/mutations/attendance\_mutations.rs28-43](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/attendance_mutations.rs#L28-L43)

## Environment Variable Security

Root uses environment variables to store sensitive configuration data, ensuring that credentials and secrets are not hardcoded in the source code.

| Environment Variable | Purpose | Impact on Security |
| --- | --- | --- |
| `ROOT_SECRET` | Secret key for HMAC verification | Critical for request authentication |
| `DATABASE_URL` | PostgreSQL connection string | Contains database credentials |
| `ROOT_ENV` | Deployment environment (development/production) | Determines logging behavior |
| `ROOT_PORT` | Port for the HTTP server | Affects network exposure |

The environment variables are loaded at application startup and stored in the `Config` struct. This approach prevents hardcoding secrets in the application code, allowing for secure deployment across different environments.

Sources: [src/main.rs21-37](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L21-L37)

## CORS Configuration

Root implements Cross-Origin Resource Sharing (CORS) to control which domains can interact with the API. This prevents unauthorized websites from making requests to the API.

The CORS configuration allows requests only from specific origins and with specific HTTP methods:

* Allowed origins: `http://127.0.0.1:3000` (development) and `https://home.amfoss.in` (production)
* Allowed methods: GET, POST, OPTIONS
* Allowed headers: Any

Sources: [src/main.rs128-139](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L128-L139)

## Database Security Measures

The Root system implements several database-level security features to protect data integrity:

### Connection Pooling

The application uses connection pooling to manage database connections securely:

Connection pooling improves security by:

* Limiting the number of active connections to the database
* Reusing connections instead of creating new ones for each request
* Managing connection lifecycle securely

Sources: [src/main.rs102-116](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L102-L116)

### Constraints and Data Validation

The database schema includes constraints that enforce data integrity and prevent invalid data:

* Foreign key constraints ensure referential integrity between tables
* Check constraints validate data before insertion/update
* Unique constraints prevent duplicate records

For example, the Attendance table has constraints that ensure:

* A member can have only one attendance record per day
* Attendance can only be marked as present if time\_in and time\_out are recorded
* time\_out must be greater than or equal to time\_in

Sources: [migrations/20250114180047\_create\_tables.sql21-37](https://github.com/amfoss/root/blob/2b58803d/migrations/20250114180047_create_tables.sql#L21-L37)

## Logging and Audit

Root implements environment-aware logging to help with security monitoring and troubleshooting:

* In production mode, logs are written to a file with ANSI encoding disabled
* In development mode, logs are written both to stdout and a file
* Different log levels are used based on the environment

This approach ensures that security events are properly recorded and can be analyzed when needed.

Sources: [src/main.rs62-100](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L62-L100)

## Overall Security Architecture

The following diagram shows how the security features interact within the Root system:

The security components work together to ensure that:

1. Only allowed origins can access the API (CORS layer)
2. Attendance updates are properly authenticated (HMAC verification)
3. Sensitive configuration is stored securely (Environment variables)
4. Database operations maintain data integrity (Database constraints)

Sources: [src/main.rs40-60](https://github.com/amfoss/root/blob/2b58803d/src/main.rs#L40-L60) [src/graphql/mutations/attendance\_mutations.rs19-62](https://github.com/amfoss/root/blob/2b58803d/src/graphql/mutations/attendance_mutations.rs#L19-L62)