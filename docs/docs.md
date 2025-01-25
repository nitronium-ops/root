# Root Documentation

## Project Structure
```
src/
├── graphql/        # GraphQL schema definitions
│   ├── mutations/  # Data modification operations
│   └── queries/    # Data retrieval operations
├── models/         # Database models and types
├── daily_task/     # Self explanatory
└── routes.rs       # HTTP routing setup
```

## GraphQL API Structure
- [Member Management](member.md) - Managing club member profiles
- [Attendance System](attendance.md) - Daily attendance tracking and summaries  
- [Status Streaks](streaks.md) - Tracking daily status update streaks

## Database Schema
- [Database](database.md) - Database structure and migrations

## Core Features
### Member Management
- Query members by ID, roll number, or Discord ID
- Create and update member profiles

### Attendance System  
- Mark daily attendance with time tracking
- Generate monthly attendance summaries

### Status Updates
- Track daily status update streaks
- Record maximum streaks achieved
