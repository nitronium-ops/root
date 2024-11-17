## GraphQL Queries

## Contents
- [getMember](#getmember)
- [getAttendance](#getattendance)

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
