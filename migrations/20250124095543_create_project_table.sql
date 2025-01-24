CREATE TABLE Project (
    project_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL,
    title TEXT,
    CONSTRAINT fkey_member FOREIGN KEY (member_id) REFERENCES Member(member_id) ON DELETE CASCADE
);
