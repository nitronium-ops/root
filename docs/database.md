# Database Migrations

## Overview
Database schema is managed using SQLx migrations. Do not tamper with the database yourself (using clients such as psql) or with the migrations files.

## Current Schema

The entire schema, including constraints, is visible in `migrations/`.

### Member Table
```sql
CREATE TABLE Member (
    member_id SERIAL PRIMARY KEY,
    roll_no VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    sex sex_type NOT NULL,
    year INT NOT NULL,
    hostel VARCHAR NOT NULL,
    mac_address VARCHAR NOT NULL,
    discord_id VARCHAR NOT NULL,
    group_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL
);
```

### Attendance Table
```sql
CREATE TABLE Attendance (
    attendance_id SERIAL PRIMARY KEY,
    member_id INT REFERENCES Member(member_id),
    date DATE NOT NULL,
    is_present BOOLEAN NOT NULL,
    time_in TIME,
    time_out TIME,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
```

### AttendanceSummary Table
```sql
CREATE TABLE AttendanceSummary (
    member_id INT REFERENCES Member(member_id),
    year INT NOT NULL,
    month INT NOT NULL,
    days_attended INT NOT NULL DEFAULT 0,
    PRIMARY KEY (member_id, year, month)
);
```

### StatusUpdateStreak Table
```sql
CREATE TABLE StatusUpdateStreak (
    member_id INT REFERENCES Member(member_id),
    current_streak INT NOT NULL DEFAULT 0,
    max_streak INT NOT NULL,
    PRIMARY KEY (member_id)
);
```

## Managing Migrations

### Create Migration
```bash
sqlx migrate add <migration_name>
```

### Run Migrations
```bash
sqlx migrate run
```

### Revert Migration
```bash
sqlx migrate revert
```
