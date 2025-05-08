#  Project Setup

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

- [Project Setup](#project-setup)
  - [Prerequisites](#prerequisites)
  - [Environment Configuration](#environment-configuration)
  - [Database Setup](#database-setup)
  - [Project Structure](#project-structure)
  - [Running the Application](#running-the-application)
  - [Developer Workflow](#developer-workflow)
  - [Technology Stack](#technology-stack)
  - [Setup Process Overview](#setup-process-overview)

# Project Setup

Relevant source files

* [.github/dependabot.yml](https://github.com/nitronium-ops/root/blob/f2ed7e90/.github/dependabot.yml)
* [.github/workflows/generate-release.yml](https://github.com/nitronium-ops/root/blob/f2ed7e90/.github/workflows/generate-release.yml)
* [.github/workflows/lint.yml](https://github.com/nitronium-ops/root/blob/f2ed7e90/.github/workflows/lint.yml)
* [.gitignore](https://github.com/nitronium-ops/root/blob/f2ed7e90/.gitignore)
* [Cargo.lock](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.lock)
* [Cargo.toml](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml)
* [README.md](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md)

This document provides comprehensive instructions for setting up the Root GraphQL Backend project for local development. It covers prerequisites, environment configuration, database setup, and running the application locally.

For information about the GraphQL API schema and available operations, see [GraphQL API](/nitronium-ops/root/4-graphql-api).

## Prerequisites

Before setting up the project, make sure you have the following software installed:

| Software | Purpose | Installation Notes |
| --- | --- | --- |
| Rust | Programming language | Latest stable version |
| PostgreSQL | Database | Version 12 or higher |
| SQLx CLI | Database migration tool | Install with `cargo install sqlx-cli` |

Sources: [README.md10-15](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L10-L15) [Cargo.toml1-26](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L1-L26)

## Environment Configuration

The project uses environment variables for configuration. Follow these steps:

1. Create an environment file by copying the sample:

2. Edit the `.env` file to configure:
   * Database connection URL (required)
   * HMAC secret for security features
   * Port and other server settings

For detailed information about all available configuration options, see [Environment Configuration](/nitronium-ops/root/2.1-environment-configuration).

Sources: [README.md17-21](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L17-L21) [.gitignore6](https://github.com/nitronium-ops/root/blob/f2ed7e90/.gitignore#L6-L6) [Cargo.toml25](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L25-L25)

## Database Setup

Set up the PostgreSQL database with the following commands:

This will create the database schema for members, attendance records, status update streaks, and projects.

Sources: [README.md23-27](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L23-L27) [Cargo.toml12-13](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L12-L13)

## Project Structure

Sources: [Cargo.toml7-27](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L7-L27) [README.md8](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L8-L8)

## Running the Application

Start the server in development mode:

The application will:

1. Load environment variables
2. Connect to the PostgreSQL database
3. Set up the GraphQL API with async-graphql
4. Create the web server using Axum

Access the GraphQL playground at `http://localhost:8000/graphiql` to interactively test queries and mutations.

Sources: [README.md29-34](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L29-L34) [Cargo.toml7-10](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L7-L10)

## Developer Workflow

When contributing to the project, follow these steps:

Key development practices:

* Always format your code with `cargo fmt` before committing
* Check for issues with `cargo clippy --all-features -- -D warnings`
* Submit pull requests to the `develop` branch, not directly to `main`
* GitHub Actions will automatically run linting checks on your PRs

Sources: [README.md70-80](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L70-L80) [.github/workflows/lint.yml1-39](https://github.com/nitronium-ops/root/blob/f2ed7e90/.github/workflows/lint.yml#L1-L39)

## Technology Stack

The Root project uses the following key technologies:

Sources: [Cargo.toml6-26](https://github.com/nitronium-ops/root/blob/f2ed7e90/Cargo.toml#L6-L26)

## Setup Process Overview

Sources: [README.md10-34](https://github.com/nitronium-ops/root/blob/f2ed7e90/README.md#L10-L34)

The setup process is designed to be straightforward, allowing developers to quickly get started with the Root GraphQL Backend. This server acts as a central data hub for managing club member information, attendance tracking, and related functionality as explained in the [Root GraphQL Backend Overview](/nitronium-ops/root/1-root-graphql-backend-overview).