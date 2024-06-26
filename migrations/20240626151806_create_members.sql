-- migrations/YYYYMMDDHHMMSS_create_member_table.sql
CREATE TABLE Member (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);
