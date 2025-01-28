-- Creates member, attendance, attendance_summary and streaks tables

-- Custom type for sex
CREATE TYPE sex_type AS ENUM ('M', 'F', 'Other');

CREATE TABLE Member (
        member_id SERIAL PRIMARY KEY,
        roll_no VARCHAR(16) NOT NULL UNIQUE,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        sex sex_type NOT NULL,
        year INT  NOT NULL,
        hostel VARCHAR(255) NOT NULL,
        mac_address VARCHAR(255) NOT NULL UNIQUE,
        discord_id VARCHAR(255) NOT NULL UNIQUE,
        group_id INT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CHECK (year BETWEEN 1 and 4)
);

CREATE TABLE Attendance (
        attendance_id SERIAL PRIMARY KEY,
        member_id INT REFERENCES Member(member_id) ON DELETE CASCADE,
        date DATE NOT NULL,
        is_present BOOLEAN NOT NULL DEFAULT FALSE,
        time_in TIME,
        time_out TIME,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CHECK (
                (is_present = TRUE AND time_in IS NOT NULL AND time_out is NOT NULL) OR
                (is_present = FALSE AND time_in IS NULL AND time_out IS NULL)
        ),
        CHECK (is_present = FALSE OR date <= CURRENT_DATE),
        CHECK (time_out IS NULL OR time_out >= time_in),
        UNIQUE (member_id, date)
);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS 
$$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_attendance_timestamp
BEFORE UPDATE ON Attendance
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TABLE AttendanceSummary (
        member_id INT REFERENCES Member(member_id) ON DELETE CASCADE,
        year INT NOT NULL,
        month INT NOT NULL,
        days_attended INT NOT NULL DEFAULT 0,
        primary key (member_id, year, month)
);

CREATE TABLE StatusUpdateStreak (
        member_id INT REFERENCES Member(member_id) ON DELETE CASCADE,
        current_streak int NOT NULL DEFAULT 0,
        max_streak INT NOT NULL,
        PRIMARY KEY (member_id)
);
