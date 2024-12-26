ALTER TABLE AttendanceStreak ADD CONSTRAINT unique_member_month UNIQUE (member_id, month);
