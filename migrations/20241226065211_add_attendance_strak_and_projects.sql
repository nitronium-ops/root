CREATE TABLE AttendanceStreak (
    id SERIAL PRIMARY KEY,
    member_id INT NOT NULL,
    month DATE,
    streak INT NOT NULL DEFAULT 0,
    CONSTRAINT fkey_member FOREIGN KEY (member_id) REFERENCES Member(id) ON DELETE CASCADE
);

CREATE TABLE ActiveProjects (
    id SERIAL PRIMARY KEY,
    member_id INT NOT NULL,
    project_title TEXT,
    CONSTRAINT fkey_member FOREIGN KEY (member_id) REFERENCES Member(id) ON DELETE CASCADE
);
