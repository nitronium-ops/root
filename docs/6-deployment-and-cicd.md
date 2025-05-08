# Deployment and CI/CD

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

- [Deployment and CI/CD](#deployment-and-cicd)
  - [Overview](#overview)
  - [Development Workflow](#development-workflow)
  - [Code Quality Automation](#code-quality-automation)
    - [Linting Workflow](#linting-workflow)
    - [Dependency Management](#dependency-management)
  - [Continuous Deployment](#continuous-deployment)
    - [Docker Image Deployment](#docker-image-deployment)
    - [Binary Releases](#binary-releases)
  - [Deployment Configuration](#deployment-configuration)
    - [Environment Variables](#environment-variables)
    - [Docker Deployment](#docker-deployment)
  - [Conclusion](#conclusion)

# Deployment and CI/CD

Relevant source files

* [.dockerignore](https://github.com/amfoss/root/blob/2b58803d/.dockerignore)
* [.env.sample](https://github.com/amfoss/root/blob/2b58803d/.env.sample)
* [.github/dependabot.yml](https://github.com/amfoss/root/blob/2b58803d/.github/dependabot.yml)
* [.github/workflows/generate-release.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/generate-release.yml)
* [.github/workflows/ghcr-deploy.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/ghcr-deploy.yml)
* [.github/workflows/lint.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/lint.yml)
* [Dockerfile](https://github.com/amfoss/root/blob/2b58803d/Dockerfile)

This document describes the Continuous Integration and Continuous Deployment (CI/CD) pipeline for the Root system, including the automation processes that ensure code quality, testing, and smooth deployment to production environments. For details about specific security features implemented in the deployment, see [Security Features](/amfoss/root/7-security-features).

## Overview

Root implements a comprehensive CI/CD pipeline using GitHub Actions to automate testing, building, and deployment processes. The system supports multiple deployment methods, including Docker containers via GitHub Container Registry (GHCR) and platform-specific binary releases.

Sources: [.github/workflows/lint.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/lint.yml) [.github/workflows/ghcr-deploy.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/ghcr-deploy.yml) [.github/workflows/generate-release.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/generate-release.yml) [.github/dependabot.yml](https://github.com/amfoss/root/blob/2b58803d/.github/dependabot.yml)

## Development Workflow

Root follows a branch-based development workflow with automated quality checks:

1. Development happens in feature branches
2. Pull requests are created against the `main` or `develop` branches
3. Automated linting and code formatting checks run on all PRs
4. Approved PRs are merged to the target branch
5. Merges to `main` trigger the deployment pipeline

Weekly dependency updates are automatically created by Dependabot, which opens pull requests that go through the same review process.

Sources: [.github/workflows/lint.yml3-5](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/lint.yml#L3-L5) [.github/dependabot.yml](https://github.com/amfoss/root/blob/2b58803d/.github/dependabot.yml)

## Code Quality Automation

### Linting Workflow

The linting workflow runs automatically on all pull requests targeting the `main` or `develop` branches and consists of two key jobs:

1. **Clippy**: Runs Rust's Clippy linter with strict warning settings
2. **Rustfmt**: Verifies that code follows the project's formatting standards

The workflow fails if either Clippy finds issues that it considers errors or if the code doesn't match the expected formatting style, preventing the PR from being merged until these issues are fixed.

Sources: [.github/workflows/lint.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/lint.yml)

### Dependency Management

Dependabot is configured to scan for outdated Cargo dependencies weekly and automatically creates pull requests to update them when newer versions are available.

Sources: [.github/dependabot.yml](https://github.com/amfoss/root/blob/2b58803d/.github/dependabot.yml)

## Continuous Deployment

### Docker Image Deployment

When changes are pushed to the `main` branch, the GHCR deploy workflow automatically:

1. Builds a Docker image using the project's Dockerfile
2. Tags the image appropriately (including `latest` for the main branch)
3. Pushes the image to GitHub Container Registry

The Dockerfile uses a multi-stage build process to minimize the final image size:

* First stage: Builds the Rust application
* Second stage: Creates a minimal runtime image with just the compiled binary

Sources: [.github/workflows/ghcr-deploy.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/ghcr-deploy.yml) [Dockerfile](https://github.com/amfoss/root/blob/2b58803d/Dockerfile)

### Binary Releases

When a new version tag (starting with "v") is pushed, the generate-release workflow:

1. Builds platform-specific binaries for multiple targets:
   * Linux (x86\_64)
   * Windows (x86\_64)
   * macOS (aarch64)
2. Strips binary files (except for Windows) to reduce size
3. Creates compressed archives of the binaries
4. Uploads these archives as assets to a GitHub Release
5. Generates release notes automatically

Sources: [.github/workflows/generate-release.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/generate-release.yml)

## Deployment Configuration

### Environment Variables

The application requires several environment variables for proper deployment, which should be configured in the production environment:

| Variable | Description | Example |
| --- | --- | --- |
| `POSTGRES_PASSWORD` | Database password | (secure password) |
| `POSTGRES_USER` | Database username | root\_user |
| `POSTGRES_DB` | Database name | root\_db |
| `POSTGRES_HOST` | Database host | postgres |
| `DATABASE_URL` | Full database connection string | postgresql://user:pass@host:5432/dbname |
| `RUST_ENV` | Application environment | production |
| `ROOT_SECRET` | Secret key for HMAC verification | (secure random string) |
| `ROOT_PORT` | Port for the application to listen on | 3000 |

Sources: [.env.sample](https://github.com/amfoss/root/blob/2b58803d/.env.sample)

### Docker Deployment

When deploying the application using Docker, you can pull the latest image from GitHub Container Registry:

Run the container with the required environment variables:

For production use, it's recommended to use Docker Compose or Kubernetes to manage the application with its database.

Sources: [.github/workflows/ghcr-deploy.yml](https://github.com/amfoss/root/blob/2b58803d/.github/workflows/ghcr-deploy.yml) [Dockerfile](https://github.com/amfoss/root/blob/2b58803d/Dockerfile) [.env.sample](https://github.com/amfoss/root/blob/2b58803d/.env.sample)

## Conclusion

The Root application employs a modern CI/CD pipeline that automates code quality checks, dependency updates, and deployment processes. The system supports both container-based deployment using Docker and GitHub Container Registry, as well as direct binary deployment for multiple platforms through GitHub Releases.

This automation ensures that:

1. Code quality is maintained through automated linting and formatting checks
2. Dependencies are kept up-to-date with automated PRs
3. Deployment is streamlined and consistent
4. Multiple deployment options are available based on operational requirements