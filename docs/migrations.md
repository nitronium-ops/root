## Managing Migrations

## Contents
- [Create Migrations](#create-migrations)
- [Run Migrations](#run-migrations)

---

### Prerequisites
1. Install `sql-cli`
    ```bash
    cargo install sqlx-cli
    ```

### Create Migrations

Run the following command to create migrations

```bash
sqlx migrate add <migration_name>
```

- Replace `<migration_name>` with a descriptive name for your migration, e.g.,`create_users_table`.
- This will add a new migration file to the `migrations` directory.

---

### Run Migrations

Run the following command to apply migrations to your local database

```bash
sqlx migrate run
```

---

### Notes
- Look for logs confirming the migration ran successfully.
- Never rename or alter migration files as their names and contents are used to track applied changes.
