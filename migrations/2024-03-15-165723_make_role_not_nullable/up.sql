-- Set a default role for users currently with a NULL role to 'Member'
UPDATE users SET role = 'Member' WHERE role IS NULL;

-- Alter the column to be NOT NULL
ALTER TABLE users ALTER COLUMN role SET NOT NULL;
