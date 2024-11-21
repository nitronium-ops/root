## GraphQL Mutations

## Contents
- [addMember](#addmember)
- [markAttendance](#markattendance)
- [addAttendance](#addattendance)

---

### addMember
Add a new member to the database.

#### GraphQL Mutation
```graphql
mutation {
    addMember(name: "name", email: "email", year: 2) {
        id
        name
        year
    }
}
```

#### Arguments (all required)
```graphql
rollno: String!
name: String!
hostel: String!
email: String!
sex: String!
year: Int!
macaddress: String!
discordId: String
```

**Note:** The `year` field represents the members's current college year
- `1` for first-year
- `2` for second-year
- `3` for third-year
- `4` for fourth-year

---

### markAttendance
Record attendance for a member.

#### GraphQL Mutation  
```graphql
mutation {
    markAttendance(id: 0, date: "YYYY-MM-DD", isPresent: true, hmacSignature: "hmac_signature") {
        id
        date
        isPresent
        hmacSignature
    }
}
```

#### Arguments (all required)
```graphql
id: Int!
date: NaiveDate!
isPresent: Boolean!
hmacSignature: String!
```

---

### addAttendance
Initiate attendance records for the day (*used internally*).

#### GraphQL Mutation
```graphql
mutation {
    addAttendance(id: 0, date: "YYYY-MM-DD", timein: "%H:%M:%S%.f", timeout: "%H:%M:%S%.f", isPresent:true) {
        id
        date
        isPresent
        hmacSignature
    }
}
```

#### Arguments (all required)
```graphql
id: Int!
date: NaiveDate!
timein: NaiveTime!
timeout: NaiveTime!
isPresent: Boolean!
```