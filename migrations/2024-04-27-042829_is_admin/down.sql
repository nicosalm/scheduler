-- This file should undo anything in `up.sql`
CREATE TYPE UserRole AS ENUM ('Admin', 'Member');
ALTER TABLE users ADD COLUMN role UserRole;
ALTER TABLE users DROP COLUMN is_admin;
