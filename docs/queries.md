## GraphQL Queries

## Contents
- [getMember](#getmember)
- [getAttendance](#getattendance)
- [getAttendanceStreak](#getattendancestreak)
- [getAttendanceSummary](#getattendancesummary)
- [getNonWorkingDays](#getnonworkingdays)
- [getProjects](#getprojects)
- [getUpdateStreak](#getupdatestreak)

---

### getMember
Retrieve all the members from the database.

#### GraphQL Query
```graphql
query {
    getMember {
        id
        name
        email
    }
}
```

#### Fields
```graphql
id: Int!
rollno: String!
name: String!
hostel: String!
email: String!
sex: String!
year: Int!
macaddress: String!
discordId: String!
```

---

### getAttendance
Retrieve attendance records for a specific date.

#### GraphQL Query
```graphql
query {
    getAttendance(date: "YYYY-MM-DD") {
        id
        date
        timein
        timeout
        isPresent
    }
}
```

#### Arguments
- `date` (required): A date in the format `YYYY-MM-DD`.

#### Fields
```graphql
id: Int!
date: NaiveDate!
timein: NaiveTime!
timeout: NaiveTime!
isPresent: Boolean!
```

---

### getAttendanceStreak
Retrieve attendance streak between date ranges.

#### GraphQL Query
```graphql
query {
    getAttendanceStreak(startDate:"YYYY-MM-DD",endDate:"YYYY-MM-DD"){
        id
        memberId
        month
        streak
    }
}
```

#### Arguments
- `startDate` (required): A date in the format `YYYY-MM-DD`.
- `endDate` (required): A date in the format `YYYY-MM-DD`.

#### Fields
```graphql
id: Int!
memberId: Int!
month: NaiveDate!
streak: Int!
```

---

### getAttendanceSummary
Retrieve attendance summary between date ranges.

#### GraphQL Query
```graphql
query {
    getAttendanceSummary(startDate:"YYYY-MM-DD",endDate:"YYYY-MM-DD") {
        maxDays
        memberAttendance {
            id
            presentDays
        }
        dailyCount {
            date
            count
        }
    }
}
```

#### Arguments
- `startDate` (required): A date in the format `YYYY-MM-DD`.
- `endDate` (required): A date in the format `YYYY-MM-DD`.

#### Fields
Type: AttendanceSummary!
```graphql
maxDays: Int!
memberAttendance: [MemberAttendance!]!
dailyCount: [DailyCount!]!
```

Type: MemberAttendance!
```graphql
id: Int!
presentDays: Int!
```

Type: DailyCount!
```graphql
date: NaiveDate!
count: Int!
```

---

### getNonWorkingDays
Retrieve Non Working Days from root.

#### GraphQL Query
```graphql
query {
    getNonWorkingDays
}
```

#### Fields
```graphql
[NaiveDate!]!
```

---

### getProjects
Retrieve active project details for all members.

#### GraphQL Query
```graphql
query {
    getProjects {
        id
        memberId
        projectTitle
    }
}
```

#### Fields
```graphql
id: Int!
memberId: Int!
projectTitle: String
```

---

### getUpdateStreak
Retrieve Update streaks for all members.

#### GraphQL Query
```graphql
query {
    getUpdateStreak {
        id
        streak
        maxStreak
    }
}
```

#### Fields
```graphql
id: Int!
streak: Int
maxStreak: Int
```