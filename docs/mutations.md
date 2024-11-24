## GraphQL Mutations

## Contents
- [addMember](#addmember)
- [editMember](#editmember)
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

### editMember
Edit details of an existing member.

#### GraphQL Mutation
```graphql
mutation {
    editMember(id:0,hostel:"hostel",year:2,macaddress:"mac_address",discordId:"discord_id",hmacSignature:"hmac_signature") {
    id
    hostel
    discord_id
    }
}
```

#### Arguments (all required)
```graphql
id: Int!
hostel: String!
year: Int!
macaddress: String!
discordId: String!
hmacSignature: String!
```

**Note:** Follow the below format if you want to leave some fields unchanged
- `''` (*empty string*) for type `String`
    - Feilds: `hostel,macaddress,discordId`
- `0` for type `Int`
    - Feilds: `year`

For example, If you want to update only `discordId`:

```graphql
mutation {
    editMember(id:1,hostel:"",year:0,macaddress:"",discordId:"discord_id",hmacSignature:"hmac_signature") {
        id
        hostel
        discord_id
    }
}
```

Note: `id` and `hmacSignature` can't be empty

#### HMAC Format

```
"{secret_key}{id}{hostel}{year}{macaddress}{discord_id}"
```

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