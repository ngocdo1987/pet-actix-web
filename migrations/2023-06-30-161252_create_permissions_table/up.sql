-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE "permission" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    guard_name TEXT NOT NULL,
    created_at TIMESTAMP   NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);
