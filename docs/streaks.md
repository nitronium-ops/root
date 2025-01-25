# Status Update Streaks

## Overview
Track members' daily status update streaks and records.

## Models

### StatusUpdateStreak
```rust
struct StatusUpdateStreak {
    member_id: i32,
    current_streak: i32,
    max_streak: i32,
}
```

## Queries

### Get Streak
```graphql
query {
    getUpdateStreak(memberId: 1) {
        currentStreak
        maxStreak
    }
}
```

## Mutations

### Increment Streak
```graphql
mutation {
    incrementStreak(
        input: {
            memberId: 1
        }
    ) {
        currentStreak
        maxStreak
    }
}
```