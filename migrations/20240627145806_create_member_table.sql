-- migrations/YYYYMMDDHHMMSS_create_member_table.sql

CREATE TABLE Member (
    id SERIAL PRIMARY KEY,
    rollno VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    hostel VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    sex VARCHAR(10) NOT NULL,
    year INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE Attendance (
    id INT REFERENCES Member(id),
    date DATE NOT NULL,
    timein TIME NOT NULL,
    timeout TIME NOT NULL,
    PRIMARY KEY (id, date)
);

